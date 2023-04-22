use crate::schema::messages;
use crate::schema::messages::dsl;
use crate::{db::establish_db_connection, models::Message};
use diesel::prelude::*;

pub fn list_messages(session_id: &String) -> Vec<Message> {
    let connection = &mut establish_db_connection();

    dsl::messages
        .filter(dsl::session_id.eq(session_id))
        .load::<Message>(connection)
        .expect("Error loading messages")
}

pub fn store_new_message(new_message: &Message) {
    let connection = &mut establish_db_connection();

    diesel::insert_into(messages::table)
        .values(new_message)
        .execute(connection)
        .expect("Error saving new message");
}

pub fn delete_session_messages(session_id: String) {
    let connection = &mut establish_db_connection();

    diesel::delete(dsl::messages)
        .filter(crate::schema::messages::dsl::session_id.eq(session_id))
        .execute(connection)
        .expect("Error deleting messages");
}
