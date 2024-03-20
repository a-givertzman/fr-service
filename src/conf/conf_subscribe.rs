use std::collections::HashMap;
use crate::{core_::cot::cot::Cot, services::multi_queue::subscription_criteria::SubscriptionCriteria};

use super::point_config::point_config::PointConfig;

///
/// Service Configuration, to be subscribed on some service / services, by number of criterias
/// ------------------------------------------------------------------------------------------
/// subscibe: MultiQueue    # - broadcast suscription to the MultiQueue
/// ------------------------------------------------------------------------------------------
/// subscribe:
///     MultiQueue: {}      # - broadcast suscription to the MultiQueue
///     AnotherService: {}  # - broadcast suscription to the AnotherService
/// ------------------------------------------------------------------------------------------
/// subscibe: 
///     MultiQueue:         # - multicast subscription to the MultiQueue
///         Inf: []         #   - on all points having Cot::Inf
/// subscribe: 
///     MultiQueue:                     # - multicast subscription to the MultiQueue
///         {cot: Inf, history: r}: []  #   - on all points having Cot::Inf and history::read
/// ------------------------------------------------------------------------------------------
/// subscibe:
///     MultiQueue:                         # - multicast subscription to the MultiQueue
///         Act: []                         #   - on all points having Cot::Act
///         {cot: Inf, history: r}:         #   - on concrete points having Cot::Inf and history::read
///             - /App/Service/Point.Name.1
///             - /App/Service/Point.Name.2
///     AnotherService:                     # - multicast subscription to the AnotherService
///         Inf: []                         #   - on all points having Cot::Inf
#[derive(Debug)]
pub struct ConfSubscribe {
    id: String,
    conf: serde_yaml::Value,
    // subscriptions: HashMap<String, Vec<SubscriptionCriteria>>,
}
///
/// Creates new instance from yaml
impl ConfSubscribe {
    ///
    /// Creates new instance of ConfSubscribe
    pub fn new(conf: serde_yaml::Value) -> Self {
        Self {
            id: "ConfSubscribe".to_owned(),
            conf,
        }
    }
    ///
    /// 
    pub fn with(&self, points: &Vec<PointConfig>) -> HashMap<String, Option<Vec<SubscriptionCriteria>>> {
        let point_configs = points;
        if self.conf.is_string() {
            let service = self.conf.as_str().unwrap().to_owned();
            HashMap::from([
                (service, Some(vec![]))
            ])
        } else if self.conf.is_mapping() {
            let mut subscriptions = HashMap::<String, Option<Vec<SubscriptionCriteria>>>::new();
            for (service, criterias) in self.conf.as_mapping().unwrap() {
                let service = service.as_str().unwrap();
                let mut points = criterias.as_mapping().unwrap().into_iter().fold(vec![], |mut points, (options, names)| {
                    let names = names.as_sequence().unwrap();
                    let point_configs = if names.is_empty() {
                        point_configs.clone()
                    } else {
                        names.iter().filter_map(|name| {
                            let name: String = serde_yaml::from_value(name.clone()).unwrap();
                            point_configs.iter().find(|&point_conf| point_conf.name == name)
                        }).cloned().collect()
                    };
                    let creterias = point_configs.into_iter().filter_map(|point_conf| {
                        if options.is_string() {
                            let cot = serde_yaml::from_value(options.clone()).unwrap();
                            Some(SubscriptionCriteria::new(point_conf.name, cot))
                        } else if options.is_mapping() {
                            let options = options.as_mapping().unwrap();
                            let cot = options.get("cot").map(|v| serde_yaml::from_value(v.clone()).unwrap()).unwrap_or(Cot::All);
                            let history = options.get("history").map(|v| serde_yaml::from_value(v.clone()).unwrap());
                            let alarm = options.get("alarm").map(|v| v.as_u64().unwrap());
                            let mut filter = false;
                            if let Some(history) = history {
                                filter |= point_conf.history != history
                            }
                            if let Some(alarm) = alarm {
                                filter |= point_conf.alarm.map(|v| v != alarm as u8).unwrap_or(true);
                            }
                            if !filter {
                                return Some(SubscriptionCriteria::new(point_conf.name, cot))
                            }
                            None
                        } else {
                            panic!("{}.new | Invalid subscribe options format: {:#?}", self.id, options);
                        }
                    });
                    points.extend(creterias);
                    points
                });
                // let points = if points.is_empty() {None} else {Some(points)};
                subscriptions.entry(service.to_owned())
                    .and_modify(|entry| {
                        entry.as_mut().and_then(|v| {
                            v.append(&mut points);
                            Some(v)
                        }).or(if points.is_empty() {None} else {Some(&mut points)});
                    })
                    .or_insert(if points.is_empty() {None} else {Some(points)});
            }
            subscriptions
        } else {
            panic!("{}.new | Invalid subscribe format: {:#?}", self.id, self.conf);
        }
    }
}