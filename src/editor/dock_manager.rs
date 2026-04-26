use egui_dock::{DockState, NodeIndex, TabPath};

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Viewport3D(usize),
    SceneGraph,
    Properties,
    Tools,
}

pub struct DockManager {
    dock_state: DockState<Tab>,
    pub tools_open: bool,
    pub tools_tab_path: Option<TabPath>,
    pub scene_graph_open: bool,
    pub scene_graph_tab_path: Option<TabPath>,
}

impl DockManager {
    pub fn new() -> Self {
        let dock_state = DockState::new(vec![Tab::Properties]);
        Self {
            dock_state,
            tools_open: false,
            tools_tab_path: None,
            scene_graph_open: false,
            scene_graph_tab_path: None,
        }
    }

    pub fn reset(&mut self) {
        self.dock_state = DockState::new(vec![Tab::Properties]);
        self.tools_open = false;
        self.tools_tab_path = None;
        self.scene_graph_open = false;
        self.scene_graph_tab_path = None;
    }

    pub fn add_viewport(&mut self, id: usize) {
        self.dock_state.push_to_focused_leaf(Tab::Viewport3D(id));
    }

    pub fn toggle_tools(&mut self) {
        self.tools_open = !self.tools_open;
        if self.tools_open {
            self.dock_state.push_to_focused_leaf(Tab::Tools);
            self.tools_tab_path = self.dock_state.find_tab(&Tab::Tools);
        } else if let Some(path) = self.tools_tab_path.take() {
            self.dock_state.remove_tab(path);
        }
    }

    pub fn toggle_scene_graph(&mut self) {
        self.scene_graph_open = !self.scene_graph_open;
        if self.scene_graph_open {
            self.dock_state.push_to_focused_leaf(Tab::SceneGraph);
            self.scene_graph_tab_path = self.dock_state.find_tab(&Tab::SceneGraph);
        } else if let Some(path) = self.scene_graph_tab_path.take() {
            self.dock_state.remove_tab(path);
        }
    }

    pub fn remove_tab(&mut self, path: TabPath) {
        self.dock_state.remove_tab(path);
    }

    pub fn dock_state_mut(&mut self) -> &mut DockState<Tab> {
        &mut self.dock_state
    }
}