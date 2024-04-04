use std::cell::RefCell;

use concat_string::concat_string;

use crate::core_::cot::cot::Cot;

///
/// Detailed definition of the subscription;
/// - "name" - the name of the point to be subscribed;
/// - "cot" - the cause & direction of the transmission to be subscribed;
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionCriteria {
    name: String,
    cot: Cot,
    dest: String,
}
///
/// 
impl SubscriptionCriteria {
    ///
    /// Detailed definition of the subscription;
    /// - "name" - full name of the point to be subscribed;
    /// - "cot" - the cause & direction of the transmission to be subscribed;
    pub fn new(name: impl Into<String>, cot: Cot) -> Self {
        let name = name.into();
        Self {
            dest: Self::dest(&cot, &name),
            name,
            cot,
        }
    }
    ///
    /// 
    pub fn destination(&self) -> String {
        self.dest.clone()
    }
    ///
    /// 
    pub fn dest(cot: &Cot, name: &str) -> String {
        match cot {
            Cot::All => name.to_owned(),
            _        => concat_string!(cot.as_str(), ":", name),
        }
    }
}