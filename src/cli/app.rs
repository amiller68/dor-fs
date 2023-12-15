use std::fmt::{self, Display};
use std::path::PathBuf;

pub use super::args::{Args, Command, Parser};
use super::config::{Config, ConfigError};
use super::ops::{diff, health, stat, DiffError, HealthError, StatError};

pub struct App;

impl App {
    pub fn run() {
        capture_error(Self::run_result());
    }

    pub fn run_result() -> Result<(), AppError> {
        let args = Args::parse();
        let config = Config::parse(&args)?;
        match args.command {
            Command::Init => {
                // i don't do anything lol
            }
            Command::Health => {
                health(&config)?;
            }
            Command::Deploy => {
                // deploy(&config)?;
            }
            Command::Clone { dir: _ } => {
                // clone(&config, dir)?;
            }
            Command::Diff { dir } => {
                let working_dir = working_dir(dir)?;
                diff(&config, working_dir)?;
            }
            Command::Stat { dir } => {
                let working_dir = working_dir(dir)?;
                let diff = stat(&config, working_dir)?;
                println!("{}", diff);
            }
            Command::Stage { dir: _ } => {
                // push(&config, dir)?;
            }
            Command::Push { dir: _ } => {
                // push(&config, dir)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // #[error("config error")]
    Config(#[from] ConfigError),
    // #[error("fs-tree error: {0}")]
    FsTree(#[from] fs_tree::Error),
    // #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    // #[error("health error: {0}")]
    Health(#[from] HealthError),
    // #[error("stat error: {0}")]
    Stat(#[from] StatError),
    // #[error("diff error: {0}")]
    Diff(#[from] DiffError),
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
