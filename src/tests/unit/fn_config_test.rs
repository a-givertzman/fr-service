#![allow(non_snake_case)]
#[cfg(test)]
use log::{trace, info};
use std::{sync::Once, env};

use crate::{
    tests::unit::init::{TestSession, LogLevel},
    // core::nested_function::fn_config::FnConfig,
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
///  - ...
fn initEach() -> () {

}

#[test]
fn test_fn_config() {
    TestSession::init(LogLevel::Trace);
    initOnce();
    initEach();
    info!("test_single");
    // let (initial, switches) = initEach();
    trace!("dir: {:?}", env::current_dir());
    let path = "./src/tests/unit/fn_config_test.yaml";
    // let fnConfig = FnConfig::read(path);
    // trace!("fnConfig: {:?}", fnConfig);
}

