use krondor_cms::prelude::App;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {

    App::run().await;
}

#[cfg(target_arch = "wasm32")]
fn main() {
    App::run();
}
