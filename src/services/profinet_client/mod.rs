//!
//! Cyclically reads adressess from the PROFINET device and yields changed to the MultiQueue
//! Writes Point to the protocol (PROFINET device) specific address
//!
pub mod profinet_client;

pub mod profinet_db;

pub mod s7;

pub mod parse_point;