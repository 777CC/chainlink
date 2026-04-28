//! Right dock — property inspector for the selected node.

use crate::app::EditorApp;
use crate::scene_doc::EditorValue;

/// Render the inspector for the currently selected node.
pub fn show(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.heading("Inspector");
    ui.separator();

    let Some(id) = app.scene.selected_id else {
        ui.label("No node selected.");
        return;
    };

    let dirty_before = app.scene.dirty;
    let mut became_dirty = false;

    if let Some(node) = app.scene.find_mut(id) {
        ui.horizontal(|ui| {
            ui.label("Name:");
            if ui
                .add(egui::TextEdit::singleline(&mut node.name))
                .changed()
            {
                became_dirty = true;
            }
        });
        ui.horizontal(|ui| {
            ui.label("Kind:");
            ui.monospace(&node.kind);
        });
        ui.horizontal(|ui| {
            ui.label("ID:");
            ui.monospace(node.id.to_string());
        });
        ui.separator();
        ui.label("Properties:");

        // Stable property order so the UI doesn't jump around.
        let mut keys: Vec<String> = node.properties.keys().cloned().collect();
        keys.sort();

        for key in keys {
            // Re-fetch each iteration so `node` is borrowed only for one prop.
            if let Some(val) = node.properties.get_mut(&key) {
                ui.horizontal(|ui| {
                    ui.label(&key);
                    match val {
                        EditorValue::Bool(b) => {
                            if ui.checkbox(b, "").changed() {
                                became_dirty = true;
                            }
                        }
                        EditorValue::Int(i) => {
                            if ui.add(egui::DragValue::new(i)).changed() {
                                became_dirty = true;
                            }
                        }
                        EditorValue::Float(f) => {
                            if ui.add(egui::DragValue::new(f).speed(0.1)).changed() {
                                became_dirty = true;
                            }
                        }
                        EditorValue::String(s) => {
                            if ui
                                .add(egui::TextEdit::singleline(s))
                                .changed()
                            {
                                became_dirty = true;
                            }
                        }
                        EditorValue::Vec2 { x, y } => {
                            if ui.add(egui::DragValue::new(x).prefix("x: ").speed(0.5)).changed() {
                                became_dirty = true;
                            }
                            if ui.add(egui::DragValue::new(y).prefix("y: ").speed(0.5)).changed() {
                                became_dirty = true;
                            }
                        }
                        EditorValue::Color { r, g, b, a } => {
                            let mut rgba = [*r, *g, *b, *a];
                            if ui.color_edit_button_rgba_unmultiplied(&mut rgba).changed() {
                                *r = rgba[0];
                                *g = rgba[1];
                                *b = rgba[2];
                                *a = rgba[3];
                                became_dirty = true;
                            }
                        }
                        EditorValue::AssetRef(path) => {
                            if ui
                                .add(egui::TextEdit::singleline(path).desired_width(160.0))
                                .changed()
                            {
                                became_dirty = true;
                            }
                            if ui.button("Browse…").clicked() {
                                if let Some(picked) = rfd::FileDialog::new().pick_file() {
                                    *path = picked.display().to_string();
                                    became_dirty = true;
                                }
                            }
                        }
                    }
                });
            }
        }
    } else {
        ui.label("Node not found.");
    }

    if became_dirty && !dirty_before {
        app.scene.dirty = true;
    } else if became_dirty {
        app.scene.dirty = true;
    }
}
