#![allow(non_snake_case)]
#[cfg(test)]
mod tests;
mod core_;
use log::debug;
use core_::{debug::debug_session::DebugSession, conf::conf_tree::ConfTree};


fn main() {
    DebugSession::init(core_::debug::debug_session::LogLevel::Debug);
    let testData = [
        r#"
            let newVar1:
                input1: const 177.3
                input2: point '/Path/Point.Name/'
                input3:
                    fn count:
                        inputConst1: const '13.5'
                        inputConst2: newVar1
        "#,
        // serde_yaml::from_str(r#"
        //     fn SqlMetric:
        //         initial: const 0
        //         sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"    
        //         inputs:
        //             input1:
        //                 let VarName2:
        //                     input: 
        //                         fn functionName:
        //                             initial: VarName2
        //                             input: 
        //                                 fn functionName:
        //                                     input1: const someValue
        //                                     input2: point '/path/Point.Name/'
        //                                     input: 
        //                                         fn functionName:
        //                                             input: point '/path/Point.Name/'        
        // "#),
        // r#"
        // serviceCMA:
        //     nodeType: API Client
        //     address: 127.0.0.1:8899
        //     cycle: 1000
        // serviceAPI:
        //     nodeType: API Client
        //     address: 127.0.0.1:8899
        //     cycle: 2000
        // serviceTask:
        //     cycle: 200
        // "#,
    ];
    let conf = serde_yaml::from_str(testData[0]).unwrap();
    let confTree = ConfTree::new(conf);
    inputs(&confTree);
    // for nodes in  {

    // }
}

fn inputs(confTree: &ConfTree) {
    match confTree.subNodes() {
        Some(nodes) => {
            for node in nodes {
                debug!("key: {:?}\t|\tnode: {:?}", &node.key, &node.conf);
                inputs(&node);
            }
        },
        None => {},
    };
}