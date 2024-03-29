use std::cell::RefCell;

use concat_string::concat_string;

use crate::core_::cot::cot::Cot;

///
/// Detailed definition of the subscription;
/// - "name" - the name of the point to be subscribed;
/// - "cot" - the cause & direction of the transmission to be subscribed;
#[derive(Debug)]
pub struct SubscriptionCriteria {
    name: String,
    cot: Cot,
    dest: RefCell<Option<String>>,
}
///
/// 
impl SubscriptionCriteria {
    ///
    /// Detailed definition of the subscription;
    /// - "name" - full name of the point to be subscribed;
    /// - "cot" - the cause & direction of the transmission to be subscribed;
    pub fn new(name: impl Into<String>, cot: Cot) -> Self {
        Self {
            name: name.into(),
            cot,
            dest: RefCell::new(None),
        }
    }
    /// deref
    /// 
    pub fn destination(&self) -> String {
        if let Some(dest) = &*self.dest.borrow() {
            return dest.clone();
        }
        let dest = concat_string!(self.cot, "/", self.name);
        *self.dest.borrow_mut() = Some(dest.clone());
        dest
    }
}