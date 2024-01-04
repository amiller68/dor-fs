use crate::cli::args::DeviceSubcommand;
use crate::cli::config::{Config, ConfigError};
use crate::device::{EthRemote, IpfsRemote};

pub fn device_subcommand(
    config: &Config,
    subcommand: &DeviceSubcommand,
) -> Result<(), DeviceSubcommandError> {
    match subcommand {
        DeviceSubcommand::Create {
            alias,
            eth_rpc,
            eth_chain_id,
            contract_address,
            ipfs_url,
            ipfs_gateway_url,
        } => {
            let eth_remote = EthRemote {
                rpc_url: eth_rpc.clone(),
                chain_id: *eth_chain_id,
            };
            let ipfs_remote = IpfsRemote {
                api_url: ipfs_url.clone(),
                gateway_url: ipfs_gateway_url.clone(),
            };
            Config::create_on_disk_device(
                alias.clone(),
                *contract_address,
                ipfs_remote,
                eth_remote,
            )?;
        }
        DeviceSubcommand::Update {
            alias: _,
            eth_rpc: _,
            eth_chain_id: _,
            contract_address: _,
            ipfs_url: _,
            ipfs_gateway_url: _,
        } => {
            todo!("updating devices is not supported yet")
        }
        DeviceSubcommand::Set { alias } => {
            Config::set_device(alias.clone())?;
        }
        DeviceSubcommand::Ls => {
            println!("{:?}", Config::list_on_disk_devices()?);
        }
        DeviceSubcommand::Show => {
            println!("{}", config.on_disk_device()?);
        }
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum DeviceSubcommandError {
    #[error("config error: {0}")]
    Config(#[from] ConfigError),
}
