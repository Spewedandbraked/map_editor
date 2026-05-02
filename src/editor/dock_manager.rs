use egui_dock::DockState;
use crate::editor::window::{PanelWindow, tools::ToolsWindow, scene_graph::SceneGraphWindow, properties::PropertiesWindow, assets::AssetsWindow};

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Viewport3D(usize),
    SceneGraph,
    Properties,
    Tools,
    Assets,  // Новая вкладка для ассетов
}

pub struct DockManager {
    dock_state: DockState<Tab>,
    pub tools: ToolsWindow,
    pub scene_graph: SceneGraphWindow,
    pub properties: PropertiesWindow,
    pub assets: AssetsWindow,  // Новое окно
}

impl DockManager {
    pub fn new() -> Self {
        Self {
            dock_state: DockState::new(vec![Tab::Properties]),
            tools: ToolsWindow::new(),
            scene_graph: SceneGraphWindow::new(),
            properties: PropertiesWindow::new(),
            assets: AssetsWindow::new(),  // Создаём новое окно
        }
    }

    pub fn reset(&mut self) {
        self.dock_state = DockState::new(vec![Tab::Properties]);
        self.tools = ToolsWindow::new();
        self.scene_graph = SceneGraphWindow::new();
        self.properties = PropertiesWindow::new();
        self.assets = AssetsWindow::new();
    }

    pub fn add_viewport(&mut self, id: usize) {
        self.dock_state.push_to_focused_leaf(Tab::Viewport3D(id));
    }

    pub fn toggle_tools(&mut self) {
        self.tools.toggle(&mut self.dock_state);
    }

    pub fn toggle_scene_graph(&mut self) {
        self.scene_graph.toggle(&mut self.dock_state);
    }

    pub fn toggle_properties(&mut self) {
        self.properties.toggle(&mut self.dock_state);
    }

    pub fn toggle_assets(&mut self) {  // Новый метод
        self.assets.toggle(&mut self.dock_state);
    }

    pub fn dock_state_mut(&mut self) -> &mut DockState<Tab> {
        &mut self.dock_state
    }

    pub fn on_close_tools(&mut self) {
        self.tools.on_close();
    }

    pub fn on_close_scene_graph(&mut self) {
        self.scene_graph.on_close();
    }

    pub fn on_close_properties(&mut self) {
        self.properties.on_close();
    }

    pub fn on_close_assets(&mut self) {  // Новый метод
        self.assets.on_close();
    }
}