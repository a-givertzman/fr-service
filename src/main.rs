#![allow(non_snake_case)]
#[cfg(test)]
mod tests;
mod core_;
mod task;
use log::debug;
use core_::{debug::debug_session::DebugSession, conf::conf_tree::ConfTree};


fn main() {
    DebugSession::init(core_::debug::debug_session::LogLevel::Debug);
    let testData = [
        r#"
            input1: const 177.3
            input2: point '/Path/Point.Name/'
            input3:
                fn count:
                    inputConst1: const '13.5'
                    inputConst2: newVar1
        "#,
    ];
    let mut conf: serde_yaml::Value = serde_yaml::from_str(testData[0]).unwrap();
    let map = conf.as_mapping_mut().unwrap();
    debug!("map: {:?}", &map);
    let removed = map.remove_entry("input2");
    debug!("removed: {:?}", &removed);
    debug!("map: {:?}", &map);
}
