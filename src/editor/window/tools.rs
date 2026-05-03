use egui_dock::DockState;
use eframe::egui;
use super::PanelWindow;
use crate::editor::dock_manager::Tab;
use crate::editor::scene_manager::SceneManager;

pub struct ToolsWindow {
    pub open: bool,
}

impl ToolsWindow {
    pub fn new() -> Self {
        Self { open: false }
    }
}

impl PanelWindow for ToolsWindow {
    fn is_open(&self) -> bool { self.open }

    fn toggle(&mut self, dock_state: &mut DockState<Tab>) {
        self.open = !self.open;
        if self.open {
            dock_state.push_to_focused_leaf(Tab::Tools);
        } else if let Some(path) = dock_state.find_tab(&Tab::Tools) {
            dock_state.remove_tab(path);
        }
    }

    fn on_close(&mut self) {
        self.open = false;
    }
}

pub fn show_tools(ui: &mut egui::Ui, scene_manager: &SceneManager) {
    ui.label("Project Asset Storage");
    let asset_registry = scene_manager.asset_registry();
    let scene_graph = scene_manager.scene_graph();
    for entity in &scene_graph.entities {
        let path = asset_registry
            .path(&entity.asset_id)
            .map(|p| p.display().to_string())
            .unwrap_or("N/A".to_string());
        ui.label(format!("{} -> {}", entity.name, path));
    }
}