#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod db;
mod models;
mod schema;
mod settings;
mod open_ai;

use commands::ask_command::ask;
use commands::session_commands::*;
use commands::message_commands::*;

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
        .invoke_handler(tauri::generate_handler![ask, new_session, list_sessions, list_messages])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
