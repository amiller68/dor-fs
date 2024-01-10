use std::fmt::Display;

use serde::{Deserialize, Serialize};
use url::Url;

#[cfg(not(target_arch = "wasm32"))]
mod client;
mod gateway;

#[cfg(not(target_arch = "wasm32"))]
pub use ipfs_api_backend_hyper::IpfsApi;
#[cfg(not(target_arch = "wasm32"))]
pub use client::{IpfsClient, IpfsClientError, add_data_request, hash_data_request};

pub use gateway::IpfsGateway;

/// A connection to an IPFS remote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpfsRemote {
    /// Url pointing to an IPFS api
    /// Must include valid authentication if required
    pub api_url: Url,
    /// Url pointing to a public IPFS gateway
    /// Should not require or include authentication
    pub gateway_url: Url,
}

impl Default for IpfsRemote {
    fn default() -> Self {
        // Just use the default kubo configuration
        Self {
            api_url: Url::parse("http://127.0.0.1:5001").unwrap(),
            gateway_url: Url::parse("http://127.0.0.1:8080").unwrap(),
        }
    }
}

impl Display for IpfsRemote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let api_url = self.api_url.clone();
        let gateway_url = self.gateway_url.clone();
        write!(f, "api_url: {}\ngateway_url: {}", api_url, gateway_url)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IpfsError {
    #[error("url parse error")]
    Url(#[from] url::ParseError),
    #[error("Failed to send request")]
    Reqwest(#[from] reqwest::Error),
    #[error("http error")]
    Http(#[from] http::Error),
    #[error("Failed to parse scheme")]
    Scheme(#[from] http::uri::InvalidUri),
    #[cfg(not(target_arch = "wasm32"))]
    #[error("Failed to build client")]
    Client(#[from] IpfsClientError),
    #[error("Failed to parse port")]
    Port(#[from] std::num::ParseIntError),
}
