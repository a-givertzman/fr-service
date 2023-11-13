#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{info, debug};
    use std::{sync::{Once, Arc, Mutex}, thread, time::Duration, net::TcpListener, io::{Read, Write}};
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
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_ApiClient");
        let path = "./src/tests/unit/api_client/api_client.yaml";
        let conf = ApiClientConfig::read(path);
        let addr = conf.address.clone();
        let apiClient = Arc::new(Mutex::new(ApiClient::new("test ApiClient", conf)));

        let mut buf = [0; 1024 * 4];

        thread::spawn(move || {
            info!("Preparing test TCP server...");
            match TcpListener::bind(addr) {
                Ok(listener) => {
                    info!("Preparing test TCP server - ok");
                    match listener.accept() {
                        Ok((mut _socket, addr)) => {
                            info!("incoming connection - ok\n\t{:?}", addr);
                            loop {
                                match _socket.read(&mut buf) {
                                    Ok(bytes) => {
                                        debug!("received bytes: {:?}", bytes);
                                        let raw = String::from_utf8(buf.to_vec()).unwrap();
                                        let raw = raw.trim_matches(char::from(0));
                                        let value: serde_json::Value = serde_json::from_str(&raw).unwrap();
                                        debug!("received: {:?}", value);
                                        match _socket.write("ok".as_bytes()) {
                                            Ok(bytes) => {
                                                debug!("sent bytes: {:?}", bytes);
                                            },
                                            Err(err) => {
                                                debug!("socket write - error: {:?}", err);
                                            },
                                        };
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
        let send = apiClient.lock().unwrap().getLink("api-link");
        let testData = vec![
            Value::Int(0),
            Value::Float(0.0),
            Value::Bool(true),
            Value::Bool(false),
            Value::String("test1".to_owned()),
            Value::String("test2".to_owned()),
        ];
        for value in testData {
            let point = format!("select from table where id = {}", value.toString()).toPoint("teset");
            send.send(point).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
        thread::sleep(Duration::from_millis(3000));
        // assert!(false)
        // assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);
    }
}