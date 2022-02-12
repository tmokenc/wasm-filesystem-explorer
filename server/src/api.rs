use actix_web::{guard, web, Scope};
use std::io::Result;
use common::{ReadDirRequest, ReadDirResponse, DirEntry};
use futures::StreamExt;

async fn read_dir(
    data: web::Json<ReadDirRequest>,
) -> Result<web::Json<ReadDirResponse>> {
    let entries = async_fs::read_dir(&data.path)
        .await?
        .filter_map(|v| async move {
            let data = v.ok()?;
            let metadata = data.metadata().await.ok()?;
            
            Some(DirEntry {
                name: data.file_name().into_string().ok()?,
                is_dir: metadata.is_dir(),
                size: metadata.len(),
                read_only: metadata.permissions().readonly(),
            })
        })
        .collect::<Vec<_>>()
        .await;
        
    
    Ok(web::Json(ReadDirResponse {
        entries
    }))
}

pub fn api() -> Scope {
    Scope::new("/")
        .guard(guard::Post())
        .route(ReadDirRequest::END_POINT, web::to(read_dir))
}