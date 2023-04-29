//! ...

use serde::{Deserialize, Serialize};
use std::fmt;

/// ...
#[derive(Deserialize, Serialize, Clone)]
pub struct SSHConfig {
    pub private_key: String,
    pub passphrase: Option<String>
}

/// ...
#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "value", rename_all = "kebab-case")]
pub enum GitAuth {
    SSH(SSHConfig),
    GithubToken(String)
}

impl fmt::Debug for GitAuth {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GitAuth::SSH(_) => write!(formatter, "SSH"),
            GitAuth::GithubToken(_) => write!(formatter, "GitHub Personal Access Token"),
        }
    }
}