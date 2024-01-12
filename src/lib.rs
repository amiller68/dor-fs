mod device;
mod types;

#[cfg(not(target_arch = "wasm32"))]
mod cli;

// TODO: massive cleanup and separation of wasm and native rust
#[allow(dead_code, unused_imports, unused_methods, unused_variables)]
#[cfg(target_arch = "wasm32")]
mod wasm;

pub mod prelude {
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::cli::App;

    #[cfg(target_arch = "wasm32")]
    pub use crate::wasm::App;
}
