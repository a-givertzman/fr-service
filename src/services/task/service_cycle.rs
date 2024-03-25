use std::{time::{Duration, Instant}, thread};
use log::{error, trace};
///
/// ServiceCycle - provides exact time interval in ms / us (future posible implementation)
///  - creates with Duration of interval
///  - method start() - begins countdown
///  - method wait() - awaiting remainder of the specified interval if not elapsed
/// 
/// [How to sleep for a few microseconds](https://stackoverflow.com/questions/4986818/how-to-sleep-for-a-few-microseconds)
pub struct ServiceCycle {
    id: String,
    instant: Instant,
    interval: Duration,
}
///
/// 
impl ServiceCycle {
    ///
    /// creates ServiceCycle with Duration of interval
    pub fn new(parent: &str, interval: Duration) ->Self {
        Self {
            id: format!("{}/ServiceCycle", parent),
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
            trace!("{}.wait | waiting: {:?}", self.id, remainder);
            thread::sleep(remainder);
        }
        if elapsed > self.interval {
            error!("{}.wait | exceeded: {:?}", self.id, elapsed - self.interval);
        }
    }
    ///
    /// returns current ellapsed time
    pub fn elapsed(&mut self) ->Duration {
        self.instant.elapsed()
    }
}