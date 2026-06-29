use rust_embed_for_web::RustEmbed;

#[derive(RustEmbed)]
#[folder = "examples/this-folder-does-not-exist"]
#[allow_missing = true]
struct Embed;

#[test]
fn missing_folder_get_is_none() {
    assert!(Embed::get("index.html").is_none());
    assert!(Embed::get("does-not-exist").is_none());
}

fn get_file_with_trait<T: RustEmbed>(path: &str) -> Option<T::File> {
    T::get(path)
}

#[test]
fn missing_folder_get_via_trait_is_none() {
    assert!(get_file_with_trait::<Embed>("index.html").is_none());
}
