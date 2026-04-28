//! Top menu bar with File / Scene / Import drop-downs.

use crate::app::EditorApp;
use crate::node_kinds::NODE_KINDS;

/// Render the menu bar inside the given [`egui::Ui`].
pub fn show(ui: &mut egui::Ui, app: &mut EditorApp) {
    egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("New Project").clicked() {
                app.action_new_project();
                ui.close_menu();
            }
            if ui.button("Open Project").clicked() {
                app.action_open_project();
                ui.close_menu();
            }
            ui.separator();
            if ui.button("New Scene").clicked() {
                app.action_new_scene();
                ui.close_menu();
            }
            if ui.button("Open Scene").clicked() {
                app.action_open_scene();
                ui.close_menu();
            }
            if ui.button("Save Scene").clicked() {
                app.action_save_scene();
                ui.close_menu();
            }
            ui.separator();
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            }
        });

        ui.menu_button("Scene", |ui| {
            ui.label("Add Node:");
            for (kind, _) in NODE_KINDS {
                if ui.button(*kind).clicked() {
                    app.action_add_node(kind);
                    ui.close_menu();
                }
            }
        });

        ui.menu_button("Import", |ui| {
            if ui.button("Image (PNG/JPG)…").clicked() {
                app.action_import_image();
                ui.close_menu();
            }
            if ui.button("Audio (WAV/OGG)…").clicked() {
                app.action_import_audio();
                ui.close_menu();
            }
            if ui.button("3D Model (OBJ)…").clicked() {
                app.action_import_model();
                ui.close_menu();
            }
        });

        ui.menu_button("Project", |ui| {
            if let Some(p) = &app.project {
                ui.label(format!("Name: {}", p.name));
                ui.label(format!("Version: {}", p.version));
                ui.label(format!("Path: {}", p.root_path.display()));
            } else {
                ui.label("No project open.");
            }
        });
    });
}
