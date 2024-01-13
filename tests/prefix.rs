use rust_embed_for_web::{RustEmbed, EmbedableFile};

#[derive(RustEmbed)]
#[folder = "examples/public"]
#[prefix = "foo/bar/"]
struct Embed;

#[test]
fn prefix_works() {
    assert!(Embed::get("index.html").is_none());
    assert!(Embed::get("foo/bar/index.html").is_some());
}
