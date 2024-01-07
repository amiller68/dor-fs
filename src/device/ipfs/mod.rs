use std::convert::TryFrom;
use std::fmt::Display;
use std::ops::Deref;
use std::path::PathBuf;

use cid::Cid;
use http::uri::Scheme;
use ipfs_api_backend_hyper::{IpfsClient as HyperIpfsClient, TryFromUri};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

pub use ipfs_api_backend_hyper::request::Add as AddRequest;
pub use ipfs_api_backend_hyper::IpfsApi;

/// Default cid version to use when adding or hashing datat against the IPFS API
const DEFAULT_CID_VERSION: u32 = 1;
/// Default hash function to use when adding or hashing data against the IPFS API
const DEFAULT_HASH_FUNCTION: &str = "blake3";

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

/// A wrapper around a gateway url
pub struct IpfsGateway(Url);

impl Default for IpfsGateway {
    fn default() -> Self {
        Self(Url::parse("http://127.0.0.1:8080").unwrap())
    }
}

impl From<IpfsRemote> for IpfsGateway {
    fn from(remote: IpfsRemote) -> Self {
        Self(remote.gateway_url.clone())
    }
}

impl IpfsGateway {
    // TODO: this isn't working quite right
    pub async fn get(&self, cid: &Cid, path: Option<PathBuf>) -> Result<Vec<u8>, IpfsError> {
        let maybe_port = self.0.port();
        let scheme = Scheme::try_from(self.0.scheme())?;
        let host_str = match maybe_port {
            Some(port) => format!("{}:{}", self.0.host_str().unwrap(), port),
            None => self.0.host_str().unwrap().to_string(),
        };
        let url = match path {
            Some(p) => Url::parse(&format!("{}://{}.ipfs.{}/{}", scheme, cid, host_str, p.display())),
            None => Url::parse(&format!("{}://{}.ipfs.{}", scheme, cid, host_str)),
        }?;
        // TODO: not 100% sure why I need to use trust_dns here, but this works
        let client = Client::builder().trust_dns(true).build()?;
        let resp = client.get(url).send().await?;
        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    }
}

/// Wrapper around a Hyper IPFS backend
#[derive(Default)]
pub struct IpfsClient(HyperIpfsClient);

impl TryFrom<IpfsRemote> for IpfsClient {
    type Error = IpfsError;

    fn try_from(remote: IpfsRemote) -> Result<Self, IpfsError> {
        let url = remote.api_url.clone();
        let scheme = Scheme::try_from(url.scheme())?;
        let username = url.username();
        let maybe_password = url.password();
        let host_str = url.host_str().unwrap();
        let port = url.port().unwrap_or(5001);
        let client = match maybe_password {
            Some(password) => HyperIpfsClient::from_host_and_port(scheme, host_str, port)?
                .with_credentials(username, password),
            None => HyperIpfsClient::from_host_and_port(scheme, host_str, port)?,
        };
        Ok(Self(client))
    }
}

impl Deref for IpfsClient {
    type Target = HyperIpfsClient;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: fix this -- wanna get rid of warnings

#[allow(clippy::field_reassign_with_default)]
pub fn hash_data_request() -> AddRequest<'static> {
    let mut add = AddRequest::default();
    add.pin = Some(false);
    add.cid_version = Some(DEFAULT_CID_VERSION);
    add.only_hash = Some(true);
    add.hash = Some(DEFAULT_HASH_FUNCTION);
    add
}

#[allow(clippy::field_reassign_with_default)]
pub fn add_data_request() -> AddRequest<'static> {
    let mut add = AddRequest::default();
    add.pin = Some(true);
    add.cid_version = Some(DEFAULT_CID_VERSION);
    add.hash = Some(DEFAULT_HASH_FUNCTION);
    add
}

pub type IpfsClientError = ipfs_api_backend_hyper::Error;

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
    #[error("Failed to build client")]
    Client(#[from] IpfsClientError),
    #[error("Failed to parse port")]
    Port(#[from] std::num::ParseIntError),
}
