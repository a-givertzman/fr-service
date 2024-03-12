use std::sync::{Arc, Mutex};

use log::{debug, warn};
use serde_json::json;

use crate::{conf::point_config::point_name::PointName, core_::{cot::cot::Cot, net::protocols::jds::{jds_routes::RouterReply, request_kind::RequestKind}, object::object::Object, point::{point::Point, point_type::PointType}, status::status::Status}, services::{multi_queue::subscription_criteria::SubscriptionCriteria, services::Services}};

pub struct JdsConnection {}
impl JdsConnection {
    ///
    /// Detecting kind of the request stored as json string in the incoming point.
    /// Performs the action depending on the Request kind.
    pub fn handle_request(parent: &str, tx_id: usize, request: PointType, services: Arc<Mutex<Services>>, tx_queue_name: &str) -> RouterReply {
        match RequestKind::from(request.name()) {
            RequestKind::AuthSecret => {
                debug!("{}/JdsConnection.handle_request | Request '{}': \n\t{:?}", parent, RequestKind::AUTH_SECRET, request);
                RouterReply::new(
                    None,
                    Some(PointType::String(Point::new(
                        tx_id, 
                        &PointName::new(&parent, "/Auth.Secret").full(),
                        r#"{
                            \"reply\": \"Auth.Secret Reply\"
                        }"#.to_string(), 
                        Status::Ok, 
                        Cot::ReqCon, 
                        chrono::offset::Utc::now(),
                    ))),
                )
            },
            RequestKind::AuthSsh => {
                debug!("{}/JdsConnection.handle_request | Request '{}': \n\t{:?}", parent, RequestKind::AUTH_SSH, request);
                RouterReply::new(
                    None,
                    Some(PointType::String(Point::new(
                        tx_id, 
                        &PointName::new(&parent, "/Auth.Secret").full(),
                        r#"{
                            \"reply\": \"Auth.Ssh Reply\"
                        }"#.to_string(), 
                        Status::Ok, 
                        Cot::ReqCon, 
                        chrono::offset::Utc::now(),
                    ))),
                )
            },
            RequestKind::Points => {
                debug!("{}/JdsConnection.handle_request | Request '{}': \n\t{:?}", parent, RequestKind::POINTS, request);
                let points = services.lock().unwrap().points();
                let points = json!(points).to_string();
                RouterReply::new(
                    None,
                    Some(PointType::String(Point::new(
                        tx_id, 
                        &PointName::new(&parent, "/Points").full(),
                        points, 
                        Status::Ok, 
                        Cot::ReqCon, 
                        chrono::offset::Utc::now(),
                    ))),
                )
            },
            RequestKind::Subscribe => {
                debug!("{}/JdsConnection.handle_request | Request '{}': \n\t{:?}", parent, RequestKind::SUBSCRIBE, request);
                let points = match serde_json::from_str(&request.value().as_string()) {
                    Ok(points) => {
                        let points: serde_json::Value = points;
                        match points.as_array() {
                            Some(points) => {
                                debug!("{}.handle_request | 'Subscribe' request (multicast): {:?}", parent, request);
                                points.iter().fold(vec![], |mut points, point| {
                                    if let Some(point_name) = point.as_str() {
                                        points.extend(
                                            Self::map_points_to_creteria(point_name, vec![Cot::Inf, Cot::ActCon, Cot::ActErr])
                                        );
                                    }
                                    points
                                })
                            },
                            None => {
                                debug!("{}.handle_request | 'Subscribe' request (broadcast): {:?}", parent, request);
                                services.lock().unwrap().points().iter().fold(vec![], |mut points, point_conf| {
                                    points.extend(
                                        Self::map_points_to_creteria(&point_conf.name, vec![Cot::Inf, Cot::ActCon, Cot::ActErr])
                                    );
                                    points
                                })        
                            },
                        }
                    },
                    Err(err) => {
                        warn!("{}.handle_request | 'Subscribe' request parsing error: {:?}\n\t request: {:?}", parent, err, request);
                        services.lock().unwrap().points().iter().fold(vec![], |mut points, point_conf| {
                            points.extend(
                                Self::map_points_to_creteria(&point_conf.name, vec![Cot::Inf, Cot::ActCon, Cot::ActErr])
                            );
                            points
                        })
                    },
                };
                if let Err(err) = services.lock().unwrap().extend_subscription(tx_queue_name, &parent, &points) {
                    warn!("{}.handle_request | extend_subscription failed with error: {:?}", parent, err);
                };
                RouterReply::new(None, None)
            },
            RequestKind::Unknown => {
                debug!("{}/JdsConnection.handle_request | Unknown request: \n\t{:?}", parent, request);
                warn!("{}/JdsConnection.handle_request | Unknown request name: {:?}", parent, request.name());
                RouterReply::new(None, None)
            },
        }
    }
    ///
    /// Creates list of SubscriptionCriteria contains all variations of given [point_name] and Cot's
    fn map_points_to_creteria<'a>(point_name: &'a str, cots: Vec<Cot>) -> Box<dyn Iterator<Item = SubscriptionCriteria> + 'a> {
        Box::new(cots.into_iter().map(|cot| {
            SubscriptionCriteria::new(point_name.to_string(), cot)
        }))
    }
}