use std::collections::HashMap;
use crate::services::multi_queue::subscription_criteria::SubscriptionCriteria;

///
/// Service Configuration, to be subscribed on some service / services, by number of criterias
/// ------------------------------------------------------------------------------------------
/// subscibe: MultiQueue    # - broadcast suscription to the MultiQueue
/// ------------------------------------------------------------------------------------------
/// subscibe: 
///     MultiQueue:         # - multicast subscription to the MultiQueue
///         Inf: *          #   - on all points having Cot::Inf
/// ------------------------------------------------------------------------------------------
/// subscibe:
///     MultiQueue:         # - multicast subscription to the MultiQueue
///         Act: *          #   - on all points having Cot::Act
///         Inf:            #   - on concrete points having Cot::Inf 
///             - /App/Service/Point.Name.1
///             - /App/Service/Point.Name.2
///     AnotherService:     # - multicast subscription to the AnotherService
///         Inf: *          #   - on all points having Cot::Inf
pub struct ConfSubscribe {
    subscriptions: HashMap<String, Vec<SubscriptionCriteria>>,
}
///
/// 
impl ConfSubscribe {
    pub fn new(conf: serde_yaml::Value) -> Self {
        let self_id = format!("ConfSubscribe");
        let subscriptions = if conf.is_string() {
            let service = conf.as_str().unwrap().to_owned();
            HashMap::from([
                (service, vec![])
            ])
        } else if conf.is_mapping() {
            let mut subscriptions = HashMap::<String, Vec<SubscriptionCriteria>>::new();
            for (service, criterias) in conf.as_mapping().unwrap() {
                let service = service.as_str().unwrap();
                for (cot, points) in criterias.as_mapping().unwrap() {
                    let mut points = points.as_sequence().unwrap().into_iter().map(|value| {
                        let cot = serde_yaml::from_value(cot.clone()).unwrap();
                        SubscriptionCriteria::new(value.as_str().unwrap(), cot)
                    }).collect();
                    subscriptions.entry(service.to_owned())
                        .and_modify(|entry| {
                            entry.append(&mut points)
                        })
                        .or_insert(points);
                }
            }
            subscriptions
        } else {
            panic!("{}.new | Invalid subscribe format: {:#?}", self_id, conf);
        };
        Self { subscriptions }
    }
}