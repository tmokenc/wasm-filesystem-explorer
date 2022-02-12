use reqwasm::Error;
use reqwasm::http::Request;
use common::{
    ReadDirRequest, ReadDirResponse,
};

pub async fn read_dir(path: &str) -> Result<ReadDirResponse, Error> {
    let request = ReadDirRequest {
        path: path.to_string(),
    };
    
    Request::post(ReadDirRequest::END_POINT)
        .body(request.json())
        .send()
        .await?
        .json()
}