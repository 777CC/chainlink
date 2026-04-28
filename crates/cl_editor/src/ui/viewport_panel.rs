//! Center dock — placeholder scene preview.

use crate::app::EditorApp;

/// Render the viewport. The Chainlink editor doesn't embed wgpu inside the
/// egui window: the preview is a stylised placeholder pointing the user at
/// `cargo run -p platformer` to actually launch the engine.
pub fn show(ui: &mut egui::Ui, app: &mut EditorApp) {
    let rect = ui.available_rect_before_wrap();
    let painter = ui.painter_at(rect);

    painter.rect_filled(rect, 0.0, egui::Color32::from_gray(28));

    let center = rect.center();
    let title = format!("Scene Preview — {}", app.scene.name);
    painter.text(
        center,
        egui::Align2::CENTER_CENTER,
        title,
        egui::FontId::proportional(20.0),
        egui::Color32::LIGHT_GRAY,
    );
    painter.text(
        center + egui::vec2(0.0, 28.0),
        egui::Align2::CENTER_CENTER,
        "Run with: cargo run -p platformer",
        egui::FontId::monospace(14.0),
        egui::Color32::GRAY,
    );

    let node_count = count_nodes(&app.scene.root);
    painter.text(
        center + egui::vec2(0.0, 60.0),
        egui::Align2::CENTER_CENTER,
        format!("{node_count} node(s) in scene"),
        egui::FontId::proportional(13.0),
        egui::Color32::DARK_GRAY,
    );

    // Allocate the rect so egui doesn't think the panel is empty.
    ui.allocate_rect(rect, egui::Sense::hover());
}

fn count_nodes(node: &crate::scene_doc::EditorNode) -> usize {
    1 + node.children.iter().map(count_nodes).sum::<usize>()
}
