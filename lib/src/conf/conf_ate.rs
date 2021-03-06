#[allow(unused_imports)]
use log::{info, error, debug};
use std::time::Duration;

use crate::crypto::KeySize;
use crate::spec::*;
use crate::mesh::RecoveryMode;
use crate::compact::CompactMode;

use super::*;

/// Configuration settings for the ATE datastore
///
#[derive(Debug, Clone)]
pub struct ConfAte
{
    /// Optimizes ATE for a specific group of usecases.
    pub(super) configured_for: ConfiguredFor,

    /// Specifies the recovery mode that the mesh will take.
    pub recovery_mode: RecoveryMode,

    /// Specifies the log compaction mode for the redo log.
    pub compact_mode: CompactMode,

    /// Compacts the redo log on bootstrapping of the program.
    pub compact_bootstrap: bool,

    /// Directory path that the redo logs will be stored.
    #[cfg(feature = "local_fs")]
    pub log_path: Option<String>,

    /// NTP pool server which ATE will synchronize its clocks with, its
    /// important to have synchronized clocks with ATE as it uses time as
    /// digest to prevent replay attacks
    pub ntp_pool: String,
    /// Port that the NTP server is listening on (defaults to 123)
    pub ntp_port: u16,
    /// Flag that indicates if the time keeper will sync with NTP or not
    /// (avoiding NTP sync means one can run fully offline but time drift
    ///  will cause issues with multi factor authentication and timestamps)
    pub ntp_sync: bool,

    /// Flag that determines if ATE will use DNSSec or just plain DNS
    pub dns_sec: bool,
    /// DNS server that queries will be made do by the chain registry
    pub dns_server: String,

    /// Synchronization tolerance whereby event duplication during connection phases
    /// and compaction efficiency are impacted. Greater tolerance will reduce the
    /// possibility of data lose on specific edge-cases while shorter tolerance will
    /// improve space and network efficiency. It is not recommended to select a value
    /// lower than a few seconds while increasing the value to days will impact performance.
    /// (default=30 seconds)
    pub sync_tolerance: Duration,

    /// Flag that indicates if encryption will be used for the underlying
    /// connections over the wire. When using a ATE's in built encryption
    /// and quantum resistant signatures it is not mandatory to use
    /// wire encryption as confidentially and integrity are already enforced however
    /// for best security it is advisable to apply a layered defence, of
    /// which double encrypting your data and the metadata around it is
    /// another defence.
    pub wire_encryption: Option<KeySize>,

    /// Size of the buffer on mesh clients, tweak this number with care
    pub buffer_size_client: usize,
    /// Size of the buffer on mesh servers, tweak this number with care
    pub buffer_size_server: usize,

    /// Size of the local cache that stores redo log entries in memory
    #[cfg(feature = "local_fs")]
    pub load_cache_size: usize,
    /// Number of seconds that redo log entries will remain in memory before
    /// they are evicted
    #[cfg(feature = "local_fs")]
    pub load_cache_ttl: u64,

    /// Serialization format of the log files
    pub log_format: MessageFormat,
    /// Serialization format of the data on the network pipes between nodes and clients
    pub wire_format: SerializationFormat,

    /// Time to wait for a connection to a server before it times out
    pub connect_timeout: Duration,

    /// Default port that the ATE protocol will run on (port 5000)
    pub default_port: u16
}

impl Default
for ConfAte
{
    fn default() -> ConfAte {
        ConfAte {
            #[cfg(feature = "local_fs")]
            log_path: None,
            dns_sec: false,
            dns_server: "8.8.8.8".to_string(),
            recovery_mode: RecoveryMode::ReadOnlyAsync,
            compact_mode: CompactMode::Never,
            compact_bootstrap: false,
            sync_tolerance: Duration::from_secs(30),
            ntp_sync: true,
            ntp_pool: "pool.ntp.org".to_string(),
            ntp_port: 123,
            wire_encryption: Some(KeySize::Bit128),
            configured_for: ConfiguredFor::default(),
            buffer_size_client: 2,
            buffer_size_server: 10,
            #[cfg(feature = "local_fs")]
            load_cache_size: 1000,
            #[cfg(feature = "local_fs")]
            load_cache_ttl: 30,
            log_format: MessageFormat {
                meta: SerializationFormat::Bincode,
                data: SerializationFormat::Json,
            },
            wire_format: SerializationFormat::Bincode,
            connect_timeout: Duration::from_secs(30),
            default_port: 5000,
        }
    }
}