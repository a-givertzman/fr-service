//! ### Services implemented for the application
//! **Service**:
//! - executed in the separate thread, can be multi thread
//! - basicaly must be defined in the main configuration file like:
//! ```yaml
//! service ServiceName Id:
//!     in queue in-queue:
//!         max-length: 10000
//!     out queue: MultiQueue.in-queue
//! ```
///
pub mod api_cient;

pub mod tcp_client;

pub mod profinet_client;

pub mod queue;

pub mod task;

pub mod service;

pub mod services;

pub mod multi_queue;

pub mod queue_name;

pub mod server;