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

    // When always-embed is not enabled, DynamicFile always returns None for compressed data
    #[cfg(not(feature = "always-embed"))]
    {
        assert!(file.data_gzip().is_none());
        assert!(file.data_br().is_none());
        assert!(file.data_zstd().is_none());
    }

    // When always-embed is enabled, EmbeddedFile may have compressed data
    #[cfg(feature = "always-embed")]
    {
        // Just verify the file exists and has data - compression depends on the build
        assert!(!file.data().is_empty());
    }

    // But it should always have the original data
    assert!(!file.data().is_empty());
}

#[test]
fn dynamic_file_image_compressed_data_is_none() {
    // Test with an image file too
    let file = DynamicAssets::get("images/flower.jpg").unwrap();

    // When always-embed is not enabled, DynamicFile always returns None for compressed data
    #[cfg(not(feature = "always-embed"))]
    {
        assert!(file.data_gzip().is_none());
        assert!(file.data_br().is_none());
        assert!(file.data_zstd().is_none());
    }

    // When always-embed is enabled, EmbeddedFile may have compressed data (usually None for images)
    #[cfg(feature = "always-embed")]
    {
        // Just verify the file exists and has data
        assert!(!file.data().is_empty());
    }

    // But it should always have the original data
    assert!(!file.data().is_empty());
}

#[test]
fn explicit_dynamic_compression_coverage() {
    // Explicitly test to ensure coverage of DynamicFile compression methods
    let file = DynamicAssets::get("index.html").unwrap();

    // When always-embed is not enabled, test the DynamicFile paths
    #[cfg(not(feature = "always-embed"))]
    {
        // Test each compression method explicitly to ensure coverage
        let gzip_result = file.data_gzip();
        assert_eq!(gzip_result, None);

        let br_result = file.data_br();
        assert_eq!(br_result, None);

        let zstd_result = file.data_zstd();
        assert_eq!(zstd_result, None);
    }

    // When always-embed is enabled, test the EmbeddedFile paths
    #[cfg(feature = "always-embed")]
    {
        // Just verify the methods work - the actual compressed data depends on build configuration
        let _gzip_result = file.data_gzip();
        let _br_result = file.data_br();
        let _zstd_result = file.data_zstd();
    }

    // Ensure we have actual data though
    let actual_data = file.data();
    assert!(!actual_data.is_empty());
}
