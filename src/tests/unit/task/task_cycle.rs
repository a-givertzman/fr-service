#![allow(non_snake_case)]
use log::warn;
#[cfg(test)]
use log::{info, debug};
use std::{sync::Once, collections::HashMap, time::{Duration, Instant}};
use rand::Rng;

use crate::{core_::{conf::{fn_config::FnConfig, fn_config_type::FnConfigType, metric_config::MetricConfig, task_config::{TaskConfig, TaskNode}}, debug::debug_session::{DebugSession, LogLevel}, aprox_eq::aprox_eq::AproxEq}, task::task_cycle::TaskCycle};

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
fn test_task_cycle() {
    DebugSession::init(LogLevel::Trace);
    initOnce();
    initEach();
    println!("");
    info!("test_task_cycle");
    fn load(num: usize) {
        for _ in 0..num {
            let _: u128 = (1..=20).product();
        }
    }
    let testCycles = 100;
    let mut errors = 0; // a few errors will be ok, but not more then 5% of test cycles
    let errorsAllowed = (testCycles as f64 * 0.05) as usize;
    // const TARGET_CYCLE_INTERVALS: [u64; 4] = [1, 10, 100, 1000];
    // const TARGET_CYCLE_INTERVALS: [u64; 3] = [1, 10, 100];
    const TARGET_CYCLE_INTERVALS: [u64; 2] = [1, 10];
    for targetCycleInterval in TARGET_CYCLE_INTERVALS {  // ms
        let mut max: usize = 10;
        println!("");
        info!("target cycle interval: {} ms", targetCycleInterval);
        let length = targetCycleInterval.checked_ilog10().unwrap_or(0) + 1;
        let digits = 4 - length as usize;
        debug!("length: {:?}", length);
        debug!("aproxEq digits: {:?}", digits);
        info!("detecting load range...");
        let t = Instant::now();
        for _ in 0..9 {
            load(max);
        }
        let elapsed = t.elapsed().as_secs_f64();
        let targetK = ((targetCycleInterval as f64) / 1000.0)  / elapsed;
        max = (max as f64 * 10.0 * 1.2 * targetK) as usize;
        let t = Instant::now();
        load(max);
        info!("load range 1...{:?}", max);
        info!("elapsed for max load: {:?}", t.elapsed());
        let mut cycle = TaskCycle::new(Duration::from_millis(targetCycleInterval));
        for _ in 0..testCycles {
            let num = rand::thread_rng().gen_range(1..max);
            debug!("load: {}", num);
            cycle.start();
            let t = Instant::now();
            load(num);
            let mathElapsed = t.elapsed();
            debug!("math done in: {:?}", mathElapsed.as_secs_f64());
            cycle.wait();
            let cycleElapsed = t.elapsed();
            debug!("cycle done in: {:?}", cycleElapsed.as_secs_f64());
            if mathElapsed.as_millis() >= targetCycleInterval.into() {
                if ! mathElapsed.as_secs_f64().aproxEq(cycleElapsed.as_secs_f64(), digits) {
                    errors += 1;
                    warn!( 
                        "values must be aprox equals ({} digits): mathElapsed: {:?} != cycleElapsed {:?}", 
                        digits, 
                        mathElapsed.as_secs_f64(), 
                        cycleElapsed.as_secs_f64(),
                    );
                }
            } else {
                let targetInSecs = (targetCycleInterval as f64) / 1000.0;
                let digits = 4 - length as usize;
                if ! targetInSecs.aproxEq(cycleElapsed.as_secs_f64(), digits) {
                    errors += 1;
                    warn!(
                        "values must be aprox equals ({} digits): targetInSecs: {:?} != cycleElapsed {:?}", 
                        digits, 
                        targetInSecs, 
                        cycleElapsed.as_secs_f64(),
                    );
                }
            }
        }
        assert!(errors < errorsAllowed, "to much errors ({}), a few errors will be ok, but not more then 5% ({}) of test cycles", errors, errorsAllowed);
    }
}