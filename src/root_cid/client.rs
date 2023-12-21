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

const ABI_STRING: &str = include_str!("../../out/RootCid.sol/RootCid.json");

// TODO: fancy looking display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthRemote {
    pub rpc: String,
    pub address: String,
    pub chain_id: u16,
}

// TODO: better error handling
pub struct EthClient {
    client: Provider<Http>,
    contract: Contract<ethers::providers::Provider<Http>>,
    signer: Option<SignerMiddleware<Provider<Http>, LocalWallet>>,
    chain_id: u16,
}

impl TryFrom<EthRemote> for EthClient {
    type Error = EthClientError;

    fn try_from(remote: EthRemote) -> Result<Self, Self::Error> {
        let client = Provider::<Http>::try_from(remote.rpc)
            .map_err(|e| EthClientError::Default(e.to_string()))?;
        let address: Address = remote
            .address
            .parse()
            .map_err(|_| EthClientError::Default("Invalid Address".to_string()))?;
        let out_value: Value = serde_json::from_str(ABI_STRING)?;
        let abi: Abi = serde_json::from_value(out_value["abi"].clone())?;
        let contract = Contract::new(address, abi, Arc::new(client.clone()));
        Ok(Self {
            client,
            contract,
            signer: None,
            chain_id: remote.chain_id,
        })
    }
}

impl EthClient {
    /// Attach SignerMiddleware to the client
    pub fn with_wallet_as_signer(&mut self, wallet: LocalWallet) -> Result<&Self, EthClientError> {
        let wallet = wallet.with_chain_id(self.chain_id);
        let signer = SignerMiddleware::new(self.client.clone(), wallet);
        self.signer = Some(signer);
        Ok(self)
    }

    /// Grant the given address the ability to write to the contract cid
    pub async fn grant_writer(
        &self,
        address: Address,
    ) -> Result<Option<TransactionReceipt>, EthClientError> {
        let data = self
            .contract
            .encode("grantWriter", (address,))
            .map_err(|e| EthClientError::Default(e.to_string()))?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data)
            .chain_id(self.chain_id);
        match self.signer {
            Some(ref signer) => {
                let signed_tx = signer
                    .send_transaction(tx, None)
                    .await
                    .map_err(|e| EthClientError::Default(e.to_string()))?;
                let reciept = signed_tx
                    .await
                    .map_err(|e| EthClientError::Default(e.to_string()))?;
                Ok(reciept)
            }
            None => Err(EthClientError::MissingSigner),
        }
    }

    /* CRUD */

    /// Read the current cid from the contract
    pub async fn read(&self) -> Result<Cid, EthClientError> {
        let cid: Cid = self
            .contract
            .method::<_, CidWrapper>("read", ())
            .map_err(|e| EthClientError::Default(e.to_string()))?
            .call()
            .await
            .map_err(|e| EthClientError::Default(e.to_string()))?
            .into();
        Ok(cid)
    }

    /// Update the current cid in the contract
    /// Requires a signer
    pub async fn update(
        &self,
        previous_cid: Cid,
        cid: Cid,
    ) -> Result<Option<TransactionReceipt>, EthClientError> {
        let data = self
            .contract
            .encode("update", (CidWrapper(previous_cid), CidWrapper(cid)))
            .map_err(|e| EthClientError::Default(e.to_string()))?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data)
            .chain_id(self.chain_id);
        // TODO: is this the right return type?
        match self.signer {
            Some(ref signer) => {
                let signed_tx = signer
                    .send_transaction(tx, None)
                    .await
                    .map_err(|e| EthClientError::Default(e.to_string()))?;
                let reciept = signed_tx
                    .await
                    .map_err(|e| EthClientError::Default(e.to_string()))?;
                Ok(reciept)
            }
            None => Err(EthClientError::MissingSigner),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum EthClientError {
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
    // TODO: better error handling
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
        let array = match token {
            ethers::abi::Token::FixedArray(array) => array,
            _ => return Err(InvalidOutputType("Invalid Array".to_string())),
        };

        // Assert that the array has two FixedBytes tokens
        if array.len() != 2 {
            return Err(InvalidOutputType("Invalid Array".to_string()));
        }

        let bytes_1 = match array.get(0) {
            Some(ethers::abi::Token::FixedBytes(bytes)) => bytes,
            _ => return Err(InvalidOutputType("Invalid Bytes".to_string())),
        };
        let bytes_2 = match array.get(1) {
            Some(ethers::abi::Token::FixedBytes(bytes)) => bytes,
            _ => return Err(InvalidOutputType("Invalid Bytes".to_string())),
        };

        let mut all_bytes = bytes_1.clone();
        all_bytes.extend(bytes_2);

        let cid = Cid::try_from(all_bytes.as_slice())
            .map_err(|_| InvalidOutputType("Invalid CID".to_string()))?;
        Ok(Self(cid))
    }

    fn into_token(self) -> ethers::abi::Token {
        // Split the cid into two FixedBytes tokens of 32 bytes each
        let buff_1 = [0u8; 32];
        let buff_2 = [0u8; 32];
        let bytes = self.0.to_bytes();
        let all_bytes = bytes
            .iter()
            .chain(buff_1.iter())
            .chain(buff_2.iter())
            .take(64)
            .copied()
            .collect::<Vec<u8>>();
        let (bytes_1, bytes_2) = all_bytes
            .split_at(32);
        let token_1 = ethers::abi::Token::FixedBytes(bytes_1.to_vec());
        let token_2 = ethers::abi::Token::FixedBytes(bytes_2.to_vec());
        // Return a FixedArray token of the two FixedBytes tokens
        ethers::abi::Token::FixedArray(vec![token_1, token_2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cid_wrapper_rt() {
        let cid = Cid::default();
        let cid_wrapper = CidWrapper(cid);
        let token = cid_wrapper.into_token();
        let from_cid = CidWrapper::from_token(token).unwrap();
        assert_eq!(cid, from_cid.into());
    }
}
