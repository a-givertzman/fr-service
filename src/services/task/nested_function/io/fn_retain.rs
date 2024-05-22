use std::{env, fs, io::Write, path::{Path, PathBuf}, sync::{atomic::{AtomicUsize, Ordering}, mpsc::Sender}};
use log::{debug, error, warn};
use concat_string::concat_string;
use crate::{
    conf::point_config::{point_config::PointConfig, point_config_type::PointConfigType},
    core_::{
        point::{point::Point, point_tx_id::PointTxId, point_type::PointType}, 
        types::{bool::Bool, fn_in_out_ref::FnInOutRef}
    }, 
    services::task::nested_function::{fn_::{FnIn, FnInOut, FnOut}, fn_kind::FnKind},
};
///
/// Function | Used for store Point value from Task service to local disk
///  - Poiont will be stored to the disk only if:
///     - [enable] 
///         - if specified and is true (or [enable] > 0)
///         - if not specified - default is true
///  - key - the key to store Point with (full path: ./assets/retain/App/TaskName/key.json)
///  - finally input Point will be returned to the parent function
#[derive(Debug)]
pub struct FnRetain {
    id: String,
    tx_id: usize,
    kind: FnKind,
    enable: Option<FnInOutRef>,
    key: String,
    input: FnInOutRef,
    path: Option<PathBuf>,
}
//
//
impl FnRetain {
    ///
    /// Creates new instance of the FnRetain
    /// - parent - the name of the parent entitie
    /// - enable - boolean (numeric) input enables the storing if true (> 0)
    /// - key - the key to store Point with (full path: ./assets/retain/App/TaskName/key.json)
    /// - input - incoming Point's
    pub fn new(parent: impl Into<String>, enable: Option<FnInOutRef>, key: String, input: FnInOutRef) -> Self {
        let self_id = format!("{}/FnRetain{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed));
        Self {
            id: self_id.clone(),
            tx_id: PointTxId::fromStr(&self_id),
            kind: FnKind::Fn,
            enable,
            key,
            input,
            path: None,
        }
    }
    ///
    /// Writes Point value to the file:
    fn write(&self, point: &PointType) -> Result<(), String> {
        let path = match self.path {
            Some(path) => Ok(path),
            None => {
                let path = Name::new("assets/retain/", name.join()).join().trim_start_matches('/')
                match Self::create_dir(&self.id, path) {
                    Ok(path) => {
                        let path = path.join(concat_string!(self.key, ".json"));
                        self.path = Some(path);
                        Ok(path)
                    }
                    Err(err) => Err(concat_string!(self.id, ".write | Error: {}", err)),
                }
            }
        };
        match path {
            Ok(path) => {
                let mut message = String::new();
                let mut cache = String::new();
                cache.push('[');
                let content: String = points.into_iter().fold(String::new(), |mut points, point| {
                    points.push_str(concat_string!("\n", json!(point).to_string(), ",").as_str());
                    points
                }).trim_end_matches(',').to_owned();
                cache.push_str(content.as_str());
                cache.push_str("\n]");
                match fs::OpenOptions::new().truncate(true) .create(true).write(true).open(&path) {
                    Ok(mut f) => {
                        match f.write_all(cache.as_bytes()) {
                            Ok(_) => {
                                debug!("{}.write | Cache stored in: {:?}", self.id, path);
                            }
                            Err(err) => {
                                message = format!("{}.write | Error writing to file: '{:?}'\n\terror: {:?}", self.id, path, err);
                                error!("{}", message);
                            }
                        };
                        if message.is_empty() {Ok(())} else {Err(message)}
                    }
                    Err(err) => {
                        let message = format!("{}.write | Error open file: '{:?}'\n\terror: {:?}", self.id, path, err);
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
        inputs.append(&mut self.input.borrow().inputs());
        inputs
    }
    //
    fn out(&mut self) -> PointType {
        let enable = match &self.enable {
            Some(enable) => enable.borrow_mut().out().to_bool().as_bool().value.0,
            None => true,
        };
        let point = self.input.borrow_mut().out();
        if enable {
            self.send(point.clone());
        }
        point
    }
    //
    fn reset(&mut self) {
        if let Some(enable) = &self.enable {
            enable.borrow_mut().reset();
        }
        self.input.borrow_mut().reset();
    }
}
//
//
impl FnInOut for FnRetain {}
///
/// Global static counter of FnRetain instances
static COUNT: AtomicUsize = AtomicUsize::new(1);
