use serde::{Serialize, Deserialize};

pub const VIDEO_EXT: &[&str] = &["mp4", "mkv"];
pub const IMG_EXT: &[&str] = &["jpg", "png", "gif"];

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum FileType {
    Video,
    Image,
    Unknown,
}

impl FileType {
    pub fn from_name(name: &str) -> Self {
        if VIDEO_EXT
            .iter()
            .map(|ext| format!(".{ext}"))
            .any(|ext| name.ends_with(&ext))
        {
            return Self::Video;
        }

        if IMG_EXT
            .iter()
            .map(|ext| format!(".{ext}"))
            .any(|ext| name.ends_with(&ext))
        {
            return Self::Image;
        }
        
        Self::Unknown
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFileRequest {
    #[serde(rename = "p")]
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum ThumbnailSize {
    Small,
    Normal,
    Large,
}

impl ThumbnailSize {
    pub const fn dimensions(&self) -> (u32, u32) {
        match self {
            Self::Small => (128, 128),
            Self::Normal => (256, 256),
            Self::Large => (512, 512),
        }
    }
}

impl Default for ThumbnailSize {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetThumbnailRequest {
    #[serde(rename = "p")]
    pub path: String,
    #[serde(rename = "s")]
    #[serde(default)]
    pub size: ThumbnailSize,
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