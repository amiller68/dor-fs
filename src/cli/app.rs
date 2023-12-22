use std::fmt::{self, Display};
use std::path::PathBuf;

pub use super::args::{Args, Command, Parser};
use super::config::{Config, ConfigError};
use super::ops::{device_subcommand, DeviceSubcommandError};

pub struct App;

impl App {
    pub async fn run() {
        tracing_subscriber::fmt::init();
        capture_error(Self::run_result().await);
    }

    async fn run_result() -> Result<(), AppError> {
        let args = Args::parse();
        let config = Config::parse_args(&args)?;
        match args.command {
            Command::Device { subcommand } => {
                device_subcommand(&config, &subcommand)?;
            }
            Command::Health => {}
            Command::Init => {}
            Command::Pull => {}
            Command::Stage => {}
            Command::Stat => {
                let change_log = config.change_log()?;
                let displayable_change_log = change_log.displayable();
                println!("{}", displayable_change_log);
            }
            Command::Push => {}
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    Config(#[from] ConfigError),
    DeviceSubcommand(#[from] DeviceSubcommandError),
    // Health(#[from] HealthError),
    // Stat(#[from] StatError),
    // Diff(#[from] DiffError),
    // Stage(#[from] StageError),
    // Push(#[from] PushError),
    // Pull(#[from] PullError),
}

fn capture_error<T>(result: Result<T, AppError>) {
    match result {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_message = format!("{:?}", self);
        let red = "\x1b[31;1m"; // Bright red
        let reset = "\x1b[0m"; // Reset to default color

        // ASCII art for visual impact (optional)
        let skull = "
        ☠️ ☠️ ☠️
        ";

        write!(
            f,
            "{}{}{}\n{}{}\n{}",
            red,
            skull,
            reset, // Skull in red
            red,
            error_message, // Error message in red
            reset          // Reset color
        )
    }
}
