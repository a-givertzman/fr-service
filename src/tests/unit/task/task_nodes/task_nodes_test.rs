#![allow(non_snake_case)]
use log::trace;
#[cfg(test)]

use log::{warn, info, debug};
use std::{sync::{Once, mpsc::{Sender, Receiver, self}}, time::{Duration, Instant}};
use crate::{
    core_::{
        debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
        conf::{fn_config::FnConfig, task_config::TaskConfig}, point::point_type::{ToPoint, PointType},
    }, 
    services::{task::{task_nodes::TaskNodes, nested_function::{nested_fn::NestedFn, metric_builder::MetricBuilder}, task_node_type::TaskNodeType}, queues::queues::Queues},
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
    let (apiSend, apiRecv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
    // queues.addRecvQueue("recv-queue", recv);
    queues.addSendQueue("api-queue", apiSend);
    let mut taskNodes = TaskNodes::new("test_task_nodes");
    let conf = TaskConfig::read(path);
    debug!("conf: {:?}", conf);
    taskNodes.buildNodes(conf, &mut queues);
    let testData = vec![
        ("/path/Point.Name1", 1.1,  0),
        ("/path/Point.Name1", 1.2,  0),
        ("/path/Point.Name1", 1.3,  0),
        ("/path/Point.Name2", 2.2,  0),
        ("/path/Point.Name3", 3.3,  0),
    ];
    for (name, value, targetValue) in testData {
        let point = value.toPoint(name);
        // let inputName = &point.name();
        debug!("input point name: {:?}  value: {:?}", name, value);
        match &taskNodes.getEvalNode(&name) {
            Some(evalNode) => {
                let input = evalNode.getInput();
                input.borrow_mut().add(point.clone());
                debug!("evalNode: {:?}", evalNode.name());
                // debug!("evalNode outs: {:?}", evalNode.getOuts());
                for evalNodeVar in evalNode.getVars() {
                    trace!("TaskEvalNode.eval | evalNode '{}' - var '{}' evaluating...", evalNode.name(), evalNodeVar.borrow().id());
                    evalNodeVar.borrow_mut().eval();
                    debug!("TaskEvalNode.eval | evalNode '{}' - var '{}' evaluated", evalNode.name(), evalNodeVar.borrow().id());
                };
                for evalNodeOut in evalNode.getOuts() {
                    trace!("TaskEvalNode.eval | evalNode '{}' out...", evalNode.name());
                    let out = evalNodeOut.borrow_mut().out();
                    debug!("TaskEvalNode.eval | evalNode '{}' out: {:?}", evalNode.name(), out);
                };
            },
            None => {
                panic!("input {:?} - not found in the current taskStuff", &name)
            },
        };
    }        
}
// clear && cargo test -- --test-threads=1 --show-output
// clear && cargo test task_nodes_test -- --test-threads=1 --show-output