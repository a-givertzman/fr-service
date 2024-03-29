#[cfg(test)]

mod task_nodes {
    use log::{info, debug, trace, warn};
    use std::{sync::{Once, mpsc::{Sender, self, Receiver}, Arc, Mutex, atomic::{Ordering, AtomicBool}}, collections::HashMap, thread};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        conf::task_config::TaskConfig, 
        core_::{object::object::Object, point::point_type::{PointType, ToPoint}}, 
        services::{
            services::Services, 
            service::{service::Service, service_handles::ServiceHandles}, 
            task::{nested_function::{fn_count, fn_ge, fn_kind::FnKind, sql_metric}, task_nodes::TaskNodes},
        },
    }; 
    ///
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
    ///  - Rc<RefCell<Box<dyn FnInOut>>>...
    fn init_each() {
        // fn_ge::COUNT.reset();
        // fn_count::COUNT.reset();
        // sql_metric::COUNT.reset();
    }
    ///
    /// 
    #[test]
    fn test() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        println!("test");
        let path = "./src/tests/unit/services/task/task_nodes/task_nodes.yaml";
        let mut task_nodes = TaskNodes::new("test");
        let conf = TaskConfig::read(path);
        debug!("conf: {:?}", conf);
        let self_id = "test";
        // let outName = format!("{}/SqlMetric1", self_id);
        // let outName = outName.as_str();
        // debug!("outName: {:?}", outName);
        let services = Arc::new(Mutex::new(Services::new(self_id)));
        let mock_service = Arc::new(Mutex::new(MockService::new(self_id, "queue")));
        services.lock().unwrap().insert("ApiClient", mock_service.clone());
        task_nodes.buildNodes("test", conf, services);
        let test_data = vec![
            (
                "/path/Point.Name1", 101, 
                HashMap::from([
                    (format!("{}/SqlMetric{}", self_id, sql_metric::COUNT.load(Ordering::SeqCst)), "101, 1102, 0, 0"),
                    (format!("{}/FnCount{}.out", self_id, fn_count::COUNT.load(Ordering::SeqCst)), "101"),
                ])
            ),
            (
                "/path/Point.Name1", 201, 
                HashMap::from([
                    (format!("{}/SqlMetric{}", self_id, sql_metric::COUNT.load(Ordering::SeqCst)), "201, 1202, 0, 0"),
                    (format!("{}/FnCount{}.out", self_id, fn_count::COUNT.load(Ordering::SeqCst)), "302"),
                ])
                
            ),
            (
                "/path/Point.Name1", 301, 
                HashMap::from([
                    (format!("{}/SqlMetric{}", self_id, sql_metric::COUNT.load(Ordering::SeqCst)), "301, 1302, 0, 0"),
                    (format!("{}/FnCount{}.out", self_id, fn_count::COUNT.load(Ordering::SeqCst)), "603"),
                ])
                
            ),
            (
                "/path/Point.Name2", 202, 
                HashMap::from([
                    (format!("{}/SqlMetric{}", self_id, sql_metric::COUNT.load(Ordering::SeqCst)), "301, 1302, 202, 0"),
                    (format!("{}/FnGe{}.out", self_id, fn_ge::COUNT.load(Ordering::SeqCst)), "true"),
                ])
                
            ),
            (
                "/path/Point.Name3", 303, 
                HashMap::from([
                    (format!("{}/SqlMetric{}", self_id, sql_metric::COUNT.load(Ordering::SeqCst)), "301, 1302, 202, 303"),
                    (format!("{}/FnGe{}.out", self_id, fn_ge::COUNT.load(Ordering::SeqCst)), "false"),
                ])
                
            ),
            (
                "/path/Point.Name3", 304, 
                HashMap::from([
                    (format!("{}/SqlMetric{}", self_id, sql_metric::COUNT.load(Ordering::SeqCst)), "301, 1302, 202, 304"),
                    (format!("{}/FnGe{}.out", self_id, fn_ge::COUNT.load(Ordering::SeqCst)), "false"),
                ])
                
            ),
        ];
        mock_service.lock().unwrap().run().unwrap();
        for (name, value, targetValue) in test_data {
            let point = value.to_point(0, name);
            // let inputName = &point.name();
            debug!("input point name: {:?}  value: {:?}", name, value);
            match &task_nodes.getEvalNode(&name) {
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
                        let out_value = match &out {
                            PointType::Bool(point) => point.value.to_string(),
                            PointType::Int(point) => point.value.to_string(),
                            PointType::Float(point) => point.value.to_string(),
                            PointType::String(point) => point.value.clone(),
                        };
                        debug!("TaskEvalNode.eval | evalNode '{}' out - '{}': {:?}", evalNode.name(), evalNodeOut.borrow().id(), out);
                        if evalNodeOut.borrow().kind() != &FnKind::Var {
                            let out_name = out.name();
                            debug!("TaskEvalNode.eval | out.name: '{}'", out_name);
                            let target = match targetValue.get(out_name.as_str()) {
                                Some(target) => target.to_string(),
                                None => panic!("TaskEvalNode.eval | out.name '{}' - not foind in {:?}", out_name, targetValue),
                            };
                            assert!(out_value == target, "\n   outValue: {} \ntargetValue: {}", out_value, target);
                        }
                    };
                },
                None => {
                    panic!("input {:?} - not found in the current taskStuff", &name)
                },
            };
        } 
        mock_service.lock().unwrap().exit();
    }
    ///
    /// 
    struct MockService {
        id: String,
        links: HashMap<String, Sender<PointType>>,
        rx_recv: Vec<Receiver<PointType>>,
        exit: Arc<AtomicBool>,
    }
    ///
    /// 
    impl MockService {
        fn new(parent: &str, link_name: &str) -> Self {
            let (send, recv) = mpsc::channel();
            Self {
                id: format!("{}/MockService", parent),
                links: HashMap::from([
                    (link_name.to_string(), send),
                ]),
                rx_recv: vec![recv],
                exit: Arc::new(AtomicBool::new(false)),
            }
        }
    }
    ///
    /// 
    impl Object for MockService {
        fn id(&self) -> &str {
            &self.id
        }
    }
    ///
    /// 
    impl Service for MockService {
        //
        //
        fn get_link(&mut self, name: &str) -> Sender<PointType> {
            match self.links.get(name) {
                Some(send) => send.clone(),
                None => panic!("{}.run | link '{:?}' - not found", self.id, name),
            }
        }
        //
        //
        fn run(&mut self) -> Result<ServiceHandles, String> {
            info!("{}.run | Starting...", self.id);
            let self_id = self.id.clone();
            let exit = self.exit.clone();
            let rx_recv = self.rx_recv.pop().unwrap();
            let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
                loop {
                    match rx_recv.recv() {
                        Ok(point) => {
                            debug!("{}.run | received: {:?}", self_id, point);
                        },
                        Err(err) => {
                            warn!("{}.run | error: {:?}", self_id, err);
                        },
                    }
                    if exit.load(Ordering::SeqCst) {
                        break;
                    }
                }
            });
            match handle {
                Ok(handle) => {
                    info!("{}.run | Starting - ok", self.id);
                    Ok(ServiceHandles::new(vec![(self.id.clone(), handle)]))
                },
                Err(err) => {
                    let message = format!("{}.run | Start faled: {:#?}", self.id, err);
                    warn!("{}", message);
                    Err(message)
                },
            }            
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
