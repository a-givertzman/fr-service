use log::debug;

use crate::{core_::conf::metric_config::MetricConfig, services::task::{task::TaskNode, nested_function::{metric_select::MetricSelect, nested_fn::NestedFn}}};

use super::fn_inputs::FnInputs;

///
/// 
pub struct MetricBuilder {

}
///
/// 
impl MetricBuilder {
    pub fn new(conf: MetricConfig, inputs: &mut FnInputs) -> TaskNode {
        match conf.name.as_str() {
            "sqlSelectMetric" => {
                debug!("MetricBuilder.new | fnConf: {:?}: {:?}", conf.name, conf);
                let (inputName, inputConf) = conf.inputs.iter_mut().next().unwrap();
                TaskNode::String(
                    MetricSelect::new(
                        conf.initial, 
                        NestedFn::new(inputConf, initial, inputs),
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