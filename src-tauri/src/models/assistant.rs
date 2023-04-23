use crate::schema::assistants;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Serialize;

#[derive(Queryable, Serialize, Insertable)]
#[diesel(table_name = assistants)]
pub struct Assistant {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
}
