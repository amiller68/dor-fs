mod cli;
mod ipfs;
mod root_cid;
mod types;

pub mod prelude {
    pub use crate::ipfs::{IpfsClient, IpfsGateway, IpfsApi, IpfsError, IpfsClientError};
    pub use crate::root_cid::EthClient;

    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::cli::App;
}
