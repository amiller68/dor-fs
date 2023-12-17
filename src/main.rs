#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    use krondor_fs::prelude::App;

    App::run().await;
}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("dor-fs is not supported on wasm32")
}
