#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod db;
mod models;
mod notification;
mod open_ai;
mod schema;
mod services;
mod settings;

use commands::ask_command::ask;
use commands::assistant_commands::*;
use commands::message_commands::*;
use commands::session_commands::*;
use commands::settings_commands::*;
use services::assistants_service;

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .setup(|_app| {
            tokio::spawn(async move {
                db::init();
                settings::Settings::init();
                assistants_service::init();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ask,
            new_session,
            list_sessions,
            get_session,
            list_messages,
            delete_session,
            get_settings,
            set_settings,
            create_assistant,
            list_assistants,
            get_assistant,
            update_assistant,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
