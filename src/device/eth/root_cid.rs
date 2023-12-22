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
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{EthClient, RootCidError};
use super::cid_token::CidToken;

const ABI_STRING: &str = include_str!("../../out/RootCid.sol/RootCid.json");

#[derive(Debug, this_error::Error)]
pub enum RootCidError {
    #[error("eth client error: {0}")]
    EthClient(#[from] E_thClientError),
    #[error()]
    #[error("No signer")]
    MissingSigner,
    #[error("Missing address")]
    MissingAddress,
    #[error("abi error: {0}")]
    Abi(#[from] ethers::abi::Error),
    #[error("serde json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("default error: {0}")]
    Default(String),
}

#[derive(Debug, Clone)]
pub struct RootCid(EthClient);

impl RootCid {    
    /// Create a new RootCid instance from an EthClient and Contract Address 
    pub fn new(eth_client: &mut EthClient, address: Address) -> Result<Self, RootCidError> {
        let abi_value: Value = serde_json::from_str(ABI_STRING)?;
        let abi: Abi = serde_json::from_value(abi_value["abi"].clone())?;
        let client = client.with_contract(address, abi)?;
        Ok(Self(client))
    }

    /// Grant the given address the ability to update the contract cid
    pub async fn grant_writer(
        &self,
        grantee_address: Address,
    ) -> Result<Option<TransactionReceipt>, RootCidError> {
        // TODO: This is janky, but we should have the contract available by now
        let contract = self.0.contract().unwrap();
        let chain_id = self.0.chain_id();
        let signer = match self.0.signer() {
            Some(signer) => signer,
            None => Err(RootCidError::MissingSigner),
        };
        
        let data = self
            .contract
            .encode("grantWriter", (address,))
            .map_err(|e| RootCidError::Default(e.to_string()))?;
        
        let tx = TransactionRequest::new()
            .to(contract.address())
            .data(data)
            .chain_id(chain_id);
                let signed_tx = signer
                    .send_transaction(tx, None)
                    .await
                    .map_err(|e| RootCidError::Default(e.to_string()))?;
                let reciept = signed_tx
                    .await
                    .map_err(|e| RootCidError::Default(e.to_string()))?;
                Ok(reciept)
    }

    /* CRUD */

    /// Read the current cid from the contract
    pub async fn read(&self) -> Result<Cid, RootCidError> {
        // TODO: This is janky, but we should have the contract available by now
        let contract = self.0.contract().unwrap();
        
        let cid: Cid = contract
            .method::<_, CidToken>("read", ())
            .map_err(|e| RootCidError::Default(e.to_string()))?
            .call()
            .await
            .map_err(|e| RootCidError::Default(e.to_string()))?
            .into();
        Ok(cid)
    }

    /// Update the current cid in the contract
    /// Requires a signer
    pub async fn update(
        &self,
        previous_cid: Cid,
        cid: Cid,
    ) -> Result<Option<TransactionReceipt>, RootCidError> {
        // TODO: This is janky, but we should have the contract available by now
        let contract = self.0.contract().unwrap();
        let chain_id = self.0.chain_id();
        let signer = match self.0.signer() {
            Some(signer) => signer,
            None => Err(RootCidError::MissingSigner),
        };
        let data = contract
            .encode("update", (CidToken(previous_cid), CidToken(cid)))
            .map_err(|e| RootCidError::Default(e.to_string()))?;
        let tx = TransactionRequest::new()
            .to(contract.address())
            .data(data)
            .chain_id(chain_id);
                let signed_tx = signer
                    .send_transaction(tx, None)
                    .await
                    .map_err(|e| RootCidError::Default(e.to_string()))?;
                let reciept = signed_tx
                    .await
                    .map_err(|e| RootCidError::Default(e.to_string()))?;
                Ok(reciept)
        }
    }
