#![allow(non_snake_case)]
use indexmap::IndexMap;
#[cfg(test)]
use log::{debug, info};
use std::sync::Once;

use crate::{
    core_::{debug::debug_session::*, testing::test_stuff::test_value::Value},
    conf::conf_tree::ConfTree,
};

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



#[derive(Clone, Debug, PartialEq, Eq)]
enum Node {
    Map(IndexMap<String, Node>),
    End(ConfTree),
}
impl Node {
    fn asMap(&self) ->&IndexMap<String, Node> {
        match self {
            Node::Map(map) => map,
            Node::End(_) => panic!("is not a map"),
        }
    }
}


#[test]
fn test_config_tree_valid() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    initOnce();
    initEach();
    info!("test_config_tree_valid");
    // let (initial, switches) = initEach();
    let testData: Vec<(&str, Node)> = vec![
        // (
        //     r#"let newVar:
        //         input const '13.55'
        //     "#, 
        //     None
        // ),
        // (
        //     r#"let newVar:
        //         input fn count:
        //             inputConst1 const '13.3'
        //             inputConst2 const '13.7'
        //     "#, 
        //     None
        // ),
        // (
        //     r#"let newVar:
        //         input1 fn count:
        //             inputConst1 const '11.3'
        //             inputConst2 const '12.7'"
        //         input2 fn count:
        //             inputConst1 const '13.3'
        //             inputConst2 const '14.7'
        //     "#, 
        //     None
        // ),
        (
            r#"
                let newVar2:
                    input: const 2.2
                let newVar3:
                    input: const 3.3
                let newVar1:
                    input: const 1.1
            "#,
            Node::Map(IndexMap::from([
                (String::from("let newVar2"), Node::Map(IndexMap::from([
                    (String::from("input"), Node::End(ConfTree { key: String::from("input"), conf: serde_yaml::from_str("const 2.2").unwrap() })), 
                ]))),
                (String::from("let newVar3"), Node::Map(IndexMap::from([
                    (String::from("input"), Node::End(ConfTree { key: String::from("input"), conf: serde_yaml::from_str("const 3.3").unwrap() })), 
                ]))),
                (String::from("let newVar1"), Node::Map(IndexMap::from([
                    (String::from("input"), Node::End(ConfTree { key: String::from("input"), conf: serde_yaml::from_str("const 1.1").unwrap() })), 
                ]))),
            ]))
        ),
        (
            r#"
                let newVar1:
                    input1: const 177.3
                    input2: point '/Path/Point.Name/'
                    input3:
                        fn count:
                            inputConst1: const '13.5'
                            inputConst2: newVar1
            "#,
            Node::Map(IndexMap::from([
                (String::from("let newVar1"), Node::Map(IndexMap::from([
                    (String::from("input2"), Node::End(ConfTree { key: String::from("input2"), conf: serde_yaml::from_str("point '/Path/Point.Name/'").unwrap() })), 
                    (String::from("input3"), Node::Map(IndexMap::from([
                        (String::from("fn count"), Node::Map(IndexMap::from([
                            (String::from("inputConst1"), Node::End(ConfTree { key: String::from("inputConst1"), conf: serde_yaml::from_str("const '13.5'").unwrap() })), 
                            (String::from("inputConst2"), Node::End(ConfTree { key: String::from("inputConst2"), conf: serde_yaml::from_str("newVar1").unwrap() }))
                        ])))
                    ]))), 
                    (String::from("input1"), Node::End(ConfTree { key: String::from("input1"), conf: serde_yaml::from_str("const 177.3").unwrap() }))
                ]))),
            ]))
        ),
        // (
        //     r#"
        //         metric sqlSelectMetric:
        //             initial: const 0
        //             sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"    
        //             inputs:
        //                 input1:
        //                     let VarName2:
        //                         input: 
        //                             fn functionName:
        //                                 initial: VarName2
        //                                 input: 
        //                                     fn functionName:
        //                                         input1: const someValue
        //                                         input2: point '/path/Point.Name/'
        //                                         input: 
        //                                             fn functionName:
        //                                                 input: point '/path/Point.Name/'        
        //     "#,
        //     None
        // ),
        (
            r#"
                serviceCMA:
                    nodeType: API Client
                    address: 127.0.0.1:8899
                    cycle: 1000
                serviceAPI:
                    nodeType: API Client
                    address: 127.0.0.1:8899
                    cycle: 2000
                serviceTask:
                    cycle: 200
            "#,
            Node::Map(IndexMap::from([
                (String::from("serviceCMA"), Node::Map(IndexMap::from([
                    (String::from("nodeType"), Node::End(ConfTree { key: String::from("nodeType"), conf: serde_yaml::from_str("API Client").unwrap() })), 
                    (String::from("address"), Node::End(ConfTree { key: String::from("address"), conf: serde_yaml::from_str("127.0.0.1:8899").unwrap() })), 
                    (String::from("cycle"), Node::End(ConfTree { key: String::from("cycle"), conf: serde_yaml::from_str("1000").unwrap() })),
                ]))), 
                (String::from("serviceAPI"), Node::Map(IndexMap::from([
                    (String::from("nodeType"), Node::End(ConfTree { key: String::from("nodeType"), conf: serde_yaml::from_str("API Client").unwrap() })), 
                    (String::from("address"), Node::End(ConfTree { key: String::from("address"), conf: serde_yaml::from_str("127.0.0.1:8899").unwrap() })), 
                    (String::from("cycle"), Node::End(ConfTree { key: String::from("cycle"), conf: serde_yaml::from_str("2000").unwrap() })),
                ]))), 
                (String::from("serviceTask"), Node::Map(IndexMap::from([
                    (String::from("cycle"), Node::End(ConfTree { key: String::from("cycle"), conf: serde_yaml::from_str("200").unwrap() }))
                ]))),
            ]))
        ),
    ];
    for (value, target) in testData {
        // debug!("test value: {:?}", value);
        let conf: serde_yaml::Value = serde_yaml::from_str(value).unwrap();
        debug!("test conf: {:?}", conf);
        // let conf = testData.get("/").unwrap();
        let confTree = ConfTree::newRoot(conf);
        debug!("confTree: {:?}", confTree);
        let res = inputs(&confTree);
        debug!("result: {:?}", res);
        println!("\n");
        assert_eq!(res, target);
        let mut target = target.asMap().iter();
        for (_name, node) in res.asMap() {
            let (_, targetNode) = target.next().unwrap();
            assert_eq!(node, targetNode);
        }
    }
}

fn inputs(confTree: &ConfTree) -> Node {
    match confTree.subNodes() {
        Some(nodes) => {
            let mut res: IndexMap<String, Node> = IndexMap::new();
            for node in nodes {
                debug!("key: {:?}\t|\tnode: {:?}", &node.key, &node.conf);
                let subRes = inputs(&node);
                res.insert(node.key.clone(), subRes);
            }
            return Node::Map(res)
        },
        None => {
            return Node::End(confTree.clone());
        },
    };
}



#[test]
fn test_config_tree_as_type() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    initOnce();
    initEach();
    info!("test_config_tree_valid");
    // let (initial, switches) = initEach();
    let testData: Vec<(&str, IndexMap<&str, Value>)> = vec![
        (
            r#"
                boolTrue: true
                boolFalse: false
                int1: 177
                int2: -177
                float1: 177.3
                float2: -177.3
                string1: /Path/Point.Name/
                string2: '/Path/Point.Name/'
                string3: "/Path/Point.Name/"
            "#,
            IndexMap::from([
                ("boolTrue", Value::Bool(true)),
                ("boolFalse", Value::Bool(false)),
                ("int1", Value::Int(177)),
                ("int2", Value::Int(-177)),
                ("float1", Value::Float(177.3)),
                ("float2", Value::Float(-177.3)),
                ("string1", Value::String("/Path/Point.Name/".to_string())),
                ("string2", Value::String("/Path/Point.Name/".to_string())),
                ("string3", Value::String("/Path/Point.Name/".to_string())),
            ])
        ),

    ];
    for (value, targets) in testData {
        // debug!("test value: {:?}", value);
        let conf: serde_yaml::Value = serde_yaml::from_str(value).unwrap();
        debug!("test conf: {:?}", conf);
        let confTree = ConfTree::newRoot(conf);
        for (key, target) in targets {
            match target {
                Value::Bool(targetValue) => assert_eq!(confTree.asBool(key).unwrap(), targetValue),
                Value::Int(targetValue) => assert_eq!(confTree.asI64(key).unwrap(), targetValue),
                Value::Float(targetValue) => assert_eq!(confTree.asF64(key).unwrap(), targetValue),
                Value::String(targetValue) => assert_eq!(confTree.asStr(key).unwrap(), targetValue),
            }
        }
    }
}
