use std::sync::Arc;

use log::debug;

use crate::{core_::conf::metric_config::MetricConfig, services::task::{task::TaskNode, nested_function::{metric_select::{MetricSelect, FnMetric}, nested_fn::NestedFn}}};

use super::fn_inputs::FnInputs;

///
/// 
pub struct MetricBuilder {

}
///
/// 
impl MetricBuilder {
    pub fn new(conf: &mut MetricConfig, inputs: &mut FnInputs) -> TaskNode {
        match conf.name.as_str() {
            "sqlSelectMetric" => {
                debug!("MetricBuilder.new | fnConf: {:?}: {:?}", conf.name, conf);
                TaskNode::String(
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