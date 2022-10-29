mod common;

use common::read_embed_as_string;
use rust_embed_for_web::RustEmbed;

#[derive(RustEmbed)]
#[folder = "examples/public"]
struct Embed;

#[test]
fn index_starts_with() {
    let file = Embed::get("index.html").unwrap();
    assert!(read_embed_as_string(file).starts_with("<!DOCTYPE html>"));
}

#[test]
fn existing_file_in_folder_is_there() {
    let file = Embed::get("images/doc.txt").unwrap();
    assert_eq!(read_embed_as_string(file), "Testing 1 2 3");
}
