mod device;
mod types;

#[cfg(not(target_arch = "wasm32"))]
mod cli;

#[allow(dead_code)]
#[cfg(target_arch = "wasm32")]
mod wasm;

pub mod prelude {
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::cli::App;

    #[cfg(target_arch = "wasm32")]
    pub use crate::wasm::App;
}
