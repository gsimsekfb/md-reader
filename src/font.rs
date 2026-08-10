use egui::{FontData, FontDefinitions, FontFamily, FontId, TextStyle};
use std::sync::Arc;

/// Load a bundled sans-serif font for headings/body and a monospace font
/// for code blocks, so every text style renders with a consistent typeface
/// instead of falling back to whatever the OS provides per-style.
pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // Body/heading font. Place the .ttf under assets/ and embed it at compile ime.
    fonts.font_data.insert(
        "inter".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/Inter-Regular.ttf"
        ))),
    );

    // Monospace font for code blocks.
    fonts.font_data.insert(
        "jetbrains_mono".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/JetBrainsMono-Regular.ttf"
        ))),
    );

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "inter".to_owned());

    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "jetbrains_mono".to_owned());

    ctx.set_fonts(fonts);
}

/// Explicit sizes per TextStyle so headings/body/code are visually
/// consistent instead of relying on egui_commonmark's own defaults.
pub fn install_text_styles(ctx: &egui::Context) {
    ctx.all_styles_mut(|style| {
        style.text_styles = [
            (TextStyle::Heading, FontId::new(24.0, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(15.0, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(14.0, FontFamily::Monospace)),
            (TextStyle::Button, FontId::new(15.0, FontFamily::Proportional)),
            (TextStyle::Small, FontId::new(12.0, FontFamily::Proportional)),
        ]
        .into();
    });
}
