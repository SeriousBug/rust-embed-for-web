use rust_embed_for_web::EmbedableFile;

pub fn read_embed_as_string<T: EmbedableFile>(file: T) -> String {
    let data = file.data();
    let index_contents = String::from_utf8_lossy(data.as_ref());
    index_contents.to_string()
}
