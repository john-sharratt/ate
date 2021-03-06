#![cfg_attr(not(debug_assertions), allow(dead_code, unused_imports, unused_variables))]

/// You can change the hashing routine with these features
/// - feature = "use_blake3"
/// - feature = "use_sha3"

/// You can change the log file format with these features
/// - feature = "use_version1"
/// - feature = "use_version2"

pub const HASH_ROUTINE: crypto::HashRoutine = if cfg!(feature = "use_blake3") {
    crypto::HashRoutine::Blake3
} else if cfg!(feature = "use_sha3") {
    crypto::HashRoutine::Sha3
} else {
    crypto::HashRoutine::Blake3
};

pub const LOG_VERSION: spec::EventVersion = spec::EventVersion::V2;

pub mod utils;
pub mod error;
pub mod spec;
pub mod crypto;
pub mod header;
pub mod meta;
pub mod event;
pub mod conf;
pub mod comms;
pub mod mesh;
pub mod redo;
pub mod sink;
pub mod session;
pub mod validator;
pub mod compact;
pub mod index;
pub mod lint;
pub mod loader;
pub mod transform;
pub mod plugin;
pub mod signature;
pub mod time;
pub mod tree;
pub mod trust;
pub mod chain;
pub mod single;
pub mod multi;
pub mod transaction;
pub mod dio;
pub mod service;
pub mod pipe;
pub mod prelude;
pub mod anti_replay;
pub mod flow;
pub mod repository;