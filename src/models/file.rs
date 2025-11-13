use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct File {
    pub id: Uuid,
    pub user_id: Uuid,
    pub filename: String,
    pub original_name: String,
    pub size: i64,
    pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UploadFileResponse {
    pub id: Uuid,
    pub filename: String,
    pub original_name: String,
    pub size: i64,
    pub uploaded_at: DateTime<Utc>,
}
