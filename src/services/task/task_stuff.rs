use std::{collections::HashMap, rc::Rc, cell::RefCell, clone};

use log::{debug, trace};

use super::{nested_function::fn_::FnInOut, task_stuff_inputs::TaskStuffInputs};


type FnInOutRef = Rc<RefCell<Box<dyn FnInOut>>>;

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
#[derive(Debug)]
pub struct TaskStuff {
    inputs: HashMap<String, (FnInOutRef, Vec<FnInOutRef>)>,
}
///
/// 
impl TaskStuff {
    ///
    /// Creates new empty TaskStuff instance 
    pub fn new() ->Self {
        Self {
            inputs: HashMap::new(),
        }
    }
    ///
    /// 
    pub fn add(&mut self, node: &mut TaskStuffInputs, out: FnInOutRef) {
        let vars = node.getVars();
        let inputs = node.getInputs();
        let mut outs: Vec<FnInOutRef> = vars.into_values().collect();
        outs.push(out);
        for (name, input) in inputs {
            self.inputs.insert(
                name,
                (input, outs.iter().map(|out|out.clone()).collect()),
            );
        };
        trace!("\nTaskStuff.add | self.inputs: {:?}\n", self.inputs);
    }
    ///
    /// Returns input by it's name
    pub fn getInput(&self, name: &str) -> Option<&(FnInOutRef, Vec<FnInOutRef>)> {
        self.inputs.get(name.into())
    }
}