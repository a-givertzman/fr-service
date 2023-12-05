#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{trace, info};
    use std::{sync::{Once, mpsc::{Sender, Receiver, self}, Arc, Mutex}, env, time::Instant, collections::HashMap};
    
    use crate::{
        core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, point::point_type::PointType}, 
        conf::task_config::TaskConfig, 
        services::{task::{task::Task, task_test_receiver::TaskTestReceiver, task_test_producer::TaskTestProducer}, queues::queues::Queues, service::Service, services::Services},
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
    fn test_task_struct() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        info!("test_task_struct");
        
        let iterations = 10;
        
        trace!("dir: {:?}", env::current_dir());
        let path = "./src/tests/unit/services/task/task_test_struct.yaml";
        let config = TaskConfig::read(path);
        trace!("config: {:?}", &config);
        
        // let testValues = vec![0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0];
    
        let services = Arc::new(Mutex::new(Services::new("test")));
        // let (send, recv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
        // let (apiSend, apiRecv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(TaskTestReceiver::new(
            "in-queue",
            iterations,
        )));
        services.lock().unwrap().insert("TaskTestReceiver", receiver.clone());
        
        
        let producer = Arc::new(Mutex::new(TaskTestProducer::new(
            iterations, 
            "Task.recv-queue",
            services.clone(),
        )));
        
        let task = Arc::new(Mutex::new(Task::new("test", config, services.clone())));
        services.lock().unwrap().insert("Task", task.clone());
        
        let receiverHandle = receiver.lock().unwrap().run().unwrap();
        let producerHandle = producer.lock().unwrap().run().unwrap();
        trace!("task runing...");
        let time = Instant::now();
        task.lock().unwrap().run().unwrap();
        trace!("task runing - ok");
        producerHandle.join().unwrap();
        receiverHandle.join().unwrap();
        // thread::sleep(Duration::from_millis(200));
        // trace!("task stopping...");
        // task.exit();
        // receiver.exit();
        // trace!("task stopping - ok");
        let target = iterations;
        let result = receiver.lock().unwrap().received().lock().unwrap().len();
        println!("elapsed: {:?}", time.elapsed());
        println!("received: {:?}", result);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
    
        // trace!("task: {:?}", &task);
    }


    // #[test]
    fn test_task_tranfer() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        info!("test_task_transfer");
        
        let iterations = 10;
        
        trace!("dir: {:?}", env::current_dir());
        let path = "./src/tests/unit/task/task_test.yaml";
        let config = TaskConfig::read(path);
        trace!("config: {:?}", &config);
    
        // let testValues = vec![0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0];

        // let mut queues = Queues::new();
        let services = Arc::new(Mutex::new(Services::new("test")));
        // let (send, recv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
        // let (apiSend, apiRecv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
        // queues.addRecvQueue("recv-queue", recv);
        // queues.addSendQueue("api-queue", apiSend);

        let receiver = Arc::new(Mutex::new(TaskTestReceiver::new(
            "queue",
            iterations,
        )));
        services.lock().unwrap().insert("TaskTestReceiver", receiver.clone());
        
    
        let producer = Arc::new(Mutex::new(TaskTestProducer::new(
            iterations, 
            "Task.queue",
            services.clone(),
        )));
    
        let task = Arc::new(Mutex::new(Task::new("test", config, services.clone())));
        services.lock().unwrap().insert("Task", task.clone());

        let receiverHandle = receiver.lock().unwrap().run().unwrap();
        let producerHandle = producer.lock().unwrap().run().unwrap();
        trace!("task runing...");
        let time = Instant::now();
        task.lock().unwrap().run().unwrap();
        trace!("task runing - ok");
        producerHandle.join().unwrap();
        receiverHandle.join().unwrap();
        // thread::sleep(Duration::from_millis(200));
        // trace!("task stopping...");
        // task.exit();
        // receiver.exit();
        // trace!("task stopping - ok");
        println!("elapsed: {:?}", time.elapsed());
        println!("received: {:?}", receiver.lock().unwrap().received());
    
        // trace!("task: {:?}", &task);
        // assert_eq!(config, target);
    }

    ///
    /// 
    struct MockService {
        id: String,
        links: HashMap<String, Sender<PointType>>,
    }
    ///
    /// 
    impl MockService {
        fn new(parent: &str, linkName: &str) -> Self {
            let (send, _recv) = mpsc::channel();
            Self {
                id: format!("{}/MockService", parent),
                links: HashMap::from([
                    (linkName.to_string(), send),
                ]),
            }
        }
    }
    ///
    /// 
    impl Service for MockService {
        fn id(&self) -> &str {
            todo!()
        }
        //
        //
        fn getLink(&self, name: &str) -> Sender<PointType> {
            match self.links.get(name) {
                Some(send) => send.clone(),
                None => panic!("{}.run | link '{:?}' - not found", self.id, name),
            }
        }
        //
        //
        fn run(&mut self) -> Result<std::thread::JoinHandle<()>, std::io::Error> {
            todo!()
        }
        //
        //
        fn exit(&self) {
            todo!()
        }
    }    
}

