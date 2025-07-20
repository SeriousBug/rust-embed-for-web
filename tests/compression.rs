use std::io::{BufReader, Write};

use flate2::write::GzDecoder;
use rust_embed_for_web::{EmbedableFile, RustEmbed};

#[derive(RustEmbed)]
#[folder = "examples/public"]
struct Embed;

#[test]
fn html_files_are_compressed() {
    assert!(Embed::get("index.html").unwrap().data_gzip().is_some());
    assert!(Embed::get("index.html").unwrap().data_br().is_some());
    assert!(Embed::get("index.html").unwrap().data_zstd().is_some());
}

#[test]
fn image_files_are_not_compressed() {
    assert!(Embed::get("images/flower.jpg")
        .unwrap()
        .data_gzip()
        .is_none());
    assert!(Embed::get("images/flower.jpg").unwrap().data_br().is_none());
    assert!(Embed::get("images/flower.jpg").unwrap().data_zstd().is_none());
}

#[test]
fn compression_gzip_roundtrip() {
    let compressed = Embed::get("index.html").unwrap().data_gzip().unwrap();
    let mut decompressed: Vec<u8> = Vec::new();
    let mut decoder = GzDecoder::new(&mut decompressed);
    decoder.write_all(&compressed[..]).unwrap();
    decoder.finish().unwrap();
    let decompressed_body = String::from_utf8_lossy(&decompressed[..]);
    assert!(decompressed_body.starts_with("<!DOCTYPE html>"));
}

#[test]
fn compression_br_roundtrip() {
    let compressed = Embed::get("index.html").unwrap().data_br().unwrap();
    let mut decompressed: Vec<u8> = Vec::new();
    let mut data_read = BufReader::new(&compressed[..]);
    brotli::BrotliDecompress(&mut data_read, &mut decompressed).unwrap();
    let decompressed_body = String::from_utf8_lossy(&decompressed[..]);
    assert!(decompressed_body.starts_with("<!DOCTYPE html>"));
}

#[test]
fn compression_zstd_roundtrip() {
    let compressed = Embed::get("index.html").unwrap().data_zstd().unwrap();
    let decompressed = zstd::bulk::decompress(&compressed, 1024 * 1024).unwrap();
    let decompressed_body = String::from_utf8_lossy(&decompressed[..]);
    assert!(decompressed_body.starts_with("<!DOCTYPE html>"));
}
