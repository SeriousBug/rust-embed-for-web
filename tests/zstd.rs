use rust_embed_for_web::{EmbedableFile, RustEmbed};

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct DefaultZstd;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
#[zstd = false]
struct FalseZstd;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
#[zstd = true]
struct TrueZstd;

#[test]
fn zstd_is_used_by_default() {
    let file = DefaultZstd::get("index.html").unwrap();
    assert!(file.data_zstd().is_some());
}

#[test]
fn zstd_is_used_when_enabled() {
    let file = TrueZstd::get("index.html").unwrap();
    assert!(file.data_zstd().is_some());
}

#[test]
fn zstd_is_not_available_when_disabled() {
    let file = FalseZstd::get("index.html").unwrap();
    assert!(file.data_zstd().is_none());
}

#[test]
fn image_files_dont_get_zstd_compressed() {
    let file = DefaultZstd::get("images/flower.jpg").unwrap();
    assert!(file.data_zstd().is_none());
}
