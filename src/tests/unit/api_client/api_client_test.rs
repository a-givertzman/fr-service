#![allow(non_snake_case)]
#[cfg(test)]

use log::info;
use std::sync::Once;
use crate::{core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, conf::api_client_config::ApiClientConfig}, services::api_cient::api_client::ApiClient}; 

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
fn test_ApiClient() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    initEach();
    println!("");
    info!("test_ApiClient");
    let path = "./src/tests/unit/api_client/api_client.yaml";
    let conf = ApiClientConfig::read(path);
    let apiClient = ApiClient::new("test ApiClient", conf);
    let send = apiClient.getLink("api-link");
    // assert!(false)
    // assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);
}