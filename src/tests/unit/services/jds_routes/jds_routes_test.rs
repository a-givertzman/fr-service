#[cfg(test)]

mod jds_routes {
    use testing::{session::test_session::TestSession, stuff::{max_test_duration::TestDuration, wait::WaitTread}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use std::{io::Write, net::TcpStream, sync::{Arc, Mutex, Once}, thread, time::Duration};
    use crate::{
        conf::{multi_queue_config::MultiQueueConfig, point_config::{point_config::PointConfig, point_name::PointName}, tcp_server_config::TcpServerConfig}, 
        core_::{cot::cot::Cot, net::protocols::jds::{jds_define::JDS_END_OF_TRANSMISSION, request_kind::RequestKind}, point::{point::Point, point_tx_id::PointTxId, point_type::PointType}, status::status::Status}, 
        services::{multi_queue::multi_queue::MultiQueue, service::service::Service, services::Services, tcp_server::tcp_server::TcpServer}, 
        tests::unit::services::{multi_queue::mock_recv_service::MockRecvService, service::moc_service_points::MockServicePoints},
    }; 
    ///    
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
    fn init_each() -> () {}
    ///
    fn point_configs(parent: &str) -> Vec<PointConfig> {
        vec![
            PointConfig::from_yaml(parent, &serde_yaml::from_str(&format!(
                r#"{}:
                    type: String      # Bool / Int / Float / String / Json
                    comment: Auth request, contains token / pass string"#, 
                RequestKind::AUTH_SECRET
            )).unwrap()),
            PointConfig::from_yaml(parent, &serde_yaml::from_str(&format!(
                r#"{}:
                    type: String      # Bool / Int / Float / String / Json
                    comment: Auth request, contains SSH key"#, 
                RequestKind::AUTH_SSH,
            )).unwrap()),
            PointConfig::from_yaml(parent, &serde_yaml::from_str(&format!(
                r#"{}:
                    type: String      # Bool / Int / Float / String / Json
                    comment: Request all Ponts configurations"#, 
                RequestKind::POINTS,
            )).unwrap()),
            PointConfig::from_yaml(parent, &serde_yaml::from_str(&format!(
                r#"{}:
                    type: String      # Bool / Int / Float / String / Json
                    comment: Request to begin transmossion of all configured Points"#, 
                RequestKind::SUBSCRIBE,
            )).unwrap()),
        ]
    }
    ///
    #[test]
    fn reject() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(20));
        test_duration.run().unwrap();
        //
        // Configuring MultiQueue service 
        let services = Arc::new(Mutex::new(Services::new(self_id)));
        let conf = serde_yaml::from_str(r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                out queue: 
                    - MockRecvService.in-queue
        "#).unwrap();
        let mq_conf = MultiQueueConfig::from_yaml(&conf);
        let mq_service = Arc::new(Mutex::new(MultiQueue::new(self_id, mq_conf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mq_service.clone());
        //
        // Configuring TcpServer service 
        let tcp_port = TestSession::free_tcp_port_str();
        let tcp_server_addr = format!("127.0.0.1:{}", tcp_port);
        let conf = format!(r#"
            service TcpServer:
                cycle: 1 ms
                reconnect: 1 s  # default 3 s
                address: {}
                auth: none      # auth: none / auth-secret: pass: ... / auth-ssh: path: ...
                in queue link:
                    max-length: 10000
                out queue: MultiQueue.in-queue
        "#, tcp_server_addr);
        let conf = serde_yaml::from_str(&conf).unwrap();
        let conf = TcpServerConfig::from_yaml(&conf);
        let tcp_server = Arc::new(Mutex::new(TcpServer::new(self_id, conf, services.clone())));
        services.lock().unwrap().insert("TcpServer", tcp_server.clone());
        println!("{} | TcpServer - ready", self_id);
        //
        // Preparing test data
        let parent = self_id;
        let test_data = [
            PointType::String(Point::new(
                0, 
                &PointName::new(&parent, "Jds/Auth.Secret").full(),
                r#"{\"reply\": \"Auth.Ssh Reply\"}"#.to_string(), 
                Status::Ok, 
                Cot::Inf, 
                chrono::offset::Utc::now(),
            )),
            PointType::String(Point::new(
                0, 
                &PointName::new(&parent, "Jds/Auth.Secret").full(),
                r#"{\"reply\": \"Auth.Ssh Reply\"}"#.to_string(), 
                Status::Ok, 
                Cot::Act, 
                chrono::offset::Utc::now(),
            )),
            PointType::String(Point::new(
                0, 
                &PointName::new(&parent, "Jds/Auth.Secret").full(),
                r#"{\"reply\": \"Auth.Ssh Reply\"}"#.to_string(), 
                Status::Ok, 
                Cot::ActCon, 
                chrono::offset::Utc::now(),
            )),
            PointType::String(Point::new(
                0, 
                &PointName::new(&parent, "Jds/Auth.Secret").full(),
                r#"{\"reply\": \"Auth.Ssh Reply\"}"#.to_string(), 
                Status::Ok, 
                Cot::ActErr, 
                chrono::offset::Utc::now(),
            )),
            PointType::String(Point::new(
                0, 
                &PointName::new(&parent, "Jds/Auth.Secret").full(),
                r#"{\"reply\": \"Auth.Ssh Reply\"}"#.to_string(), 
                Status::Ok, 
                Cot::ReqCon, 
                chrono::offset::Utc::now(),
            )),
            PointType::String(Point::new(
                0, 
                &PointName::new(&parent, "Jds/Auth.Secret").full(),
                r#"{\"reply\": \"Auth.Ssh Reply\"}"#.to_string(), 
                Status::Ok, 
                Cot::ReqErr, 
                chrono::offset::Utc::now(),
            )),
        ];        
        let test_items_count = test_data.len();
        //
        // preparing MockServicePoints with the Vec<PontConfig>
        let service_points = Arc::new(Mutex::new(MockServicePoints::new(self_id, point_configs(self_id))));
        services.lock().unwrap().insert("MockServicePoints", service_points);
        //
        // Configuring Receiver
        let receiver = Arc::new(Mutex::new(MockRecvService::new(self_id, "in-queue", Some(test_items_count))));
        services.lock().unwrap().insert("MockRecvService", receiver.clone());
        println!("{} | MockRecvService - ready", self_id);
        println!("\n{} | All configurations - ok\n", self_id);
        //
        // Starting all services
        let receiver_handle = receiver.lock().unwrap().run().unwrap();
        let mq_service_handle = mq_service.lock().unwrap().run().unwrap();
        let tcp_server_handle = tcp_server.lock().unwrap().run().unwrap();
        println!("{} | All services - are executed", self_id);
        thread::sleep(Duration::from_millis(1000));
        //
        // Sending tcp test events / receiver must not receive anything before subscription activated
        println!("{} | Sending tcp test events - to be rejected (not authenticated)", self_id);
        let mut tcp_stream = TcpStream::connect(tcp_server_addr).unwrap();
        for request in test_data {
            let mut request = serde_json::to_vec(&request).unwrap();
            request.push(JDS_END_OF_TRANSMISSION);
            tcp_stream.write_all(&request).unwrap();
        }
        thread::sleep(Duration::from_millis(2000));
        receiver.lock().unwrap().exit();
        receiver_handle.wait().unwrap();
        let received = receiver.lock().unwrap().received();
        let result = received.lock().unwrap().len();
        assert!(result == 0, "All points must be rejected, but some of them passed: \nresult: {:?}\ntarget: {:?}", result, 0);
        receiver.lock().unwrap().exit();
        tcp_server.lock().unwrap().exit();
        mq_service.lock().unwrap().exit();
        //
        // Waiting while all services being finished
        mq_service_handle.wait().unwrap();
        tcp_server_handle.wait().unwrap();
        //
        // Reseting dureation timer
        test_duration.exit();
    }
    ///
    #[test]
    fn auth_secret() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(20));
        test_duration.run().unwrap();
        //
        // Configuring MultiQueue service 
        let services = Arc::new(Mutex::new(Services::new(self_id)));
        let conf = serde_yaml::from_str(r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                out queue: 
                    - MockRecvService.in-queue
        "#).unwrap();
        let mq_conf = MultiQueueConfig::from_yaml(&conf);
        let mq_service = Arc::new(Mutex::new(MultiQueue::new(self_id, mq_conf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mq_service.clone());
        //
        // Configuring TcpServer service 
        let tcp_port = TestSession::free_tcp_port_str();
        let tcp_server_addr = format!("127.0.0.1:{}", tcp_port);
        let conf = format!(r#"
            service TcpServer:
                cycle: 1 ms
                reconnect: 1 s  # default 3 s
                address: {}
                auth-secret: 123!@#qwe      # auth: none / auth-secret: pass: ... / auth-ssh: path: ...
                in queue link:
                    max-length: 10000
                out queue: MultiQueue.in-queue
        "#, tcp_server_addr);
        let conf = serde_yaml::from_str(&conf).unwrap();
        let conf = TcpServerConfig::from_yaml(&conf);
        let tcp_server = Arc::new(Mutex::new(TcpServer::new(self_id, conf, services.clone())));
        services.lock().unwrap().insert("TcpServer", tcp_server.clone());
        println!("{} | TcpServer - ready", self_id);
        //
        // Preparing test data
        let tx_id = PointTxId::fromStr(self_id);
        let parent = self_id;
        let test_data = [
            PointType::String(Point::new(
                tx_id, 
                &PointName::new(parent, "JdsService/Auth.Secret").full(),
                r#"{
                    \"secret\": \"Auth.Secret\"
                }"#.to_string(), 
                Status::Ok, 
                Cot::Req, 
                chrono::offset::Utc::now(),
            )),
            PointType::String(Point::new(
                tx_id, 
                &PointName::new(parent, "JdsService/Auth.Ssh").full(),
                r#"{
                    \"ssh\": \"Auth.Ssh\"
                }"#.to_string(), 
                Status::Ok, 
                Cot::Req, 
                chrono::offset::Utc::now(),
            )),
            PointType::String(Point::new(
                tx_id, 
                &PointName::new(parent, "JdsService/Points").full(),
                r#"{
                    \"points\": []
                }"#.to_string(), 
                Status::Ok, 
                Cot::Req, 
                chrono::offset::Utc::now(),
            )),
            PointType::String(Point::new(
                tx_id, 
                &PointName::new(parent, "JdsService/Subscribe").full(),
                r#"{
                    \"points\": []
                }"#.to_string(), 
                Status::Ok, 
                Cot::Req, 
                chrono::offset::Utc::now(),
            )),
        ];
        let test_items_count = test_data.len();
        //
        // preparing MockServicePoints with the Vec<PontConfig>
        let service_points = Arc::new(Mutex::new(MockServicePoints::new(self_id, point_configs(self_id))));
        services.lock().unwrap().insert("MockServicePoints", service_points);
        //
        // Configuring Receiver
        let receiver = Arc::new(Mutex::new(MockRecvService::new(self_id, "in-queue", Some(test_items_count * 2))));
        services.lock().unwrap().insert("MockRecvService", receiver.clone());
        println!("{} | MockRecvService - ready", self_id);
        println!("\n{} | All configurations - ok\n", self_id);
        //
        // Starting all services
        let receiver_handle = receiver.lock().unwrap().run().unwrap();
        let mq_service_handle = mq_service.lock().unwrap().run().unwrap();
        let tcp_server_handle = tcp_server.lock().unwrap().run().unwrap();
        println!("{} | All services - are executed", self_id);
        thread::sleep(Duration::from_millis(1000));
        //
        // Sending tcp test events / receiver must not receive anything before subscription activated
        println!("{} | Sending tcp test events - to be rejected (not authenticated)", self_id);
        let mut tcp_stream = TcpStream::connect(tcp_server_addr).unwrap();
        let test_requests = [
            PointType::String(Point::new(
                0, 
                &PointName::new(&parent, "Jds/Auth.Secret").full(),
                r#"{\"reply\": \"Auth.Ssh Reply\"}"#.to_string(), 
                Status::Ok, 
                Cot::Inf, 
                chrono::offset::Utc::now(),
            )),
            PointType::String(Point::new(
                0, 
                &PointName::new(&parent, "Jds/Auth.Secret").full(),
                r#"{\"reply\": \"Auth.Ssh Reply\"}"#.to_string(), 
                Status::Ok, 
                Cot::Act, 
                chrono::offset::Utc::now(),
            )),
            PointType::String(Point::new(
                0, 
                &PointName::new(&parent, "Jds/Auth.Secret").full(),
                r#"{\"reply\": \"Auth.Ssh Reply\"}"#.to_string(), 
                Status::Ok, 
                Cot::ActCon, 
                chrono::offset::Utc::now(),
            )),
            PointType::String(Point::new(
                0, 
                &PointName::new(&parent, "Jds/Auth.Secret").full(),
                r#"{\"reply\": \"Auth.Ssh Reply\"}"#.to_string(), 
                Status::Ok, 
                Cot::ActErr, 
                chrono::offset::Utc::now(),
            )),
            PointType::String(Point::new(
                0, 
                &PointName::new(&parent, "Jds/Auth.Secret").full(),
                r#"{\"reply\": \"Auth.Ssh Reply\"}"#.to_string(), 
                Status::Ok, 
                Cot::ReqCon, 
                chrono::offset::Utc::now(),
            )),
            PointType::String(Point::new(
                0, 
                &PointName::new(&parent, "Jds/Auth.Secret").full(),
                r#"{\"reply\": \"Auth.Ssh Reply\"}"#.to_string(), 
                Status::Ok, 
                Cot::ReqErr, 
                chrono::offset::Utc::now(),
            )),
        ];
        for request in test_requests {
            let mut request = serde_json::to_vec(&request).unwrap();
            request.push(JDS_END_OF_TRANSMISSION);
            tcp_stream.write_all(&request).unwrap();
        }
        thread::sleep(Duration::from_millis(2000));
        receiver.lock().unwrap().exit();
        receiver_handle.wait().unwrap();
        let received = receiver.lock().unwrap().received();
        let result = received.lock().unwrap().len();
        assert!(result == 0, "\nresult: {:?}\ntarget: {:?}", result, 0);

        // //
        // // Sending test events
        // println!("{} | Try to get send from MultiQueue...", self_id);
        // let send = services.lock().unwrap().get_link("MultiQueue.in-queue");
        // println!("{} | Try to get send from MultiQueue - ok", self_id);
        // let mut sent = 0;
        // for point in test_data {
        //     match send.send(point.clone()) {
        //         Ok(_) => {
        //             sent += 1;
        //             println!("{} | \t sent: {:?}", self_id, point);
        //         },
        //         Err(err) => {
        //             panic!("{} | Send error: {:?}", self_id, err)
        //         },
        //     }
        // }
        // println!("{} | Total sent: {}", self_id, sent);
        // //
        // // Waiting while all events being received
        // receiver_handle.wait().unwrap();
        // thread::sleep(Duration::from_millis(1000));
        // //
        // Stopping all services
        receiver.lock().unwrap().exit();
        tcp_server.lock().unwrap().exit();
        mq_service.lock().unwrap().exit();
        // //
        // // Verivications
        // let received = receiver.lock().unwrap().received();
        // let received_len = received.lock().unwrap().len();
        // let result = received_len;
        // let target = test_items_count * 2;
        // println!("{} | Total received: {}", self_id, received_len);
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        // //
        // // Verifing JdsService replies
        // let mut replies = 0;
        // let mut reply_errors = 0;
        // for point in received.lock().unwrap().iter() {
        //     match point.cot() {
        //         // Cot::Inf => todo!(),
        //         // Cot::Act => todo!(),
        //         // Cot::ActCon => todo!(),
        //         // Cot::ActErr => todo!(),
        //         // Cot::Req => todo!(),
        //         Cot::ReqCon => {
        //             replies += 1;
        //             println!("{} | Received ReqCon reply: {:?}", self_id, point);
        //             if point.name() == PointName::new(parent, "JdsService/Points").full() {
        //                 let result: Vec<PointConfig> = serde_json::from_str(point.value().as_string().as_str()).unwrap();
        //                 let target = point_configs(self_id);
        //                 // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        //             }
        //         },
        //         Cot::ReqErr => {
        //             reply_errors += 1;
        //             println!("{} | Received ReqErr reply: {:?}", self_id, point);
        //         },
        //         // Cot::Read => todo!(),
        //         // Cot::Write => todo!(),
        //         // Cot::All => todo!(),
        //         _ => {
        //             println!("{} | Received unknown point: {:?}", self_id, point);
        //         },
        //     }
        // }
        // let result = replies;
        // let target = test_items_count;
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        // let result = reply_errors;
        // let target = 0;
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        // //
        // Waiting while all services being finished
        mq_service_handle.wait().unwrap();
        tcp_server_handle.wait().unwrap();
        //
        // Reseting dureation timer
        test_duration.exit();
    }

    // #[test]
    fn auth_ssh() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test JdsService";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        //
        // Configuring MultiQueue service 
        let services = Arc::new(Mutex::new(Services::new(self_id)));
        let conf = serde_yaml::from_str(r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                out queue: 
                    - MockRecvService.in-queue
        "#).unwrap();
        let mq_conf = MultiQueueConfig::from_yaml(&conf);
        let mq_service = Arc::new(Mutex::new(MultiQueue::new(self_id, mq_conf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mq_service.clone());
        //
        // Configuring TcpServer service 
        let tcp_port = TestSession::free_tcp_port_str();
        let tcp_addr = format!("127.0.0.1:{}", tcp_port);
        let conf = format!(r#"
            service TcpServer:
                cycle: 1 ms
                reconnect: 1 s  # default 3 s
                address: {}
                auth: none      # auth: none / auth-secret: pass: ... / auth-ssh: path: ...
                in queue link:
                    max-length: 10000
                out queue: MultiQueue.in-queue
        "#, tcp_addr);
        let conf = serde_yaml::from_str(&conf).unwrap();
        let conf = TcpServerConfig::from_yaml(&conf);
        let tcp_server = Arc::new(Mutex::new(TcpServer::new(self_id, conf, services.clone())));
        services.lock().unwrap().insert("TcpServer", tcp_server.clone());
        println!("{} | TcpServer - ready", self_id);
        //
        // Preparing test data
        let tx_id = PointTxId::fromStr(self_id);
        let parent = self_id;
        let test_data = [
            PointType::String(Point::new(
                tx_id, 
                &PointName::new(parent, "JdsService/Auth.Secret").full(),
                r#"{
                    \"secret\": \"Auth.Secret\"
                }"#.to_string(), 
                Status::Ok, 
                Cot::Req, 
                chrono::offset::Utc::now(),
            )),
            PointType::String(Point::new(
                tx_id, 
                &PointName::new(parent, "JdsService/Auth.Ssh").full(),
                r#"{
                    \"ssh\": \"Auth.Ssh\"
                }"#.to_string(), 
                Status::Ok, 
                Cot::Req, 
                chrono::offset::Utc::now(),
            )),
            PointType::String(Point::new(
                tx_id, 
                &PointName::new(parent, "JdsService/Points").full(),
                r#"{
                    \"points\": []
                }"#.to_string(), 
                Status::Ok, 
                Cot::Req, 
                chrono::offset::Utc::now(),
            )),
            PointType::String(Point::new(
                tx_id, 
                &PointName::new(parent, "JdsService/Subcribe").full(),
                r#"{
                    \"points\": []
                }"#.to_string(), 
                Status::Ok, 
                Cot::Req, 
                chrono::offset::Utc::now(),
            )),
        ];
        let test_items_count = test_data.len();
        //
        // Configuring Receiver
        let receiver = Arc::new(Mutex::new(MockRecvService::new(self_id, "in-queue", Some(test_items_count * 2))));
        services.lock().unwrap().insert("MockRecvService", receiver.clone());
        println!("{} | MockRecvService - ready", self_id);
        //
        // Starting all services
        let receiver_handle = receiver.lock().unwrap().run().unwrap();
        let mq_service_handle = mq_service.lock().unwrap().run().unwrap();
        let jds_service_handle = tcp_server.lock().unwrap().run().unwrap();
        println!("{} | All services - are executed", self_id);
        thread::sleep(Duration::from_millis(200));
        //
        // Sending test events
        println!("{} | Try to get send from MultiQueue...", self_id);
        let send = services.lock().unwrap().get_link("MultiQueue.in-queue");
        println!("{} | Try to get send from MultiQueue - ok", self_id);
        let mut sent = 0;
        for point in test_data {
            match send.send(point.clone()) {
                Ok(_) => {
                    sent += 1;
                    println!("{} | \t sent: {:?}", self_id, point);
                },
                Err(err) => {
                    panic!("{} | Send error: {:?}", self_id, err)
                },
            }
        }
        println!("{} | Total sent: {}", self_id, sent);
        //
        // Waiting while all events being received
        receiver_handle.wait().unwrap();
        thread::sleep(Duration::from_millis(800));
        //
        // Stopping all services
        receiver.lock().unwrap().exit();
        tcp_server.lock().unwrap().exit();
        mq_service.lock().unwrap().exit();
        //
        // Verivications
        let received = receiver.lock().unwrap().received();
        let received_len = received.lock().unwrap().len();
        let result = received_len;
        let target = test_items_count * 2;
        println!("{} | Total received: {}", self_id, received_len);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        //
        // Verifing JdsService replies
        let mut replies = 0;
        let mut reply_errors = 0;
        for point in received.lock().unwrap().iter() {
            match point.cot() {
                // Cot::Inf => todo!(),
                // Cot::Act => todo!(),
                // Cot::ActCon => todo!(),
                // Cot::ActErr => todo!(),
                // Cot::Req => todo!(),
                Cot::ReqCon => {
                    replies += 1;
                    println!("{} | Received ReqCon reply: {:?}", self_id, point);
                },
                Cot::ReqErr => {
                    reply_errors += 1;
                    println!("{} | Received ReqErr reply: {:?}", self_id, point);
                },
                // Cot::Read => todo!(),
                // Cot::Write => todo!(),
                // Cot::All => todo!(),
                _ => {
                    println!("{} | Received unknown point: {:?}", self_id, point);
                },
            }
        }
        let result = replies;
        let target = test_items_count;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        let result = reply_errors;
        let target = 0;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        //
        // Waiting while all services being finished
        mq_service_handle.wait().unwrap();
        jds_service_handle.wait().unwrap();
        //
        // Reseting dureation timer
        test_duration.exit();
    }
}
