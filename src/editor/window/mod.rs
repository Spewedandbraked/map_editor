pub mod tools;
pub mod scene_graph;
pub mod properties;
pub mod assets;  // Новый модуль

use egui_dock::DockState;
use crate::editor::dock_manager::Tab;

pub trait PanelWindow {
    fn is_open(&self) -> bool;
    fn toggle(&mut self, dock_state: &mut DockState<Tab>);
    fn on_close(&mut self);
}