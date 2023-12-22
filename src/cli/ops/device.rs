

pub fn handle_config_subcommand(
    _config: &Config,
    subcommand: ConfigureSubcommand,
) -> Result<(), ConfigError> {
    match subcommand {
        ConfigureSubcommand::Create { subcommand } => match subcommand {
            ConfigureCreateSubcommand::Eth {
                alias,
                rpc,
                address,
                chain_id,
            } => {
                let eth_remote = EthRemote {
                    rpc,
                    address,
                    chain_id,
                };
                let _eth_client = RootCidClient::try_from(eth_remote.clone())?;
                OnDiskConfig::create_eth_remote(alias, eth_remote)?;
            }
            ConfigureCreateSubcommand::Ipfs {
                alias,
                url,
                gateway_url,
            } => {
                let ipfs_remote = IpfsRemote { url, gateway_url };
                let _ipfs_client = IpfsClient::try_from(ipfs_remote.clone())?;
                OnDiskConfig::create_ipfs_remote(alias, ipfs_remote)?;
            }
        },
        ConfigureSubcommand::Set { subcommand } => match subcommand {
            ConfigureSetSubcommand::Eth { alias } => {
                let mut on_disk_config = OnDiskConfig::load()?;
                on_disk_config.set_eth_remote_alias(alias)?;
            }
            ConfigureSetSubcommand::Ipfs { alias } => {
                let mut on_disk_config = OnDiskConfig::load()?;
                on_disk_config.set_ipfs_remote_alias(alias)?;
            }
        },
        ConfigureSubcommand::Ls => {
            let on_disk_config = OnDiskConfig::load()?;
            let eth_remotes = on_disk_config.eth_remotes()?;
            let ipfs_remote = on_disk_config.ipfs_remotes()?;
            println!("Eth remotes:");
            for (alias, eth_remote) in eth_remotes.iter() {
                println!("{}: {:?}", alias, eth_remote);
            }
            println!("Ipfs remotes:");
            for (alias, ipfs_remote) in ipfs_remote.iter() {
                println!("{}: {:?}", alias, ipfs_remote);
            }
        }
        ConfigureSubcommand::Show => {
            let on_disk_config = OnDiskConfig::load()?;
            let eth_remote = on_disk_config.eth_remote()?;
            let ipfs_remote = on_disk_config.ipfs_remote()?;
            println!("Eth remote: {:?}", eth_remote);
            println!("Ipfs remote: {:?}", ipfs_remote);
        }
    }
    Ok(())
}
