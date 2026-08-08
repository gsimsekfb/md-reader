use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const MAX_RECENT: usize = 20;
const CONFIG_DIR_NAME: &str = "md-reader";
const RECENT_FILE_NAME: &str = "recent.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecentFiles {
    pub entries: Vec<PathBuf>,
}

impl RecentFiles {

    /// Load recent files from the platform config directory.
    /// Returns an empty list if the file doesn't exist or can't be parsed.
    pub fn load() -> Self {
        let Some(path) = Self::config_path() else {
            return Self::default();
        };

        match std::fs::read_to_string(&path) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Persist the recent files list to disk.
    pub fn save(&self) {
        let Some(path) = Self::config_path() else {
            return;
        };

        // Ensure the config directory exists
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(&path, data);
        }
    }

    /// Add a path to the front of the list, deduplicating and trimming.
    pub fn add(&mut self, path: &Path) {
        let canonical = path.to_path_buf();

        // Remove existing duplicate
        self.entries.retain(|p| p != &canonical);

        // Insert at front
        self.entries.insert(0, canonical);

        // Trim to max
        self.entries.truncate(MAX_RECENT);
    }

    /// Returns the path to the recent files JSON config file.
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|dir| dir.join(CONFIG_DIR_NAME).join(RECENT_FILE_NAME))
    }
}
