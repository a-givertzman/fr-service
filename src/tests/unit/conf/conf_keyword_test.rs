#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::debug;
    use std::{sync::Once, str::FromStr};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::conf::conf_keywd::{ConfKeywd, ConfKeywdValue, ConfKind};
    
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
    fn test_create_valid() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        println!("test_create_valid");
        // let (initial, switches) = initEach();
        let testData = vec![
            ("service ApiClient", ConfKeywd::Service( ConfKeywdValue {prefix: String::new(), kind: ConfKind::Service, name: String::from("ApiClient")} )),
            ("service MultiQueue", ConfKeywd::Service( ConfKeywdValue {prefix: String::new(), kind: ConfKind::Service, name: String::from("MultiQueue")} )),
            ("task Task1", ConfKeywd::Task( ConfKeywdValue {prefix: String::new(), kind: ConfKind::Task, name: String::from("Task1")} )),
            ("task task1", ConfKeywd::Task( ConfKeywdValue {prefix: String::new(), kind: ConfKind::Task, name: String::from("task1")} )),
            ("in queue queue", ConfKeywd::Queue( ConfKeywdValue {prefix: String::from("in"), kind: ConfKind::Queue, name: String::from("queue")} )),
            ("in link link", ConfKeywd::Link( ConfKeywdValue {prefix: String::from("in"), kind: ConfKind::Link, name: String::from("link")} )),
            ("in queue in-queue", ConfKeywd::Queue( ConfKeywdValue {prefix: String::from("in"), kind: ConfKind::Queue, name: String::from("in-queue")} )),
            ("out queue", ConfKeywd::Queue( ConfKeywdValue {prefix: String::from("out"), kind: ConfKind::Queue, name: String::new()} )),
            ("out link", ConfKeywd::Link( ConfKeywdValue {prefix: String::from("out"), kind: ConfKind::Link, name: String::new()} )),
        ];
        for (value, target) in testData {
            let fnConfigType = ConfKeywd::from_str(value).unwrap();
            debug!("value: {:?}   |   ConfKind: {:?}   |   target: {:?}", value, fnConfigType, target);
            assert_eq!(fnConfigType, target);
        }
    }
    
    // #[test]
    // fn test_create_invalid() {
    //     DebugSession::init(LogLevel::Info, Backtrace::Short);
    //     initOnce();
    //     initEach();
    //     info!("test_create_invalid");
    //     // let (initial, switches) = initEach();
    //     let testData: Vec<(&str, Result<&str, ()>)> = vec![
    //         ("fn:name", Err(())),
    //         ("fn\nname", Err(())),
    //         ("fn: name", Err(())),
    //         ("fn :name", Err(())),
    //         ("fn : name", Err(())),
    //         ("Fn name", Err(())),
    //         ("FN name", Err(())),
    //         ("fnName", Err(())),
    //         ("fn_name", Err(())),
    //         ("let:name", Err(())),
    //         ("Let name", Err(())),
    //         ("LET name", Err(())),
    //         ("letName", Err(())),
    //         ("let_name", Err(())),
    //         ("const:name", Err(())),
    //         ("Const name", Err(())),
    //         ("CONST name", Err(())),
    //         ("constName", Err(())),
    //         ("const_name", Err(())),
    //         ("point:name", Err(())),
    //         ("Point name", Err(())),
    //         ("POINT name", Err(())),
    //         ("pointName", Err(())),
    //         ("point_name", Err(())),
    //     ];
    //     for (value, target) in testData {
    //         let fnConfigType = ConfKeywd::from_str(value);
    //         debug!("value: {:?}   |   fnConfigType: {:?}   |   target: {:?}", value, fnConfigType, target);
    //         assert_eq!(fnConfigType.is_err(), true);
    //     }
    // }
}
