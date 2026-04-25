use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use fltk::window::GlutWindow;

pub fn new_project() {
    let mut gl = GlutWindow::new(0, 30, 1280, 690, "");
    gl.end();
    gl.show();
}