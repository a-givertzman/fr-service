//!
//! JdsService implements behavior on the JDS communication protocol for the following kinds of requests:
//! - "Subscribe" - request to begin Point's transmission
//!   - with list of Point names - initiate transfering only Point's subscribed on
//!   - without list of points -  initiate all Point's transfering
//! - "Points" - all points configurations requested
//! - "Auth" request - authentication requested
///
pub mod jds_service;
pub mod request_kind;