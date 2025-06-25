use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    pub id: i64,
    pub name: String,
    pub keyword: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
