use crate::models::Message;
use crate::services::messages_service;

#[tauri::command]
pub fn list_messages(session_id: String) -> Vec<Message> {
    messages_service::list_messages(&session_id)
}
