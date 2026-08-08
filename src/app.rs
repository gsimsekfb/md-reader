use crate::menu::{self, MenuAction};
use crate::recent::RecentFiles;
use crate::tab::{self, TabAction, TabState};
use egui_commonmark::CommonMarkViewer;
use std::path::Path;

/// Top-level application state.
pub struct MdReaderApp {
    /// Currently open tabs.
    tabs: Vec<TabState>,
    /// Index of the active (visible) tab.
    active_tab: usize,
    /// Persistent recent-files list.
    recent_files: RecentFiles,
    /// Global zoom level (1.0 = 100%).
    zoom_level: f32,
    /// The default pixels_per_point from the system, captured once at startup.
    base_pixels_per_point: f32,
}

impl MdReaderApp {

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Capture the system's default scaling so zoom is relative to it.
        let base_ppp = cc.egui_ctx.pixels_per_point();

        // Increase scroll speed (default is 40.0)
        cc.egui_ctx.options_mut(|o| o.input_options.line_scroll_speed = 120.0);

        Self {
            tabs: Vec::new(),
            active_tab: 0,
            recent_files: RecentFiles::load(),
            zoom_level: 1.0,
            base_pixels_per_point: base_ppp,
        }
    }

    /// Open a file by path. If already open, switch to that tab.
    fn open_file(&mut self, path: &Path) {
        // Check if already open
        if let Some(idx) = self.tabs.iter().position(|t| t.file_path == path) {
            self.active_tab = idx;
            return;
        }

        if let Some(tab) = TabState::from_file(path) {
            self.recent_files.add(path);
            self.recent_files.save();
            self.tabs.push(tab);
            self.active_tab = self.tabs.len() - 1;
        }
    }

    /// Show the native file-open dialog and open the selected file.
    fn open_file_dialog(&mut self) {
        let file = rfd::FileDialog::new()
            .add_filter("Markdown", &["md", "markdown", "txt"])
            .pick_file();

        if let Some(path) = file {
            self.open_file(&path);
        }
    }

    /// Close the tab at the given index.
    fn close_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.tabs.remove(index);
            // Adjust active tab
            if self.tabs.is_empty() {
                self.active_tab = 0;
            } else if self.active_tab >= self.tabs.len() {
                self.active_tab = self.tabs.len() - 1;
            }
        }
    }

    /// Apply zoom by adjusting pixels_per_point relative to the system default.
    fn apply_zoom(&self, ctx: &egui::Context) {
        ctx.set_pixels_per_point(self.base_pixels_per_point * self.zoom_level);
    }

    /// Handle keyboard shortcuts.
    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        // Ctrl+O → Open file
        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::O))) {
            self.open_file_dialog();
        }

        // Ctrl+W → Close tab
        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::W))) {
            let idx = self.active_tab;
            self.close_tab(idx);
        }

        // Ctrl+= → Zoom in
        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::Equals))) {
            self.zoom_level = (self.zoom_level + 0.1).min(3.0);
        }

        // Ctrl+- → Zoom out
        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::Minus))) {
            self.zoom_level = (self.zoom_level - 0.1).max(0.5);
        }

        // Ctrl+0 → Reset zoom
        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::Num0))) {
            self.zoom_level = 1.0;
        }

        // Ctrl+Tab → Next tab
        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::Tab))) {
            if !self.tabs.is_empty() {
                self.active_tab = (self.active_tab + 1) % self.tabs.len();
            }
        }
    }
}

impl eframe::App for MdReaderApp {

    /// Called every frame by eframe to draw the entire application UI —
    /// menu bar, tab bar, and the active tab's markdown content.
    /// Note: `eframe::run_native` calls this fn automatically once per 
    /// frame as part of the event loop, since it's the required method of 
    /// the `eframe::App` trait.
    /// 
    /// # Parameters
    /// - `self`: Mutable app state (tabs, zoom, recent files).
    /// - `ui`: The root egui UI for this frame.
    /// - `_frame`: Native window/frame handle (unused here).
    /// 
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.apply_zoom(ui.ctx());

        // Handle keyboard shortcuts
        self.handle_shortcuts(ui.ctx());

        // ── Top panel: menu bar ────────────────────────────
        let mut menu_action = MenuAction::None;
        egui::Panel::top("menu_bar").show(ui, |ui| {
            menu_action = menu::render_menu_bar(
                ui,
                &self.recent_files,
                self.zoom_level,
                !self.tabs.is_empty(),
            );
        });

        // Process menu action
        match menu_action {
            MenuAction::OpenFile => self.open_file_dialog(),
            MenuAction::OpenRecent(path) => {
                let path = path.clone();
                self.open_file(&path);
            }
            MenuAction::CloseTab => {
                let idx = self.active_tab;
                self.close_tab(idx);
            }
            MenuAction::Exit => {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
            MenuAction::ZoomIn => {
                self.zoom_level = (self.zoom_level + 0.1).min(3.0);
            }
            MenuAction::ZoomOut => {
                self.zoom_level = (self.zoom_level - 0.1).max(0.5);
            }
            MenuAction::ZoomReset => {
                self.zoom_level = 1.0;
            }
            MenuAction::None => {}
        }

        // ── Tab bar (only if tabs exist) ───────────────────
        if !self.tabs.is_empty() {
            let mut tab_action = TabAction::None;
            egui::Panel::top("top_bar").show(ui, |ui| {
                tab_action = tab::render_tab_bar(ui, &self.tabs, self.active_tab);
            });

            match tab_action {
                TabAction::Switch(idx) => self.active_tab = idx,
                TabAction::Close(idx) => self.close_tab(idx),
                TabAction::None => {}
            }
        }

        // ── Central panel: markdown content ────────────────
        egui::CentralPanel::default().show(ui, |ui| {
            if self.tabs.is_empty() {
                // Empty state
                ui.vertical_centered(|ui| {
                    ui.add_space(ui.available_height() / 3.0);
                    ui.heading("MD Reader");
                    ui.add_space(8.0);
                    ui.weak("Open a Markdown file to get started");
                    ui.add_space(16.0);
                    ui.weak("Ctrl+O to open a file");
                });
            } else if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                // Show URL tooltips on hover
                ui.style_mut().url_in_tooltip = true;

                egui::ScrollArea::vertical()
                    .id_salt(format!("scroll_{}", tab.file_path.display()))
                    .show(ui, |ui| {
                        // Add some horizontal padding for a nicer reading experience
                        let max_width = (ui.available_width() - 80.0).max(400.0);
                        ui.set_max_width(max_width);

                        CommonMarkViewer::new()
                            .show(ui, &mut tab.cache, &tab.content);
                    });
            }
        });
    }
}
