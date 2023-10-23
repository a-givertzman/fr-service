use std::{sync::mpsc::Sender, thread, time::Duration};
use rand::Rng;

use log::{debug, warn, info};

use crate::core_::{point::{point_type::PointType, point::Point}, types::bool::Bool};




fn points() ->Vec<PointType> {
    vec![
        PointType::Bool(  Point { value: Bool(true),   name:String::from("bool1"),  status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value: 13,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value: 43,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Bool(  Point { value: Bool(false),  name:String::from("bool1"),  status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Float( Point { value: 12.77,  name:String::from("float1"), status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value: 65,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
    ]
}

pub struct ProducerChannel {
    iterations: usize, 
    send: Vec<Sender<PointType>>,
}
impl ProducerChannel {
    pub fn new(iterations: usize, send: Sender<PointType>) -> Self {
        Self {
            iterations,
            send: vec![send],
        }
    }
    ///
    /// 
    pub fn run(&mut self) {
        let iterations = self.iterations;
        let send = self.send.pop().unwrap();
        let h = thread::Builder::new().name("name".to_owned()).spawn(move || {
            let name = "prodicer";
            debug!("Task({}).run | calculating step...", name);
            let points = points();
            let mut random = rand::thread_rng();
            let max = points.len();
            let mut sent = 0;
            for _ in 0..iterations {
                let index = random.gen_range(0..max);
                let point = &points[index];
                match send.send(point.clone()) {
                    Ok(_) => {
                        sent += 1;
                    },
                    Err(err) => {
                        warn!("Error write to queue: {:?}", err);
                    },
                }
                // thread::sleep(Duration::from_micros(10));
            }
            info!("Sent points: {}", sent);
            // thread::sleep(Duration::from_secs_f32(0.1));
            // debug!("Task({}).run | calculating step - done ({:?})", name, cycle.elapsed());
        }).unwrap();    
    }
}
