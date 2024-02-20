#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::trace;
    use std::{sync::Once, env};
    use indexmap::IndexMap;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::conf::{fn_config::FnConfig, fn_conf_kind::FnConfKind, fn_conf_keywd::FnConfPointType};
    
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
    fn test_fn_config_read_valid() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test FnConfig | read valid";
        println!("{}", selfId);
        let target = FnConfKind::Var( FnConfig { 
            name: "VarName2".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                ("input".to_string(), FnConfKind::Fn( FnConfig { 
                    name: "functionName".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                        ("initial".to_string(), FnConfKind::Var( FnConfig { 
                            name: "VarName2".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::new() 
                        } )),
                        ("input".to_string(), FnConfKind::Fn( FnConfig { 
                            name: "functionName".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                ("input".to_string(), FnConfKind::Fn( FnConfig { 
                                    name: "functionName".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                        ("input".to_string(), FnConfKind::Point( FnConfig { 
                                            name: "/path/Point.Name/".to_string(), type_: FnConfPointType::Bool, inputs: IndexMap::from([]) 
                                        } )),
                                    ]),
                                } )),
                                ("input2".to_string(), FnConfKind::Point( FnConfig { 
                                    name: "/path/Point.Name/".to_string(), type_: FnConfPointType::Float, inputs: IndexMap::from([]) 
                                } )),
                                ("input1".to_string(), FnConfKind::Const( FnConfig { 
                                    name: "someValue".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([]) 
                                } )),
                            ]),
                        } )),
                    ]),
                } )),
            ]),
        } );
        
        // let (initial, switches) = initEach();
        trace!("dir: {:?}", env::current_dir());
        let path= "./src/tests/unit/conf/fn_config/fn_config_test.yaml";
        let fnConfig = FnConfig::read(selfId, path);
        trace!("fnConfig: {:?}", fnConfig);
        assert_eq!(fnConfig, target);
    }
}    
