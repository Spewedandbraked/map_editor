pub mod tools;
pub mod scene_graph;
pub mod properties;
pub mod assets;
pub mod viewport;

pub use tools::show_tools;
pub use scene_graph::show_scene_graph;
pub use properties::show_properties;
pub use assets::show_assets;
pub use viewport::show_viewport;

pub use tools::ToolsWindow;
pub use scene_graph::SceneGraphWindow;
pub use properties::PropertiesWindow;
pub use assets::AssetsWindow;

use egui_dock::DockState;
use crate::editor::dock_manager::Tab;

pub trait PanelWindow {
    fn is_open(&self) -> bool;
    fn toggle(&mut self, dock_state: &mut DockState<Tab>);
    fn on_close(&mut self);
}