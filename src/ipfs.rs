use std::convert::TryFrom;
use std::ops::Deref;
use std::path::Path;
use std::str::FromStr;

use cid::Cid;
use http::uri::Scheme;
use ipfs_api_backend_hyper::{IpfsClient as HyperIpfsClient, TryFromUri};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

pub use ipfs_api_backend_hyper::IpfsApi;
pub use ipfs_api_backend_hyper::request::Add as AddRequest;

/// A wrapper around a gateway url
pub struct IpfsGateway(Url);

impl IpfsGateway {
    pub async fn get(&self, cid: &Cid, path: &Path) -> Result<Vec<u8>, IpfsError> {
        let url = Url::parse(&format!("{}.{}/{}", cid, self.0, path.display()))?;
        let client = Client::builder().build()?;
        let resp = client.get(url).send().await?;
        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    }
}

// TODO: fancy looking display
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A remote connection to an ipfs node
pub struct IpfsRemote {
    pub url: String,
    pub gateway_url: Option<String>,
}

#[derive(Default)]
pub struct IpfsClient(HyperIpfsClient);

impl TryFrom<IpfsRemote> for IpfsClient {
    type Error = IpfsError;

    fn try_from(remote: IpfsRemote) -> Result<Self, IpfsError> {
        let url = Url::parse(&remote.url)?;
        let scheme = Scheme::try_from(url.scheme())?;
        let username = url.username();
        let password = url.password().unwrap_or("");
        let host_str = url.host_str().unwrap();
        let port = url.port().unwrap_or(5001);
        let client = HyperIpfsClient::from_host_and_port(scheme, host_str, port)?
            .with_credentials(username, password);
        Ok(Self(client))
    }
}

impl Deref for IpfsClient {
    type Target = HyperIpfsClient;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn hash_file_request() -> AddRequest<'static> {
    let mut add = AddRequest::default();
    add.pin = Some(false);
    add.cid_version = Some(1);
    add.only_hash = Some(true);
    add.hash = Some("blake3");
    add
}

pub fn add_file_request() -> AddRequest<'static> {
    let mut add = AddRequest::default();
    add.pin = Some(true);
    add.cid_version = Some(1);
    add.hash = Some("blake3");
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
