use chrono::DateTime;
use crate::core_::{cot::cot::Cot, status::status::Status, types::bool::Bool};

///
/// Entity of the information 
#[derive(Clone, Debug, PartialEq)]
pub struct Point<T> {
    pub tx_id: usize,
    pub name: String,
    // pub path: PointPath,
    pub value: T,
    pub status: Status,
    pub cot: Cot,
    pub timestamp: DateTime<chrono::Utc>,
}
///
/// 
impl<T> Point<T> {
    ///
    /// Creates new instance of the Point
    ///     - txId: usize - unique id of the producer of the point, necessary only for internal purposes, like identify the producer of the point in the MultiQueue to prevent send back to the producer
    ///     - name: &str - full name of the point like '/AppName/DeviceName/Point.Name' unique within the entire system, for the Write direction name can be not a full
    ///     - value: T - supported types: bool, i64, f64, String
    ///     - status: Status - indicates Ok or some kind of invalidity
    ///     - direction: Direction - the kind of the direction Read / Write
    ///     - timestamp: DateTime<chrono::Utc> - registration timestamp
    pub fn new(tx_id: usize, name: &str, value: T, status: Status, cot: Cot, timestamp: DateTime<chrono::Utc>) -> Point<T> {
        Self {
            tx_id,
            name: name.to_owned(),
            value,
            status,
            cot,
            timestamp,
        }
    }
}
///
/// 
impl Point<Bool> {
    ///
    /// creates Point<Bool> with given name & value, taking current timestamp, Status::Ok, Direction::Read
    pub fn new_bool(tx_id: usize, name: &str, value: bool) -> Point<Bool> {
        Point {
            tx_id,
            name: name.into(),
            value: Bool(value),
            status: Status::Ok,
            cot: Cot::default(),
            timestamp: chrono::offset::Utc::now(),
        }
    }
}
///
/// 
impl Point<i64> {
    ///
    /// creates Point<i64> with given name & value, taking current timestamp, Status::Ok, Direction::Read
    pub fn new_int(tx_id: usize, name: &str, value: i64) -> Point<i64> {
        Point {
            tx_id,
            name: name.into(),
            value,
            status: Status::Ok,
            cot: Cot::default(),
            timestamp: chrono::offset::Utc::now(),
        }
    }
}
///
/// 
impl Point<f32> {
    ///
    /// creates Point<f32> with given name & value, taking current timestamp, Status::Ok, Direction::Read
    pub fn new_real(tx_id: usize, name: &str, value: f32) -> Point<f32> {
        Point {
            tx_id,
            name: name.into(),
            value,
            status: Status::Ok,
            cot: Cot::default(),
            timestamp: chrono::offset::Utc::now(),
        }
    }
}
///
/// 
impl Point<f64> {
    ///
    /// creates Point<f64> with given name & value, taking current timestamp, Status::Ok, Direction::Read
    pub fn new_double(tx_id: usize, name: &str, value: f64) -> Point<f64> {
        Point {
            tx_id,
            name: name.into(),
            value,
            status: Status::Ok,
            cot: Cot::default(),
            timestamp: chrono::offset::Utc::now(),
        }
    }
}
///
/// 
impl Point<String> {
    ///
    /// creates Point<String> with given name & value, taking current timestamp, Status::Ok, Direction::Read
    pub fn new_string(tx_id: usize, name: &str, value: impl Into<String>) -> Point<String> {
        Point {
            tx_id,
            name: name.into(),
            value: value.into(),
            status: Status::Ok,
            cot: Cot::default(),
            timestamp: chrono::offset::Utc::now(),
        }
    }
}
///
/// 
impl<T: std::ops::Add<Output = T> + Clone> std::ops::Add for Point<T> {
    type Output = Point<T>;
    fn add(self, rhs: Self) -> Self::Output {
        let status = match self.status.cmp(&rhs.status) {
            std::cmp::Ordering::Less => rhs.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let (tx_id, timestamp) = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => (rhs.tx_id, rhs.timestamp),
            std::cmp::Ordering::Equal => (self.tx_id, self.timestamp),
            std::cmp::Ordering::Greater => (self.tx_id, self.timestamp),
        };
        let direction = if self.cot == rhs.cot {
            self.cot
        } else {
            panic!("Point.add | Directions are not equals")
        };
        Point {
            tx_id,
            name: String::from("Point.Add"),
            value: self.value + rhs.value,
            status,
            cot: direction,
            timestamp,
        }
    }
}


impl<T: std::ops::BitOr<Output = T>> std::ops::BitOr for Point<T> {
    type Output = Point<T>;
    fn bitor(self, rhs: Self) -> Self::Output {
        let status = match self.status.cmp(&rhs.status) {
            std::cmp::Ordering::Less => rhs.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let (tx_id, timestamp) = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => (rhs.tx_id, rhs.timestamp),
            std::cmp::Ordering::Equal => (self.tx_id, self.timestamp),
            std::cmp::Ordering::Greater => (self.tx_id, self.timestamp),
        };
        let direction = if self.cot == rhs.cot {
            self.cot
        } else {
            panic!("Point.add | Directions are not equals")
        };
        Point {
            tx_id,
            name: String::from("Point.BitOr"),
            value: self.value | rhs.value,
            status,
            cot: direction,
            timestamp,
        }        
    }
}


impl<T: std::cmp::PartialOrd> std::cmp::PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.tx_id.partial_cmp(&other.tx_id) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.value.partial_cmp(&other.value) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.status.partial_cmp(&other.status) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.cot.partial_cmp(&other.cot) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.timestamp.partial_cmp(&other.timestamp)
    }
}
