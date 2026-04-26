use egui_dock::DockState;
use super::PanelWindow;
use crate::editor::dock_manager::Tab;

pub struct SceneGraphWindow {
    pub open: bool,
}

impl SceneGraphWindow {
    pub fn new() -> Self {
        Self { open: false }
    }
}

impl PanelWindow for SceneGraphWindow {
    fn is_open(&self) -> bool { self.open }
    fn set_open(&mut self, open: bool) { self.open = open; }

    fn toggle(&mut self, dock_state: &mut DockState<Tab>) {
        self.open = !self.open;
        if self.open {
            dock_state.push_to_focused_leaf(Tab::SceneGraph);
        } else if let Some(path) = dock_state.find_tab(&Tab::SceneGraph) {
            dock_state.remove_tab(path);
        }
    }

    fn on_close(&mut self) {
        self.open = false;
    }
}