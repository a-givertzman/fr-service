#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::debug;
    use std::sync::Once;
    use indexmap::IndexMap;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::conf::fn_::{fn_config::FnConfig, fn_conf_kind::FnConfKind, fn_conf_keywd::FnConfPointType};
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
    
    static INIT: Once = Once::new();
    
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
            }
        )
    }
    
    
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {
    
    }
    
    #[test]
    fn test_fn_config_new_valid() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test FnConfig | new valid";
        println!("\n{}", self_id);
        let test_data = [
            (
                r#"let newVar:
                    input: const '13.55'
                "#, 
                FnConfKind::Var( FnConfig { name: "newVar".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                    ("input".to_string(), FnConfKind::Const( FnConfig { name: "13.55".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::new() })),
                ]) })
            ),
            (
                r#"let newVar:
                    input fn count:
                        inputConst1: const '13.3'
                        inputConst2: const '13.7'
                "#, 
                FnConfKind::Var( FnConfig { name: "newVar".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                    ("input".to_string(), FnConfKind::Fn( FnConfig { name: "count".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                        ("inputConst1".to_string(), FnConfKind::Const( FnConfig { name: "13.3".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } )),
                        ("inputConst2".to_string(), FnConfKind::Const( FnConfig { name: "13.7".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } )),
                    ]) } )),
                ]) } )
            ),
            (
                r#"let newVar:
                    input1 fn count:
                        inputConst1: const '11.3'
                        inputConst2: const '12.7'"
                    input2 fn count:
                        inputConst1: const float '13.3'
                        inputConst2: const int '147'
                "#, 
                FnConfKind::Var( FnConfig { name: "newVar".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                    ("input1".to_string(), FnConfKind::Fn( FnConfig { name: "count".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                        ("inputConst1".to_string(), FnConfKind::Const( FnConfig { name: "11.3".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } )),
                        ("inputConst2".to_string(), FnConfKind::Const( FnConfig { name: "12.7".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } )),
                    ]) } )),
                    ("input2".to_string(), FnConfKind::Fn( FnConfig { name: "count".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                        ("inputConst1".to_string(), FnConfKind::Const( FnConfig { name: "13.3".to_string(), type_: FnConfPointType::Float, inputs: IndexMap::new() } )),
                        ("inputConst2".to_string(), FnConfKind::Const( FnConfig { name: "147".to_string(), type_: FnConfPointType::Int, inputs: IndexMap::new() } )),
                    ]) } )),
                ]) } )
            ),
            (
                r#"let VarName2:
                    param: "string param"
                    input fn functionName1:
                        initial: VarName2
                        input fn functionName2:
                            input1: const someValue
                            input2: point int '/path/Point.Name/'
                            input3 fn functionName3:
                                    input: point bool '/path/Point.Name/'
                "#,
                FnConfKind::Var( FnConfig { name: "VarName2".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                    ("param".to_string(), FnConfKind::Param( "string param".to_string() )),
                    ("input".to_string(), FnConfKind::Fn( FnConfig { name: "functionName1".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                        ("initial".to_string(), FnConfKind::Var( FnConfig { name: "VarName2".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } )),
                        ("input".to_string(), FnConfKind::Fn( FnConfig { name: "functionName2".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                            ("input1".to_string(), FnConfKind::Const( FnConfig { name: "someValue".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } )),
                            ("input2".to_string(), FnConfKind::Point( FnConfig { name: "/path/Point.Name/".to_string(), type_: FnConfPointType::Int, inputs: IndexMap::new() } )),
                            ("input3".to_string(), FnConfKind::Fn( FnConfig { name: "functionName3".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                ("input".to_string(), FnConfKind::Point( FnConfig { name: "/path/Point.Name/".to_string(), type_: FnConfPointType::Bool, inputs: IndexMap::new() } )),
                            ]) } )),
                        ])} )),
                    ]) } )),
                ]) } )
            ),
            (
                r#"fn metricName1:
                    initial: 0.123
                    table: SelectMetric_test_table_name
                    sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"    
                    input fn functionName1:
                        initial: const int 1234567
                        input fn functionName2:
                            input1: const someValue
                            input2: point int '/path/Point.Name/'
                            input3 fn functionName3:
                                    input: point bool '/path/Point.Name/'
                "#,
                FnConfKind::Fn( FnConfig { name: "metricName1".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                    ("initial".to_string(), FnConfKind::Param( "0.123".to_string() )),
                    ("table".to_string(), FnConfKind::Param( "SelectMetric_test_table_name".to_string() )),
                    ("sql".to_string(), FnConfKind::Param( "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';".to_string() )),
                    ("input".to_string(), FnConfKind::Fn( FnConfig { name: "functionName1".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                        ("initial".to_string(), FnConfKind::Const( FnConfig { name: "1234567".to_string(), type_: FnConfPointType::Int, inputs: IndexMap::new() } )),
                        ("input".to_string(), FnConfKind::Fn( FnConfig { name: "functionName2".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                            ("input1".to_string(), FnConfKind::Const( FnConfig { name: "someValue".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } )),
                            ("input2".to_string(), FnConfKind::Point( FnConfig { name: "/path/Point.Name/".to_string(), type_: FnConfPointType::Int, inputs: IndexMap::new() } )),
                            ("input3".to_string(), FnConfKind::Fn( FnConfig { name: "functionName3".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                ("input".to_string(), FnConfKind::Point( FnConfig { name: "/path/Point.Name/".to_string(), type_: FnConfPointType::Bool, inputs: IndexMap::new() } )),
                            ]) } )),
                        ])} )),
                    ]) } )),
                ]) } )
            ),
        ];
        for (value, target) in test_data {
            debug!("test value: {:?}", value);
            let conf: serde_yaml::Value = serde_yaml::from_str(value).unwrap();
            debug!("value: {:?}   |   conf: {:?}   |   target: {:?}", "_", conf, target);
            let mut vars = vec![];
            let fnConfig = FnConfig::from_yaml(self_id, &conf, &mut vars);
            debug!("\tfnConfig: {:?}", fnConfig);
            assert_eq!(fnConfig, target);
        }
    }
}
