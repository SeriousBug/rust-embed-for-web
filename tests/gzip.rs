use rust_embed_for_web::RustEmbed;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct DefaultGzip;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
#[gzip = "false"]
struct FalseGzip;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
#[gzip = true]
struct TrueGzip;

#[test]
fn gzip_is_used_by_default() {
  let file = DefaultGzip::get("index.html").unwrap();
  assert!(file.data_gzip.is_some());
}

#[test]
fn gzip_is_used_when_enabled() {
  let file = TrueGzip::get("index.html").unwrap();
  assert!(file.data_gzip.is_some());
}

#[test]
fn gzip_is_not_available_when_disabled() {
  let file = FalseGzip::get("index.html").unwrap();
  assert!(file.data_gzip.is_none());
}

#[test]
fn image_files_dont_get_gzipped() {
  let file = DefaultGzip::get("images/flower.jpg").unwrap();
  assert!(file.data_gzip.is_none());
}
