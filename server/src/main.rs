mod api;
mod config;

use actix_files::{Files, NamedFile};
use actix_web::{get, web, App, HttpServer};
use config::Config;
use common::GetFileRequest;
use std::io::Result;

#[get("/file")]
async fn get_file(query: web::Query<GetFileRequest>) -> Result<NamedFile> {
    NamedFile::open(&query.path)
}

#[get("/thumbnail")]
async fn get_thumbnail(query: web::Query<GetThumbnailRequest>) -> impl Responder {
    todo!()
}

#[rustfmt::skip]
#[actix_web::main] // or #[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load("./config.toml").unwrap();

    HttpServer::new(|| {
        App::new()
            .service(get_file)
            .service(get_thumbnail)
            .service(api::api())
            .service(Files::new("/static", "./web/static").prefer_utf8(true))
            .service(Files::new("/wasm", "./web/pkg").prefer_utf8(true))
            .route("*", web::get().to(|| async { NamedFile::open("./web/index.html") }))
    })
    .bind((config.ip, config.port))?
    .run()
    .await
}
