#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{info, debug, trace, warn};
    use std::{sync::{Once, mpsc::{Sender, self, Receiver}, Arc, Mutex, atomic::{Ordering, AtomicBool}}, collections::HashMap, thread};
    use crate::{
        core_::{
            debug::debug_session::{DebugSession, LogLevel, Backtrace},
            point::point_type::{ToPoint, PointType},
        },
        conf::task_config::TaskConfig, 
        services::{task::{task_nodes::TaskNodes, nested_function::{fn_kind::FnKind, fn_count::{self}, fn_ge}}, services::Services, service::Service},
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
    ///  - Rc<RefCell<Box<dyn FnInOut>>>...
    fn initEach() {
        fn_ge::resetCount();
        fn_count::resetCount();
    }
    
    #[test]
    fn test_task_nodes() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_task_nodes");
        let path = "./src/tests/unit/services/task/task_nodes/task.yaml";
        let mut taskNodes = TaskNodes::new("test_task_nodes");
        let conf = TaskConfig::read(path);
        debug!("conf: {:?}", conf);
        let services = Arc::new(Mutex::new(Services::new("test")));
        let mockService = Arc::new(Mutex::new(MockService::new("test", "queue")));
        services.lock().unwrap().insert("ApiClient", mockService.clone());
        taskNodes.buildNodes("test", conf, services);
        let testData = vec![
            (
                "/path/Point.Name1", 101, 
                HashMap::from([
                    ("MetricSelect.out", "101, 1102, 0, 0"),
                    ("FnCount1.out", "101"),
                ])
            ),
            (
                "/path/Point.Name1", 201, 
                HashMap::from([
                    ("MetricSelect.out", "201, 1202, 0, 0"),
                    ("FnCount1.out", "302"),
                ])
                
            ),
            (
                "/path/Point.Name1", 301, 
                HashMap::from([
                    ("MetricSelect.out", "301, 1302, 0, 0"),
                    ("FnCount1.out", "603"),
                ])
                
            ),
            (
                "/path/Point.Name2", 202, 
                HashMap::from([
                    ("MetricSelect.out", "301, 1302, 202, 0"),
                    ("FnGe1.out", "true"),
                ])
                
            ),
            (
                "/path/Point.Name3", 303, 
                HashMap::from([
                    ("MetricSelect.out", "301, 1302, 202, 303"),
                    ("FnGe1.out", "false"),
                ])
                
            ),
            (
                "/path/Point.Name3", 304, 
                HashMap::from([
                    ("MetricSelect.out", "301, 1302, 202, 304"),
                    ("FnGe1.out", "false"),
                ])
                
            ),
        ];
        mockService.lock().unwrap().run().unwrap();
        for (name, value, targetValue) in testData {
            let point = value.toPoint(0, name);
            // let inputName = &point.name();
            debug!("input point name: {:?}  value: {:?}", name, value);
            match &taskNodes.getEvalNode(&name) {
                Some(evalNode) => {
                    let input = evalNode.getInput();
                    input.borrow_mut().add(point.clone());
                    debug!("evalNode: {:?}", evalNode.name());
                    // debug!("evalNode outs: {:?}", evalNode.getOuts());
                    for evalNodeVar in evalNode.getVars() {
                        trace!("TaskEvalNode.eval | evalNode '{}' - var '{}' evaluating...", evalNode.name(), evalNodeVar.borrow().id());
                        evalNodeVar.borrow_mut().eval();
                        debug!("TaskEvalNode.eval | evalNode '{}' - var '{}' evaluated", evalNode.name(), evalNodeVar.borrow().id());
                    };
                    for evalNodeOut in evalNode.getOuts() {
                        trace!("TaskEvalNode.eval | evalNode '{}' out...", evalNode.name());
                        let out = evalNodeOut.borrow_mut().out();
                        let outValue = match &out {
                            PointType::Bool(point) => point.value.to_string(),
                            PointType::Int(point) => point.value.to_string(),
                            PointType::Float(point) => point.value.to_string(),
                            PointType::String(point) => point.value.clone(),
                        };
                        debug!("TaskEvalNode.eval | evalNode '{}' out - '{}': {:?}", evalNode.name(), evalNodeOut.borrow().id(), out);
                        if evalNodeOut.borrow().kind() != &FnKind::Var {
                            if evalNodeOut.borrow().kind() != &FnKind::Var {
                                let outName = out.name();
                                let outName = outName.as_str();
                                debug!("TaskEvalNode.eval | out.name: '{}'", outName);
                                let target = targetValue.get(&outName).unwrap().to_string();
                                assert!(outValue == target, "\n   outValue: {} \ntargetValue: {}", outValue, target);
                            }
                        }
                    };
                },
                None => {
                    panic!("input {:?} - not found in the current taskStuff", &name)
                },
            };
        } 
        mockService.lock().unwrap().exit();
    }
    ///
    /// 
    struct MockService {
        id: String,
        links: HashMap<String, Sender<PointType>>,
        rxRecv: Vec<Receiver<PointType>>,
        exit: Arc<AtomicBool>,
    }
    ///
    /// 
    impl MockService {
        fn new(parent: &str, linkName: &str) -> Self {
            let (send, recv) = mpsc::channel();
            Self {
                id: format!("{}/MockService", parent),
                links: HashMap::from([
                    (linkName.to_string(), send),
                ]),
                rxRecv: vec![recv],
                exit: Arc::new(AtomicBool::new(false)),
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
        fn getLink(&mut self, name: &str) -> Sender<PointType> {
            match self.links.get(name) {
                Some(send) => send.clone(),
                None => panic!("{}.run | link '{:?}' - not found", self.id, name),
            }
        }
        //
        //
        fn run(&mut self) -> Result<std::thread::JoinHandle<()>, std::io::Error> {
            info!("{}.run | starting...", self.id);
            let selfId = self.id.clone();
            let exit = self.exit.clone();
            let rxRecv = self.rxRecv.pop().unwrap();
            let handle = thread::Builder::new().name(format!("{}.run", selfId.clone())).spawn(move || {
                loop {
                    match rxRecv.recv() {
                        Ok(point) => {
                            debug!("{}.run | received: {:?}", selfId, point);
                        },
                        Err(err) => {
                            warn!("{}.run | error: {:?}", selfId, err);
                        },
                    }
                    if exit.load(Ordering::SeqCst) {
                        break;
                    }
                }
            });
            info!("{}.run | started", self.id);
            handle
        }
        //
        //
        fn exit(&self) {
            self.exit.store(true, Ordering::SeqCst);
        }
    }
}


// clear && cargo test -- --test-threads=1 --show-output
// clear && cargo test task_nodes_test -- --test-threads=1 --show-output
