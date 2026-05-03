use crate::editor::dock_manager::Tab;
use crate::editor::scene_manager::SceneManager;
use crate::editor::Command;
use crate::editor::window;
use crate::ui::menus::viewport::Viewport3DState;
use eframe::egui;
use egui_dock::tab_viewer::OnCloseResponse;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::Arc;

pub struct TabViewer<'a> {
    pub gl: &'a Option<Arc<glow::Context>>,
    pub viewports: &'a mut HashMap<usize, Viewport3DState>,
    pub tabs_to_remove: &'a mut Vec<usize>,
    pub scene_manager: &'a mut SceneManager,
    pub command_sender: Sender<Command>,
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::Viewport3D(id) => format!("3D Viewport {}", id).into(),
            Tab::SceneGraph => "Scene Graph".into(),
            Tab::Properties => "Properties".into(),
            Tab::Tools => "Tools".into(),
            Tab::Assets => "Assets".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::Viewport3D(id) => {
                if let Some(state) = self.viewports.get_mut(id) {
                    if let Some(gl) = self.gl.as_ref() {
                        window::show_viewport(ui, state, gl, self.scene_manager);
                    }
                }
            }
            Tab::SceneGraph => {
                window::show_scene_graph(ui, self.scene_manager);
            }
            Tab::Properties => {
                window::show_properties(ui, self.scene_manager);
            }
            Tab::Tools => {
                window::show_tools(ui, self.scene_manager);
            }
            Tab::Assets => {
                window::show_assets(ui, self.scene_manager);
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
            }
            Tab::SceneGraph => {
                let _ = self.command_sender.send(Command::CloseSceneGraph);
            }
            Tab::Properties => {
                let _ = self.command_sender.send(Command::CloseProperties);
            }
            Tab::Assets => {
                let _ = self.command_sender.send(Command::CloseAssets);
            }
        }
        OnCloseResponse::Close
    }
}