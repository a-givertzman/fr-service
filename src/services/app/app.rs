use log::{debug, error, info};
use std::{collections::HashMap, process::exit, sync::{Arc, Mutex}, thread, time::Duration};
use libc::{
    SIGABRT, SIGHUP, SIGINT, SIGKILL, SIGQUIT, SIGTERM, SIGUSR1, SIGUSR2,
    // SIGFPE, SIGILL, SIGSEGV, 
};
use signal_hook::iterator::Signals;
use testing::stuff::wait::WaitTread;
use crate::{
    conf::{
        api_client_config::ApiClientConfig, 
        app::app_config::AppConfig, 
        conf_tree::ConfTree, 
        multi_queue_config::MultiQueueConfig, 
        profinet_client_config::profinet_client_config::ProfinetClientConfig, 
        task_config::TaskConfig, 
        tcp_client_config::TcpClientConfig, 
        tcp_server_config::TcpServerConfig,
    }, 
    services::{
        service::{service::Service, service_handles::ServiceHandles}, 
        services::Services, 
        api_cient::api_client::ApiClient, 
        multi_queue::multi_queue::MultiQueue, 
        profinet_client::profinet_client::ProfinetClient, 
        server::tcp_server::TcpServer, 
        task::task::Task, 
        tcp_client::tcp_client::TcpClient,
    },
};

pub struct App {
    id: String,
    handles: HashMap<String, ServiceHandles>,
    conf: AppConfig,
}
///
/// 
impl App {
    ///
    /// Creates new instance of the ReatinBuffer
    ///     - path - path to the application configuration
    pub fn new(path: impl Into<String>) -> Self {
        let path: String = path.into();
        let self_id = format!("App");
        info!("{}.run | Configuration path: '{}'", self_id, path);
        info!("{}.run | Reading configuration...", self_id);
        let conf: AppConfig = AppConfig::read(&path);
        info!("{}.run | Reading configuration - ok", self_id);
        Self {
            id: self_id,
            handles: HashMap::new(),
            conf: conf,
        }
    }

    ///
    /// Executes all services
    pub fn run(mut self) -> Result<(), String>  {
        let self_id = self.id.clone();
        info!("{}.run | Starting application...", self_id);
        let conf = self.conf.clone();
        let parent = conf.name.clone();
        let services = Arc::new(Mutex::new(Services::new(&self_id)));
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
        let mut services_lock = services.lock().unwrap();
        let services_iter = services_lock.all();
        for (name, service) in services_iter {
            info!("{}.run |         Starting service: {}...", self_id, name);
            let handles = service.lock().unwrap().run();
            match handles {
                Ok(handles) => {
                    self.insert_handles(&name, handles);
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
        let signals = Signals::new([
            SIGHUP,     // code: 1	This signal is sent to a process when its controlling terminal is closed or disconnected
            SIGINT,     // code: 2	This signal is sent to a process when the user presses Control+C to interrupt its execution
            SIGQUIT,    // code: 3	This signal is similar to SIGINT but is used to initiate a core dump of the process, which is useful for debugging
            // SIGILL,     // code: 4	This signal is sent to a process when it attempts to execute an illegal instruction
            SIGABRT,    // code: 6	This signal is sent to a process when it calls the abort() function
            // SIGFPE,     // code: 8	This signal is sent to a process when it attempts to perform an arithmetic operation that is not allowed, such as division by zero
            // SIGKILL,    // code: 9	This signal is used to terminate a process immediately and cannot be caught or ignored
            // SIGSEGV,    // code: 11	This signal is sent to a process when it attempts to access memory that is not allocated to it
            SIGTERM,    // Code: 15	This signal is sent to a process to request that it terminate gracefully.
            SIGUSR1,    // code: 10	These signals can be used by a process for custom purposes
            SIGUSR2,    // code: 12	Same as SIGUSR1, code: 10
        ]);
        match signals {
            Ok(mut signals) => {
                let self_id = self_id_clone;
                thread::spawn(move || {
                    let signals_handle = signals.handle();
                    let handle = thread::Builder::new().name(format!("{}.run", self_id)).spawn(move || {
                        for signal in signals.forever() {
                            println!("{}.run Received signal {:?}", self_id, signal);
                            match signal {
                                SIGINT | SIGQUIT | SIGTERM => {
                                    println!("{}.run Received signal {:?}", self_id, signal);
                                    println!("{}.run Application exit...", self_id);
                                    for (_id, service) in services_clone.lock().unwrap().all() {
                                        println!("{}.run Stopping service '{}'...", self_id, _id);
                                        service.lock().unwrap().exit();
                                        println!("{}.run Stopping service '{}' - Ok", self_id, _id);
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
                    }).unwrap();
                    handle.wait().unwrap();
                    signals_handle.close();
                });
            },
            Err(err) => {
                panic!("{}.run | Application hook system signals error; {:#?}", self_id, err);
            },
        }
        loop {
            let servece_ids: Vec<String> = self.handles.keys().cloned().collect();
            match servece_ids.first() {
                Some(service_id) => {
                    info!("{}.run | Waiting for service '{}' being finished...", self_id, service_id);
                    let (_, handles) = self.handles.remove_entry(service_id).unwrap();
                    handles.wait().unwrap();
                    info!("{}.run | Waiting for service '{}' being finished - Ok", self_id, service_id);
                },
                None => {
                    break;
                },
            }
        }
        info!("{}.run | Application exit - Ok\n", self_id);
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