#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod db;
mod models;
mod open_ai;
mod schema;
mod services;
mod settings;

use commands::ask_command::ask;
use commands::message_commands::*;
use commands::session_commands::*;
use commands::settings_commands::*;

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .setup(|_app| {
            tokio::spawn(async move {
                db::init();
                settings::Settings::init();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ask,
            new_session,
            list_sessions,
            list_messages,
            delete_session,
            get_settings,
            set_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
