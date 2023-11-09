#![allow(non_snake_case)]
#[cfg(test)]

use log::{warn, info, debug};
use serde_json::json;
use std::{sync::Once, time::{Duration, Instant}};
use crate::{core_::debug::debug_session::{DebugSession, LogLevel, Backtrace}, services::api_cient::api_query::ApiQuery}; 

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
fn test_api_query() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    initEach();
    println!("");
    info!("test_api_query");
    let testData = vec![
        (
            ApiQueryStruct { 
                authToken: "123zxy456!@#".to_string(), 
                id: "123".to_string(), 
                database: "database name".to_string(), 
                sql: "Some valid sql query".to_string(), 
                keepAlive: true, 
                debug: false},
            r#"{"auth_token":"123zxy456!@#","id":"123","sql":{"database":"database name","sql":"Some valid sql query"},"keep-alive":true,"debug":false}"#
        ),
    ];
    for (value, target) in testData {
        let query = ApiQuery::new(
            value.authToken,
            value.id,
            value.database,
            value.sql,
            value.keepAlive,
            value.debug,
        );
        let json = query.toJson().to_string();
        let json = json!(json);
        let target = json!(target);
        assert!(json.as_object() == target.as_object(), "\n  json: {:?}\ntarget: {:?}", json, target);
    }
}

struct ApiQueryStruct {
    authToken: String,
    id: String,
    database: String,
    sql: String,
    keepAlive: bool,
    debug: bool,
}
