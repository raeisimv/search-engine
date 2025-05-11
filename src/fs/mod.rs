#[cfg(not(target_arch = "wasm32"))]
mod real_fs;
#[cfg(not(target_arch = "wasm32"))]
pub use real_fs::*;

#[cfg(target_arch = "wasm32")]
mod web_fs;
#[cfg(target_arch = "wasm32")]
pub use web_fs::*;
