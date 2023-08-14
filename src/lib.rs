#![warn(clippy::all, rust_2018_idioms)]

mod app;

#[cfg(target_arch = "wasm32")]
mod frontend;

pub use app::App;
