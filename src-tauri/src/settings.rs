use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub open_ai_secret: String,
}

impl Settings {
    pub fn init() {
        if !file_exists() {
            create_file();
        }
    }

    pub fn get() -> Self {
        let string_content = fs::read_to_string(get_file_path()).unwrap();

        serde_json::from_str::<Settings>(&string_content).unwrap()
    }

    pub fn set(&self) {
        fs::write(get_file_path(), serde_json::to_string_pretty(self).unwrap()).ok();
    }
}

fn create_file() {
    let path = get_file_path();
    let dir = Path::new(&path).parent().unwrap();

    if !dir.exists() {
        fs::create_dir_all(dir).unwrap();
    }

    let mut file = fs::File::create(path).unwrap();

    file.write_all(
        b"{
        open_ai_secret: \"\"
    }",
    )
    .unwrap();
}

fn file_exists() -> bool {
    let path = get_file_path();
    Path::new(&path).exists()
}

fn get_file_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/orion/settings.json"
}
