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
    pub fn new(name: &str, cot: Cot) -> Self {
        Self {
            name: name.into(),
            cot,
            dest: RefCell::new(None),
        }
    }
    ///
    /// 
    pub fn destination(&self) -> String {
        let dest = self.dest.borrow_mut();
        match dest.as_deref() {
            Some(dest) => dest.to_owned(),
            None => {
                let dest = concat_string!(self.name, "/", self.cot);
                *self.dest.borrow_mut() = Some(dest.clone());
                dest
            },
        }
    }
}