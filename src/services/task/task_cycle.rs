use std::{time::{Duration, Instant}, thread};

use log::debug;
///
/// TaskCycle - provides exact time interval in ms / us (future posible implementation)
///  - creates with Duration of interval
///  - method start() - begins countdown
///  - method wait() - awaiting remainder of the specified interval if not elapsed
/// 
/// [How to sleep for a few microseconds](https://stackoverflow.com/questions/4986818/how-to-sleep-for-a-few-microseconds)
pub struct TaskCycle {
    instant: Instant,
    interval: Duration,
}
///
/// 
impl TaskCycle {
    ///
    /// creates TaskCycle with Duration of interval
    pub fn new(interval: Duration) ->Self {
        Self {
            instant: Instant::now(),
            interval,
        }
    }
    ///
    /// 
    pub fn start(&mut self) {
        self.instant = Instant::now();
    }
    ///
    /// 
    pub fn wait(&self) {
        let elapsed = self.instant.elapsed();
        if elapsed < self.interval {
            let remainder = self.interval - elapsed;
            debug!("TaskCycle.wait | waiting: {:?}", remainder);
            thread::sleep(remainder);
        }
    }
    ///
    /// returns current ellapsed time
    pub fn elapsed(&mut self) ->Duration {
        self.instant.elapsed()
    }
}