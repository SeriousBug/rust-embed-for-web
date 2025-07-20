use rust_embed_for_web::{EmbedableFile, RustEmbed};

// This test is designed to run in debug mode without always-embed feature
// to test the DynamicFile code paths that always return None for compressed data
#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct DynamicAssets;

#[test]
fn dynamic_file_compressed_data_is_none() {
    // In debug mode without always-embed, this should use DynamicFile
    let file = DynamicAssets::get("index.html").unwrap();
    
    // DynamicFile always returns None for compressed data
    assert!(file.data_gzip().is_none());
    assert!(file.data_br().is_none());
    assert!(file.data_zstd().is_none());
    
    // But it should still have the original data
    assert!(!file.data().is_empty());
}

#[test]
fn dynamic_file_image_compressed_data_is_none() {
    // Test with an image file too
    let file = DynamicAssets::get("images/flower.jpg").unwrap();
    
    // DynamicFile always returns None for compressed data
    assert!(file.data_gzip().is_none());
    assert!(file.data_br().is_none());
    assert!(file.data_zstd().is_none());
    
    // But it should still have the original data
    assert!(!file.data().is_empty());
}