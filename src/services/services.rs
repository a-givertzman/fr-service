use std::{collections::HashMap, sync::{mpsc::{Receiver, Sender}, Arc, Mutex}, thread::JoinHandle};
use log::{debug, error, info};
use crate::{
    conf::{api_client_config::ApiClientConfig, conf_tree::ConfTree, multi_queue_config::MultiQueueConfig, point_config::point_config::PointConfig, profinet_client_config::profinet_client_config::ProfinetClientConfig, services::services_config::ServicesConfig, task_config::TaskConfig, tcp_client_config::TcpClientConfig, tcp_server_config::TcpServerConfig}, core_::{object::object::Object, point::point_type::PointType}, services::{
        api_cient::api_client::ApiClient, multi_queue::subscription_criteria::SubscriptionCriteria, queue_name::QueueName, service::service::Service, task::task::Task
    }
};

use super::{multi_queue::multi_queue::MultiQueue, profinet_client::profinet_client::ProfinetClient, server::tcp_server::TcpServer, tcp_client::tcp_client::TcpClient};

///
/// Holds a map of the all services in app by there names
pub struct Services {
    id: String,
    map: HashMap<String, Arc<Mutex<dyn Service + Send>>>,
    handles: HashMap<String, JoinHandle<()>>,
}
///
/// 
impl Services {
    const API_CLIENT: &str = "ApiClient";
    const MULTI_QUEUE: &str = "MultiQueue";
    const PROFINET_CLIENT: &str = "ProfinetClient";
    const TASK: &str = "Task";
    const TCP_CLIENT: &str = "TcpClient";
    const TCP_SERVER: &str = "TcpServer";
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

        info!("{}.run |     Starting services...", self_id);
        for (name, service) in &services.lock().unwrap().map {
            info!("{}.run |         Starting service: {}...", self_id, name);
            match service.lock().unwrap().run() {
                Ok(handle) => {
                    services.lock().unwrap().insert_handle(name, handle);
                    info!("{}.run |         Starting service: {} - ok", self_id, name);
                },
                Err(err) => {
                    error!("{}.run | Error starting service '{}': {:#?}", self_id, name, err);
                },
            };
        }
        info!("{}.run |     All services started\n", self_id);

        info!("{}.run | Application started\n", self_id);
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
    fn insert_handle(&mut self, id:&str, handle: JoinHandle<()>) {
        if self.handles.contains_key(id) {
            panic!("{}.insert | Duplicated service name '{:?}'", self.id, id);
        }
        self.handles.insert(id.to_string(), handle);
    }
}
