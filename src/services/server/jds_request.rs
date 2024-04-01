use std::{collections::HashMap, sync::{Arc, Mutex, RwLock}};
use concat_string::concat_string;
use log::{debug, warn};
use serde_json::json;
use crate::{
    conf::point_config::{name::Name, point_config::PointConfig},
    core_::{
        auth::ssh::auth_ssh::AuthSsh, 
        cot::cot::Cot, 
        net::protocols::jds::request_kind::RequestKind, 
        point::{point::Point, point_type::PointType},
        status::status::Status,
    }, services::{
        multi_queue::subscription_criteria::SubscriptionCriteria,
        safe_lock::SafeLock,
        server::{jds_routes::RouterReply, jds_cnnection::JdsState},
        services::Services,
    }
};
use super::jds_cnnection::Shared;

pub struct JdsRequest {}
impl JdsRequest {
    ///
    /// Detecting kind of the request stored as json string in the incoming point.
    /// Performs the action depending on the Request kind.
    pub fn handle(parent_id: &str, parent: &Name, tx_id: usize, request: PointType, services: Arc<Mutex<Services>>, shared: Arc<RwLock<Shared>>) -> RouterReply {
        let mut shared = shared.write().unwrap();
        let self_id = concat_string!(parent_id, "/JdsRequest");
        let requester_name = &parent.join();
        match RequestKind::from(request.name()) {
            RequestKind::AuthSecret => {
                debug!("{}.handle | Request '{}': \n\t{:?}", self_id, RequestKind::AUTH_SECRET, request);
                let (cot, message) = match &shared.auth {
                    crate::services::server::jds_auth::TcpServerAuth::Secret(auth_secret) => {
                        let secret = match request {
                            PointType::String(request) => request.value,
                            _ => String::new(),
                        };
                        if secret == auth_secret.token() {
                            shared.jds_state = JdsState::Authenticated;
                            (Cot::ReqCon, "Authentication successful")
                        } else {
                            (Cot::ReqErr, "Authentication error: Invalid secret or kind of auth request")
                        }
                    },
                    _ => {
                        (Cot::ReqErr, "Authentication error: Invalid secret or kind of auth request")
                    }
                };
                RouterReply::new(
                    None,
                    Some(PointType::String(Point::new(
                        tx_id, 
                        &Name::new(parent, "/Auth.Secret").join(),
                        message.to_owned(), 
                        Status::Ok, 
                        cot, 
                        chrono::offset::Utc::now(),
                    ))),
                )
            },
            RequestKind::AuthSsh => {
                debug!("{}.handle | Request '{}': \n\t{:?}", self_id, RequestKind::AUTH_SSH, request);
                let (cot, message) = match &shared.auth {
                    crate::services::server::jds_auth::TcpServerAuth::Ssh(auth_ssh_path) => {
                        let secret = match request {
                            PointType::String(request) => request.value,
                            _ => String::new(),
                        };
                        match AuthSsh::new(&auth_ssh_path.path()).validate(&secret) {
                            Ok(_) => {
                                shared.jds_state = JdsState::Authenticated;
                                (Cot::ReqCon, "Authentication successful".to_owned())
                            },
                            Err(err) => {
                                (Cot::ReqErr, format!("Authentication error: {}", err))
                            },
                        }
                    },
                    _ => {
                        (Cot::ReqErr, "Authentication error: Invalid secret or kind of auth request".to_owned())
                    }
                };
                RouterReply::new(
                    None,
                    Some(PointType::String(Point::new(
                        tx_id, 
                        &Name::new(parent, "/Auth.Ssh").join(),
                        message.to_owned(), 
                        Status::Ok, 
                        cot, 
                        chrono::offset::Utc::now(),
                    ))),
                )
            },
            RequestKind::Points => {
                debug!("{}.handle | Request '{}': \n\t{:?}", self_id, RequestKind::POINTS, request);
                let points = services.slock().points(requester_name);
                let points: HashMap<String, &PointConfig> = points.iter().map(|conf| {
                    (conf.name.clone(), conf)
                }).collect();
                let points = json!(points).to_string();
                let reply = RouterReply::new(
                    None,
                    Some(PointType::String(Point::new(
                        tx_id, 
                        &Name::new(parent, "/Points").join(),
                        points, 
                        Status::Ok, 
                        Cot::ReqCon, 
                        chrono::offset::Utc::now(),
                    ))),
                );
                debug!("{}.handle | Reply: \n\t{:?}", self_id, reply);
                reply
            },
            RequestKind::Subscribe => {
                debug!("{}.handle | Request '{}': \n\t{:?}", self_id, RequestKind::SUBSCRIBE, request);
                let points = match serde_json::from_str(&request.value().as_string()) {
                    Ok(points) => {
                        let points: serde_json::Value = points;
                        match points.as_array() {
                            Some(points) => {
                                debug!("{}.handle_request | 'Subscribe' request (multicast): {:?}", self_id, request);
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
                                debug!("{}.handle_request | 'Subscribe' request (broadcast): {:?}", self_id, request);
                                services.slock().points(requester_name).iter().fold(vec![], |mut points, point_conf| {
                                    points.extend(
                                        Self::map_points_to_creteria(&point_conf.name, vec![Cot::Inf, Cot::ActCon, Cot::ActErr])
                                    );
                                    points
                                })        
                            },
                        }
                    },
                    Err(err) => {
                        warn!("{}.handle_request | 'Subscribe' request parsing error: {:?}\n\t request: {:?}", self_id, err, request);
                        services.slock().points(requester_name).iter().fold(vec![], |mut points, point_conf| {
                            points.extend(
                                Self::map_points_to_creteria(&point_conf.name, vec![Cot::Inf, Cot::ActCon, Cot::ActErr])
                            );
                            points
                        })
                    },
                };
                let receiver_name = Name::new(parent, &shared.connection_id).join();
                let (cot, message) = match services.slock().extend_subscription(&shared.subscribe, &receiver_name, &points) {
                    Ok(_) => (Cot::ReqCon, "".to_owned()),
                    Err(err) => {
                        let message = format!("{}.handle_request | extend_subscription failed with error: {:?}", self_id, err);
                        warn!("{}", message);
                        (Cot::ReqErr, message)
                    },
                };
                RouterReply::new(
                    None,
                    Some(PointType::String(Point::new(
                        tx_id, 
                        &Name::new(parent, "/Subscribe").join(),
                        message, 
                        Status::Ok, 
                        cot, 
                        chrono::offset::Utc::now(),
                    ))),
                )                
            },
            RequestKind::Unknown => {
                debug!("{}.handle | Unknown request: \n\t{:?}", self_id, request);
                warn!("{}.handle | Unknown request name: {:?}", self_id, request.name());
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