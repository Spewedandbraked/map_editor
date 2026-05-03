use egui_dock::DockState;
use eframe::egui;
use super::PanelWindow;
use crate::editor::dock_manager::Tab;
use crate::editor::scene_manager::SceneManager;

pub struct SceneGraphWindow {
    pub open: bool,
}

impl SceneGraphWindow {
    pub fn new() -> Self {
        Self { open: false }
    }
}

impl PanelWindow for SceneGraphWindow {
    fn is_open(&self) -> bool { self.open }

    fn toggle(&mut self, dock_state: &mut DockState<Tab>) {
        self.open = !self.open;
        if self.open {
            dock_state.push_to_focused_leaf(Tab::SceneGraph);
        } else if let Some(path) = dock_state.find_tab(&Tab::SceneGraph) {
            dock_state.remove_tab(path);
        }
    }

    fn on_close(&mut self) {
        self.open = false;
    }
}

pub fn show_scene_graph(ui: &mut egui::Ui, scene_manager: &mut SceneManager) {
    ui.label("Scene Graph");
    let items: Vec<(usize, String)> = {
        let scene_graph = scene_manager.scene_graph();
        scene_graph
            .entities
            .iter()
            .map(|e| (e.id, format!("{} ({})", e.name, e.asset_id)))
            .collect()
    };
    let selected_id = scene_manager.selected_entity_id();
    let mut new_selection = selected_id;
    for (id, label) in &items {
        let response = ui.selectable_label(selected_id == Some(*id), label);
        if response.clicked() {
            new_selection = Some(*id);
        }
    }
    if new_selection != selected_id {
        if let Some(id) = new_selection {
            scene_manager.select_entity(id);
        }
    }
}