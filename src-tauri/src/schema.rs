// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Integer,
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
        name -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    sessions,
);
