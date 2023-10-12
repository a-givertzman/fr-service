use crate::core_::conf::task_config::TaskConfig;

/// Task implements entity, which provides cyclically (by event) executing calculations
///  - executed in the cycle mode (current impl)
///  - executed event mode (future impl..)
///  - has some number of functions / variables / metrics or additional entities
pub struct Task {
    cycle: TaskCycle,
}

impl Task {
    pub fn new(cfg: TaskConfig) ->Self {
        Self {}
    }
    pub fn run() {

    }
}