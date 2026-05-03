use eframe::egui;
use crate::ui::menus::viewport::Viewport3DState;
use crate::editor::scene_manager::SceneManager;
use std::sync::Arc;

pub fn show_viewport(
    ui: &mut egui::Ui,
    state: &mut Viewport3DState,
    gl: &Arc<glow::Context>,
    scene_manager: &mut SceneManager,
) {
    state.ui(ui, gl, scene_manager);
}