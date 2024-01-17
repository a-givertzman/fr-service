#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{Arc, Mutex}};

use indexmap::IndexMap;
use log::{debug, trace};

use crate::{
    core_::{
        types::fn_in_out_ref::FnInOutRef,
        point::{point_type::{PointType, ToPoint}, point::Point, point_tx_id::PointTxId}, 
        format::format::Format, 
    }, 
    conf::fn_config::FnConfig, 
    services::{task::task_nodes::TaskNodes, services::Services},
};

use super::{fn_::{FnInOut, FnOut, FnIn}, nested_fn::NestedFn, fn_kind::FnKind};


///
/// Counts number of raised fronts of boolean input
#[derive(Debug)]
pub struct SqlMetric {
    id: String,
    txId: usize,
    kind: FnKind,
    inputs: IndexMap<String, FnInOutRef>,
    // initial: f64,
    // table: String,
    sql: Format,
    sqlNames: HashMap<String, (String, Option<String>)>,
}
///
/// 
impl SqlMetric {
    //
    //
    pub fn new(parent: &str, conf: &mut FnConfig, taskNodes: &mut TaskNodes, services: Arc<Mutex<Services>>) -> SqlMetric {
        let selfId = format!("{}/SqlMetric({})", parent, conf.name.clone());
        let txId = PointTxId::fromStr(&selfId);
        let mut inputs = IndexMap::new();
        let inputConfs = conf.inputs.clone();
        let inputConfNames = inputConfs.keys().filter(|v| {
            let delete = match v.as_str() {
                "initial" => true,
                "table" => true,
                "sql" => true,
                _ => false
            };
            !delete
        });
        // let v: Vec<&String> = inputConfNames.collect();
        // inputConfs.remove("initial");
        // inputConfs.remove("table");
        // inputConfs.remove("sql");
        for name in inputConfNames {
            debug!("{}.new | input name: {:?}", selfId, name);
            let inputConf = conf.inputConf(&name);
            inputs.insert(
                name.to_string(), 
                NestedFn::new(&selfId, txId, inputConf, taskNodes, services.clone()),
            );
        }
        let id = conf.name.clone();
        // let initial = conf.param("initial").name.parse().unwrap();
        let table = conf.param("table").name.clone();
        let mut sql = Format::new(&conf.param("sql").name);
        sql.insert("id", id.clone().toPoint(txId, ""));
        sql.insert("table", table.clone().toPoint(txId, ""));
        sql.prepare();
        let mut sqlNames = sql.names();
        sqlNames.remove("initial");
        sqlNames.remove("table");
        sqlNames.remove("sql");
        sqlNames.remove("id");
        SqlMetric {
            id: selfId,
            txId,
            kind: FnKind::Fn,
            inputs: inputs,
            // initial: initial,
            // table: table,
            sql,
            sqlNames: sqlNames,
        }
    }
}
///
/// 
impl FnIn for SqlMetric {
    fn add(&mut self, _point: PointType) {
        panic!("{}.add | method is not used", self.id)
    }
}
///
/// 
impl FnOut for SqlMetric {
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
        for (_, input) in &self.inputs {
            inputs.extend(input.borrow().inputs());
        }
        inputs
    }
    //
    fn out(&mut self) -> PointType {
        let selfId = self.id.clone();
        for (fullName, (name, sufix)) in &self.sqlNames {
            trace!("{}.out | name: {:?}, sufix: {:?}", selfId, name, sufix);
            match self.inputs.get(name) {
                Some(input) => {
                    trace!("{}.out | input: {:?} - found", selfId, name);
                    let point = input.borrow_mut().out();
                    self.sql.insert(&fullName, point);
                },
                None => {
                    panic!("{}.out | input: {:?} - not found", selfId, name);
                },
            };
        }
        debug!("{}.out | sql: {:?}", selfId, self.sql.out());
        PointType::String(Point::newString(
            self.txId,
            &selfId, 
            self.sql.out(),
        ))
    }
    //
    fn reset(&mut self) {
        todo!()
    }
}
///
/// 
impl FnInOut for SqlMetric {}
