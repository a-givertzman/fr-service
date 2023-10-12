use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::core_::nested_function::fn_::FnInput;


pub enum FnInputBox {
    Bool(Arc<Mutex<dyn FnInput<bool>>>),
    U64(Arc<Mutex<dyn FnInput<u64>>>),
    I64(Arc<Mutex<dyn FnInput<i64>>>),
    F64(Arc<Mutex<dyn FnInput<f64>>>),
}

/// TaskShame / TaskProgram / TaskPlan / TaskStuff / TaskNodes - holds the entities of the Task in the following structure:
///   ```
///   {
///       inputId1: {
///           input: inputRef,
///           outpots: [
///               var1
///               var2
///               var...
///               metric1
///               metric2
///               metric...
///           ]
///       },
///       inputId2: {
///           ...
///       },
///   }
///   ```
pub struct TaskStuff {
    inputs: HashMap<String, FnInputBox>
}
///
/// 
impl TaskStuff {
    pub fn new() ->Self {
        Self {
            inputs: HashMap::new(),
        }
    }
}