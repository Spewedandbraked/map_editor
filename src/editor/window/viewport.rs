use eframe::egui;
use crate::ui::menus::viewport::Viewport3DState;
use std::sync::Arc;

pub fn show_viewport(
    ui: &mut egui::Ui,
    state: &mut Viewport3DState,
    gl: &Arc<glow::Context>,
) {
    state.ui(ui, gl);
}