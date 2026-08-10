use egui_commonmark::CommonMarkCache;
use std::path::{Path, PathBuf};

/// Represents a single open document tab.
#[derive(Debug)]
pub struct TabState {
    /// Absolute path to the source file.
    pub file_path: PathBuf,
    /// Display title (filename only) for the tab label.
    pub title: String,
    /// Raw markdown content. Kept as a mutable String to support future editing.
    pub content: String,
    /// Per-tab rendering cache for egui_commonmark.
    pub cache: CommonMarkCache,
}

impl TabState {

    /// Open a file and create a new tab from it.
    /// Returns `None` if the file cannot be read.
    pub fn from_file(path: &Path) -> Option<Self> {
        let content = std::fs::read_to_string(path).ok()?;
        let title = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Untitled".to_string());

        Some(Self {
            file_path: path.to_path_buf(),
            title,
            content,
            cache: CommonMarkCache::default(),
        })
    }
}

pub enum TabAction {
    None,
    Switch(usize),
    Close(usize),
}

/// Renders the horizontal tab bar with switch/close controls for each open tab.
///
/// # Parameters
/// - `ui`: The egui UI context to draw into.
/// - `tabs`: The list of currently open tabs.
/// - `active`: Index of the currently active tab.
///
/// # Returns
/// The `TabAction` triggered by user interaction, if any.
pub fn render_tab_bar(ui: &mut egui::Ui, tabs: &[TabState], active: usize) -> TabAction {
    let mut action = TabAction::None;

    ui.horizontal(|ui| {
        for (i, tab) in tabs.iter().enumerate() {
            let is_active = i == active;

            let label = if is_active {
                egui::RichText::new(&tab.title).strong()
            } else {
                egui::RichText::new(&tab.title)
            };

            // Tab button
            let response = ui.selectable_label(is_active, label);
            if response.clicked() && !is_active {
                action = TabAction::Switch(i);
            }

            // Close button (small ×)
            let close_response = ui.small_button("×");
            if close_response.clicked() {
                action = TabAction::Close(i);
            }

            ui.separator();
        }
    });

    action
}
