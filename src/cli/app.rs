use std::fmt::{self, Display};
use std::path::PathBuf;

pub use super::args::{Args, Command, Parser};
use super::config::{handle_config_subcommand, Config, ConfigError};
use super::ops::{
    diff, health, pull, push, stage, stat, DiffError, HealthError, PullError, PushError,
    StageError, StatError,
};

pub struct App;

impl App {
    pub async fn run() {
        capture_error(Self::run_result().await);
    }

    async fn run_result() -> Result<(), AppError> {
        let args = Args::parse();
        let config = Config::parse(&args)?;
        match args.command {
            Command::Init => {
                // i don't do anything lol
            }
            Command::Configure { subcommand } => {
                handle_config_subcommand(&config, subcommand)?;
            }
            Command::Health { dir } => {
                let working_dir = working_dir(dir)?;
                health(&config, working_dir)?;
            }
            Command::Wipe { dir: _ } => {
                // wipe(&config, dir)?;
            }
            Command::Clone { dir: _ } => {
                // clone(&config, dir)?;
            }
            // TODO: this is more like 'add'
            Command::Diff { dir } => {
                let working_dir = working_dir(dir)?;
                diff(&config, working_dir).await?;
            }
            Command::Stat { dir } => {
                let working_dir = working_dir(dir)?;
                let diff = stat(&config, working_dir)?;
                print!("{}", diff);
            }
            // TODO: this is more like 'commit'
            Command::Stage { dir } => {
                let working_dir = working_dir(dir)?;
                stage(&config, working_dir).await?;
            }
            Command::Push { dir } => {
                let working_dir = working_dir(dir)?;
                push(&config, working_dir).await?;
            }
            Command::Pull { dir } => {
                let working_dir = working_dir(dir)?;
                pull(&config, working_dir).await?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    Config(#[from] ConfigError),
    FsTree(#[from] fs_tree::Error),
    Io(#[from] std::io::Error),
    Health(#[from] HealthError),
    Stat(#[from] StatError),
    Diff(#[from] DiffError),
    Stage(#[from] StageError),
    Push(#[from] PushError),
    Pull(#[from] PullError),
}

fn capture_error<T>(result: Result<T, AppError>) {
    match result {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

fn working_dir(dir: Option<String>) -> Result<PathBuf, AppError> {
    match dir {
        Some(dir) => Ok(PathBuf::from(dir)),
        None => std::env::current_dir().map_err(AppError::Io),
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
