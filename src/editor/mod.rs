pub mod dock_manager;
pub mod scene_manager;
pub mod tab_viewer;

use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use eframe::egui::{self, Ui};
use eframe::CreationContext;
use egui_dock::{DockArea, Style};
use crate::asset::registry::AssetRegistry;
use crate::editor::dock_manager::DockManager;
use crate::editor::scene_manager::SceneManager;
use crate::editor::tab_viewer::TabViewer;
use crate::ui::menus::viewport_3d::Viewport3DState;
use crate::ui::functions;

pub enum Command {
    AddViewport,
    NewProject,
    ToggleTools,
}

pub struct Editor {
    command_sender: Sender<Command>,
    command_receiver: Receiver<Command>,
    gl: Option<Arc<glow::Context>>,
    viewports: HashMap<usize, Viewport3DState>,
    next_viewport_id: usize,
    tabs_to_remove: Vec<usize>,
    dock_manager: DockManager,
    scene_manager: SceneManager,
}

impl Editor {
    pub fn new(cc: &CreationContext<'_>, asset_registry: AssetRegistry) -> Self {
        let (tx, rx) = mpsc::channel();
        let gl = cc.gl.as_ref().map(|gl| gl.clone());

        Self {
            command_sender: tx,
            command_receiver: rx,
            gl,
            viewports: HashMap::new(),
            next_viewport_id: 0,
            tabs_to_remove: Vec::new(),
            dock_manager: DockManager::new(),
            scene_manager: SceneManager::new(asset_registry),
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

        // Копируем флаги, чтобы избежать конфликта с dock_state_mut()
        let mut tools_open = self.dock_manager.tools_open;
        let mut tools_tab_path = self.dock_manager.tools_tab_path.take();

        let dock_state = self.dock_manager.dock_state_mut();
        let mut tab_viewer = TabViewer {
            gl: &self.gl,
            viewports: &mut self.viewports,
            tabs_to_remove: &mut self.tabs_to_remove,
            scene_manager: &mut self.scene_manager,
            tools_open: &mut tools_open,
            tools_tab_path: &mut tools_tab_path,
        };

        DockArea::new(dock_state)
            .style(Style::from_egui(ctx.global_style().as_ref()))
            .show_inside(ui, &mut tab_viewer);

        // Синхронизируем флаги обратно
        self.dock_manager.tools_open = tools_open;
        if tools_open {
            self.dock_manager.tools_tab_path = tools_tab_path;
        }
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
                    self.dock_manager.add_viewport(id);
                }
                Command::NewProject => {
                    self.viewports.clear();
                    self.next_viewport_id = 0;
                    self.dock_manager.reset();
                    self.scene_manager.reset();
                    let gl = self.gl.as_ref().expect("No GL context").clone();
                    let id = self.next_viewport_id;
                    self.next_viewport_id += 1;
                    self.viewports.insert(id, Viewport3DState::new(&gl));
                    self.dock_manager.add_viewport(id);
                }
                Command::ToggleTools => {
                    self.dock_manager.toggle_tools();
                }
            }
        }
    }
}