use egui_dock::DockState;
use eframe::egui;
use super::PanelWindow;
use crate::editor::dock_manager::Tab;
use crate::editor::scene_manager::SceneManager;

// Структура окна
pub struct PropertiesWindow {
    pub open: bool,
}

impl PropertiesWindow {
    pub fn new() -> Self {
        Self { open: true }
    }
}

impl PanelWindow for PropertiesWindow {
    fn is_open(&self) -> bool { self.open }

    fn toggle(&mut self, dock_state: &mut DockState<Tab>) {
        self.open = !self.open;
        if self.open {
            dock_state.push_to_focused_leaf(Tab::Properties);
        } else if let Some(path) = dock_state.find_tab(&Tab::Properties) {
            dock_state.remove_tab(path);
        }
    }

    fn on_close(&mut self) {
        self.open = false;
    }
}

// Функция отображения содержимого
pub fn show_properties(ui: &mut egui::Ui, scene_manager: &SceneManager) {
    let scene_graph = scene_manager.scene_graph();
    if let Some(id) = scene_manager.selected_entity_id() {
        if let Some(entity) = scene_graph.get(id) {
            ui.label(format!("Name: {}", entity.name));
            ui.label(format!("Asset: {}", entity.asset_id));
            ui.label(format!(
                "Position: ({:.2}, {:.2}, {:.2})",
                entity.translation.x, entity.translation.y, entity.translation.z
            ));
            ui.label(format!(
                "Rotation: ({:.2}, {:.2}, {:.2})",
                entity.rotation.to_euler(glam::EulerRot::XYZ).0,
                entity.rotation.to_euler(glam::EulerRot::XYZ).1,
                entity.rotation.to_euler(glam::EulerRot::XYZ).2
            ));
            ui.label(format!(
                "Scale: ({:.2}, {:.2}, {:.2})",
                entity.scale.x, entity.scale.y, entity.scale.z
            ));
        } else {
            ui.label("Selected entity not found");
        }
    } else {
        ui.label("No entity selected");
    }
}