use std::sync::{Arc, Mutex};
use log::{debug, warn};
use serde_json::json;
use crate::{conf::point_config::point_name::PointName, core_::{auth::ssh::auth_ssh::AuthSsh, cot::cot::Cot, net::protocols::jds::{jds_routes::RouterReply, request_kind::RequestKind}, point::{point::Point, point_type::PointType}, status::status::Status}, services::{multi_queue::subscription_criteria::SubscriptionCriteria, services::Services, tcp_server::tcp_server_cnnection::JdsState}};
use super::tcp_server_cnnection::Shared;

pub struct JdsConnection {}
impl JdsConnection {
    ///
    /// Detecting kind of the request stored as json string in the incoming point.
    /// Performs the action depending on the Request kind.
    pub fn handle_request(parent: &str, tx_id: usize, request: PointType, services: Arc<Mutex<Services>>, shared: &mut Shared) -> RouterReply {
        match RequestKind::from(request.name()) {
            RequestKind::AuthSecret => {
                debug!("{}/JdsConnection.handle_request | Request '{}': \n\t{:?}", parent, RequestKind::AUTH_SECRET, request);
                let (cot, message) = match &shared.auth {
                    crate::services::tcp_server::tcp_server_auth::TcpServerAuth::Secret(auth_secret) => {
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
                        &PointName::new(&parent, "/Auth.Secret").full(),
                        message.to_owned(), 
                        Status::Ok, 
                        cot, 
                        chrono::offset::Utc::now(),
                    ))),
                )
            },
            RequestKind::AuthSsh => {
                debug!("{}/JdsConnection.handle_request | Request '{}': \n\t{:?}", parent, RequestKind::AUTH_SSH, request);
                let (cot, message) = match &shared.auth {
                    crate::services::tcp_server::tcp_server_auth::TcpServerAuth::Ssh(auth_ssh_path) => {
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
                        &PointName::new(&parent, "/Auth.Ssh").full(),
                        message.to_owned(), 
                        Status::Ok, 
                        cot, 
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
                if let Err(err) = services.lock().unwrap().extend_subscription(&shared.tx_queue_name, &parent, &points) {
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