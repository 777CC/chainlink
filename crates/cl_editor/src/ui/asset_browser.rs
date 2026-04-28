//! Generic asset list with file-size summaries.
//!
//! This module is split out so panels other than the bottom dock can render
//! the same kind of asset list (e.g. inside an "Open" dialog). The bottom
//! dock renders its own compact grid in [`crate::ui::file_panel`].

use std::path::PathBuf;

/// Sum byte sizes of the given paths, ignoring missing files.
pub fn total_size(paths: &[PathBuf]) -> u64 {
    paths
        .iter()
        .map(|p| std::fs::metadata(p).map(|m| m.len()).unwrap_or(0))
        .sum()
}

/// Render a generic single-column asset list. Returns `Some(path)` when
/// the user clicks an entry.
pub fn show(ui: &mut egui::Ui, label: &str, paths: &[PathBuf]) -> Option<PathBuf> {
    ui.strong(format!(
        "{label} — {} files, {} B",
        paths.len(),
        total_size(paths)
    ));
    let mut clicked = None;
    egui::ScrollArea::vertical()
        .id_source(format!("asset_browser_{label}"))
        .show(ui, |ui| {
            for p in paths {
                let name = p
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("?");
                if ui.selectable_label(false, name).clicked() {
                    clicked = Some(p.clone());
                }
            }
        });
    clicked
}
