use diesel::prelude::*;
use crate::db::establish_db_connection;
use crate::models::Message;

#[tauri::command]
pub fn list_messages() -> Vec<Message> {
    use crate::schema::messages::dsl::*;

    let connection = &mut establish_db_connection();

    messages
        .load::<Message>(connection)
        .expect("Error loading messages")
}
