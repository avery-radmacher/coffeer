use crate::io;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{net::SocketAddr, path::PathBuf};

/// Represents app-level configuration.
#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    /// The socket on which to listen for requests.
    listen_on: SocketAddr,
    /// The parties to the bet.
    parties: (String, String),
}

pub fn try_get_app_config() -> Option<AppConfig> {
    let path = PathBuf::from_str(".\\data\\config.json").expect("Error creating path");
    if !path.exists() {
        return None;
    }

    Some(io::read_json_file(&path))
}
