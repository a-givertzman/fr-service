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
    subscriptions: HashMap<String, Vec<SubscriptionCriteria>>,
}
///
/// Creates new instance from yaml
impl ConfSubscribe {
    pub fn new(conf: serde_yaml::Value, points: Vec<PointConfig>) -> Self {
        let self_id = "ConfSubscribe".to_owned();
        let point_configs = points;
        let subscriptions = if conf.is_string() {
            let service = conf.as_str().unwrap().to_owned();
            HashMap::from([
                (service, vec![])
            ])
        } else if conf.is_mapping() {
            let mut subscriptions = HashMap::<String, Vec<SubscriptionCriteria>>::new();
            for (service, criterias) in conf.as_mapping().unwrap() {
                let service = service.as_str().unwrap();
                let mut points = criterias.as_mapping().unwrap().into_iter().fold(vec![], |mut points, (options, names)| {
                    let names = names.as_sequence().unwrap();
                    let point = if names.is_empty() {
                        point_configs.clone()
                    } else {
                        names.iter().filter_map(|name| {
                            let name: String = serde_yaml::from_value(name.clone()).unwrap();
                            point_configs.iter().find(|&point_conf| point_conf.name == name)
                        }).cloned().collect()
                    };

                    let creterias = point.into_iter().filter_map(|point_conf| {
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
                                filter &= point_conf.history != history
                            }
                            if let Some(alarm) = alarm {
                                filter &= point_conf.alarm.map(|v| v != alarm as u8).unwrap_or(true);
                            }
                            if !filter {
                                return Some(SubscriptionCriteria::new(point_conf.name, cot))
                            }
                            None
                        } else {
                            panic!("{}.new | Invalid subscribe options format: {:#?}", self_id, options);
                        }
                    });
                    points.extend(creterias);
                    points
                });
                subscriptions.entry(service.to_owned())
                    .and_modify(|entry| {
                        entry.append(&mut points)
                    })
                    .or_insert(points);
            }
            subscriptions
        } else {
            panic!("{}.new | Invalid subscribe format: {:#?}", self_id, conf);
        };
        Self { subscriptions }
    }
    // ///
    // /// Reurns list of SubscriptionCriteria's based on passed points (PointConfig list) and subscribe configuration
    // pub fn with(points: Vec<PointConfig>) -> Vec<SubscriptionCriteria> {
    //     panic!("ConfSubscribe.with | Not implemented yet");
    // }
}