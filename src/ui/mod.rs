pub mod functions;
pub mod menus;

use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;

use eframe::egui::{self, Ui};
use eframe::CreationContext;
use egui_dock::{DockArea, DockState, NodeIndex, Style};

use crate::ui::menus::viewport_3d::Viewport3DState;

pub enum Command {
    AddViewport,
}

#[derive(Debug, Clone, PartialEq)]
enum Tab {
    Viewport3D(usize),
    SceneGraph,
    Properties,
    Console,
}

pub struct EditorApp {
    dock_state: DockState<Tab>,
    command_sender: Sender<Command>,
    command_receiver: Receiver<Command>,
    gl: Option<Arc<glow::Context>>,
    viewports: HashMap<usize, Viewport3DState>,
    next_viewport_id: usize,
}

impl EditorApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let (tx, rx) = mpsc::channel();
        let gl = cc.gl.as_ref().map(|gl| gl.clone());

        let mut dock_state = DockState::new(vec![Tab::SceneGraph]);
        let tree = dock_state.main_surface_mut();
        tree.split_right(
            NodeIndex::root(),
            0.75,
            vec![Tab::Properties, Tab::Console],
        );

        Self {
            dock_state,
            command_sender: tx,
            command_receiver: rx,
            gl,
            viewports: HashMap::new(),
            next_viewport_id: 0,
        }
    }
}

impl eframe::App for EditorApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        self.process_commands();

        let ctx = ui.ctx().clone();

        egui::Panel::top("menu_bar").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Project").clicked() {
                        ui.close();
                        functions::new_project();
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
                        functions::tools_menu();
                    }
                });
            });
        });

        let viewports = &mut self.viewports;
        let gl = &self.gl;
        let mut tab_viewer = TabViewer { gl, viewports };

        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ctx.global_style().as_ref()))
            .show_inside(ui, &mut tab_viewer);
    }
}

impl EditorApp {
    fn process_commands(&mut self) {
        while let Ok(cmd) = self.command_receiver.try_recv() {
            match cmd {
                Command::AddViewport => {
                    let gl = self.gl.as_ref().expect("No GL context").clone();
                    let id = self.next_viewport_id;
                    self.next_viewport_id += 1;
                    self.viewports.insert(
                        id,
                        Viewport3DState::new(&gl),
                    );
                    self.dock_state
                        .push_to_focused_leaf(Tab::Viewport3D(id));
                }
            }
        }
    }
}

struct TabViewer<'a> {
    gl: &'a Option<Arc<glow::Context>>,
    viewports: &'a mut HashMap<usize, Viewport3DState>,
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::Viewport3D(id) => format!("3D Viewport {}", id).into(),
            Tab::SceneGraph => "Scene Graph".into(),
            Tab::Properties => "Properties".into(),
            Tab::Console => "Console".into(),
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
            }
            Tab::Properties => {
                ui.label("Properties");
            }
            Tab::Console => {
                ui.label("Console");
            }
        }
    }
}