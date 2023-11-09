#![allow(non_snake_case)]

use std::collections::HashMap;

use serde::Serialize;

///
/// Wrap a structure of an API query
/// {
///     "auth_token": "123zxy456!@#",
///     "id": "123",
///     "keep-alive": true,
///     "sql": {
///         "database": "database name",
///         "sql": "Some valid sql query"
///     },
///     "debug": false
/// }
#[derive(Serialize)]    // , Deserialize
pub struct ApiQuery {
    authToken: String,
    id: String,
    keepAlive: bool,
    sql:  HashMap<String, String>,
    debug: bool,
}
///
/// 
impl ApiQuery {
    ///
    /// Creates new instance of ApiQuery
    pub fn new(
        authToken: String,
        id: String,
        keepAlive: bool,
        database: String,
        sql: String,
        debug: bool
    ) -> Self {
        Self {
            authToken,
            id,
            keepAlive,
            sql: HashMap::from([
                ("database".to_string(), database),
                ("sql".to_string(), sql),
            ]),
            debug,
        }
    }
    ///
    /// Returns a JSON representation of the ApiQuery
    pub fn toJson(&self) -> String {
        match serde_json::to_string(self) {
            Ok(json) => json,
            Err(err) => panic!("ApiQuery.toJson | convertion error: {:?}", err),
        }
    }
}