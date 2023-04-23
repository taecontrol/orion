use crate::settings::Settings;

#[tauri::command]
pub fn get_settings() -> Settings {
    Settings::get()
}

#[tauri::command]
pub fn set_settings(settings: Settings) {
    settings.set();
}
