use std::collections::HashMap;

use crate::services::multi_queue::subscription_criteria::SubscriptionCriteria;

use super::conf_tree::ConfTree;

///
/// Service Configuration, to be subscribed on some service / services, by number of criterias
pub struct ConfSubscribe {
    subscriptions: HashMap<String, SubscriptionCriteria>,
}
///
/// 
impl ConfSubscribe {
    pub fn new(conf: ConfTree) -> Self {
        Self { subscriptions: HashMap::new() }
    }
}