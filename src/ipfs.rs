use std::convert::TryFrom;
use std::ops::Deref;
use std::path::Path;

use cid::Cid;
use http::uri::Scheme;
use ipfs_api_backend_hyper::{IpfsClient as HyperIpfsClient, TryFromUri};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A remote connection to an ipfs node
pub struct IpfsRemote {
    pub scheme: String,
    pub host: String,
    pub port: u16,
    pub creds: Option<(String, String)>,
    pub gateway_url: Option<String>,
}

pub struct IpfsClient(HyperIpfsClient);

impl Default for IpfsClient {
    fn default() -> Self {
        Self(HyperIpfsClient::default())
    }
}

impl TryFrom<IpfsRemote> for IpfsClient {
    type Error = IpfsError;

    fn try_from(remote: IpfsRemote) -> Result<Self, IpfsError> {
        let client = HyperIpfsClient::from_host_and_port(
            Scheme::try_from(remote.scheme.as_str())?,
            &remote.host,
            remote.port,
        )?;
        Ok(Self(client))
    }
}

impl Deref for IpfsClient {
    type Target = HyperIpfsClient;

    fn deref(&self) -> &Self::Target {
        &self.0
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
    #[error("Failed to build client")]
    Client(#[from] ipfs_api_backend_hyper::Error),
    #[error("Failed to parse port")]
    Port(#[from] std::num::ParseIntError),
}
