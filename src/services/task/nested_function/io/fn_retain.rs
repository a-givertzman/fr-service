use std::{env, fs, io::{Read, Write}, path::PathBuf, sync::atomic::{AtomicUsize, Ordering}};
use chrono::Utc;
use log::{debug, error};
use concat_string::concat_string;
use crate::{
    conf::point_config::{name::Name, point_config_type::PointConfigType},
    core_::{
        cot::cot::Cot, point::{point::Point, point_tx_id::PointTxId, point_type::PointType}, status::status::Status, types::{bool::Bool, fn_in_out_ref::FnInOutRef}
    }, 
    services::task::nested_function::{fn_::{FnIn, FnInOut, FnOut}, fn_kind::FnKind},
};
///
/// Function | Used for store input Point value to the local disk
///  - Point will be read from disk if:
///     - if retain file already exists, read will be done only once
///     - if retain file does not exists, [default] value will be returned
///  - Point will be stored to the disk if:
///     - [input] is specified
///     - [enable] 
///         - if specified and is true (or [enable] > 0)
///         - if not specified - default is true
///  - [key] - the key to store Point with (full path: ./assets/retain/App/TaskName/key.json)
///  - Returns
///     - read Point if [input] is not specified (read will be done only once)
///     - input Point if [input] is specified
#[derive(Debug)]
pub struct FnRetain {
    id: String,
    name: Name,
    tx_id: usize,
    kind: FnKind,
    enable: Option<FnInOutRef>,
    key: String,
    default: Option<FnInOutRef>,
    input: Option<FnInOutRef>,
    path: Option<PathBuf>,
}
//
//
impl FnRetain {
    ///
    /// Creates new instance of the FnRetain
    /// - parent - the name of the parent entitie
    /// - name - the name of the parent
    /// - enable - boolean (numeric) input enables the storing if true (> 0)
    /// - key - the key to store Point with (full path: ./assets/retain/App/TaskName/key.json)
    /// - input - incoming Point's
    pub fn new(parent: &Name, enable: Option<FnInOutRef>, key: String, default: Option<FnInOutRef>, input: Option<FnInOutRef>) -> Self {
        let self_id = format!("{}/FnRetain{}", parent.join(), COUNT.fetch_add(1, Ordering::Relaxed));
        Self {
            id: self_id.clone(),
            name: parent.clone(),
            tx_id: PointTxId::fromStr(&self_id),
            kind: FnKind::Fn,
            enable,
            key,
            default,
            input,
            path: None,
        }
    }
    ///
    /// 
    fn path(&mut self) -> Result<PathBuf, String> {
        match self.path.clone() {
            Some(path) => Ok(path),
            None => {
                let path = Name::new("assets/retain/", self.name.join()).join();
                let path = path.trim_start_matches('/');
                match Self::create_dir(&self.id, path) {
                    Ok(path) => {
                        let path = path.join(concat_string!(self.key, ".json"));
                        self.path = Some(path.clone());
                        Ok(path)
                    }
                    Err(err) => Err(concat_string!(self.id, ".path | Error: {}", err)),
                }
            }
        }
    }
    ///
    /// Writes Point value to the file
    fn store(&mut self, point: &PointType) -> Result<(), String> {
        match self.path() {
            Ok(path) => {
                let value = match point {
                    PointType::Bool(point) => point.value.0.to_string(),
                    PointType::Int(point) => point.value.to_string(),
                    PointType::Real(point) => point.value.to_string(),
                    PointType::Double(point) => point.value.to_string(),
                    PointType::String(point) => point.value.clone(),
                };
                match fs::OpenOptions::new().truncate(true).create(true).write(true).open(&path) {
                    Ok(mut f) => {
                        match f.write_all(value.as_bytes()) {
                            Ok(_) => {
                                debug!("{}.store | Retain stored in: {:?}", self.id, path);
                                Ok(())
                            }
                            Err(err) => {
                                let message = format!("{}.store | Error writing to file: '{:?}'\n\terror: {:?}", self.id, path, err);
                                error!("{}", message);
                                Err(message)
                            }
                        }
                    }
                    Err(err) => {
                        let message = format!("{}.store | Error open file: '{:?}'\n\terror: {:?}", self.id, path, err);
                        error!("{}", message);
                        Err(message)
                    }
                }
            }
            Err(err) => Err(err),
        }
    }
    ///
    /// Creates directiry (all necessary folders in the 'path' if not exists)
    ///  - path is relative, will be joined with current working dir
    fn create_dir(self_id: &str, path: &str) -> Result<PathBuf, String> {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.join(path);
        match path.exists() {
            true => Ok(path),
            false => {
                match fs::create_dir_all(&path) {
                    Ok(_) => Ok(path),
                    Err(err) => {
                        let message = format!("{}.create_dir | Error create path: '{:?}'\n\terror: {:?}", self_id, path, err);
                        error!("{}", message);
                        Err(message)
                    }
                }
            }
        }
    }
    ///
    /// Loads retained Point value from the disk
    fn load(&mut self, type_: PointConfigType) -> Option<PointType> {
        match self.path() {
            Ok(path) => {
                match fs::OpenOptions::new().read(true).open(&path) {
                    Ok(mut f) => {
                        let mut input = String::new();
                        match f.read_to_string(&mut input) {
                            Ok(_) => {
                                match type_ {
                                    PointConfigType::Bool => match input.as_str() {
                                        "true" => Some(PointType::Bool(Point::new(self.tx_id, &self.id, Bool(true), Status::Ok, Cot::Inf, Utc::now()))),
                                        "false" => Some(PointType::Bool(Point::new(self.tx_id, &self.id, Bool(false), Status::Ok, Cot::Inf, Utc::now()))),
                                        _ => {
                                            error!("{}.load | Error parse 'bool' from '{}' \n\tretain: '{:?}'", self.id, input, path);
                                            None
                                        }
                                    }
                                    PointConfigType::Int => match input.as_str().parse() {
                                        Ok(value) => {
                                            Some(PointType::Int(Point::new(self.tx_id, &self.id, value, Status::Ok, Cot::Inf, Utc::now())))
                                        }
                                        Err(err) => {
                                            error!("{}.load | Error parse 'Int' from '{}' \n\tretain: '{:?}'\n\terror: {:?}", self.id, input, path, err);
                                            None
                                        }
                                    }
                                    PointConfigType::Real => match input.as_str().parse() {
                                        Ok(value) => {
                                            Some(PointType::Real(Point::new(self.tx_id, &self.id, value, Status::Ok, Cot::Inf, Utc::now())))
                                        }
                                        Err(err) => {
                                            error!("{}.load | Error parse 'Real' from '{}' \n\tretain: '{:?}'\n\terror: {:?}", self.id, input, path, err);
                                            None
                                        }
                                    }
                                    PointConfigType::Double => match input.as_str().parse() {
                                        Ok(value) => {
                                            Some(PointType::Double(Point::new(self.tx_id, &self.id, value, Status::Ok, Cot::Inf, Utc::now())))
                                        }
                                        Err(err) => {
                                            error!("{}.load | Error parse 'Double' from '{}' \n\tretain: '{:?}'\n\terror: {:?}", self.id, input, path, err);
                                            None
                                        }
                                    }
                                    PointConfigType::String => {
                                        Some(PointType::String(Point::new(self.tx_id, &self.id, input, Status::Ok, Cot::Inf, Utc::now())))
                                    }
                                    PointConfigType::Json => {
                                        Some(PointType::String(Point::new(self.tx_id, &self.id, input, Status::Ok, Cot::Inf, Utc::now())))
                                    }
                                }

                            }
                            Err(err) => {
                                error!("{}.load | Error read from retain: '{:?}'\n\terror: {:?}", self.id, path, err);
                                None
                            }
                        }
                    }
                    Err(err) => {
                        error!("{}.load | Error open file: '{:?}'\n\terror: {:?}", self.id, path, err);
                        None
                    }
                }
            }
            Err(err) => {
                error!("{}.load | Error: {:?}", self.id, err);
                None
            }
        }
    }
}
//
//
impl FnIn for FnRetain {}
//
//
impl FnOut for FnRetain {
    //
    fn id(&self) -> String {
        self.id.clone()
    }
    //
    fn kind(&self) -> &FnKind {
        &self.kind
    }
    //
    fn inputs(&self) -> Vec<String> {
        let mut inputs = vec![];
        if let Some(enable) = &self.enable {
            inputs.append(&mut enable.borrow().inputs());
        }
        if let Some(input) = &self.input {
            inputs.append(&mut input.borrow().inputs());
        }
        if let Some(default) = &self.default {
            inputs.append(&mut default.borrow().inputs());
        }
        inputs
    }
    //
    fn out(&mut self) -> PointType {
        let point = match &self.input {
            Some(input) => {
                input.borrow_mut().out()
            }
            None => {
                let default = match &mut self.default {
                    Some(default) => {
                        default.borrow_mut().out()
                    }
                    None => panic!("{}.out | The [default] input is not specified", self.id),
                };

                match self.load(default.type_()) {
                    Some(point) => point,
                    None => {
                        match &mut self.default {
                            Some(default) => {
                                default.borrow_mut().out()
                            }
                            None => panic!("{}.out | The [default] input is not specified", self.id),
                        }
                    }
                }
            }
        };
        let enable = match &self.enable {
            Some(enable) => enable.borrow_mut().out().to_bool().as_bool().value.0,
            None => true,
        };
        if enable {
            if let Err(err) = self.store(&point) {
                error!("{}.out | Error: '{:?}'", self.id, err);
            };
        }
        point
    }
    //
    fn reset(&mut self) {
        if let Some(enable) = &self.enable {
            enable.borrow_mut().reset();
        }
        if let Some(default) = &self.default {
            default.borrow_mut().reset();
        }
        if let Some(input) = &self.input {
            input.borrow_mut().reset();
        }
    }
}
//
//
impl FnInOut for FnRetain {}
///
/// Global static counter of FnRetain instances
static COUNT: AtomicUsize = AtomicUsize::new(1);
