use crate::{db::establish_db_connection, models::Message, open_ai::OpenAIMessage};
use diesel::prelude::*;

pub fn list_messages(parent_session_id: &String) -> Vec<OpenAIMessage> {
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

pub fn store_new_message(new_message: &Message) {
    use crate::schema::messages;

    let connection = &mut establish_db_connection();

    diesel::insert_into(messages::table)
        .values(new_message)
        .execute(connection)
        .expect("Error saving new message");
}
