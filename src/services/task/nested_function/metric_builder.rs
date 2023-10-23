use std::{sync::Arc, rc::Rc, cell::RefCell};

use log::debug;

use crate::{
    core_::conf::fn_config::FnConfig, 
    services::task::{nested_function::metric_select::MetricSelect, task_stuff::TaskStuff},
};

use super::fn_::FnInOut;

///
/// 
pub struct MetricBuilder {

}
///
/// 
impl MetricBuilder {
    pub fn new(conf: &mut FnConfig, taskStuff: &mut TaskStuff) -> Rc<RefCell<Box<(dyn FnInOut)>>> {
        match conf.name.as_str() {
            "sqlSelectMetric" => {
                debug!("MetricBuilder.new | fnConf: {:?}: {:?}", conf.name, conf);
                Rc::new(RefCell::new(                    
                    Box::new(
                        MetricSelect::new(
                            conf, 
                            taskStuff
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