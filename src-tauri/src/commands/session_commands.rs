use crate::models::session::{NewSession, Session};
use crate::services::messages_service;
use crate::services::sessions_service;
use tauri::Manager;
use uuid::Uuid;

#[tauri::command]
pub fn list_sessions() -> Vec<Session> {
    sessions_service::list_sessions()
}

#[tauri::command]
pub fn new_session() -> Session {
    let new_session = NewSession {
        id: Uuid::new_v4().to_string(),
        name: String::from("New chat"),
        created_at: chrono::Utc::now().naive_utc(),
    };

    sessions_service::store_session(&new_session);

    new_session.into()
}

#[tauri::command]
pub fn delete_session(app_handle: tauri::AppHandle, parent_session_id: String) {
    messages_service::delete_session_messages(parent_session_id.clone());
    sessions_service::delete_session(parent_session_id.clone());

    app_handle.emit_all("session_deleted", {}).unwrap();
}
