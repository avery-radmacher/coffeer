use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Represents app-level configuration.
#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    /// The socket on which to listen for requests.
    listen_on: SocketAddr,
    /// The parties to the bet.
    parties: (String, String),
}
