use rust_embed_for_web::RustEmbed;

#[derive(RustEmbed)]
#[folder = "examples/public"]
#[exclude = "images/*"]
#[include = "*.jpg"]
struct Embed;

#[test]
fn unexcluded_file_exists() {
    assert!(Embed::get("index.html").is_some());
}

#[test]
fn excluded_file_is_missing() {
    assert!(Embed::get("images/doc.txt").is_none());
    assert!(Embed::get("images/llama.png").is_none());
}

#[test]
fn included_overrides_the_exclude() {
    assert!(Embed::get("images/flower.jpg").is_some());
}
