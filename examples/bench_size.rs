//! Minimal example used to measure the embedded binary size.
//!
//! It embeds the large generated fixture folder and looks up a key taken from
//! a runtime-provided argument, so the lookup can't be const-folded away. Build
//! with:
//!
//!     cargo build --release --example bench_size --features bench-fixtures
//!
//! then strip and measure the resulting binary.

use rust_embed_for_web::{EmbedableFile, RustEmbed};

#[derive(RustEmbed)]
#[folder = "${BENCH_FIXTURE_DIR}"]
struct Assets;

fn main() {
    // Runtime arg keeps the lookup from being optimized away.
    let key = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "file_00000.txt".to_string());
    match Assets::get(&key) {
        Some(file) => println!("found {}: {} bytes", key, file.data().len()),
        None => println!("not found: {key}"),
    }
}
