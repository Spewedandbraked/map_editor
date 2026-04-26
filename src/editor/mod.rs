use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;

use eframe::egui::{self, Ui};
use eframe::CreationContext;
use egui_dock::tab_viewer::OnCloseResponse;
use egui_dock::{DockArea, DockState, NodeIndex, Style, TabPath};

use crate::asset::registry::AssetRegistry;
use crate::scene::SceneGraph;
use crate::ui::menus::viewport_3d::Viewport3DState;
use crate::ui::functions;

pub enum Command {
    AddViewport,
    NewProject,
    ToggleTools,
}

#[derive(Debug, Clone, PartialEq)]
enum Tab {
    Viewport3D(usize),
    SceneGraph,
    Properties,
    Console,
    Tools,
}

pub struct Editor {
    dock_state: DockState<Tab>,
    command_sender: Sender<Command>,
    command_receiver: Receiver<Command>,
    gl: Option<Arc<glow::Context>>,
    viewports: HashMap<usize, Viewport3DState>,
    next_viewport_id: usize,
    tabs_to_remove: Vec<usize>,
    scene_graph: SceneGraph,
    asset_registry: AssetRegistry,
    tools_open: bool,
    tools_tab_path: Option<TabPath>,
    selected_entity_id: Option<usize>,
}

impl Editor {
    pub fn new(cc: &CreationContext<'_>, asset_registry: AssetRegistry) -> Self {
        let (tx, rx) = mpsc::channel();
        let gl = cc.gl.as_ref().map(|gl| gl.clone());

        let mut dock_state = DockState::new(vec![Tab::SceneGraph]);
        let tree = dock_state.main_surface_mut();
        tree.split_right(NodeIndex::root(), 0.75, vec![Tab::Properties, Tab::Console]);

        Self {
            dock_state,
            command_sender: tx,
            command_receiver: rx,
            gl,
            viewports: HashMap::new(),
            next_viewport_id: 0,
            tabs_to_remove: Vec::new(),
            scene_graph: SceneGraph::new(),
            asset_registry,
            tools_open: false,
            tools_tab_path: None,
            selected_entity_id: None,
        }
    }
}

impl eframe::App for Editor {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        self.process_commands();

        for id in self.tabs_to_remove.drain(..) {
            self.viewports.remove(&id);
        }

        let ctx = ui.ctx().clone();

        egui::Panel::top("menu_bar").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Project").clicked() {
                        ui.close();
                        functions::new_project(&self.command_sender);
                    }
                    if ui.button("Open Project").clicked() {
                        ui.close();
                        functions::open_project();
                    }
                    if ui.button("Save Project").clicked() {
                        ui.close();
                        functions::save_project();
                    }
                    if ui.button("Export Project").clicked() {
                        ui.close();
                        functions::export_project();
                    }
                });
                ui.menu_button("View", |ui| {
                    if ui.button("3D View").clicked() {
                        ui.close();
                        functions::open_3d_view(&self.command_sender);
                    }
                    if ui.button("Tools").clicked() {
                        ui.close();
                        functions::tools_menu(&self.command_sender);
                    }
                });
            });
        });

        let viewports = &mut self.viewports;
        let gl = &self.gl;
        let tabs_to_remove = &mut self.tabs_to_remove;
        let scene_graph = &self.scene_graph;
        let asset_registry = &self.asset_registry;
        let tools_open = &mut self.tools_open;
        let tools_tab_path = &mut self.tools_tab_path;
        let selected_entity_id = &mut self.selected_entity_id;

        let mut tab_viewer = TabViewer {
            gl,
            viewports,
            tabs_to_remove,
            scene_graph,
            asset_registry,
            tools_open,
            tools_tab_path,
            selected_entity_id,
        };

        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ctx.global_style().as_ref()))
            .show_inside(ui, &mut tab_viewer);
    }
}

impl Editor {
    fn process_commands(&mut self) {
        while let Ok(cmd) = self.command_receiver.try_recv() {
            match cmd {
                Command::AddViewport => {
                    let gl = self.gl.as_ref().expect("No GL context").clone();
                    let id = self.next_viewport_id;
                    self.next_viewport_id += 1;
                    self.viewports.insert(id, Viewport3DState::new(&gl));
                    self.dock_state.push_to_focused_leaf(Tab::Viewport3D(id));
                }
                Command::NewProject => {
                    self.scene_graph.clear();
                    self.viewports.clear();
                    self.next_viewport_id = 0;
                    self.selected_entity_id = None;
                    self.dock_state = DockState::new(vec![Tab::SceneGraph]);
                    let tree = self.dock_state.main_surface_mut();
                    tree.split_right(NodeIndex::root(), 0.75, vec![Tab::Properties, Tab::Console]);
                    let gl = self.gl.as_ref().expect("No GL context").clone();
                    let id = self.next_viewport_id;
                    self.next_viewport_id += 1;
                    self.viewports.insert(id, Viewport3DState::new(&gl));
                    self.dock_state.push_to_focused_leaf(Tab::Viewport3D(id));
                    self.scene_graph.add_entity("Default Cube".to_string(), "default_cube".to_string());
                }
                Command::ToggleTools => {
                    self.tools_open = !self.tools_open;
                    if self.tools_open {
                        self.dock_state.push_to_focused_leaf(Tab::Tools);
                        self.tools_tab_path = self.dock_state.find_tab(&Tab::Tools);
                    } else if let Some(path) = self.tools_tab_path.take() {
                        self.dock_state.remove_tab(path);
                    }
                }
            }
        }
    }
}

struct TabViewer<'a> {
    gl: &'a Option<Arc<glow::Context>>,
    viewports: &'a mut HashMap<usize, Viewport3DState>,
    tabs_to_remove: &'a mut Vec<usize>,
    scene_graph: &'a SceneGraph,
    asset_registry: &'a AssetRegistry,
    tools_open: &'a mut bool,
    tools_tab_path: &'a mut Option<TabPath>,
    selected_entity_id: &'a mut Option<usize>,
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::Viewport3D(id) => format!("3D Viewport {}", id).into(),
            Tab::SceneGraph => "Scene Graph".into(),
            Tab::Properties => "Properties".into(),
            Tab::Console => "Console".into(),
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
                for entity in &self.scene_graph.entities {
                    let response = ui.selectable_label(
                        *self.selected_entity_id == Some(entity.id),
                        format!("{} ({})", entity.name, entity.asset_id),
                    );
                    if response.clicked() {
                        *self.selected_entity_id = Some(entity.id);
                    }
                }
            }
            Tab::Properties => {
                if let Some(id) = *self.selected_entity_id {
                    if let Some(entity) = self.scene_graph.get(id) {
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
            Tab::Console => {
                ui.label("Console");
            }
            Tab::Tools => {
                ui.label("Project Asset Storage");
                for entity in &self.scene_graph.entities {
                    let path = self
                        .asset_registry
                        .path(&entity.asset_id)
                        .map(|s| s.as_str())
                        .unwrap_or("N/A");
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
                *self.tools_open = false;
                *self.tools_tab_path = None;
            }
            _ => {}
        }
        OnCloseResponse::Close
    }
}