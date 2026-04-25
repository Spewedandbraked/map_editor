pub mod functions;

use eframe::egui::{self, Ui};
use egui_dock::{DockArea, DockState, NodeIndex, Style};

pub struct EditorApp {
    dock_state: DockState<Tab>,
}

#[derive(Debug, Clone, PartialEq)]
enum Tab {
    Viewport3D,
    SceneGraph,
    Properties,
    Console,
}

impl EditorApp {
    pub fn new() -> Self {
        let mut dock_state = DockState::new(vec![
            Tab::Viewport3D,
            Tab::SceneGraph,
        ]);

        let tree = dock_state.main_surface_mut();
        tree.split_right(
            NodeIndex::root(),
            0.75,
            vec![Tab::Properties, Tab::Console],
        );

        Self { dock_state }
    }
}

impl eframe::App for EditorApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
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
                        functions::open_3d_view();
                    }
                    if ui.button("Tools").clicked() {
                        ui.close();
                        functions::tools_menu();
                    }
                });
            });
        });

        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ctx.global_style().as_ref()))
            .show_inside(ui, &mut TabViewer {});
    }
}

struct TabViewer;

impl egui_dock::TabViewer for TabViewer {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::Viewport3D => "3D Viewport".into(),
            Tab::SceneGraph => "Scene Graph".into(),
            Tab::Properties => "Properties".into(),
            Tab::Console => "Console".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::Viewport3D => {
                ui.label("3D Viewport - will be rendered here");
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