use std::{sync::mpsc::Sender, thread::{self, JoinHandle}};
use rand::Rng;

use log::{debug, warn, info};

use crate::core_::point::point_type::{PointType, ToPoint};




// fn points() ->Vec<PointType> {
//     vec![
//         true.toPoint("bool1"),
//         13.toPoint("int1"),
//         43.toPoint("int1"),
//         false.toPoint("bool1"),
//         0.0077.toPoint("/path/Point.Name"),
//         65.toPoint("int1"),
//     ]
// }

pub struct TaskTestProducer {
    iterations: usize, 
    send: Vec<Sender<PointType>>,
    handle: Option<JoinHandle<()>>,
}
impl TaskTestProducer {
    pub fn new(iterations: usize, send: Sender<PointType>) -> Self {
        Self {
            iterations,
            send: vec![send],
            handle: None,
        }
    }
    ///
    /// 
    pub fn run(&mut self) {
        let iterations = self.iterations;
        let send = self.send.pop().unwrap();
        let _h = thread::Builder::new().name("name".to_owned()).spawn(move || {
            let name = "prodicer";
            debug!("TaskTestProducer({}).run | calculating step...", name);
            // let points = points();
            let mut random = rand::thread_rng();
            let max = 1.0;//points.len();
            let mut sent = 0;
            for _ in 0..iterations {
                let value = random.gen_range(0.0..max);
                let point = value.toPoint("/path/Point.Name");
                match send.send(point.clone()) {
                    Ok(_) => {
                        sent += 1;
                    },
                    Err(err) => {
                        warn!("TaskTestProducer({}).run | Error write to queue: {:?}", name, err);
                    },
                }
                // thread::sleep(Duration::from_micros(10));
            }
            info!("TaskTestProducer({}).run | Sent points: {}", name, sent);
            // thread::sleep(Duration::from_secs_f32(0.1));
            // debug!("TaskTestProducer({}).run | calculating step - done ({:?})", name, cycle.elapsed());
        }).unwrap();    
        self.handle = Some(_h);
    }
    pub fn join(self) {
        match &self.handle {
            Some(_) => {
                self.handle.unwrap().join().unwrap()
            },
            None => {},
        };
    }
}
