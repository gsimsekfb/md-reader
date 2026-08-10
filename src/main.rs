mod app;
mod font;
mod menu;
mod recent;
mod settings;
mod tab;

use app::MdReaderApp;

fn main() -> eframe::Result<()> {
    let saved_settings = settings::WindowSettings::load();
    let initial_size = saved_settings.size.unwrap_or([1200.0, 800.0]);
    let initial_position = saved_settings.position.unwrap_or([100.0, 100.0]);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(initial_size)
            .with_position(initial_position)
            .with_min_inner_size([600.0, 400.0]),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "MD Reader",
        options,
        Box::new(|cc| {
            font::install_fonts(&cc.egui_ctx);
            font::install_text_styles(&cc.egui_ctx);

            // Enable image loading for egui_commonmark
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MdReaderApp::new(cc)))
        }),
    )
}
