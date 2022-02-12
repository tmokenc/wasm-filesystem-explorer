use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFileRequest {
    #[serde(rename = "p")]
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadDirRequest {
    #[serde(rename = "p")]
    pub path: String,
}

impl ReadDirRequest {
    pub const END_POINT: &'static str = "/api/readdir";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadDirResponse {
    #[serde(rename = "e")]
    pub entries: Vec<DirEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirEntry {
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "s")]
    pub size: u64,
    #[serde(rename = "d")]
    pub is_dir: bool,
    #[serde(rename = "ro")]
    pub read_only: bool,
}