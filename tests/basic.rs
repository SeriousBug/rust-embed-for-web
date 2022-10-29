use rust_embed_for_web::RustEmbed;

#[derive(RustEmbed)]
#[folder = "examples/public"]
struct Embed;

#[test]
fn existing_file_at_root_is_there() {
    assert!(Embed::get("index.html").is_some());
}

#[test]
fn existing_file_in_folder_is_there() {
    assert!(Embed::get("images/doc.txt").is_some());
}

#[test]
fn missing_file_is_none() {
    assert!(Embed::get("does-not-exist").is_none());
}

fn get_file_with_trait<T: RustEmbed>(path: &str) -> Option<T::File> {
    T::get(path)
}

#[test]
fn using_trait_also_works() {
    assert!(get_file_with_trait::<Embed>("index.html").is_some());
    assert!(get_file_with_trait::<Embed>("does-not-exist").is_none());
}
