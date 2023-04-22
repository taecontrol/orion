use crate::{db::establish_db_connection, schema::sessions::dsl};
use diesel::prelude::*;

pub fn update_session_name(session_id: String, name: String) {
    let connection = &mut establish_db_connection();

    diesel::update(dsl::sessions)
        .filter(dsl::id.eq(session_id.clone()))
        .set(dsl::name.eq(name.clone()))
        .execute(connection)
        .expect("Error updating session name");
}
