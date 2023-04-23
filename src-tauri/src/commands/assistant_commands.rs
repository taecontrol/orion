use uuid::Uuid;

use crate::{models::assistant::Assistant, services::assistants_service};

#[derive(serde::Deserialize)]
pub struct NewAssistantRequest {
    name: String,
    description: String,
}

#[tauri::command]
pub fn create_assistant(new_assistant: NewAssistantRequest) {
    let assistant = Assistant {
        id: Uuid::new_v4().to_string(),
        name: new_assistant.name,
        description: new_assistant.description,
        created_at: chrono::Utc::now().naive_utc(),
    };

    assistants_service::store_new_assistant(&assistant);
}
