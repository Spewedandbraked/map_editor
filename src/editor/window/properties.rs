use egui_dock::DockState;
use eframe::egui;
use super::PanelWindow;
use crate::editor::dock_manager::Tab;
use crate::editor::scene_manager::SceneManager;

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

pub fn show_properties(ui: &mut egui::Ui, scene_manager: &mut SceneManager) {
    if let Some(id) = scene_manager.selected_entity_id() {
        if let Some(entity) = scene_manager.scene_graph_mut().get_mut(id) {
            ui.heading(format!("Entity: {}", entity.name));
            ui.separator();
            
            ui.label("Name:");
            ui.label(&entity.name);
            
            ui.separator();
            
            ui.label("Asset ID:");
            ui.label(&entity.asset_id);
            
            ui.separator();
            
            ui.label("Position:");
            let mut translation = entity.translation;
            
            let mut x = translation.x;
            let mut y = translation.y;
            let mut z = translation.z;
            
            ui.horizontal(|ui| {
                ui.label("X:");
                if ui.add(egui::DragValue::new(&mut x).speed(0.1)).changed() {
                    translation.x = x;
                }
            });
            
            ui.horizontal(|ui| {
                ui.label("Y:");
                if ui.add(egui::DragValue::new(&mut y).speed(0.1)).changed() {
                    translation.y = y;
                }
            });
            
            ui.horizontal(|ui| {
                ui.label("Z:");
                if ui.add(egui::DragValue::new(&mut z).speed(0.1)).changed() {
                    translation.z = z;
                }
            });
            
            if translation != entity.translation {
                entity.translation = translation;
                println!("Entity {} moved to: ({:.2}, {:.2}, {:.2})", id, translation.x, translation.y, translation.z);
            }
            
            ui.separator();
            
            ui.label("Rotation (Euler):");
            let euler = entity.rotation.to_euler(glam::EulerRot::XYZ);
            ui.label(format!("X: {:.2}°, Y: {:.2}°, Z: {:.2}°", 
                euler.0.to_degrees(), 
                euler.1.to_degrees(), 
                euler.2.to_degrees()));
            
            ui.separator();
            
            ui.label("Scale:");
            ui.label(format!("X: {:.2}, Y: {:.2}, Z: {:.2}", 
                entity.scale.x, entity.scale.y, entity.scale.z));
            
        } else {
            ui.label("Selected entity not found");
        }
    } else {
        ui.label("No entity selected. Click on an entity in Scene Graph to edit its properties.");
    }
}