use egui_dock::DockState;
use super::PanelWindow;
use crate::editor::dock_manager::Tab;

pub struct AssetsWindow {
    pub open: bool,
}

impl AssetsWindow {
    pub fn new() -> Self {
        Self { open: false }
    }
}

impl PanelWindow for AssetsWindow {
    fn is_open(&self) -> bool { self.open }

    fn toggle(&mut self, dock_state: &mut DockState<Tab>) {
        self.open = !self.open;
        if self.open {
            dock_state.push_to_focused_leaf(Tab::Assets);
        } else if let Some(path) = dock_state.find_tab(&Tab::Assets) {
            dock_state.remove_tab(path);
        }
    }

    fn on_close(&mut self) {
        self.open = false;
    }
}