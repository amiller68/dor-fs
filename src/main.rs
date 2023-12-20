#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    use dor_store::prelude::App;

    App::run().await;
}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("dor-store is not supported on wasm32")
}
