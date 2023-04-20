use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use tauri::Manager;
use uuid::Uuid;
use crate::db::establish_db_connection;
use crate::models::{NewSession, Session};
use crate::schema::sessions::dsl::sessions;

#[tauri::command]
pub fn list_sessions() -> Vec<Session> {
    use crate::schema::sessions::dsl::*;

    let connection = &mut establish_db_connection();

    sessions
        .order_by(created_at.desc())
        .load::<Session>(connection)
        .expect("Error loading sessions")
}

#[tauri::command]
pub fn new_session() -> Session {
    use crate::schema::sessions;

    let connection = &mut establish_db_connection();

    let new_session = NewSession {
        id: Uuid::new_v4().to_string(),
        name: String::from("New chat"),
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(sessions::table)
        .values(&new_session)
        .execute(connection)
        .expect("Error saving new session");

    new_session.into()
}

#[tauri::command]
pub fn delete_session(app_handle: tauri::AppHandle, parent_session_id: String) {
    let connection = &mut establish_db_connection();

    diesel::delete(sessions)
        .filter(crate::schema::sessions::dsl::id.eq(parent_session_id.clone()))
        .execute(connection)
        .expect("Error deleting session");

    app_handle.emit_all("session_deleted", {}).unwrap();
}
