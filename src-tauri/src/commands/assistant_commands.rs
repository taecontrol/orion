use uuid::Uuid;

use crate::{models::assistant::Assistant, services::assistants_service};

#[derive(serde::Deserialize)]
pub struct AssistantRequest {
    name: String,
    description: String,
}

#[tauri::command]
pub fn create_assistant(new_assistant: AssistantRequest) {
    let assistant = Assistant {
        id: Uuid::new_v4().to_string(),
        name: new_assistant.name,
        description: new_assistant.description,
        created_at: chrono::Utc::now().naive_utc(),
    };

    assistants_service::store_new_assistant(&assistant);
}

#[tauri::command]
pub fn list_assistants() -> Vec<Assistant> {
    assistants_service::list_assistants()
}

#[tauri::command]
pub fn get_assistant(id: String) -> Option<Assistant> {
    assistants_service::get_assistant(id)
}

#[tauri::command]
pub fn update_assistant(id: String, assistant: AssistantRequest) {
    assistants_service::update_assistant(id, assistant.name, assistant.description);
}
