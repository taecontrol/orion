use crate::{db::establish_db_connection, models::assistant::Assistant, schema};
use diesel::prelude::*;

pub fn store_new_assistant(assistant: &Assistant) {
    let connection = &mut establish_db_connection();

    diesel::insert_into(schema::assistants::dsl::assistants)
        .values(assistant)
        .execute(connection)
        .expect("Error saving new assistant");
}
