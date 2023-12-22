mod cli;
mod device;
mod types;

pub mod prelude {
    pub use crate::ipfs::{IpfsApi, IpfsClient, IpfsClientError, IpfsError, IpfsGateway};
    pub use crate::root_cid::EthClient;

    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::cli::App;
}
