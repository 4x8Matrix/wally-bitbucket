use semver::Version;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{auth::AuthMode, storage::StorageMode};
use libwally::git_auth::GitAuth;

#[derive(Deserialize, Serialize)]
pub struct Config {
    /// The URL of the Git repository containing the registry's package index.
    pub index_url: Url,

    pub git_auth: Option<GitAuth>,

    /// What kind of authentication is required to access endpoints.
    pub auth: AuthMode,

    /// Which storage backend to use.
    pub storage: StorageMode,

    /// The minimum wally cli version required to publish to the registry
    pub minimum_wally_version: Option<Version>,
}
