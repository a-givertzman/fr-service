#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{info, debug, error};
    use std::{sync::{Once, Arc, Mutex}, thread, time::{Duration, Instant}, net::TcpListener, io::{Read, Write}};
    use testing::{entities::test_value::Value, session::test_session::TestSession, stuff::{max_test_duration::TestDuration, random_test_values::RandomTestValues}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use api_tools::{error::api_error::ApiError, reply::api_reply::SqlReply};
    use crate::{
        core_::point::point_type::ToPoint,
        conf::api_client_config::ApiClientConfig,  
        services::{api_cient::api_client::ApiClient, service::Service},
    }; 
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
    
    static INIT: Once = Once::new();
    
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
            }
        )
    }
    
    
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {
    
    }
    
    
    #[test]
    fn test_ApiClient() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test ApiClient";
        println!("{}", self_id);
        let path = "./src/tests/unit/services/api_client/api_client.yaml";
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let mut conf = ApiClientConfig::read(path);
        // let addr = conf.address.clone();
        let addr = "127.0.0.1:".to_owned() + &TestSession::free_tcp_port_str();
        conf.address = addr.parse().unwrap();

        let mut apiClient = ApiClient::new("test ApiClient", conf);

        // let test_duration = Duration::from_secs(10);
        let count = 300;
        let mut state = 0;
        let test_data = RandomTestValues::new(
            self_id, 
            vec![
                Value::Int(i64::MIN),
                Value::Int(i64::MAX),
                Value::Int(-7),
                Value::Int(0),
                Value::Int(12),
                Value::Float(f64::MAX),
                Value::Float(f64::MIN),
                Value::Float(f64::MIN_POSITIVE),
                Value::Float(-f64::MIN_POSITIVE),
                Value::Float(0.0),
                Value::Float(1.33),
                Value::Bool(true),
                Value::Bool(false),
                Value::Bool(false),
                Value::Bool(true),
                Value::String("test1".to_string()),
                Value::String("test1test1test1test1test1test1test1test1test1test1test1test1test1test1test1".to_string()),
                Value::String("test2".to_string()),
                Value::String("test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2".to_string()),
            ], 
            count, 
        );
        let test_data: Vec<Value> = test_data.collect();

        let mut sent = vec![];
        let received = Arc::new(Mutex::new(vec![]));
        let receivedRef = received.clone();
        let mut buf = [0; 1024 * 4];

        let receiverHandle = thread::spawn(move || {
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



        apiClient.run().unwrap();
        let timer = Instant::now();
        let send = apiClient.get_link("api-link");
        for value in test_data {
            let point = format!("select from table where id = {}", value.to_string()).to_point(0, "teset");
            send.send(point.clone()).unwrap();
            sent.push(point.as_string().value);
        }
        receiverHandle.join().unwrap();
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
        test_duration.exit();
    }
}