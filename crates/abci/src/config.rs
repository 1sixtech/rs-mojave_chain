use serde::{Deserialize, de::Error};
use std::path::{Path, PathBuf};
use tendermint::{Moniker, Timeout, node::Id};
use tendermint_config::{
    AbciMode, CorsHeader, CorsMethod, CorsOrigin, LogFormat, LogLevel, TransferRate, TxIndexer,
    net::Address,
};

#[derive(Clone, Debug, Deserialize)]
pub struct CometBftConfig {
    pub proxy_app: Address,
    pub moniker: Moniker,
    pub db_backend: DbBackend,
    pub db_dir: PathBuf,
    pub log_level: LogLevel,
    pub log_format: LogFormat,
    pub genesis_file: PathBuf,
    pub priv_validator_key_file: Option<PathBuf>,
    pub priv_validator_state_file: PathBuf,
    #[serde(deserialize_with = "deserialize_optional_value")]
    pub priv_validator_laddr: Option<Address>,
    pub node_key_file: PathBuf,
    pub abci: AbciMode,
    pub filter_peers: bool,
    pub rpc: Rpc,
    pub p2p: P2p,
    pub mempool: Mempool,
    pub statesync: Statesync,
    pub blocksync: Blocksync,
    pub consensus: Consensus,
    pub storage: Storage,
    pub tx_index: TxIndex,
    pub instrumentation: Instrumentation,
}

impl CometBftConfig {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, CometBftConfigError> {
        let config_string = std::fs::read_to_string(path).map_err(CometBftConfigError::Read)?;
        toml::from_str(&config_string).map_err(CometBftConfigError::Deserialize)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum DbBackend {
    #[serde(rename = "badgerdb")]
    BadgerDb,
    #[serde(rename = "goleveldb")]
    GoLevelDb,
    #[serde(rename = "pebbledb")]
    PebbleDb,
    #[serde(rename = "rocksdb")]
    RocksDb,
    #[serde(rename = "cleveldb")]
    ClevelDb,
    #[serde(rename = "boltdb")]
    BoltDb,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Rpc {
    pub laddr: Address,
    pub cors_allowed_origins: Vec<CorsOrigin>,
    pub cors_allowed_methods: Vec<CorsMethod>,
    pub cors_allowed_headers: Vec<CorsHeader>,
    #[serde(rename = "unsafe")]
    pub unsafe_commands: bool,
    pub max_open_connections: u64,
    pub max_subscription_clients: u64,
    pub max_subscriptions_per_client: u64,
    pub experimental_subscription_buffer_size: u64,
    pub experimental_websocket_write_buffer_size: u64,
    pub experimental_close_on_slow_client: bool,
    pub timeout_broadcast_tx_commit: Timeout,
    pub max_request_batch_size: u64,
    pub max_body_bytes: u64,
    pub max_header_bytes: u64,
    #[serde(deserialize_with = "deserialize_optional_value")]
    pub tls_cert_file: Option<PathBuf>,
    #[serde(deserialize_with = "deserialize_optional_value")]
    pub tls_key_file: Option<PathBuf>,
    #[serde(deserialize_with = "deserialize_optional_value")]
    pub pprof_laddr: Option<Address>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Grpc {
    #[serde(deserialize_with = "deserialize_optional_value")]
    pub laddr: Option<Address>,
    pub version_service: GrpcVersionService,
    pub block_service: GrpcBlockService,
    pub block_results_service: GrpcBlockResultsService,
    pub privileged: GrpcPrivileged,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GrpcVersionService {
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GrpcBlockService {
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GrpcBlockResultsService {
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GrpcPrivileged {
    #[serde(deserialize_with = "deserialize_optional_value")]
    pub laddr: Option<Address>,
    pub pruning_service: PruningService,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PruningService {
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct P2p {
    pub laddr: Address,
    #[serde(deserialize_with = "deserialize_optional_value")]
    pub external_address: Option<Address>,
    #[serde(deserialize_with = "deserialize_comma_separated_list")]
    pub seeds: Vec<Address>,
    #[serde(deserialize_with = "deserialize_comma_separated_list")]
    pub persistent_peers: Vec<Address>,
    pub addr_book_file: PathBuf,
    pub addr_book_strict: bool,
    pub max_num_inbound_peers: u64,
    pub max_num_outbound_peers: u64,
    #[serde(deserialize_with = "deserialize_comma_separated_list")]
    pub unconditional_peer_ids: Vec<Id>,
    pub persistent_peers_max_dial_period: Timeout,
    pub flush_throttle_timeout: Timeout,
    pub max_packet_msg_payload_size: u64,
    pub send_rate: TransferRate,
    pub recv_rate: TransferRate,
    pub pex: bool,
    pub seed_mode: bool,
    #[serde(deserialize_with = "deserialize_comma_separated_list")]
    pub private_peer_ids: Vec<Id>,
    pub allow_duplicate_ip: bool,
    pub handshake_timeout: Timeout,
    pub dial_timeout: Timeout,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Mempool {
    #[serde(rename = "type")]
    pub mempool_type: MempoolType,
    pub recheck: bool,
    pub recheck_timeout: Timeout,
    pub broadcast: bool,
    #[serde(deserialize_with = "deserialize_optional_value")]
    pub wal_dir: Option<PathBuf>,
    pub size: u64,
    pub max_tx_bytes: u64,
    pub max_txs_bytes: u64,
    pub cache_size: u64,
    #[serde(rename = "keep-invalid-txs-in-cache")]
    pub keep_invalid_txs_in_cache: bool,
    pub experimental_max_gossip_connections_to_persistent_peers: u64,
    pub experimental_max_gossip_connections_to_non_persistent_peers: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum MempoolType {
    #[serde(rename = "flood")]
    Flood,
    #[serde(rename = "nop")]
    Nop,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Statesync {
    pub enable: bool,
    #[serde(deserialize_with = "deserialize_comma_separated_list")]
    pub rpc_servers: Vec<String>,
    pub trust_height: u64,
    pub trust_hash: String,
    pub trust_period: String,
    pub discovery_time: Timeout,
    pub temp_dir: Option<PathBuf>,
    pub chunk_request_timeout: Timeout,
    pub chunk_fetchers: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Blocksync {
    pub version: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Consensus {
    pub wal_file: PathBuf,
    pub timeout_propose: Timeout,
    pub timeout_propose_delta: Timeout,
    pub timeout_prevote: Timeout,
    pub timeout_prevote_delta: Timeout,
    pub timeout_precommit: Timeout,
    pub timeout_precommit_delta: Timeout,
    pub timeout_commit: Timeout,
    pub double_sign_check_height: u64,
    pub skip_timeout_commit: bool,
    pub create_empty_blocks: bool,
    pub create_empty_blocks_interval: Timeout,
    pub peer_gossip_sleep_duration: Timeout,
    pub peer_query_maj23_sleep_duration: Timeout,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Storage {
    pub discard_abci_responses: bool,
    pub experimental_db_key_layout: DbKeyLayout,
    pub compact: bool,
    pub compaction_interval: String,
    pub pruning: StoragePruning,
}

#[derive(Clone, Debug, Deserialize)]
pub enum DbKeyLayout {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StoragePruning {
    pub interval: Timeout,
    pub data_companion: StoragePruningDataCompanion,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StoragePruningDataCompanion {
    pub enabled: bool,
    pub initial_block_retain_height: u64,
    pub initial_block_results_retain_height: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TxIndex {
    pub indexer: TxIndexer,
    #[serde(rename = "psql-conn")]
    pub psql_conn: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Instrumentation {
    pub prometheus: bool,
    pub prometheus_listen_addr: String,
    pub max_open_connections: u64,
    pub namespace: String,
}

/// Deserialize a comma separated list of types that impl `FromStr` as a `Vec`
fn deserialize_comma_separated_list<'de, D, T, E>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: std::str::FromStr<Err = E>,
    E: std::fmt::Display,
{
    let mut result = vec![];
    let string = String::deserialize(deserializer)?;

    if string.is_empty() {
        return Ok(result);
    }

    for item in string.split(',') {
        result.push(
            item.parse()
                .map_err(|e| D::Error::custom(format_args!("{e}")))?,
        );
    }

    Ok(result)
}

/// Deserialize `Option<T: FromStr>` where an empty string indicates `None`
fn deserialize_optional_value<'de, D, T, E>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: std::str::FromStr<Err = E>,
    E: std::fmt::Display,
{
    let string = Option::<String>::deserialize(deserializer).map(|str| str.unwrap_or_default())?;

    if string.is_empty() {
        return Ok(None);
    }

    string
        .parse()
        .map(Some)
        .map_err(|e| D::Error::custom(format!("{e}")))
}

pub enum CometBftConfigError {
    Read(std::io::Error),
    Deserialize(toml::de::Error),
}

impl std::fmt::Debug for CometBftConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(error) => write!(f, "{error}"),
            Self::Deserialize(error) => write!(f, "{error}"),
        }
    }
}

impl std::fmt::Display for CometBftConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for CometBftConfigError {}
