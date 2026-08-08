use crate::recent::RecentFiles;
use std::path::PathBuf;

/// Actions that can be triggered from the menu bar.
pub enum MenuAction {
    None,
    OpenFile,
    OpenRecent(PathBuf),
    CloseTab,
    Exit,
    ZoomIn,
    ZoomOut,
    ZoomReset,
}

/// Render the top menu bar and return any triggered action.
pub fn render_menu_bar(
    ui: &mut egui::Ui,
    recent_files: &RecentFiles,
    zoom_level: f32,
    has_tabs: bool,
) -> MenuAction {
    let mut action = MenuAction::None;

    egui::MenuBar::new().ui(ui, |ui| {
    
        // ── File menu ──────────────────────────────────────
        ui.menu_button("File", |ui| {
            if ui
                .add_enabled(true, egui::Button::new("📂 Open File…").shortcut_text("Ctrl+O"))
                .clicked()
            {
                action = MenuAction::OpenFile;
                ui.close();
            }

            // Recent files submenu
            ui.menu_button("📋 Recent Files", |ui| {
                if recent_files.entries.is_empty() {
                    ui.weak("(none)");
                } else {
                    for path in &recent_files.entries {
                        let label = path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| path.to_string_lossy().to_string());

                        let response = ui.button(&label).on_hover_text(path.to_string_lossy());
                        if response.clicked() {
                            action = MenuAction::OpenRecent(path.clone());
                            ui.close();
                        }
                    }
                }
            });

            ui.separator();

            if ui
                .add_enabled(
                    has_tabs,
                    egui::Button::new("✕ Close Tab").shortcut_text("Ctrl+W"),
                )
                .clicked()
            {
                action = MenuAction::CloseTab;
                ui.close();
            }

            ui.separator();

            if ui.button("Exit").clicked() {
                action = MenuAction::Exit;
                ui.close();
            }
        });

        // ── View menu ──────────────────────────────────────
        ui.menu_button("View", |ui| {
            let zoom_pct = format!("{:.0}%", zoom_level * 100.0);
            ui.weak(format!("Zoom: {zoom_pct}"));
            ui.separator();

            if ui
                .add_enabled(
                    zoom_level < 3.0,
                    egui::Button::new("🔍+ Zoom In").shortcut_text("Ctrl+="),
                )
                .clicked()
            {
                action = MenuAction::ZoomIn;
                ui.close();
            }

            if ui
                .add_enabled(
                    zoom_level > 0.5,
                    egui::Button::new("🔍− Zoom Out").shortcut_text("Ctrl+−"),
                )
                .clicked()
            {
                action = MenuAction::ZoomOut;
                ui.close();
            }

            ui.separator();

            if ui
                .add(egui::Button::new("↺ Reset Zoom").shortcut_text("Ctrl+0"))
                .clicked()
            {
                action = MenuAction::ZoomReset;
                ui.close();
            }
        });
    });

    action
}
