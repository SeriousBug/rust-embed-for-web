use rust_embed_for_web::{EmbedableFile, RustEmbed};

#[derive(RustEmbed)]
#[folder = "examples/public"]
#[mtime = false]
struct NoMtime;

#[derive(RustEmbed)]
#[folder = "examples/public"]
struct WithMtime;

#[test]
fn mtime_is_dropped_when_disabled() {
    let index = NoMtime::get("index.html").unwrap();
    assert!(index.last_modified().is_none());
    assert!(index.last_modified_timestamp().is_none());
}

#[test]
fn mtime_is_included_by_default() {
    let index = WithMtime::get("index.html").unwrap();
    assert!(index.last_modified().is_some());
    assert!(index.last_modified_timestamp().is_some());
}
