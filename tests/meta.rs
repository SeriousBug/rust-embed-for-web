use rust_embed_for_web::{EmbedableFile, RustEmbed};

#[derive(RustEmbed)]
#[folder = "examples/public"]
struct Embed;

#[test]
fn file_metadata_is_present() {
    let index = Embed::get("index.html").unwrap();
    assert_eq!(index.mime_type().unwrap(), "text/html");
    assert_eq!(index.hash(), "dw}&lqvibq6Pamh$9AE0mnu|Jvn!Sm9fC^IC^7gk");
    assert_eq!(index.etag(), format!("\"{}\"", index.hash()));
}
