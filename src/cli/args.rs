use std::path::PathBuf;

use clap::{command, Subcommand};
use ethers::types::Address;

use url::Url;

pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Command passed
    #[command(subcommand)]
    pub command: Command,

    /// Working dir to run a command on
    #[clap(short, long)]
    pub dir: Option<String>,
    /// Private Secp256k1 Admin Key (should be contract deployer)
    #[clap(long)]
    pub admin_key: Option<String>,
}

// TODO: balance this

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Subcommand, Clone, PartialEq)]
pub enum Command {
    /// Manages Devices
    Device {
        #[clap(subcommand)]
        subcommand: DeviceSubcommand,
    },
    /// Check the health of the device (connection to remote resources)
    Health,
    /// Initialize a new space to pull and work on changes
    Init,
    /// Pull the remote to the local dot directory -- overwrites any changes
    Pull,
    /// Stage changes against the local ipfs instance -- may be run mutliple times in a row
    Stage,
    /// Stat changes
    Stat,
    /// Tag an object with one of our schemas. These effect the schema definitions in the dot directory
    /// Changes to schemas will be reflected in the next push
    Tag {
        #[clap(long, short)]
        name: String,
        #[clap(long, short)]
        path: PathBuf,
        #[clap(long, short)]
        value: String,
    },
    /// Squash and sync changes with the remote
    Push,
}

// TODO: add ability to manage keystores here
#[derive(Debug, Subcommand, Clone, PartialEq)]
pub enum DeviceSubcommand {
    /// Create a new device configuration
    /// Initializes an xdg path if non exists
    Create {
        #[clap(long, short)]
        alias: String,
        #[clap(long)]
        eth_rpc: Url,
        #[clap(long)]
        eth_chain_id: u16,
        #[clap(long)]
        contract_address: Address,
        #[clap(long, short)]
        ipfs_url: Url,
        #[clap(long)]
        ipfs_gateway_url: Url,
    },
    /// Update a Device
    Update {
        #[clap(long, short)]
        alias: String,
        #[clap(long)]
        eth_rpc: Option<Url>,
        #[clap(long)]
        eth_chain_id: Option<u16>,
        #[clap(long)]
        contract_address: Option<Address>,
        #[clap(long, short)]
        ipfs_url: Option<Url>,
        #[clap(long)]
        ipfs_gateway_url: Option<Url>,
    },
    /// Set a configuration value
    Set { alias: String },
    /// List all devices
    Ls,
    /// Show the current device
    Show,
}
