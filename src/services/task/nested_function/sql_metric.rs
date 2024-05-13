use std::{collections::HashMap, sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}}};
use indexmap::IndexMap;
use log::{debug, error, trace};
use crate::{
    conf::{fn_::fn_config::FnConfig, point_config::name::Name}, core_::{
        format::format::Format, point::{point::Point, point_tx_id::PointTxId, point_type::{PointType, ToPoint}}, types::fn_in_out_ref::FnInOutRef 
    },
    services::{
        services::Services, task::task_nodes::TaskNodes,
        task::nested_function::{
            fn_::{FnIn, FnInOut, FnOut, FnResult}, fn_kind::FnKind, nested_fn::NestedFn
        }
    }
};
///
/// Function | SqlMetric
///     - on Ok values received from the [input]s puts into the target sql query
///     - on Err or None sql query will canceled
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
    name: Name,
    tx_id: usize,
    kind: FnKind,
    inputs: IndexMap<String, FnInOutRef>,
    // initial: f64,
    // table: String,
    sql: Format,
    sql_names: HashMap<String, (String, Option<String>)>,
}
///
/// 
impl SqlMetric {
    //
    //
    pub fn new(parent: impl Into<String>, conf: &mut FnConfig, task_nodes: &mut TaskNodes, services: Arc<Mutex<Services>>) -> SqlMetric {
        let self_name = Name::new(parent, format!("SqlMetric{}", COUNT.fetch_add(1, Ordering::Relaxed)));
        let self_id = self_name.join();
        let tx_id = PointTxId::fromStr(&self_name.join());
        let mut inputs = IndexMap::new();
        let input_confs = conf.inputs.clone();
        let input_conf_names = input_confs.keys().filter(|v| {
            // let delete = match v.as_str() {
            //     "initial" => true,
            //     "table" => true,
            //     "sql" => true,
            //     _ => false
            // };
            let delete = matches!(v.as_str(), "initial" | "table" | "sql");
            !delete
        });
        for name in input_conf_names {
            debug!("{}.new | input name: {:?}", self_id, name);
            let input_conf = conf.input_conf(name);
            inputs.insert(
                name.to_string(), 
                NestedFn::new(&self_name, tx_id, input_conf, task_nodes, services.clone()),
            );
        }
        let id = conf.name.clone();
        // let initial = conf.param("initial").name.parse().unwrap();
        let table = conf.param("table").name();
        let mut sql = Format::new(&conf.param("sql").name());
        sql.insert("id", id.clone().to_point(tx_id, ""));
        sql.insert("table", table.clone().to_point(tx_id, ""));
        sql.prepare();
        let mut sql_names = sql.names();
        sql_names.remove("initial");
        sql_names.remove("table");
        sql_names.remove("sql");
        sql_names.remove("id");
        SqlMetric {
            id: self_id,
            name: self_name,
            tx_id,
            kind: FnKind::Fn,
            inputs,
            sql,
            sql_names,
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
    //
    fn id(&self) -> String {
        self.id.clone()
    }
    //
    //
    fn kind(&self) -> &FnKind {
        &self.kind
    }
    //
    //
    fn inputs(&self) -> Vec<String> {
        let mut inputs = vec![];
        for (_, input) in &self.inputs {
            inputs.extend(input.borrow().inputs());
        }
        inputs
    }
    //
    //
    fn out(&mut self) -> FnResult {
        let self_id = self.id.clone();
        for (full_name, (name, sufix)) in &self.sql_names {
            trace!("{}.out | name: {:?}, sufix: {:?}", self_id, name, sufix);
            match self.inputs.get(name) {
                Some(input) => {
                    trace!("{}.out | input: {:?} - found", self_id, name);
                    let point = input.borrow_mut().out();
                    match point {
                        FnResult::Ok(point) => {
                            self.sql.insert(full_name, point);
                        }
                        FnResult::Err(err) => {
                            let message = format!("{}.out | input error: {}", self_id, err);
                            error!("{}", message);
                            return FnResult::Err(message);
                        }
                        FnResult::None => return FnResult::None,
                    }
                }
                None => {
                    panic!("{}.out | input: {:?} - not found", self_id, name);
                }
            };
        }
        trace!("{}.out | sql: {:?}", self_id, self.sql.out());
        FnResult::Ok(PointType::String(Point::new_string(
            self.tx_id,
            &self.name.join(), 
            self.sql.out(),
        )))
    }
    //
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
pub static COUNT: AtomicUsize = AtomicUsize::new(1);
