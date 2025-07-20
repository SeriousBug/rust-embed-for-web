use std::io::{BufReader, Write};

use flate2::write::GzDecoder;
use rust_embed_for_web::{EmbedableFile, RustEmbed};

#[derive(RustEmbed)]
#[folder = "examples/public"]
struct Embed;

#[test]
fn html_files_gzip_and_br_compression() {
    assert!(Embed::get("index.html").unwrap().data_gzip().is_some());
    assert!(Embed::get("index.html").unwrap().data_br().is_some());
}

#[test]
fn zstd_behavior_without_feature() {
    // When compression-zstd feature is not enabled, data_zstd should return None
    #[cfg(not(feature = "compression-zstd"))]
    {
        assert!(Embed::get("index.html").unwrap().data_zstd().is_none());
    }

    // When compression-zstd feature is enabled, it may return Some or None based on effectiveness
    #[cfg(feature = "compression-zstd")]
    {
        // Just test that the method doesn't panic
        let _ = Embed::get("index.html").unwrap().data_zstd();
    }
}

#[test]
fn image_files_are_not_compressed() {
    assert!(Embed::get("images/flower.jpg")
        .unwrap()
        .data_gzip()
        .is_none());
    assert!(Embed::get("images/flower.jpg").unwrap().data_br().is_none());
    assert!(Embed::get("images/flower.jpg")
        .unwrap()
        .data_zstd()
        .is_none());
}

#[test]
fn compression_gzip_roundtrip() {
    let compressed = Embed::get("index.html").unwrap().data_gzip().unwrap();
    let mut decompressed: Vec<u8> = Vec::new();
    let mut decoder = GzDecoder::new(&mut decompressed);
    decoder.write_all(compressed).unwrap();
    decoder.finish().unwrap();
    let decompressed_body = String::from_utf8_lossy(&decompressed[..]);
    assert!(decompressed_body.starts_with("<!DOCTYPE html>"));
}

#[test]
fn compression_br_roundtrip() {
    let compressed = Embed::get("index.html").unwrap().data_br().unwrap();
    let mut decompressed: Vec<u8> = Vec::new();
    let mut data_read = BufReader::new(compressed);
    brotli::BrotliDecompress(&mut data_read, &mut decompressed).unwrap();
    let decompressed_body = String::from_utf8_lossy(&decompressed[..]);
    assert!(decompressed_body.starts_with("<!DOCTYPE html>"));
}
