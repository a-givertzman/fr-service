#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use chrono::{DateTime, Utc};
    use log::{info, debug, trace, error};
    use rand::Rng;
    use std::{sync::{Once, atomic::{AtomicUsize, Ordering}, Arc}, time::{Duration, Instant}, net::{TcpStream, TcpListener}, thread, io::{Write, BufReader}};
    use testing::session::test_session::TestSession;
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use crate::core_::{
        cot::cot::Cot, net::{connection_status::ConnectionStatus, protocols::jds::{jds_decode_message::JdsDecodeMessage, jds_deserialize::JdsDeserialize}}, point::{point::Point, point_type::PointType}, status::status::Status, types::bool::Bool 
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
    
    fn ts() -> DateTime<Utc> {
        chrono::offset::Utc::now()
    }
    fn tsStr(ts: DateTime<Utc>) -> String {
        ts.to_rfc3339()
    }

    #[test]
    fn test_jds_deserialize() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        println!();
        println!("test JdsDeserialize");
        let name = "/server/line1/ied1/test1";
        let ts = ts();
        let txId = 0;
        // debug!("timestamp: {:?}", ts);j
        let test_data = [
            (
                format!(r#"{{"id": "1", "type": "Bool",  "name": "{}", "value": false,   "status": 0, "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Bool(Point::new(txId, name, Bool(false), Status::Ok, Cot::default(), ts))
            ),
            (
                format!(r#"{{"id": "1", "type": "Bool",  "name": "{}", "value": true,    "status": 0, "cot": "read", "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Bool(Point::new(txId, name, Bool(true), Status::Ok, Cot::default(), ts))
            ),
            (
                format!(r#"{{"id": "1", "type": "Int",   "name": "{}", "value": 1,   "status": 0, "cot": "Inf", "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Int(Point::new(txId, name, 1, Status::Ok, Cot::Inf, ts))
            ),
            (
                format!(r#"{{"id": "1", "type": "Int",   "name": "{}", "value": -9223372036854775808,   "status": 0, "cot": "Act", "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Int(Point::new(txId, name, -9223372036854775808, Status::Ok, Cot::Act, ts))
            ),
            (
                format!(r#"{{"id": "1", "type": "Int",   "name": "{}", "value":  9223372036854775807,   "status": 0, "cot": "act", "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Int(Point::new(txId, name,  9223372036854775807, Status::Ok, Cot::Act, ts))
            ),
            (
                format!(r#"{{"id": "1", "type": "Float", "name": "{}", "value":  0.0, "status": 0, "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Float(Point::new(txId, name,  0.0, Status::Ok, Cot::default(), ts))
            ),
            (
                format!(r#"{{"id": "1", "type": "Float", "name": "{}", "value": -1.1, "status": 0, "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Float(Point::new(txId, name, -1.1, Status::Ok, Cot::default(), ts))
            ),
            (
                format!(r#"{{"id": "1", "type": "Float", "name": "{}", "value":  1.1, "status": 0, "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Float(Point::new(txId, name,  1.1, Status::Ok, Cot::default(), ts))
            ),
            (
                format!(r#"{{"id": "1", "type": "Float", "name": "{}", "value": -1.7976931348623157e308, "status": 0, "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Float(Point::new(txId, name, -1.7976931348623157e308, Status::Ok, Cot::default(), ts))
            ),
            (
                format!(r#"{{"id": "1", "type": "Float", "name": "{}", "value":  1.7976931348623157e308, "status": 0, "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Float(Point::new(txId, name,  1.7976931348623157e308, Status::Ok, Cot::default(), ts))
            ),
            (
                format!(r#"{{"id": "1", "type": "String","name": "{}", "value": "~!@#$%^&*()_+`1234567890-=","status": 0, "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::String(Point::new(txId, name, "~!@#$%^&*()_+`1234567890-=".to_string(), Status::Ok, Cot::default(), ts))
            ),
        ];
        //
        //
        let addr = "127.0.0.1:".to_owned() + &TestSession::free_tcp_port_str();
        let received = Arc::new(AtomicUsize::new(0));
        let count = 1000;
        let test_dataLen = test_data.len();
        let total = count * test_dataLen;
        mockTcpServer(addr.to_string(), count, test_data.clone(), received.clone());
        thread::sleep(Duration::from_micros(100));
        {
            println!("\nReading from stream.read(byte)...");
            let time = Instant::now();
            'main: loop {
                match TcpStream::connect(&addr) {
                    Ok(tcpStream) => {
                        let mut tcpStream = BufReader::new(tcpStream);
                        let mut stream = JdsDeserialize::new(
                            "test", 
                            JdsDecodeMessage::new("test")
                        );
                        'read: loop {
                            match stream.read(&mut tcpStream) {
                                ConnectionStatus::Active(point) => {
                                    match point {
                                        Ok(point) => {
                                            received.fetch_add(1, Ordering::SeqCst);
                                            let recvIndex = (received.load(Ordering::SeqCst) - 1) % test_dataLen;
                                            trace!("socket read - received[{}]: {:?}", recvIndex, point);
                                            assert!(point.name() == test_data[recvIndex].1.name(), "\nreceived: {:?}\nexpected: {:?}", point.name(), test_data[recvIndex].1.name());
                                            assert!(point.status() == test_data[recvIndex].1.status(), "\nreceived: {:?}\nexpected: {:?}", point.status(), test_data[recvIndex].1.status());
                                            assert!(point.cot() == test_data[recvIndex].1.cot(), "\nreceived: {:?}\nexpected: {:?}", point.cot(), test_data[recvIndex].1.cot());
                                            assert!(point.timestamp() == test_data[recvIndex].1.timestamp(), "\nreceived: {:?}\nexpected: {:?}", point.timestamp(), test_data[recvIndex].1.timestamp());
                                            match point {
                                                PointType::Bool(point) => assert!(point.value == test_data[recvIndex].1.as_bool().value, "\nreceived: {:?}\nexpected: {:?}", point.value, test_data[recvIndex].1.as_bool().value),
                                                PointType::Int(point) => assert!(point.value == test_data[recvIndex].1.as_int().value, "\nreceived: {:?}\nexpected: {:?}", point.value, test_data[recvIndex].1.as_int().value),
                                                PointType::Float(point) => assert!(point.value == test_data[recvIndex].1.as_float().value, "\nreceived: {:?}\nexpected: {:?}", point.value, test_data[recvIndex].1.as_float().value),
                                                PointType::String(point) => assert!(point.value == test_data[recvIndex].1.as_string().value, "\nreceived: {:?}\nexpected: {:?}", point.value, test_data[recvIndex].1.as_string().value),
                                            }
                                            // debug!("socket read - received: {:?}", received.load(Ordering::SeqCst));
                                            if received.load(Ordering::SeqCst) >= total {
                                                break 'read;
                                            }
                                        },
                                        Err(err) => {
                                            panic!("socket read - parsing error: {:?}", err);
                                        },
                                    }
                                },
                                ConnectionStatus::Closed(_err) => {
                                    break 'read;
                                },
                            }
                        }
                        println!("elapsed: {:?}", time.elapsed());
                        println!("received: {:?}", received.load(Ordering::SeqCst));
                        // println!("buffer: {:?}", buffer);
                        break 'main;
                    },
                    Err(_) => {},
                };
            }
        }
    }
    ///
    /// TcpServer setup
    fn mockTcpServer(addr: String, count: usize, test_data: [(String, PointType); 11], received: Arc<AtomicUsize>) {
        let mut sent = 0;
        thread::spawn(move || {
            info!("TCP server | Preparing test server...");
            let mut rng = rand::thread_rng();
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
                                let EOT = [4];
                                for _ in 0..count {
                                    for (msg, _) in &test_data {
                                        let pos: usize = rng.gen_range(5..(msg.len() - 5));
                                        // for e in buf.iter_mut() {*e = 0;}
                                        let (msg1, msg2) = msg.split_at(pos);
                                        let bytes1 = msg1.as_bytes();
                                        let bytes2 = msg2.as_bytes();

                                        match _socket.write(bytes1) {
                                            Ok(_bytes) => {
                                                sent += 1;
                                                trace!("socket sent: {:?}", msg);
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
                                        _socket.flush().unwrap();
                                        match _socket.write(&[bytes2, &EOT].concat()) {
                                            Ok(_bytes) => {
                                                sent += 1;
                                                trace!("socket sent: {:?}", msg);
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
                                    }
                                }
                                info!("TCP server | all sent: {:?}", sent);
                                while received.load(Ordering::SeqCst) < count {
                                    thread::sleep(Duration::from_micros(10));
                                }
                                // while received.len() < count {}
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
    }
}

