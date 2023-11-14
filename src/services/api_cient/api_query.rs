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
    sql:  HashMap<String, String>,
    keepAlive: bool,
    debug: bool,
}
///
/// 
impl ApiQuery {
    ///
    /// Creates new instance of ApiQuery
    pub fn new(
        authToken: impl Into<String>,
        id: impl Into<String>,
        database: impl Into<String>,
        sql: impl Into<String>,
        keepAlive: bool,
        debug: bool
    ) -> Self {
        Self {
            authToken: authToken.into(),
            id: id.into(),
            sql: HashMap::from([
                ("database".to_string(), database.into()),
                ("sql".to_string(), sql.into()),
            ]),
            keepAlive,
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
    ///
    /// Returns ApiQuery in bytes, ready to write to socket
    pub fn asBytes(&self) -> Vec<u8> {
        self.toJson().as_bytes().to_vec()
    }
}