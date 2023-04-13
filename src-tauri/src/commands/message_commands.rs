use diesel::prelude::*;
use crate::db::establish_db_connection;
use crate::models::Message;
use crate::schema::messages::session_id;

#[tauri::command]
pub fn list_messages(parent_session_id: String) -> Vec<Message> {
    use crate::schema::messages::dsl::*;

    let connection = &mut establish_db_connection();

    messages
        .filter(session_id.eq(parent_session_id))
        .load::<Message>(connection)
        .expect("Error loading messages")
}
