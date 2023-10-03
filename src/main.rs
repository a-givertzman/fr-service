#![allow(non_snake_case)]
#[cfg(test)]
// #[path = "./tests"]
mod tests;

mod core;

use std::{env, collections::HashMap};

use log::{info, debug};

use crate::core::nested_function::fn_config::{FnConfig, FnConfigType};




fn main() {
    env::set_var("RUST_LOG", "debug");  // off / error / warn / info / debug / trace
    // env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    info!("test_create_valid_fn");
    // let (initial, switches) = initEach();
    let testData = [
        (serde_yaml::from_str(r#"
            let newVar1:
                input1:
                    fn count:
                        inputConst1: const '13.5'
                        inputConst2: newVar1
        "#), 
        FnConfig{ fnType: FnConfigType::Const, name: "".to_string(), inputs: HashMap::new() }),
                    // input2:
                    //     fn count:
                    //         inputConst1: const '13.5'
                    //         inputConst2: const '13.5'
    ];
    for (value, target) in testData {
        debug!("test value: {:?}", value);
        let conf: serde_yaml::Value = value.unwrap();
        // let conf = testData.get("/").unwrap();

        debug!("value: {:?}   |   conf: {:?}   |   target: {:?}", "_", conf, target);
        // let fnKeyword = FnConfigKeyword::from_str(conf.as_str().unwrap()).unwrap();
        // debug!("\tfnKeyword: {:?}", fnKeyword);
        let mut vars = vec![];
        let fnConfig = FnConfig::new(&conf, &mut vars);
        debug!("\tfnConfig: {:?}", fnConfig);
        debug!("\tvars: {:?}", vars);
        // assert_eq!(fnConfigType, target);
    }

}
