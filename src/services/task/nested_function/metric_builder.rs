use std::{rc::Rc, cell::RefCell};

use log::debug;

use crate::{
    conf::fn_config::FnConfig, 
    services::{task::{nested_function::metric_select::MetricSelect, task_nodes::TaskNodes}, queues::queues::Queues},
};

use super::fn_::FnInOut;

///
/// 
pub struct MetricBuilder {

}
///
/// 
impl MetricBuilder {
    pub fn new(conf: &mut FnConfig, taskNodes: &mut TaskNodes, queues: &mut Queues) -> Rc<RefCell<Box<(dyn FnInOut)>>> {
        match conf.name.as_str() {
            "sqlSelectMetric" => {
                debug!("MetricBuilder.new | fnConf: {:?}: {:?}", conf.name, conf);
                Rc::new(RefCell::new(                    
                    Box::new(
                        MetricSelect::new(
                            conf, 
                            taskNodes,
                            queues,
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