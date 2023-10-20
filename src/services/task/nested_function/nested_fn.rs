#![allow(non_snake_case)]

use std::{rc::Rc, cell::RefCell};

use crate::core_::{
    point::{point_type::PointType, point::Point},
    conf::{fn_config::FnConfig, fn_conf_kind::FnConfKind, conf_keywd::FnConfPointType}, 
};

use super::{fn_inputs::FnInputs, fn_::FnInOut, fn_input::FnInput, fn_sum::FnSum, fn_timer::FnTimer};

///
/// Creates nested functions tree from it config
pub struct NestedFn {}
impl NestedFn {
    ///
    /// Creates nested functions tree from it config
    pub fn new(conf: &mut FnConfig, inputs: &mut FnInputs) -> Rc<RefCell<Box<dyn FnInOut>>> {
        Self::function("", conf, inputs)
    }
    ///
    /// 
    fn function(inputName: &str, conf: &mut FnConfig, inputs: &mut FnInputs) -> Rc<RefCell<Box<dyn FnInOut>>> {
        match conf.fnKind {
            FnConfKind::Fn => {
                match conf.name.as_str() {
                    "sum" => {
                        println!("NestedFn.function | function sum");
                        let name = "input1";
                        let conf = conf.inputs.get_mut(name).unwrap();
                        let input1 = Self::function(name, conf, inputs);
                        let name = "input1";
                        let conf = conf.inputs.get_mut(name).unwrap();
                        let input2 = Self::function(name, conf, inputs);
                        let func = Self::fnSum(inputName, input1, input2);
                        func
                    }
                    "timer" => {
                        println!("NestedFn.function | function timer");
                        let name = "input1";
                        let conf = conf.inputs.get_mut(name).unwrap();
                        let input = Self::function(name, conf, inputs);
                        let func = Self::fnTimer(inputName, 0.0, input, true);
                        func
                    },
                    _ => panic!("NestedFn.function | Unknown function name: {:?}", conf.name)
                }
            },
            FnConfKind::Var => {
                panic!("NestedFn.function | Var not implemented yet")
            },
            FnConfKind::Const => {
                panic!("NestedFn.function | Const not implemented yet")
            },
            FnConfKind::Point => {                
                println!("NestedFn.function | function input: {:?}...", inputName);
                let initial = match conf.pointType.clone() {
                    FnConfPointType::Bool => PointType::Bool(Point::newBool("initial", false)),
                    FnConfPointType::Int => PointType::Int(Point::newInt("initial", 0)),
                    FnConfPointType::Float => PointType::Float(Point::newFloat("initial", 0.0)),
                    FnConfPointType::String => PointType::String(Point::newString("initial", "")),
                    FnConfPointType::Unknown => panic!("NestedFn.function | Point type required"),
                };
                let input = Self::fnInput(inputName, initial);
                inputs.add(inputName, input.clone());
                println!("NestedFn.function | function input: {:?}", input);
                input
            },
            FnConfKind::Metric => {
                panic!("NestedFn.function | Netric nested in the function is not implemented");
            },
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
    fn fnInput(inputName: &str, initial: PointType) -> Rc<RefCell<Box<dyn FnInOut>>> {
        Rc::new(RefCell::new(
            Self::boxFnInput(
                FnInput::new( 
                    inputName,
                    initial, 
                )
            )
        ))
    }
    ///
    /// 
    fn boxFnSum(input: FnSum) -> Box<(dyn FnInOut)> {
        Box::new(input)
    }
    fn fnSum(
        id: &str, 
        input1: Rc<RefCell<Box<dyn FnInOut>>>, 
        input2: Rc<RefCell<Box<dyn FnInOut>>>
    ) -> Rc<RefCell<Box<dyn FnInOut>>> {
        Rc::new(RefCell::new(
            Self::boxFnSum(        
                FnSum {
                    id: id.into(),
                    input1: input1, 
                    input2: input2, 
                }
            )
        ))
    }    
    ///
    /// 
    fn boxFnTimer(input: FnTimer) -> Box<(dyn FnInOut)> {
        Box::new(input)
    }
    fn fnTimer(
        id: &str, 
        initial: impl Into<f64> + Clone,
        input: Rc<RefCell<Box<dyn FnInOut>>>, 
        repeat: bool,
    ) -> Rc<RefCell<Box<dyn FnInOut>>> {
        Rc::new(RefCell::new(
            Self::boxFnTimer(        
                FnTimer::new(
                    id,
                    initial, 
                    input, 
                    repeat
                )
            )
        ))
    }    
}
