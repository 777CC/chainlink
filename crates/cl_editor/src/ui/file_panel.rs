//! Bottom dock — project asset browser split by category.

use std::path::PathBuf;

use crate::app::EditorApp;

/// Render the file/asset panel.
pub fn show(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.heading("Files");
    ui.separator();

    let Some(project) = app.project.clone() else {
        ui.label("No project open. Use File → New Project / Open Project.");
        return;
    };

    ui.horizontal(|ui| {
        section(ui, "Images", &project.list_dir("images"));
        ui.separator();
        section(ui, "Audio", &project.list_dir("audio"));
        ui.separator();
        section(ui, "Models", &project.list_dir("models"));
        ui.separator();
        section(ui, "Scenes", &project.list_dir("scenes"));
    });
}

fn section(ui: &mut egui::Ui, label: &str, files: &[PathBuf]) {
    ui.vertical(|ui| {
        let total: u64 = files
            .iter()
            .map(|p| std::fs::metadata(p).map(|m| m.len()).unwrap_or(0))
            .sum();
        ui.strong(format!("{label} ({} files, {} B)", files.len(), total));
        egui::ScrollArea::vertical()
            .id_source(format!("file_section_{label}"))
            .max_height(140.0)
            .show(ui, |ui| {
                if files.is_empty() {
                    ui.weak("(empty)");
                } else {
                    for f in files {
                        let display = f
                            .file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or("?");
                        ui.label(display);
                    }
                }
            });
    });
}
