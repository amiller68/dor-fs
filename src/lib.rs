mod cli;
mod device;
mod types;

pub mod prelude {
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::cli::App;
}
