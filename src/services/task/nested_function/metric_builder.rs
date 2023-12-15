#![allow(non_snake_case)]

use std::{rc::Rc, cell::RefCell, sync::{Arc, Mutex}};

use log::debug;

use crate::{
    conf::fn_config::FnConfig, 
    services::{task::{nested_function::metric_select::SqlMetric, task_nodes::TaskNodes}, services::Services},
};

use super::fn_::FnInOut;

///
/// 
pub struct MetricBuilder {

}
///
/// 
impl MetricBuilder {
    pub fn new(parent: &str, conf: &mut FnConfig, taskNodes: &mut TaskNodes, services: Arc<Mutex<Services>>) -> Rc<RefCell<Box<(dyn FnInOut)>>> {
        match conf.name.as_str() {
            "SqlMetric" => {
                debug!("MetricBuilder.new | fnConf: {:?}: {:?}", conf.name, conf);
                Rc::new(RefCell::new(                    
                    Box::new(
                        SqlMetric::new(
                            parent,
                            conf, 
                            taskNodes,
                            services,
                        )
                    )
                ))
            },
            "sqlUpdateMetric" => {
                panic!("MetricBuilder.new | Metric sqlUpdateMetric not implemented yet");
            },
            "sqlInsertMetric" => {
                panic!("MetricBuilder.new | Metric sqlInsertMetric not implemented yet");
            },
            _ => {
                panic!("MetricBuilder.new | Unknown metric name: {:?}", conf.name);
            },
        }
    }
}