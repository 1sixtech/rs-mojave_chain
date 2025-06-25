use std::{
    fmt,
    path::{Path, PathBuf},
};

use ethrex_common::types::{Genesis, GenesisError};

#[derive(Debug, Clone)]
pub enum Network {
    Mainnet,
    Testnet,
    GenesisPath(PathBuf),
}

impl From<&str> for Network {
    fn from(value: &str) -> Self {
        match value {
            "mainnet" => Network::Mainnet,
            "testnet" => Network::Testnet,
            s => Network::GenesisPath(PathBuf::from(s)),
        }
    }
}

impl From<PathBuf> for Network {
    fn from(value: PathBuf) -> Self {
        Network::GenesisPath(value)
    }
}

impl Default for Network {
    fn default() -> Self {
        Network::Mainnet
    }
}

impl Network {
    pub fn get_genesis_path(&self) -> &Path {
        match self {
            Network::Mainnet => todo!(),
            Network::Testnet => todo!(),
            Network::GenesisPath(s) => s,
        }
    }
    pub fn get_genesis(&self) -> Result<Genesis, GenesisError> {
        Genesis::try_from(self.get_genesis_path())
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Network::Mainnet => write!(f, "Mainnet"),
            Network::Testnet => write!(f, "Testnet"),
            Network::GenesisPath(path) => write!(f, "{:?}", path),
        }
    }
}
