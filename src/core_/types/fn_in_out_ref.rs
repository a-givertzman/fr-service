use std::{rc::Rc, cell::RefCell};

use crate::services::task::nested_function::fn_::FnInOut;

pub type FnInOutRef = Rc<RefCell<Box<dyn FnInOut>>>;
