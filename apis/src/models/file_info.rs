use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub size: usize,
    pub mime_type: String,
}
