use crate::models::Message;
use crate::open_ai::{OpenAI, OpenAIMessage, OpenAIRequest, OpenAIResponse};
use crate::services::{messages_service, sessions_service};
use tauri::Manager;
use uuid::Uuid;

#[tauri::command]
pub async fn ask(
    app_handle: tauri::AppHandle,
    session_id: String,
    message: String,
) -> Vec<Message> {
    let previous_messages = get_context(session_id.clone(), message.clone());

    rename_session_if_new(
        &previous_messages,
        session_id.clone(),
        message.clone(),
        app_handle,
    );

    let response = make_open_ai_request(previous_messages).await;

    match response {
        Ok(response) => {
            let user_message = create_user_message(session_id.clone(), message.clone());
            let assistant_message = create_assistant_message(session_id.clone(), response);

            vec![user_message, assistant_message]
        }
        Err(_error) => {
            vec![]
        }
    }
}
async fn make_open_ai_request(
    messages: Vec<OpenAIMessage>,
) -> Result<OpenAIResponse, reqwest::Error> {
    let data = OpenAIRequest {
        model: String::from("gpt-3.5-turbo"),
        messages,
        temperature: 0.7,
    };

    OpenAI::new().chat().create(&data).await
}

fn rename_session_if_new(
    previous_messages: &Vec<OpenAIMessage>,
    session_id: String,
    message: String,
    app_handle: tauri::AppHandle,
) {
    if previous_messages.len() == 0 {
        sessions_service::update_session_name(session_id, message);
        app_handle.emit_all("session_updated", {}).unwrap();
    }
}

fn get_context(session_id: String, message: String) -> Vec<OpenAIMessage> {
    let mut previous_messages = messages_service::list_messages(&session_id);

    previous_messages.push(OpenAIMessage {
        role: String::from("user"),
        content: message,
    });

    previous_messages
}

fn create_user_message(session_id: String, message: String) -> Message {
    let user_message = Message {
        id: Uuid::new_v4().to_string(),
        session_id,
        content: message.clone(),
        role: String::from("user"),
        finish_reason: String::from(""),
        prompt_tokens: 0,
        completion_tokens: 0,
        created_at: chrono::Utc::now().naive_utc(),
    };

    messages_service::store_new_message(&user_message);

    user_message
}

fn create_assistant_message(session_id: String, open_ai_response: OpenAIResponse) -> Message {
    let assistant_message = Message {
        id: Uuid::new_v4().to_string(),
        session_id,
        content: open_ai_response.choices[0].message.content.clone(),
        role: open_ai_response.choices[0].message.role.clone(),
        finish_reason: open_ai_response.choices[0].finish_reason.clone(),
        prompt_tokens: open_ai_response.usage.prompt_tokens,
        completion_tokens: open_ai_response.usage.completion_tokens,
        created_at: chrono::Utc::now().naive_utc(),
    };

    messages_service::store_new_message(&assistant_message);

    assistant_message
}
