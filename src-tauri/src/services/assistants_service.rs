use crate::{db::establish_db_connection, models::assistant::Assistant, schema::assistants::dsl};
use diesel::prelude::*;

pub fn init() {
    let assistants = list_assistants();

    if assistants.len() > 0 {
        return;
    }

    let default_assistant = Assistant {
        id: String::from("chatgpt"),
        name: "ChatGPT".to_string(),
        description: "You are chatGPT".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    store_new_assistant(&default_assistant);
}

pub fn list_assistants() -> Vec<Assistant> {
    let connection = &mut establish_db_connection();

    dsl::assistants
        .order_by(dsl::created_at.desc())
        .load::<Assistant>(connection)
        .expect("Error loading assistants")
}

pub fn get_assistant(id: String) -> Option<Assistant> {
    let connection = &mut establish_db_connection();

    dsl::assistants
        .filter(dsl::id.eq(id))
        .first::<Assistant>(connection)
        .ok()
}

pub fn store_new_assistant(assistant: &Assistant) {
    let connection = &mut establish_db_connection();

    diesel::insert_into(dsl::assistants)
        .values(assistant)
        .execute(connection)
        .expect("Error saving new assistant");
}

pub fn update_assistant(id: String, name: String, description: String) {
    let connection = &mut establish_db_connection();

    diesel::update(dsl::assistants.filter(dsl::id.eq(id)))
        .set((dsl::name.eq(name), dsl::description.eq(description)))
        .execute(connection)
        .expect("Error updating assistant");
}
