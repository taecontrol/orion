use diesel::prelude::*;
use diesel::query_builder::IntoUpdateTarget;
use uuid::Uuid;
use crate::db::establish_db_connection;
use crate::open_ai::{OpenAI, OpenAIMessage, OpenAIRequest};
use crate::models::{Message};
use crate::schema::sessions::dsl::sessions;
use crate::schema::sessions::name;

#[tauri::command]
pub async fn ask(parent_session_id: String, message: String) -> Vec<Message> {
    let mut previous_messages = get_messages(&parent_session_id);

    if previous_messages.len() == 0 {
        let connection = &mut establish_db_connection();

        diesel::update(sessions)
            .filter(crate::schema::sessions::dsl::id.eq(parent_session_id.clone()))
            .set(name.eq(message.clone()))
            .execute(connection)
            .expect("Error updating session name");
    }

    previous_messages.push(OpenAIMessage {
        role: String::from("user"),
        content: message.clone(),
    });

    let data = OpenAIRequest {
        model: String::from("gpt-3.5-turbo"),
        messages: previous_messages,
        temperature: 0.7,
    };

    let response = OpenAI::new().chat().create(&data).await;

    match response {
        Ok(response) => {
            let user_message = Message {
                id: Uuid::new_v4().to_string(),
                session_id: parent_session_id.clone(),
                content: message.clone(),
                role: String::from("user"),
                finish_reason: String::from(""),
                prompt_tokens: 0,
                completion_tokens: 0,
                created_at: chrono::Utc::now().naive_utc(),
            };

            store_new_message(&parent_session_id, &user_message);


            let assistant_message = Message {
                id: Uuid::new_v4().to_string(),
                session_id: parent_session_id.clone(),
                content: response.choices[0].message.content.clone(),
                role: response.choices[0].message.role.clone(),
                finish_reason: response.choices[0].finish_reason.clone(),
                prompt_tokens: response.usage.prompt_tokens,
                completion_tokens: response.usage.completion_tokens,
                created_at: chrono::Utc::now().naive_utc(),
            };

            store_new_message(&parent_session_id, &assistant_message);

            vec![user_message, assistant_message]
        },
        Err(error) => {
            vec![]
        }
    }
}

fn get_messages(parent_session_id: &String) -> Vec<OpenAIMessage> {
    use crate::schema::messages::dsl::*;

    let connection = &mut establish_db_connection();

    messages
        .filter(session_id.eq(parent_session_id))
        .load::<Message>(connection)
        .expect("Error loading messages")
        .into_iter()
        .map(|message| OpenAIMessage {
            role: message.role,
            content: message.content,
        })
        .collect()
}

fn store_new_message(parent_session_id: &String, new_message: &Message) {
    use crate::schema::messages;

    let connection = &mut establish_db_connection();

    diesel::insert_into(messages::table)
        .values(new_message)
        .execute(connection)
        .expect("Error saving new message");
}
