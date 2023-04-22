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
    use crate::schema::messages;

    let connection = &mut establish_db_connection();

    diesel::insert_into(messages::table)
        .values(new_message)
        .execute(connection)
        .expect("Error saving new message");
}
