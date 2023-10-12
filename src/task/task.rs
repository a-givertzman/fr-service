use std::thread;
use std::time::Duration;

use crate::core_::conf::task_config::TaskConfig;
use crate::task::task_cycle::TaskCycle;

/// Task implements entity, which provides cyclically (by event) executing calculations
///  - executed in the cycle mode (current impl)
///  - executed event mode (future impl..)
///  - has some number of functions / variables / metrics or additional entities
pub struct Task {
    cycle: TaskCycle,
}
///
/// 
impl Task {
    ///
    /// 
    pub fn new(cfg: TaskConfig) ->Self {
        Self {
            cycle: TaskCycle::new(Duration::from_millis(cfg.cycle))
        }
    }
    ///
    /// 
    pub fn run() {
        thread::Builder::new().name("name".to_owned()).spawn(|| {

        }).unwrap();
    }
}