use serde::Serialize;
use tauri::Manager;

#[derive(Serialize, Clone)]
pub struct Notification {
    pub r#type: NotificationType,
    pub title: String,
    pub body: String,
}

#[derive(Clone)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

impl Notification {
    pub fn info(app_handle: tauri::AppHandle, title: &str, body: &str) {
        Notification::notify(app_handle, NotificationType::Info, title, body);
    }

    pub fn success(app_handle: tauri::AppHandle, title: &str, body: &str) {
        Notification::notify(app_handle, NotificationType::Success, title, body);
    }

    pub fn warning(app_handle: tauri::AppHandle, title: &str, body: &str) {
        Notification::notify(app_handle, NotificationType::Warning, title, body);
    }

    pub fn error(app_handle: tauri::AppHandle, title: &str, body: &str) {
        Notification::notify(app_handle, NotificationType::Error, title, body);
    }

    fn notify(app_handle: tauri::AppHandle, r#type: NotificationType, title: &str, body: &str) {
        app_handle
            .emit_all(
                "notification",
                Self {
                    r#type,
                    title: title.to_string(),
                    body: body.to_string(),
                },
            )
            .unwrap();
    }
}

impl NotificationType {
    pub fn as_str(&self) -> &str {
        match self {
            NotificationType::Info => "info",
            NotificationType::Success => "success",
            NotificationType::Warning => "warning",
            NotificationType::Error => "error",
        }
    }
}

impl Serialize for NotificationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
