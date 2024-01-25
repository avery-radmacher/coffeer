use crate::io;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, path::PathBuf};

/// Represents app-level configuration.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    /// The socket on which to listen for requests.
    pub listen_on: SocketAddr,
    /// The parties to the bet.
    pub parties: (String, String),
}

pub fn try_get_app_config() -> Option<AppConfig> {
    let path = PathBuf::from(".\\data\\config.json");
    if !path.exists() {
        return None;
    }

    Some(io::read_json_file(&path))
}
