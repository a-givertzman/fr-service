#![allow(non_snake_case)]
use log::trace;
#[cfg(test)]

use log::{warn, info, debug};
use std::{sync::Once, time::{Duration, Instant}, rc::Rc, cell::RefCell};
use crate::{
    core_::{
        debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
        conf::{fn_config::FnConfig, task_config::TaskConfig}, types::fn_in_out_ref::FnInOutRef, point::point_type::ToPoint,
    }, 
    services::{task::{task_nodes::TaskNodes, nested_function::{metric_select::MetricSelect, nested_fn::NestedFn, metric_builder::MetricBuilder}, task_node_type::TaskNodeType}, queues::queues::Queues},
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
///  - Rc<RefCell<Box<dyn FnInOut>>>...
fn initEach(conf: &mut FnConfig, taskNodes: &mut TaskNodes, queues: &mut Queues) -> TaskNodeType {
    match conf.fnKind {
        crate::core_::conf::fn_conf_kind::FnConfKind::Fn => TaskNodeType::Metric(NestedFn::new(conf, taskNodes, queues)),
        crate::core_::conf::fn_conf_kind::FnConfKind::Var => TaskNodeType::Var(NestedFn::new(conf, taskNodes, queues)),
        crate::core_::conf::fn_conf_kind::FnConfKind::Const => panic!("Const builder not implemented"),
        crate::core_::conf::fn_conf_kind::FnConfKind::Point => panic!("Const builder not implemented"),
        crate::core_::conf::fn_conf_kind::FnConfKind::Metric => TaskNodeType::Metric(MetricBuilder::new(conf, taskNodes, queues)),
        crate::core_::conf::fn_conf_kind::FnConfKind::Param => panic!("Const builder not implemented"),
    }
}

#[test]
fn test_task_nodes() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    println!("");
    info!("test_task_nodes");
    let path = "./src/tests/unit/task/task_nodes/task.yaml";
    let mut queues = Queues::new();
    let mut taskNodes = TaskNodes::new();
    let conf = TaskConfig::read(path);
    debug!("conf: {:?}", conf);
    for (_nodeName, mut nodeConf) in conf.nodes {
        taskNodes.beginNewNode();
        let metric = initEach(
            &mut nodeConf, 
            &mut taskNodes,
            &mut queues,
        );
        taskNodes.finishNewNode(metric);
    }
    let testData = vec![
        ("/path/Point.Name1",   0),
        ("/path/Point.Name",    0),
    ];
    for (name, targetValue) in testData {
        let point = 0.001.toPoint(name);
        // let inputName = &point.name();
        match &taskNodes.getEvalNode(&name) {
            Some(evalNode) => {
                let input = evalNode.getInput();
                input.borrow_mut().add(point.clone());
                for out in evalNode.getOuts() {
                    match out {
                        TaskNodeType::Var(var) => {
                            var.borrow_mut().eval();
                            debug!("var evalueted: {:?}", var.borrow_mut().out());
                        },
                        TaskNodeType::Metric(metric) => {
                            // debug!("input: {:?}", &input);
                            let state = metric.borrow_mut().out();
                            let out = state.asString().value;
                            trace!("out: {}", out);                    
                            debug!("value: {:?}   |   state: {:?}", point.asFloat().value, state.asString().value);
                            assert_eq!(
                                out, 
                                format!("UPDATE SelectMetric_test_table_name SET kind = '{:.1}' WHERE id = '{}';",targetValue, 1.11),
                            );
                        },
                    }
                }
            },
            None => {
                panic!("input {:?} - not found in the current taskStuff", &name)
            },
        };
    }        
}
// clear && cargo test -- --test-threads=1 --show-output
// clear && cargo test task_nodes_test -- --test-threads=1 --show-output