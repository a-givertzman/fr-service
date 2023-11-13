#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::info;
    use std::{sync::{Once, Arc, Mutex}, thread, time::Duration};
    use crate::{core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, conf::api_client_config::ApiClientConfig, point::point_type::{ToPoint, PointType}}, services::api_cient::api_client::ApiClient}; 
    
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
    
    enum Value {
        Bool(bool),
        Int(i64),
        Float(f64),
        String(String),
    }
    impl Value {
        fn toPoint(&self, name: &str) -> PointType {
            match &self {
                Value::Bool(v) => v.clone().toPoint(name),
                Value::Int(v) => v.clone().toPoint(name),
                Value::Float(v) => v.clone().toPoint(name),
                Value::String(v) => v.clone().toPoint(name),
            }
        }
    }
    
    #[test]
    fn test_ApiClient() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_ApiClient");
        let path = "./src/tests/unit/api_client/api_client.yaml";
        let conf = ApiClientConfig::read(path);
        let apiClient = Arc::new(Mutex::new(ApiClient::new("test ApiClient", conf)));
        apiClient.lock().unwrap().run();
        let send = apiClient.lock().unwrap().getLink("api-link");
        let testData = vec![
            Value::Int(0),
            Value::Float(0.0),
            Value::Bool(true),
            Value::Bool(false),
            Value::String("test1".to_owned()),
            Value::String("test2".to_owned()),
        ];
        for value in testData {
            let point = value.toPoint("teset");
            send.send(point).unwrap();
        }
        thread::sleep(Duration::from_millis(10000));
        // assert!(false)
        // assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);
    }
}