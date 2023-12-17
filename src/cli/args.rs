use std::path::PathBuf;

use clap::{command, Subcommand};

pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Command passed
    #[command(subcommand)]
    pub command: Command,

    #[clap(long)]
    pub contract: Option<String>,
    #[clap(long)]
    pub device_keystore: Option<PathBuf>,
    #[clap(long)]
    pub local_ipfs_scheme: Option<String>,
    #[clap(long)]
    pub local_ipfs_host: Option<String>,
    #[clap(long)]
    pub local_ipfs_port: Option<String>,
    #[clap(long)]
    pub admin_key: Option<String>,
}

#[derive(Debug, Subcommand, Clone, PartialEq)]
pub enum Command {
    /// Initialize a new device
    Init,
    /// Configure dor-rs
    Configure {
        #[clap(subcommand)]
        subcommand: ConfigureSubcommand,
    },
    /// Healthcheck systems and configuration
    Health {
        #[clap(short, long)]
        dir: Option<String>,
    },
    /// Wipe changes from local dot directory
    Wipe {
        #[clap(short, long)]
        dir: Option<String>,
    },
    /// Clone a copy of the remote to a directory.
    Clone {
        #[clap(short, long)]
        dir: Option<String>,
    },
    /// Construct a diff between the last pull and the current state
    /// Stores the diff in the local dot directory
    Diff {
        #[clap(short, long)]
        dir: Option<String>,
    },
    /// List changes in the local dot directory
    Stat {
        #[clap(short, long)]
        dir: Option<String>,
    },
    Stage {
        #[clap(short, long)]
        dir: Option<String>,
    },
    /// Push the local dot directory to the remote
    Push {
        #[clap(short, long)]
        dir: Option<String>,
    },
}

#[derive(Debug, Subcommand, Clone, PartialEq)]
pub enum ConfigureSubcommand {
    /// Create a new configuration
    Create {
        #[clap(subcommand)]
        subcommand: ConfigureCreateSubcommand,
    },
    /// Set a configuration value
    Set {
        #[clap(subcommand)]
        subcommand: ConfigureSetSubcommand,
    },
}

#[derive(Debug, Subcommand, Clone, PartialEq)]
pub enum ConfigureCreateSubcommand {
    /// Create a new Remote Eth configuration
    /// This will configure an rpc connection, contract address, and chain id
    Eth {
        #[clap(long, short)]
        alias: String,
        #[clap(long)]
        rpc: String,
        #[clap(long)]
        address: String,
        #[clap(long)]
        chain_id: u16,
    },
    /// Create a new Remote IPFS configuration
    Ipfs {
        #[clap(long, short)]
        alias: String,
        #[clap(long, short)]
        url: String,
        #[clap(long)]
        gateway_url: Option<String>,
    },
}

#[derive(Debug, Subcommand, Clone, PartialEq)]
pub enum ConfigureSetSubcommand {
    /// Set the default remote Eth configuration
    Eth {
        #[clap(long, short)]
        alias: String,
    },
    /// Set the default remote IPFS configuration
    Ipfs {
        #[clap(long, short)]
        alias: String,
    },
}
