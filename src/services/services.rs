use std::{collections::HashMap, process::exit, sync::{mpsc::{Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}, time::Duration};
use libc::{SIGABRT, SIGFPE, SIGHUP, SIGILL, SIGINT, SIGKILL, SIGQUIT, SIGSEGV, SIGTERM, SIGUSR1, SIGUSR2};
use log::{debug, error, info};
use signal_hook::iterator::Signals;
use testing::stuff::wait::WaitTread;
use crate::{
    core_::point::point_type::PointType, 
    conf::{api_client_config::ApiClientConfig, conf_tree::ConfTree, multi_queue_config::MultiQueueConfig, point_config::point_config::PointConfig, profinet_client_config::profinet_client_config::ProfinetClientConfig, services::services_config::ServicesConfig, task_config::TaskConfig, tcp_client_config::TcpClientConfig, tcp_server_config::TcpServerConfig}, 
    services::{
        api_cient::api_client::ApiClient, multi_queue::subscription_criteria::SubscriptionCriteria, queue_name::QueueName, service::service::Service, task::task::Task
    }
};

use super::{multi_queue::multi_queue::MultiQueue, profinet_client::profinet_client::ProfinetClient, server::tcp_server::TcpServer, service::service_handles::ServiceHandles, tcp_client::tcp_client::TcpClient};

///
/// Holds a map of the all services in app by there names
pub struct Services {
    id: String,
    map: HashMap<String, Arc<Mutex<dyn Service + Send>>>,
    handles: HashMap<String, ServiceHandles>,
}
///
/// 
impl Services {
    const API_CLIENT: &'static str = "ApiClient";
    const MULTI_QUEUE: &'static str = "MultiQueue";
    const PROFINET_CLIENT: &'static str = "ProfinetClient";
    const TASK: &'static str = "Task";
    const TCP_CLIENT: &'static str = "TcpClient";
    const TCP_SERVER: &'static str = "TcpServer";
    // const : &str = "";
    // const : &str = "";
    ///
    /// Creates new instance of the ReatinBuffer
    pub fn new(parent: impl Into<String>) -> Self {
        Self {
            id: format!("{}/Services", parent.into()),
            map: HashMap::new(),
            handles: HashMap::new(),
        }
    }
    ///
    /// 
    pub fn insert(&mut self, id:&str, service: Arc<Mutex<dyn Service + Send>>) {
        if self.map.contains_key(id) {
            panic!("{}.insert | Duplicated service name '{:?}'", self.id, id);
        }
        self.map.insert(id.to_string(), service);
    }
    ///
    /// Returns Service
    pub fn get(&self, name: &str) -> Arc<Mutex<dyn Service>> {
        match self.map.get(name) {
            Some(srvc) => srvc.clone(),
            None => panic!("{}.get | service '{:?}' - not found", self.id, name),
        }
    }
    ///
    /// Returns copy of the Sender - service's incoming queue
    pub fn get_link(&self, name: &str) -> Sender<PointType> {
        let name = QueueName::new(name);
        match self.map.get(name.service()) {
            Some(srvc) => srvc.lock().unwrap().get_link(name.queue()),
            None => panic!("{}.get | service '{:?}' - not found", self.id, name),
        }
    }
    ///
    /// Returns Receiver
    /// - service - the name of the service to subscribe on
    pub fn subscribe(&mut self, service: &str, receiver_id: &str, points: &[SubscriptionCriteria]) -> (Sender<PointType>, Receiver<PointType>) {
        match self.map.get(service) {
            Some(srvc) => {
                debug!("{}.subscribe | Lock service '{:?}'...", self.id, service);
                let r = srvc.lock().unwrap().subscribe(receiver_id, points);
                debug!("{}.subscribe | Lock service '{:?}' - ok", self.id, service);
                r
            },
            None => panic!("{}.get | service '{:?}' - not found", self.id, service),
        }
    }
    ///
    /// Returns ok if subscription extended sucessfully
    /// - service - the name of the service to extend subscribtion on
    pub fn extend_subscription(&mut self, service: &str, receiver_id: &str, points: &[SubscriptionCriteria]) -> Result<(), String> {
        // panic!("{}.extend_subscription | Not implemented yet", self.id);
        match self.map.get(service) {
            Some(srvc) => {
                debug!("{}.extend_subscription | Lock service '{:?}'...", self.id, service);
                let r = srvc.lock().unwrap().extend_subscription(receiver_id, points);
                debug!("{}.extend_subscription | Lock service '{:?}' - ok", self.id, service);
                r
            },
            None => panic!("{}.get | service '{:?}' - not found", self.id, service),
        }
    }
    ///
    /// Returns ok if subscription removed sucessfully
    /// - service - the name of the service to unsubscribe on
    fn unsubscribe(&mut self, service: &str, receiver_id: &str, points: &[SubscriptionCriteria]) -> Result<(), String> {
        match self.map.get(service) {
            Some(srvc) => {
                debug!("{}.unsubscribe | Lock service '{:?}'...", self.id, service);
                let r = srvc.lock().unwrap().unsubscribe(receiver_id, points);
                debug!("{}.unsubscribe | Lock service '{:?}' - ok", self.id, service);
                r
            },
            None => panic!("{}.get | service '{:?}' - not found", self.id, service),
        }
    }
    ///
    /// Returns list of point configurations over the all services
    pub fn points(&self) -> Vec<PointConfig> {
        let mut points = vec![];
        for service in self.map.values() {
            let mut service_points = service.lock().unwrap().points();
            points.append(&mut service_points);
        };
        points
    }
    ///
    /// Executes all hollded services
    pub fn run(self) -> Result<(), String>  {
        let self_id = self.id.clone();
        info!("{}.run | Starting application...", self_id);
        let path = "config.yaml";
        info!("{}.run |     Reading configuration...", self_id);
        let conf: ServicesConfig = ServicesConfig::read(path);
        info!("{}.run |     Reading configuration - ok", self_id);
        let parent = conf.name.clone();
        let services = Arc::new(Mutex::new(self));
        info!("{}.run |     Configuring services...", self_id);
        for (node_keywd, mut node_conf) in conf.nodes {
            let node_name = node_keywd.name();
            let node_sufix = node_keywd.sufix();
            info!("{}.run |         Configuring service: {}({})...", self_id, node_name, node_sufix);
            debug!("{}.run |         Config: {:#?}", self_id, node_conf);
            let service = Self::match_service(&self_id, &parent, &node_name, &node_sufix, &mut node_conf, services.clone());
            let id = if node_sufix.is_empty() {&node_name} else {&node_sufix};
            services.lock().unwrap().insert(id, service);
            info!("{}.run |         Configuring service: {}({}) - ok\n", self_id, node_name, node_sufix);
        }
        info!("{}.run |     All services configured\n", self_id);

        thread::sleep(Duration::from_millis(1000));
        info!("{}.run |     Starting services...", self_id);
        let services_iter = services.lock().unwrap().map.clone();
        for (name, service) in services_iter {
            info!("{}.run |         Starting service: {}...", self_id, name);
            let handles = service.lock().unwrap().run();
            match handles {
                Ok(handles) => {
                    services.lock().unwrap().insert_handles(&name, handles);
                    info!("{}.run |         Starting service: {} - ok", self_id, name);
                },
                Err(err) => {
                    error!("{}.run |         Error starting service '{}': {:#?}", self_id, name, err);
                },
            };
        }
        info!("{}.run |     All services started\n", self_id);

        info!("{}.run | Application started\n", self_id);

        let self_id_clone = self_id.clone();
        let services_clone = services.clone();
        let handle = thread::Builder::new().name(format!("{}.run", self_id)).spawn(move || {
            let self_id = self_id_clone;
            let signals = Signals::new(&[
                SIGHUP,     // code: 1	This signal is sent to a process when its controlling terminal is closed or disconnected
                SIGINT,     // code: 2	This signal is sent to a process when the user presses Control+C to interrupt its execution
                SIGQUIT,    // code: 3	This signal is similar to SIGINT but is used to initiate a core dump of the process, which is useful for debugging
                SIGILL,     // code: 4	This signal is sent to a process when it attempts to execute an illegal instruction
                SIGABRT,    // code: 6	This signal is sent to a process when it calls the abort() function
                SIGFPE,     // code: 8	This signal is sent to a process when it attempts to perform an arithmetic operation that is not allowed, such as division by zero
                SIGKILL,    // code: 9	This signal is used to terminate a process immediately and cannot be caught or ignored
                SIGSEGV,    // code: 11	This signal is sent to a process when it attempts to access memory that is not allocated to it
                SIGTERM,    // Code: 15	This signal is sent to a process to request that it terminate gracefully.
                SIGUSR1,    // code: 10	These signals can be used by a process for custom purposes
                SIGUSR2,    // code: 12	Same as SIGUSR1, code: 10
            ]);
            match signals {
                Ok(mut signals) => {
                    thread::spawn(move || {
                        for signal in signals.forever() {
                            match signal {
                                SIGINT | SIGQUIT | SIGTERM => {
                                    println!("{}.run Received signal {:?}", self_id, signal);
                                    println!("{}.run Application exit...", self_id);
                                    for (_id, service) in &services_clone.lock().unwrap().map {
                                        service.lock().unwrap().exit()
                                    }
                                    break;
                                },
                                SIGKILL => {
                                    println!("{}.run Received signal {:?}", self_id, signal);
                                    println!("{}.run Application halt...", self_id);
                                    exit(0);
                                },
                                _ => {
                                    println!("{}.run Received unknown signal {:?}", self_id, signal);
                                },
                            }
                        }
                    });
                },
                Err(err) => {
                    panic!("{}.run | Application hook system signals error; {:#?}", self_id, err);
                },
            }
        });
        
        loop {
            match services.lock().unwrap().handles.keys().next() {
                Some(service_id) => {
                    info!("{}.run | Waiting for service '{}' being finished...", self_id, service_id);
                    match services.lock().unwrap().handles.remove_entry(service_id) {
                        Some((_, handles)) => {
                            handles.wait().unwrap()
                        },
                        None => {
                            error!("{}.run | Service '{}' can't be found", self_id, service_id);
                        },
                    };

                },
                None => {
                    break;
                },
            }
        }
        Ok(())
    }
    ///
    /// 
    fn match_service(self_id: &str, parent: &str, node_name: &str, node_sufix: &str, node_conf: &mut ConfTree, services: Arc<Mutex<Services>>) -> Arc<Mutex<dyn Service + Send>> {
        match node_name {
            Services::API_CLIENT => {
                Arc::new(Mutex::new(ApiClient::new(parent, ApiClientConfig::new(node_conf))))
            },
            Services::MULTI_QUEUE => {
                Arc::new(Mutex::new(MultiQueue::new(parent, MultiQueueConfig::new(node_conf), services)))
            },
            Services::PROFINET_CLIENT => {
                Arc::new(Mutex::new(ProfinetClient::new(parent, ProfinetClientConfig::new(node_conf), services)))
            },
            Services::TASK => {
                Arc::new(Mutex::new(Task::new(parent, TaskConfig::new(node_conf), services.clone())))
            },
            Services::TCP_CLIENT => {
                Arc::new(Mutex::new(TcpClient::new(parent, TcpClientConfig::new(node_conf), services.clone())))
            },
            Services::TCP_SERVER => {
                Arc::new(Mutex::new(TcpServer::new(parent, TcpServerConfig::new(node_conf), services.clone())))
            },
            _ => {
                panic!("{}.run | Unknown service: {}({})", self_id, node_name, node_sufix);
            },
        }
    }
    ///
    /// Inserts new pair service_id & service_join_handle
    fn insert_handles(&mut self, id:&str, handles: ServiceHandles) {
        if self.handles.contains_key(id) {
            panic!("{}.insert | Duplicated service name '{:?}'", self.id, id);
        }
        self.handles.insert(id.to_string(), handles);
    }
}
