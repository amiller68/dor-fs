#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use krondor_fs::prelude::App;

    App::run();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("krondor-fs is not supported on wasm32")
}
