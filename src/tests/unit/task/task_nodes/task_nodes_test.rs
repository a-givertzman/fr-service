#![allow(non_snake_case)]
use log::trace;
#[cfg(test)]

use log::{info, debug};
use std::{sync::{Once, mpsc::{Sender, Receiver, self}}, collections::HashMap};
use crate::{
    core_::{
        debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
        conf::task_config::TaskConfig, point::point_type::{ToPoint, PointType},
    }, 
    services::{task::{task_nodes::TaskNodes, nested_function::{fn_kind::FnKind, fn_count::{self}, fn_ge}}, queues::queues::Queues},
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
fn initEach() {
    fn_ge::resetCount();
    fn_count::resetCount();
}

#[test]
fn test_task_nodes() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    initEach();
    println!("");
    info!("test_task_nodes");
    let path = "./src/tests/unit/task/task_nodes/task.yaml";
    let mut queues = Queues::new();
    let (apiSend, _apiRecv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
    // queues.addRecvQueue("recv-queue", recv);
    queues.addSendQueue("api-queue", apiSend);
    let mut taskNodes = TaskNodes::new("test_task_nodes");
    let conf = TaskConfig::read(path);
    debug!("conf: {:?}", conf);
    taskNodes.buildNodes(conf, &mut queues);
    let testData = vec![
        (
            "/path/Point.Name1", 101, 
            HashMap::from([
                ("MetricSelect.out", "101, 1102, 0, 0"),
                ("FnCount1.out", "101"),
            ])
        ),
        (
            "/path/Point.Name1", 201, 
            HashMap::from([
                ("MetricSelect.out", "201, 1202, 0, 0"),
                ("FnCount1.out", "302"),
            ])
            
        ),
        (
            "/path/Point.Name1", 301, 
            HashMap::from([
                ("MetricSelect.out", "301, 1302, 0, 0"),
                ("FnCount1.out", "603"),
            ])
            
        ),
        (
            "/path/Point.Name2", 202, 
            HashMap::from([
                ("MetricSelect.out", "301, 1302, 202, 0"),
                ("FnGe1.out", "true"),
            ])
            
        ),
        (
            "/path/Point.Name3", 303, 
            HashMap::from([
                ("MetricSelect.out", "301, 1302, 202, 303"),
                ("FnGe1.out", "false"),
            ])
            
        ),
        (
            "/path/Point.Name3", 304, 
            HashMap::from([
                ("MetricSelect.out", "301, 1302, 202, 304"),
                ("FnGe1.out", "false"),
            ])
            
        ),
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
                    let outValue = match &out {
                        PointType::Bool(point) => point.value.to_string(),
                        PointType::Int(point) => point.value.to_string(),
                        PointType::Float(point) => point.value.to_string(),
                        PointType::String(point) => point.value.clone(),
                    };
                    debug!("TaskEvalNode.eval | evalNode '{}' out - '{}': {:?}", evalNode.name(), evalNodeOut.borrow().id(), out);
                    if evalNodeOut.borrow().kind() != &FnKind::Var {
                        if evalNodeOut.borrow().kind() != &FnKind::Var {
                            debug!("TaskEvalNode.eval | out.name: '{}'", out.name());
                            let target = targetValue.get(&out.name().as_str()).unwrap().to_string();
                            assert!(outValue == target, "\n   outValue: {} \ntargetValue: {}", outValue, target);
                        }
                    }
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