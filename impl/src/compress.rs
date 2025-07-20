use std::io::{BufReader, Write};

use brotli::enc::BrotliEncoderParams;
use flate2::{write::GzEncoder, Compression};
#[cfg(feature = "compression-zstd")]
use zstd::stream::write::Encoder as ZstdEncoder;

/// Only include the compressed version if it is at least this much smaller than
/// the uncompressed version.
const COMPRESSION_INCLUDE_THRESHOLD: f64 = 0.95;

pub(crate) fn compress_gzip(data: &[u8]) -> Option<Vec<u8>> {
    let mut data_gzip: Vec<u8> = Vec::new();
    let mut encoder = GzEncoder::new(&mut data_gzip, Compression::default());
    encoder
        .write_all(data)
        .expect("Failed to compress gzip data");
    encoder
        .finish()
        .expect("Failed to finish compression of gzip data");

    if data_gzip.len() < ((data.len() as f64) * COMPRESSION_INCLUDE_THRESHOLD) as usize {
        Some(data_gzip)
    } else {
        None
    }
}

pub(crate) fn compress_br(data: &[u8]) -> Option<Vec<u8>> {
    let mut data_read = BufReader::new(data);
    let mut data_br: Vec<u8> = Vec::new();
    brotli::BrotliCompress(
        &mut data_read,
        &mut data_br,
        &BrotliEncoderParams::default(),
    )
    .expect("Failed to compress br data");
    if data_br.len() < ((data.len() as f64) * COMPRESSION_INCLUDE_THRESHOLD) as usize {
        Some(data_br)
    } else {
        None
    }
}

#[cfg(feature = "compression-zstd")]
pub(crate) fn compress_zstd(data: &[u8]) -> Option<Vec<u8>> {
    let mut data_zstd: Vec<u8> = Vec::new();
    let mut encoder = ZstdEncoder::new(&mut data_zstd, 3).expect("Failed to create zstd encoder");
    encoder
        .write_all(data)
        .expect("Failed to compress zstd data");
    encoder
        .finish()
        .expect("Failed to finish compression of zstd data");

    if data_zstd.len() < ((data.len() as f64) * COMPRESSION_INCLUDE_THRESHOLD) as usize {
        Some(data_zstd)
    } else {
        None
    }
}

#[cfg(not(feature = "compression-zstd"))]
pub(crate) fn compress_zstd(_data: &[u8]) -> Option<Vec<u8>> {
    None
}
