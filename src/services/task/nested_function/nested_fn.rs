use std::{rc::Rc, cell::RefCell};

use crate::{
    core_::{conf::fn_config::FnConfig, point::point::PointType}
};

use super::{fn_inputs::FnInputs, fn_::FnInOut, fn_input::FnInput, fn_sum::FnSum, fn_timer::FnTimer};

///
/// Creates nested functions tree from it config
pub struct NestedFn {}
impl NestedFn {
    ///
    /// Creates nested functions tree from it config
    pub fn new(conf: &mut FnConfig, initial: PointType, inputs: &mut FnInputs) -> Rc<RefCell<Box<dyn FnInOut>>> {
        Self::function(conf, initial, String::new(), inputs)
    }
    ///
    /// 
    fn function(conf: &mut FnConfig, initial: PointType, inputName: String, inputs: &mut FnInputs) -> Rc<RefCell<Box<dyn FnInOut>>> {
        match conf.name().as_str() {
            "input" => {
                println!("input function {:?}...", inputName);
                let input = Self::fnInput(inputName.clone(), initial);
                inputs.add(inputName, input.clone());
                // let a = input.borrow_mut();
                println!("input function: {:?}", input);
                input
            },
            "sum" => {
                println!("sum function");
                let in1Name = String::from("input1");
                let in2Name = String::from("input2");
                let input1 = Self::function(conf.nested(&in1Name), initial.clone(), in1Name, inputs);
                let input2 = Self::function(conf.nested(&in2Name), initial, in2Name, inputs);
                let func = Self::fnSum(inputName, input1, input2);
                func
            }
            "timer" => {
                println!("sum function");
                // let input = Self::function()
                let func = Self::fnTimer(inputName, initial, input, true);
                func
            },
            _ => panic!("Unknown function name: {:?}", conf.name())
        }
    }
    ///
    /// 
    /// 
    ///
    /// 
    fn boxFnInput(input: FnInput) -> Box<(dyn FnInOut)> {
        Box::new(input)
    }
    fn fnInput(inputName: String, initial: PointType) -> Rc<RefCell<Box<dyn FnInOut>>> {
        Rc::new(RefCell::new(
            Self::boxFnInput(
                FnInput { 
                    id: inputName.clone(),
                    point: initial, 
                }
            )
        ))
    }
    ///
    /// 
    fn boxFnSum(input: FnSum) -> Box<(dyn FnInOut)> {
        Box::new(input)
    }
    fn fnSum(
        inputName: String, 
        input1: Rc<RefCell<Box<dyn FnInOut>>>, 
        input2: Rc<RefCell<Box<dyn FnInOut>>>
    ) -> Rc<RefCell<Box<dyn FnInOut>>> {
        Rc::new(RefCell::new(
            Self::boxFnSum(        
                FnSum {
                    id: inputName,
                    input1: input1, 
                    input2: input2, 
                }
            )
        ))
    }    
    ///
    /// 
    fn boxFnTimer(input: FnSum) -> Box<(dyn FnInOut)> {
        Box::new(input)
    }
    fn fnTimer(
        inputName: String, 
        initial: impl Into<f64> + Clone,
        input: Rc<RefCell<Box<dyn FnInOut>>>, 
        repeat: bool,
    ) -> Rc<RefCell<Box<dyn FnInOut>>> {
        Rc::new(RefCell::new(
            Self::boxFnTimer(        
                FnTimer::new(
                    initial, 
                    input, 
                    repeat
                )
            )
        ))
    }    
}
