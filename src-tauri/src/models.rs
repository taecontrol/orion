use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Serialize;
use crate::schema::sessions;

#[derive(Queryable, Serialize)]
pub struct Session {
    pub id: String,
    pub name: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

impl From<NewSession> for Session {
    fn from(new_session: NewSession) -> Self {
        Session {
            id: new_session.id,
            name: Some(new_session.name),
            created_at: new_session.created_at,
        }
    }
}
