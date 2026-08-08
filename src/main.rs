mod app;
mod menu;
mod recent;
mod tab;

use app::MdReaderApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([600.0, 400.0]),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "MD Reader",
        options,
        Box::new(|cc| {
            // Enable image loading for egui_commonmark
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MdReaderApp::new(cc)))
        }),
    )
}
