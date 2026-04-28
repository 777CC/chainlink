//! [`EditorApp`] — the eframe application that drives the IDE.
//!
//! Holds the open project + active scene, exposes high-level actions
//! triggered by menus/buttons, and lays the panels out in [`eframe::App::update`].

use std::path::PathBuf;

use crate::importer;
use crate::node_kinds;
use crate::project::Project;
use crate::scene_doc::EditorScene;
use crate::ui;

/// Top-level editor state.
pub struct EditorApp {
    pub project: Option<Project>,
    pub scene: EditorScene,
    pub scene_path: Option<PathBuf>,
    pub status: String,
    pub last_error: Option<String>,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorApp {
    pub fn new() -> Self {
        Self {
            project: None,
            scene: EditorScene::new("MainScene"),
            scene_path: None,
            status: "Welcome to Chainlink Editor".into(),
            last_error: None,
        }
    }

    // --- project lifecycle -------------------------------------------------

    pub fn action_new_project(&mut self) {
        self.last_error = None;
        let Some(folder) = rfd::FileDialog::new().pick_folder() else {
            self.status = "New project cancelled.".into();
            return;
        };
        let name = folder
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .to_string();
        match Project::create(&folder, &name) {
            Ok(p) => {
                self.status = format!("Created project: {} at {}", p.name, p.root_path.display());
                self.project = Some(p);
            }
            Err(e) => self.last_error = Some(format!("Create project failed: {e}")),
        }
    }

    pub fn action_open_project(&mut self) {
        self.last_error = None;
        let dialog = rfd::FileDialog::new().add_filter("Chainlink project", &["cls"]);
        let Some(file) = dialog.pick_file().or_else(|| rfd::FileDialog::new().pick_folder())
        else {
            self.status = "Open project cancelled.".into();
            return;
        };
        match Project::open(&file) {
            Ok(p) => {
                self.status = format!("Opened project: {}", p.name);
                self.project = Some(p);
            }
            Err(e) => self.last_error = Some(format!("Open project failed: {e}")),
        }
    }

    // --- scene lifecycle ---------------------------------------------------

    pub fn action_new_scene(&mut self) {
        self.scene = EditorScene::new("UntitledScene");
        self.scene_path = None;
        self.status = "New scene created.".into();
    }

    pub fn action_save_scene(&mut self) {
        self.last_error = None;
        let dest_path = match self.scene_path.clone() {
            Some(p) => p,
            None => {
                let default_dir = self
                    .project
                    .as_ref()
                    .map(|p| p.scenes_dir())
                    .unwrap_or_else(|| PathBuf::from("."));
                let mut dialog = rfd::FileDialog::new()
                    .set_file_name(format!("{}.cls.scene", self.scene.name))
                    .add_filter("Chainlink scene", &["scene", "json"]);
                if default_dir.exists() {
                    dialog = dialog.set_directory(&default_dir);
                }
                let Some(p) = dialog.save_file() else {
                    self.status = "Save scene cancelled.".into();
                    return;
                };
                p
            }
        };

        match serde_json::to_string_pretty(&self.scene) {
            Ok(json) => match std::fs::write(&dest_path, json) {
                Ok(()) => {
                    self.scene.dirty = false;
                    self.status = format!("Saved scene to {}", dest_path.display());
                    self.scene_path = Some(dest_path);
                }
                Err(e) => self.last_error = Some(format!("Save scene failed: {e}")),
            },
            Err(e) => self.last_error = Some(format!("Serialise scene failed: {e}")),
        }
    }

    pub fn action_open_scene(&mut self) {
        self.last_error = None;
        let mut dialog =
            rfd::FileDialog::new().add_filter("Chainlink scene", &["scene", "json"]);
        if let Some(p) = &self.project {
            let dir = p.scenes_dir();
            if dir.exists() {
                dialog = dialog.set_directory(&dir);
            }
        }
        let Some(path) = dialog.pick_file() else {
            self.status = "Open scene cancelled.".into();
            return;
        };
        match std::fs::read(&path) {
            Ok(bytes) => match serde_json::from_slice::<EditorScene>(&bytes) {
                Ok(mut scene) => {
                    scene.dirty = false;
                    if scene.selected_id.is_none() {
                        scene.selected_id = Some(scene.root.id);
                    }
                    self.scene = scene;
                    self.status = format!("Opened scene: {}", path.display());
                    self.scene_path = Some(path);
                }
                Err(e) => self.last_error = Some(format!("Parse scene failed: {e}")),
            },
            Err(e) => self.last_error = Some(format!("Read scene failed: {e}")),
        }
    }

    // --- scene editing -----------------------------------------------------

    pub fn action_add_node(&mut self, kind: &str) {
        let id = self.scene.next_id();
        let parent = self.scene.selected_id.unwrap_or(self.scene.root.id);
        let node = node_kinds::make_node(kind, id);
        if self.scene.add_child(parent, node) {
            self.scene.selected_id = Some(id);
            self.scene.dirty = true;
            self.status = format!("Added {} node (id {})", kind, id);
        } else {
            self.last_error =
                Some(format!("Could not find parent id {parent} to add {kind}"));
        }
    }

    // --- asset import ------------------------------------------------------

    pub fn action_import_image(&mut self) {
        self.last_error = None;
        let Some(project) = self.project.clone() else {
            self.last_error =
                Some("Import failed: no project is open. Use File → New Project first.".into());
            return;
        };
        let Some(src) = rfd::FileDialog::new()
            .add_filter("Image", &["png", "jpg", "jpeg"])
            .pick_file()
        else {
            self.status = "Import image cancelled.".into();
            return;
        };
        match importer::image::import(&project, &src) {
            Ok(info) => {
                self.status = format!(
                    "Imported image {} ({}x{}, {} B)",
                    info.path.display(),
                    info.width,
                    info.height,
                    info.byte_count
                );
            }
            Err(e) => self.last_error = Some(format!("Image import failed: {e}")),
        }
    }

    pub fn action_import_audio(&mut self) {
        self.last_error = None;
        let Some(project) = self.project.clone() else {
            self.last_error =
                Some("Import failed: no project is open. Use File → New Project first.".into());
            return;
        };
        let Some(src) = rfd::FileDialog::new()
            .add_filter("Audio", &["wav", "ogg"])
            .pick_file()
        else {
            self.status = "Import audio cancelled.".into();
            return;
        };
        match importer::audio::import(&project, &src) {
            Ok(info) => {
                self.status = format!(
                    "Imported audio {} (.{}, {} B)",
                    info.path.display(),
                    info.ext,
                    info.byte_count
                );
            }
            Err(e) => self.last_error = Some(format!("Audio import failed: {e}")),
        }
    }

    pub fn action_import_model(&mut self) {
        self.last_error = None;
        let Some(project) = self.project.clone() else {
            self.last_error =
                Some("Import failed: no project is open. Use File → New Project first.".into());
            return;
        };
        let Some(src) = rfd::FileDialog::new()
            .add_filter("Wavefront OBJ", &["obj"])
            .pick_file()
        else {
            self.status = "Import model cancelled.".into();
            return;
        };
        match importer::model::import(&project, &src) {
            Ok(info) => {
                self.status = format!(
                    "Imported model {} ({} verts, {} mesh)",
                    info.path.display(),
                    info.vertex_count,
                    info.mesh_count
                );
            }
            Err(e) => self.last_error = Some(format!("Model import failed: {e}")),
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| ui::menu_bar::show(ui, self));

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status);
                if let Some(err) = &self.last_error {
                    ui.colored_label(egui::Color32::RED, err);
                }
                if self.project.is_none() {
                    ui.weak(" | No project open");
                }
            });
        });

        egui::SidePanel::left("scene_tree")
            .resizable(true)
            .default_width(260.0)
            .show(ctx, |ui| ui::scene_tree_panel::show(ui, self));

        egui::SidePanel::right("inspector")
            .resizable(true)
            .default_width(320.0)
            .show(ctx, |ui| ui::inspector_panel::show(ui, self));

        egui::TopBottomPanel::bottom("files")
            .resizable(true)
            .default_height(180.0)
            .show(ctx, |ui| ui::file_panel::show(ui, self));

        egui::CentralPanel::default().show(ctx, |ui| ui::viewport_panel::show(ui, self));
    }
}
