mod api;
mod config;
mod utils;

use actix_files::{Files, NamedFile};
use actix_web::{get, web, App, HttpServer};
use config::Config;
use common::{GetFileRequest, GetThumbnailRequest};
use serde::{Serialize, Deserialize};
use bytes::Bytes;
use utils::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[get("/file")]
async fn get_file(query: web::Query<GetFileRequest>) -> std::io::Result<NamedFile> {
    NamedFile::open(&query.path)
}

#[get("/thumbnail")]
async fn get_thumbnail(
    query: web::Query<GetThumbnailRequest>,
    db: web::Data<sled::Db>,
) -> Result<Bytes> {
    #[derive(Serialize, Deserialize)]
    struct ThumbnailCache {
        modified: std::time::SystemTime,
        bytes: Bytes,
    }
    
    let path = &query.path;
    let modified = async_fs::metadata(&path).await?.modified()?;
    
    let key = bincode::serialize(&*query)?;
    let cache = db.open_tree("thumbnail")?;
    let cached = cache
        .get(&key)?
        .and_then(|bytes| bincode::deserialize::<ThumbnailCache>(&bytes).ok())
        .filter(|v| v.modified == modified);
        
    if let Some(data) = cached {
        return Ok(data.bytes)
    }
    
    let thumbnail = generate_thumbnail(&path, query.size).await?;
    let data = bincode::serialize(&ThumbnailCache {
        modified, bytes: thumbnail.clone(),
    })?;
    
    cache.insert(&key, &*data)?;
    
    Ok(thumbnail)
}

#[rustfmt::skip]
#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load_from_file("./config.toml").unwrap();

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
    .await?;

    Ok(())
}
