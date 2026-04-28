//! Chainlink Editor — eframe entrypoint.

use cl_editor::app::EditorApp;

fn main() -> Result<(), eframe::Error> {
    env_logger::try_init().ok();

    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Chainlink Editor"),
        ..Default::default()
    };

    eframe::run_native(
        "Chainlink Editor",
        opts,
        Box::new(|_cc| Ok(Box::new(EditorApp::new()))),
    )
}
