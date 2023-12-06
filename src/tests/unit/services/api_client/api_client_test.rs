#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{info, debug, error};
    use rand::Rng;
    use std::{sync::{Once, Arc, Mutex}, thread, time::{Duration, Instant}, net::TcpListener, io::{Read, Write}};
    use crate::{
        core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, point::point_type::ToPoint, testing::{test_session::TestSession, test_stuff::test_value::Value}},
        conf::api_client_config::ApiClientConfig,  
        services::{api_cient::{api_client::ApiClient, api_reply::SqlReply, api_error::ApiError}, service::Service},
    }; 
    
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
    
    
    #[test]
    fn test_ApiClient() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_ApiClient");
        let mut rnd = rand::thread_rng();
        let path = "./src/tests/unit/services/api_client/api_client.yaml";
        let mut conf = ApiClientConfig::read(path);
        // let addr = conf.address.clone();
        let addr = "127.0.0.1:".to_owned() + &TestSession::freeTcpPortStr();
        conf.address = addr.parse().unwrap();

        let mut apiClient = ApiClient::new("test ApiClient", conf);

        let maxTestDuration = Duration::from_secs(10);
        let count = 300;
        let mut state = 0;
        let testData = vec![
            Value::Int(7),
            Value::Float(1.3),
            Value::Bool(true),
            Value::Bool(false),
            Value::String("test1".to_string()),
            Value::String("test2".to_string()),
        ];
        let testDataLen = testData.len();

        let mut sent = vec![];
        let received = Arc::new(Mutex::new(vec![]));
        let receivedRef = received.clone();
        let mut buf = [0; 1024 * 4];

        thread::spawn(move || {
            let mut received = receivedRef.lock().unwrap();
            info!("TCP server | Preparing test server...");
            match TcpListener::bind(addr) {
                Ok(listener) => {
                    info!("TCP server | Preparing test server - ok");
                    let mut acceptCount = 2;
                    let mut maxReadErrors = 3;
                    while acceptCount > 0 {
                        acceptCount -= 1;
                        match listener.accept() {
                            Ok((mut _socket, addr)) => {
                                info!("TCP server | accept connection - ok\n\t{:?}", addr);
                                _socket.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
                                while received.len() < count {
                                    for e in buf.iter_mut() {*e = 0;}
                                    match _socket.read(&mut buf) {
                                        Ok(bytes) => {
                                            debug!("TCP server | received bytes: {:?}", bytes);
                                            let raw = String::from_utf8(buf.to_vec()).unwrap();
                                            let raw = raw.trim_matches(char::from(0));
                                            debug!("TCP server | received raw: {:?}", raw);
                                            match serde_json::from_str(&raw) {
                                                Ok(value) => {
                                                    let value: serde_json::Value = value;
                                                    debug!("TCP server | received: {:?}", value);
                                                    received.push(value.clone());
                                                    let obj = value.as_object().unwrap();
                                                    let reply = SqlReply {
                                                        authToken: obj.get("authToken").unwrap().as_str().unwrap().to_string(),
                                                        id: obj.get("id").unwrap().as_str().unwrap().to_string(),
                                                        keepAlive: obj.get("keepAlive").unwrap().as_bool().unwrap(),
                                                        query: "".into(),
                                                        data: vec![],
                                                        error: ApiError::empty(),
                                                    };
                                                    match _socket.write(&reply.asBytes()) {
                                                        Ok(bytes) => {
                                                            debug!("TCP server | sent bytes: {:?}", bytes);
                                                        },
                                                        Err(err) => {
                                                            debug!("TCP server | socket write - error: {:?}", err);
                                                        },
                                                    };
                                                    // debug!("TCP server | received / count: {:?}", received.len() / count);
                                                    if (state == 0) && received.len() as f64 / count as f64 > 0.333 {
                                                        state = 1;
                                                        let duration = Duration::from_millis(500);
                                                        debug!("TCP server | beaking socket connection for {:?}", duration);
                                                        _socket.flush().unwrap();
                                                        _socket.shutdown(std::net::Shutdown::Both).unwrap();
                                                        thread::sleep(duration);
                                                        debug!("TCP server | beaking socket connection for {:?} - elapsed, restoring...", duration);
                                                        break;
                                                    }
                                                },
                                                Err(err) => {
                                                    debug!("TCP server | parse read data error: {:?}", err);
                                                },
                                            };
                                        },
                                        Err(err) => {
                                            debug!("socket read - error: {:?}", err);
                                            maxReadErrors -= 1;
                                            if maxReadErrors <= 0 {
                                                error!("TCP server | socket read error: {:?}", err);
                                                break;
                                            }
                                        },
                                    };
                                    thread::sleep(Duration::from_micros(100));
                                }
                            },
                            Err(err) => {
                                info!("incoming connection - error: {:?}", err);
                            },
                        }
                    }
                },
                Err(err) => {
                    // connectExit.send(true).unwrap();
                    // okRef.store(false, Ordering::SeqCst);
                    panic!("Preparing test TCP server - error: {:?}", err);
                },
            };
        });



        apiClient.run();
        let timer = Instant::now();
        let send = apiClient.getLink("api-link");
        for _ in 0..count {
            let index = rnd.gen_range(0..testDataLen);
            let value = testData.get(index).unwrap();
            let point = format!("select from table where id = {}", value.toString()).toPoint(0, "teset");
            send.send(point.clone()).unwrap();
            sent.push(point.asString().value);
        }
        let waitDuration = Duration::from_millis(10);
        let mut waitAttempts = maxTestDuration.as_micros() / waitDuration.as_micros();
        while received.lock().unwrap().len() < count {
            debug!("waiting while all data beeng received {}/{}...", received.lock().unwrap().len(), count);
            thread::sleep(waitDuration);
            waitAttempts -= 1;
            assert!(waitAttempts > 0, "Transfering {}/{} points taks too mach time {:?} of {:?}", received.lock().unwrap().len(), count, timer.elapsed(), maxTestDuration);
        }
        println!("elapsed: {:?}", timer.elapsed());
        println!("total test events: {:?}", count);
        println!("sent events: {:?}", sent.len());
        let mut received = received.lock().unwrap();
        println!("recv events: {:?}", received.len());
        assert!(sent.len() == count, "sent: {:?}\ntarget: {:?}", sent.len(), count);
        assert!(received.len() == count, "received: {:?}\ntarget: {:?}", received.len(), count);
        while &sent.len() > &0 {
            let target = sent.pop().unwrap();
            let result = received.pop().unwrap();
            let result = result.as_object().unwrap().get("sql").unwrap().as_object().unwrap().get("sql").unwrap().as_str().unwrap();
            debug!("\nresult({}): {:?}\ntarget({}): {:?}", received.len(), result, sent.len(), target);
            assert!(result == &target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
    }
}