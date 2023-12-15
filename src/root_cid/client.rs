use std::sync::Arc;

use cid::Cid;
use ethers::{
    abi::Abi,
    abi::{InvalidOutputType, Tokenizable},
    contract::Contract,
    prelude::*,
    providers::{Http, Provider},
    signers::LocalWallet,
    types::{Address, TransactionRequest},
};

const ABI_STRING: &str = include_str!("../../out/RootCid.sol/RootCid.json");

// TODO: better error handling
pub struct Client {
    pub(crate) contract: Contract<ethers::providers::Provider<Http>>,
    signer: Option<SignerMiddleware<Provider<Http>, LocalWallet>>,
    chain_id: u64,
}

impl Client {
    pub fn new(
        url: String,
        address: String,
        chain_id: u64,
        key: Option<String>,
    ) -> Result<Self, ClientError> {
        let client =
            Provider::<Http>::try_from(url).map_err(|e| ClientError::Default(e.to_string()))?;
        let address: Address = address
            .parse()
            .map_err(|_| ClientError::Default("Invalid Address".to_string()))?;
        let abi: Abi =
            serde_json::from_str(ABI_STRING).map_err(|e| ClientError::Default(e.to_string()))?;
        let contract = Contract::new(address, abi, Arc::new(client.clone()));
        let signer = match key {
            Some(key) => {
                let wallet = key
                    .parse::<LocalWallet>()
                    .map_err(|_| ClientError::Default("Invalid Key".to_string()))?
                    .with_chain_id(chain_id);
                let signer = SignerMiddleware::new(client, wallet);
                Some(signer)
            }
            None => None,
        };
        Ok(Self {
            contract,
            signer,
            chain_id,
        })
    }

    /* Permissions */

    /// Grant the given address the ability to write to the contract cid
    pub async fn grant_writer(
        &self,
        address: Address,
    ) -> Result<Option<TransactionReceipt>, ClientError> {
        let data = self
            .contract
            .encode("grantWriter", (address,))
            .map_err(|e| ClientError::Default(e.to_string()))?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data)
            .chain_id(self.chain_id);
        match self.signer {
            Some(ref signer) => {
                let signed_tx = signer
                    .send_transaction(tx, None)
                    .await
                    .map_err(|e| ClientError::Default(e.to_string()))?;
                let reciept = signed_tx
                    .await
                    .map_err(|e| ClientError::Default(e.to_string()))?;
                Ok(reciept)
            }
            None => {
                Err(ClientError::NoSigner)
            }
        }
    }

    /* CRUD */

    /// Read the current cid from the contract
    pub async fn read(&self) -> Result<Cid, ClientError> {
        let cid: Cid = self
            .contract
            .method::<_, CidWrapper>("get", ())
            .map_err(|e| ClientError::Default(e.to_string()))?
            .call()
            .await
            .map_err(|e| ClientError::Default(e.to_string()))?
            .into();
        Ok(cid)
    }

    /// Update the current cid in the contract
    /// Requires a signer
    pub async fn update(&self, cid: Cid) -> Result<Option<TransactionReceipt>, ClientError> {
        let data = self
            .contract
            .encode("update", (CidWrapper(cid),))
            .map_err(|e| ClientError::Default(e.to_string()))?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data)
            .chain_id(self.chain_id);
        match self.signer {
            Some(ref signer) => {
                let signed_tx = signer
                    .send_transaction(tx, None)
                    .await
                    .map_err(|e| ClientError::Default(e.to_string()))?;
                let reciept = signed_tx
                    .await
                    .map_err(|e| ClientError::Default(e.to_string()))?;
                Ok(reciept)
            }
            None => {
                Err(ClientError::NoSigner)
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("Client Error: {0}")]
    Default(String),
    #[error("No Signer")]
    NoSigner,
}

struct CidWrapper(Cid);

impl From<Cid> for CidWrapper {
    fn from(cid: Cid) -> Self {
        Self(cid)
    }
}

impl From<CidWrapper> for Cid {
    fn from(val: CidWrapper) -> Self {
        val.0
    }
}

impl Tokenizable for CidWrapper {
    fn from_token(token: ethers::abi::Token) -> Result<Self, InvalidOutputType> {
        let bytes = token
            .into_bytes()
            .ok_or(InvalidOutputType("Invalid Bytes".to_string()))?;
        let cid = Cid::try_from(bytes).map_err(|_| InvalidOutputType("Invalid CID".to_string()))?;
        Ok(Self(cid))
    }

    fn into_token(self) -> ethers::abi::Token {
        let bytes = self.0.to_bytes();
        ethers::abi::Token::Bytes(bytes)
    }
}

// TODO: client unit tests
