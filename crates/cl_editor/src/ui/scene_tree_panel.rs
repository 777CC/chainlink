//! Left dock — hierarchical view of the active scene.

use crate::app::EditorApp;
use crate::scene_doc::EditorNode;

/// Operation requested by the tree UI; applied to the scene after the
/// recursive walk so we don't borrow the tree mutably while iterating.
enum TreeAction {
    Select(u64),
    AddChild(u64),
    Delete(u64),
    Rename(u64, String),
}

/// Render the scene tree panel.
pub fn show(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.heading("Scene");
    ui.separator();
    ui.label(format!(
        "Scene: {}{}",
        app.scene.name,
        if app.scene.dirty { " *" } else { "" }
    ));
    ui.separator();

    let mut actions: Vec<TreeAction> = Vec::new();
    let selected_id = app.scene.selected_id;

    egui::ScrollArea::vertical().show(ui, |ui| {
        draw_node(ui, &app.scene.root, selected_id, &mut actions);
    });

    ui.separator();
    ui.horizontal(|ui| {
        if ui.button("+ Add Child").clicked() {
            if let Some(id) = app.scene.selected_id {
                actions.push(TreeAction::AddChild(id));
            }
        }
        if ui.button("Delete").clicked() {
            if let Some(id) = app.scene.selected_id {
                if id != app.scene.root.id {
                    actions.push(TreeAction::Delete(id));
                }
            }
        }
    });

    for action in actions {
        match action {
            TreeAction::Select(id) => app.scene.selected_id = Some(id),
            TreeAction::AddChild(parent_id) => {
                let id = app.scene.next_id();
                let node = crate::node_kinds::make_node("Node", id);
                app.scene.add_child(parent_id, node);
                app.scene.selected_id = Some(id);
            }
            TreeAction::Delete(id) => {
                app.scene.remove(id);
            }
            TreeAction::Rename(id, name) => {
                if let Some(n) = app.scene.find_mut(id) {
                    n.name = name;
                    app.scene.dirty = true;
                }
            }
        }
    }
}

fn draw_node(
    ui: &mut egui::Ui,
    node: &EditorNode,
    selected_id: Option<u64>,
    actions: &mut Vec<TreeAction>,
) {
    let id = node.id;
    let label = format!("{} [{}]", node.name, node.kind);
    let is_selected = selected_id == Some(id);

    let header_id = ui.make_persistent_id(("scene_tree_node", id));
    egui::collapsing_header::CollapsingState::load_with_default_open(
        ui.ctx(),
        header_id,
        true,
    )
    .show_header(ui, |ui| {
        ui.horizontal(|ui| {
            let resp = ui.selectable_label(is_selected, &label);
            if resp.clicked() {
                actions.push(TreeAction::Select(id));
            }
            if ui.small_button("+").on_hover_text("Add child Node").clicked() {
                actions.push(TreeAction::AddChild(id));
            }
            if ui.small_button("x").on_hover_text("Remove").clicked() {
                actions.push(TreeAction::Delete(id));
            }
        });
    })
    .body(|ui| {
        for child in &node.children {
            draw_node(ui, child, selected_id, actions);
        }
        // Inline rename when this node is selected.
        if is_selected {
            ui.horizontal(|ui| {
                ui.label("Rename:");
                let mut buf = node.name.clone();
                let resp = ui.add(egui::TextEdit::singleline(&mut buf));
                if resp.lost_focus() && buf != node.name {
                    actions.push(TreeAction::Rename(id, buf));
                }
            });
        }
    });
}
