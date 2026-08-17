use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_DIR_NAME: &str = "md-reader";
const SESSION_FILE_NAME: &str = "session.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Session {
    pub open_files: Vec<PathBuf>,
    pub active_tab: usize,
}

impl Session {
    pub fn load() -> Self {
        let Some(path) = Self::config_path() else {
            println!("Error: Failed to load session - last opened files");
            return Self::default();
        };

        match std::fs::read_to_string(&path) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) {
        let Some(path) = Self::config_path() else {
            println!("Error: Failed to save session - last opened files");
            return;
        };

        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(&path, data);
            println!("Ok: Saved session - last opened files");
        }
    }

    /// Build a Session from the current app state.
    pub fn from_tabs(paths: &[PathBuf], active_tab: usize) -> Self {
        Self {
            open_files: paths.to_vec(),
            active_tab,
        }
    }

    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|dir| dir.join(CONFIG_DIR_NAME).join(SESSION_FILE_NAME))
    }
}
