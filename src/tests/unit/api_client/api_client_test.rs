#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{info, debug};
    use rand::Rng;
    use std::{sync::{Once, Arc, Mutex}, thread, time::{Duration, Instant}, net::TcpListener, io::{Read, Write}};
    use crate::{core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, conf::api_client_config::ApiClientConfig, point::point_type::{ToPoint, PointType}}, services::api_cient::api_client::ApiClient}; 
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
    
    static INIT: Once = Once::new();
    
    ///
    /// once called initialisation
    fn initOnce() {
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
            }
        )
    }
    
    
    ///
    /// returns:
    ///  - ...
    fn initEach() -> () {
    
    }
    
    enum Value {
        Bool(bool),
        Int(i64),
        Float(f64),
        String(String),
    }
    impl Value {
        fn toString(&self) -> String {
            match &self {
                Value::Bool(v) => v.to_string(),
                Value::Int(v) => v.to_string(),
                Value::Float(v) => v.to_string(),
                Value::String(v) => v.to_string(),
            }
        }
    }
    
    #[test]
    fn test_ApiClient() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_ApiClient");
        let mut rnd = rand::thread_rng();
        let path = "./src/tests/unit/api_client/api_client.yaml";
        let conf = ApiClientConfig::read(path);
        let addr = conf.address.clone();
        let apiClient = Arc::new(Mutex::new(ApiClient::new("test ApiClient", conf)));

        let count = 100;
        let testData = vec![
            Value::Int(7),
            Value::Float(1.3),
            Value::Bool(true),
            Value::Bool(false),
            Value::String("test1".to_owned()),
            Value::String("test2".to_owned()),
        ];
        let testDataLen = testData.len();

        let mut sent = vec![];
        let received = Arc::new(Mutex::new(vec![]));
        let receivedRef = received.clone();
        let mut buf = [0; 1024 * 4];

        thread::spawn(move || {
            let mut received = receivedRef.lock().unwrap();
            info!("Preparing test TCP server...");
            match TcpListener::bind(addr) {
                Ok(listener) => {
                    info!("Preparing test TCP server - ok");
                    match listener.accept() {
                        Ok((mut _socket, addr)) => {
                            info!("incoming connection - ok\n\t{:?}", addr);
                            while received.len() < count {
                                for e in buf.iter_mut() {*e = 0;}
                                match _socket.read(&mut buf) {
                                    Ok(bytes) => {
                                        debug!("received bytes: {:?}", bytes);
                                        let raw = String::from_utf8(buf.to_vec()).unwrap();
                                        let raw = raw.trim_matches(char::from(0));
                                        debug!("received raw: {:?}", raw);
                                        let value: serde_json::Value = serde_json::from_str(&raw).unwrap();
                                        debug!("received: {:?}", value);
                                        received.push(value);
                                        match _socket.write("ok".as_bytes()) {
                                            Ok(bytes) => {
                                                debug!("sent bytes: {:?}", bytes);
                                            },
                                            Err(err) => {
                                                debug!("socket write - error: {:?}", err);
                                            },
                                        };
                                        // _socket.shutdown(std::net::Shutdown::Both).unwrap();
                                        // break;
                                    },
                                    Err(err) => {
                                        debug!("socket read - error: {:?}", err);
                                    },
                                };
                            }
                        },
                        Err(err) => {
                            info!("incoming connection - error: {:?}", err);
                        },
                    }
                },
                Err(err) => {
                    // connectExit.send(true).unwrap();
                    // okRef.store(false, Ordering::SeqCst);
                    panic!("Preparing test TCP server - error: {:?}", err);
                },
            };
        });



        apiClient.lock().unwrap().run();
        let timer = Instant::now();
        let send = apiClient.lock().unwrap().getLink("api-link");
        for _ in 0..count {
            let index = rnd.gen_range(0..testDataLen);
            let value = testData.get(index).unwrap();
            let point = format!("select from table where id = {}", value.toString()).toPoint("teset");
            send.send(point.clone()).unwrap();
            sent.push(point.asString().value);
        }
        let mut waitAttempts = 100;
        while received.lock().unwrap().len() < sent.len() {
            thread::sleep(Duration::from_millis(10));
            waitAttempts -= 1;
            assert!(waitAttempts > 0, "Transfering {} points taks too mach time {:?}", count, timer.elapsed());
        }
        println!("elapsed: {:?}", timer.elapsed());
        let mut received = received.lock().unwrap();
        while &sent.len() > &0 {
            let target = sent.pop().unwrap();
            let result = received.pop().unwrap();
            let result = result.as_object().unwrap().get("sql").unwrap().as_object().unwrap().get("sql").unwrap().as_str().unwrap();
            debug!("\nresult: {:?}\ntarget: {:?}", result, target);
            assert!(result == &target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        // assert!(false)
        // assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);
    }
}