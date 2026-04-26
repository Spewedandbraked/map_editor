use egui_dock::{DockState, NodeIndex, TabPath};

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Viewport3D(usize),
    SceneGraph,
    Properties,
    Console,
    Tools,
}

pub struct DockManager {
    dock_state: DockState<Tab>,
    pub tools_open: bool,
    pub tools_tab_path: Option<TabPath>,
}

impl DockManager {
    pub fn new() -> Self {
        let mut dock_state = DockState::new(vec![Tab::SceneGraph]);
        let tree = dock_state.main_surface_mut();
        tree.split_right(NodeIndex::root(), 0.75, vec![Tab::Properties, Tab::Console]);

        Self {
            dock_state,
            tools_open: false,
            tools_tab_path: None,
        }
    }

    pub fn reset(&mut self) {
        self.dock_state = DockState::new(vec![Tab::SceneGraph]);
        let tree = self.dock_state.main_surface_mut();
        tree.split_right(NodeIndex::root(), 0.75, vec![Tab::Properties, Tab::Console]);
        self.tools_open = false;
        self.tools_tab_path = None;
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

    // pub fn remove_tab(&mut self, path: TabPath) {
    //     self.dock_state.remove_tab(path);
    // }

    pub fn dock_state_mut(&mut self) -> &mut DockState<Tab> {
        &mut self.dock_state
    }
}