#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}}};

use indexmap::IndexMap;
use log::{debug, trace};

use crate::{
    core_::{
        types::fn_in_out_ref::FnInOutRef,
        point::{point_type::{PointType, ToPoint}, point::Point, point_tx_id::PointTxId}, 
        format::format::Format, 
    }, 
    conf::fn_::fn_config::FnConfig, 
    services::{task::task_nodes::TaskNodes, services::Services},
};

use super::{fn_::{FnInOut, FnOut, FnIn}, nested_fn::NestedFn, fn_kind::FnKind};


///
/// Function | SqlMetric
///     - values received from the [input]s puts into the target sql query
///     - sql query buit by replacing markers with current values:
///         - table = 'point_values'
///         - input1.name = 'test-point'
///         - input1.value = 123.456
///         - inpur1.timestamp = '20'
///         - input1.status = 
///         - "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"    =>  UPDATE table SET kind = input1 WHERE id = '{input2}';
/// ```
/// fn SqlMetric:
///     initial: 0.123      # начальное значение
///     table: SelectMetric_test_table_name
///     sql: "UPDATE {table} SET value = '{input1}' WHERE id = '{input2}';"
///     input1 point int '/path/Point.Name'
///     input2: const int 11
///     
/// ```
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
        COUNT.fetch_add(1, Ordering::SeqCst);
        let self_id = format!("{}/SqlMetric{}", parent, COUNT.load(Ordering::Relaxed));
        let txId = PointTxId::fromStr(&self_id);
        let mut inputs = IndexMap::new();
        let inputConfs = conf.inputs.clone();
        let inputConfNames = inputConfs.keys().filter(|v| {
            // let delete = match v.as_str() {
            //     "initial" => true,
            //     "table" => true,
            //     "sql" => true,
            //     _ => false
            // };
            let delete = matches!(v.as_str(), "initial" | "table" | "sql");
            !delete
        });
        for name in inputConfNames {
            debug!("{}.new | input name: {:?}", self_id, name);
            let inputConf = conf.input_conf(name);
            inputs.insert(
                name.to_string(), 
                NestedFn::new(&self_id, txId, inputConf, taskNodes, services.clone()),
            );
        }
        let id = conf.name.clone();
        // let initial = conf.param("initial").name.parse().unwrap();
        let table = conf.param("table").name();
        let mut sql = Format::new(&conf.param("sql").name());
        sql.insert("id", id.clone().to_point(txId, ""));
        sql.insert("table", table.clone().to_point(txId, ""));
        sql.prepare();
        let mut sqlNames = sql.names();
        sqlNames.remove("initial");
        sqlNames.remove("table");
        sqlNames.remove("sql");
        sqlNames.remove("id");
        SqlMetric {
            id: self_id,
            txId,
            kind: FnKind::Fn,
            inputs,
            // initial: initial,
            // table: table,
            sql,
            sqlNames,
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
        let self_id = self.id.clone();
        for (fullName, (name, sufix)) in &self.sqlNames {
            trace!("{}.out | name: {:?}, sufix: {:?}", self_id, name, sufix);
            match self.inputs.get(name) {
                Some(input) => {
                    trace!("{}.out | input: {:?} - found", self_id, name);
                    let point = input.borrow_mut().out();
                    self.sql.insert(fullName, point);
                },
                None => {
                    panic!("{}.out | input: {:?} - not found", self_id, name);
                },
            };
        }
        debug!("{}.out | sql: {:?}", self_id, self.sql.out());
        PointType::String(Point::new_string(
            self.txId,
            &self_id, 
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
///
/// Global static counter of SqlMetric instances
pub static COUNT: AtomicUsize = AtomicUsize::new(0);
