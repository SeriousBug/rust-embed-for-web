use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rust_embed_for_web::*;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => {
            print_sizes(&content);
            let mut resp = HttpResponse::Ok();
            resp.append_header(("ETag", content.etag()));
            if let Some(last_modified) = content.last_modified() {
                resp.append_header(("Last-Modified", last_modified));
            }
            if let Some(body) = content.data_br() {
                // This part will only work on release builds, try running with:
                //
                //     cargo run --example actix --release
                //
                // This example might fail if the browser doesn't support br
                // compression, a real server should respect the Accept-Encoding
                // header. Check
                // `https://github.com/SeriousBug/actix-web-rust-embed-responder`
                // for a real implementation.
                resp.append_header(("Content-Encoding", "br"));
                resp.body(body)
            } else {
                resp.body(content.data())
            }
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    handle_embedded_file("index.html")
}

#[actix_web::get("/dist/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(&path)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Launching server at http://127.0.0.1:8000");
    HttpServer::new(|| App::new().service(index).service(dist))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

fn print_sizes<F: EmbedableFile>(file: &F) {
    println!(
        "{}: {} bytes, {} compressed with BR, {} compressed with GZIP",
        file.name().as_ref(),
        file.data().as_ref().len(),
        file.data_br()
            .map(|v| format!("{} bytes", v.as_ref().len()))
            .unwrap_or("not".to_string()),
        file.data_gzip()
            .map(|v| format!("{} bytes", v.as_ref().len()))
            .unwrap_or("not".to_string()),
    );
}
