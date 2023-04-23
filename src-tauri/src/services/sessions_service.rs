use crate::{
    db::establish_db_connection,
    models::session::{NewSession, Session},
    schema::sessions,
    schema::sessions::dsl,
};
use diesel::prelude::*;

pub fn list_sessions(assistant_id: &String) -> Vec<Session> {
    let connection = &mut establish_db_connection();

    dsl::sessions
        .filter(dsl::assistant_id.eq(assistant_id))
        .order_by(dsl::created_at.desc())
        .load::<Session>(connection)
        .expect("Error loading sessions")
}

pub fn store_session(new_session: &NewSession) {
    let connection = &mut establish_db_connection();

    diesel::insert_into(sessions::table)
        .values(new_session)
        .execute(connection)
        .expect("Error saving new session");
}

pub fn update_session_name(session_id: String, name: String) {
    let connection = &mut establish_db_connection();

    diesel::update(dsl::sessions)
        .filter(dsl::id.eq(session_id.clone()))
        .set(dsl::name.eq(name.clone()))
        .execute(connection)
        .expect("Error updating session name");
}

pub fn delete_session(session_id: String) {
    let connection = &mut establish_db_connection();

    diesel::delete(dsl::sessions)
        .filter(crate::schema::sessions::dsl::id.eq(session_id))
        .execute(connection)
        .expect("Error deleting session");
}
