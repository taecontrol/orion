// @generated automatically by Diesel CLI.

diesel::table! {
    assistants (id) {
        id -> Text,
        name -> Text,
        description -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    messages (id) {
        id -> Text,
        session_id -> Text,
        content -> Text,
        role -> Text,
        finish_reason -> Text,
        prompt_tokens -> Integer,
        completion_tokens -> Integer,
        created_at -> Timestamp,
    }
}

diesel::table! {
    sessions (id) {
        id -> Text,
        assistant_id -> Text,
        name -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    assistants,
    messages,
    sessions,
);
