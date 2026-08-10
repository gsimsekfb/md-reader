use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const CONFIG_DIR_NAME: &str = "md-reader";
const SETTINGS_FILE_NAME: &str = "settings.json";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowSettings {
    pub position: Option<[f32; 2]>,
    pub size: Option<[f32; 2]>,
    pub zoom_level: f32,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            position: None,
            size: None,
            zoom_level: 1.0,
        }
    }
}

impl WindowSettings {
    pub fn load() -> Self {
        Self::load_from(&Self::config_path())
    }

    pub fn load_from(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        match std::fs::read_to_string(path) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) {
        self.save_to(&Self::config_path());
    }

    pub fn save_to(&self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, data);
        }
    }

    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(CONFIG_DIR_NAME)
            .join(SETTINGS_FILE_NAME)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_load_round_trip() {
        let path = std::env::temp_dir().join("md-reader-settings-roundtrip.json");
        let expected = WindowSettings {
            position: Some([120.0, 240.0]),
            size: Some([1024.0, 768.0]),
            zoom_level: 1.25,
        };

        expected.save_to(&path);
        let loaded = WindowSettings::load_from(&path);

        assert_eq!(loaded, expected);
        let _ = std::fs::remove_file(path);
    }
}
