#![allow(non_snake_case)]

use std::{rc::Rc, cell::RefCell, sync::{Arc, Mutex}};

use log::debug;

use crate::{
    conf::fn_config::FnConfig, 
    services::{
        services::Services,
        task::{nested_function::metric_select::SqlMetric, task_nodes::TaskNodes}, 
    }, 
    core_::types::fn_in_out_ref::FnInOutRef,
};

///
/// 
pub struct MetricBuilder {

}
///
/// 
impl MetricBuilder {
    pub fn new(parent: &str, conf: &mut FnConfig, taskNodes: &mut TaskNodes, services: Arc<Mutex<Services>>) -> FnInOutRef {
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