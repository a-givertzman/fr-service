#![allow(non_snake_case)]
#[cfg(test)]

use log::info;
use serde_json::json;
use std::sync::Once;
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
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    initOnce();
    initEach();
    println!("");
    info!("test_api_query");
    let testData = vec![
        (
            ApiQueryStruct { 
                authToken: "123zxy456!@#".to_string(), 
                id: "111".to_string(), 
                database: "database name".to_string(), 
                sql: "Some valid sql query".to_string(), 
                keepAlive: true, 
                debug: false},
            r#"{"authToken":"123zxy456!@#","id":"111","sql":{"database":"database name","sql":"Some valid sql query"},"keepAlive":true,"debug":false}"#
        ),
        (
            ApiQueryStruct { 
                authToken: "123zxy456!@#".to_string(), 
                id: "112".to_string(), 
                database: "database name".to_string(), 
                sql: "Some valid sql query".to_string(), 
                keepAlive: false, 
                debug: true},
            r#"{"authToken":"123zxy456!@#","id":"112","sql":{"database":"database name","sql":"Some valid sql query"},"keepAlive":false,"debug":true}"#
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
