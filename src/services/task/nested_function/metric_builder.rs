use std::sync::Arc;

use log::debug;

use crate::{
    core_::conf::fn_config::FnConfig, 
    services::task::{task::TaskNode, nested_function::metric_select::MetricSelect},
};

use super::fn_inputs::FnInputs;

///
/// 
pub struct MetricBuilder {

}
///
/// 
impl MetricBuilder {
    pub fn new(conf: &mut FnConfig, inputs: &mut FnInputs) -> TaskNode {
        match conf.name.as_str() {
            "sqlSelectMetric" => {
                debug!("MetricBuilder.new | fnConf: {:?}: {:?}", conf.name, conf);
                TaskNode::Metric(
                    Arc::new(
                        MetricSelect::new(
                            conf, 
                            inputs
                        )
                    )
                )
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