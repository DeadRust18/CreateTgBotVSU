use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Student {
    pub id: Uuid,
    pub telegram_id: i64,
    pub faculty: String,
    pub group_name: String,
    pub created_at: DateTime<Utc>,
}