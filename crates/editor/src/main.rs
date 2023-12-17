#![warn(clippy::all, rust_2018_idioms)]

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> eframe::Result<()> {
    editor_core::launch::launch().await
}

#[cfg(target_arch = "wasm32")]
fn main() {
    editor_core::launch::launch();
}
