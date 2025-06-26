use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexedFile {
    pub path: String,
    pub name: String,
    pub parent_path: String,
    pub file_type: String, // "file", "directory"
    pub last_modified: i64, // unix timestamp
}
