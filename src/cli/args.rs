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
    /// Deploy a new contract / instance of the krondor contract
    Deploy,
    /// Healthcheck systems and configuration
    Health,
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
