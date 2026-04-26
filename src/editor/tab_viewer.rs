use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use eframe::egui;
use egui_dock::tab_viewer::OnCloseResponse;
use crate::editor::dock_manager::Tab;
use crate::editor::Command;
use crate::editor::scene_manager::SceneManager;
use crate::ui::menus::viewport::Viewport3DState;

pub struct TabViewer<'a> {
    pub gl: &'a Option<Arc<glow::Context>>,
    pub viewports: &'a mut HashMap<usize, Viewport3DState>,
    pub tabs_to_remove: &'a mut Vec<usize>,
    pub scene_manager: &'a mut SceneManager,
    pub command_sender: Sender<Command>,
    pub tools_open: bool,
    pub scene_graph_open: bool,
    pub properties_open: bool,
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::Viewport3D(id) => format!("3D Viewport {}", id).into(),
            Tab::SceneGraph => "Scene Graph".into(),
            Tab::Properties => "Properties".into(),
            Tab::Tools => "Tools".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::Viewport3D(id) => {
                if let Some(state) = self.viewports.get_mut(id) {
                    if let Some(gl) = self.gl.as_ref() {
                        state.ui(ui, gl);
                    }
                }
            }
            Tab::SceneGraph => {
                ui.label("Scene Graph");
                let items: Vec<(usize, String)> = {
                    let scene_graph = self.scene_manager.scene_graph();
                    scene_graph.entities.iter().map(|e| (e.id, format!("{} ({})", e.name, e.asset_id))).collect()
                };
                let selected_id = self.scene_manager.selected_entity_id();
                let mut new_selection = selected_id;
                for (id, label) in &items {
                    let response = ui.selectable_label(selected_id == Some(*id), label);
                    if response.clicked() {
                        new_selection = Some(*id);
                    }
                }
                if new_selection != selected_id {
                    self.scene_manager.select_entity(new_selection.unwrap());
                }
            }
            Tab::Properties => {
                let scene_graph = self.scene_manager.scene_graph();
                if let Some(id) = self.scene_manager.selected_entity_id() {
                    if let Some(entity) = scene_graph.get(id) {
                        ui.label(format!("Name: {}", entity.name));
                        ui.label(format!("Asset: {}", entity.asset_id));
                        ui.label(format!(
                            "Position: ({:.2}, {:.2}, {:.2})",
                            entity.translation.x, entity.translation.y, entity.translation.z
                        ));
                    } else {
                        ui.label("Selected entity not found");
                    }
                } else {
                    ui.label("No entity selected");
                }
            }
            Tab::Tools => {
                ui.label("Project Asset Storage");
                let asset_registry = self.scene_manager.asset_registry();
                let scene_graph = self.scene_manager.scene_graph();
                for entity in &scene_graph.entities {
                    let path = asset_registry.path(&entity.asset_id).map(|s| s.as_str()).unwrap_or("N/A");
                    ui.label(format!("{} -> {}", entity.name, path));
                }
            }
        }
    }

    fn on_close(&mut self, tab: &mut Self::Tab) -> OnCloseResponse {
        match tab {
            Tab::Viewport3D(id) => {
                self.tabs_to_remove.push(*id);
            }
            Tab::Tools => {
                let _ = self.command_sender.send(Command::CloseTools);
                self.tools_open = false;
            }
            Tab::SceneGraph => {
                let _ = self.command_sender.send(Command::CloseSceneGraph);
                self.scene_graph_open = false;
            }
            Tab::Properties => {
                let _ = self.command_sender.send(Command::CloseProperties);
                self.properties_open = false;
            }
        }
        OnCloseResponse::Close
    }
}