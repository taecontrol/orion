use crate::{db::establish_db_connection, models::assistant::Assistant, schema::assistants::dsl};
use diesel::prelude::*;

pub fn list_assistants() -> Vec<Assistant> {
    let connection = &mut establish_db_connection();

    dsl::assistants
        .order_by(dsl::created_at.desc())
        .load::<Assistant>(connection)
        .expect("Error loading assistants")
}

pub fn store_new_assistant(assistant: &Assistant) {
    let connection = &mut establish_db_connection();

    diesel::insert_into(dsl::assistants)
        .values(assistant)
        .execute(connection)
        .expect("Error saving new assistant");
}
