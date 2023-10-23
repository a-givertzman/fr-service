use std::{sync::mpsc::Sender, thread::{self, JoinHandle}, time::Duration};
use rand::Rng;

use log::{debug, warn, info};

use crate::core_::{point::{point_type::PointType, point::Point}, types::bool::Bool};




fn points() ->Vec<PointType> {
    vec![
        PointType::Bool(  Point { value: Bool(true),   name:String::from("bool1"),  status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value: 13,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value: 43,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Bool(  Point { value: Bool(false),  name:String::from("bool1"),  status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Float( Point { value: 0.0077,  name:String::from("/path/Point.Name"), status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value: 65,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
    ]
}
fn pointFloat(value: f64) -> PointType {
    PointType::Float( Point { value: value,  name:String::from("/path/Point.Name"), status: 0, timestamp: chrono::offset::Utc::now() })
}

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
                let point = pointFloat(value);
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
