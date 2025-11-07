use rust_embed_for_web::{EmbedableFile, RustEmbed};

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Assets;

#[test]
fn zstd_feature_behavior() {
    let file = Assets::get("index.html").unwrap();

    // Test that zstd behavior matches feature flag
    #[cfg(feature = "compression-zstd")]
    {
        // When feature is enabled, data_zstd might return Some or None
        // depending on build configuration and compression effectiveness
        let zstd_data = file.data_zstd();
        // Just verify that the method exists and doesn't panic
        match zstd_data {
            Some(_) => {
                // Zstd compression was effective
            }
            None => {
                // Zstd compression was not effective or disabled for this file
            }
        }
    }

    #[cfg(not(feature = "compression-zstd"))]
    {
        // When feature is disabled, data_zstd should always return None
        assert!(file.data_zstd().is_none());
    }
}

#[test]
fn zstd_default_trait_implementation() {
    use rust_embed_for_web::{DynamicFile, EmbedableFile};

    let file = DynamicFile::read_from_fs("examples/public/index.html").unwrap();

    // For DynamicFile, data_zstd should always return None regardless of feature
    assert!(file.data_zstd().is_none());
}
