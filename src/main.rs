mod types;
mod eth;

#[cfg(not(target_arch = "wasm32"))]
mod cli;
#[cfg(not(target_arch = "wasm32"))]
mod ipfs;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    use cli::App;
    App::run().await;
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use wasm::App;
    App::run();
}
