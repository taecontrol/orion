use crate::schema::messages;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Serialize;

#[derive(Queryable, Serialize, Insertable)]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: String,
    pub session_id: String,
    pub content: String,
    pub role: String,
    pub finish_reason: String,
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub created_at: NaiveDateTime,
}
