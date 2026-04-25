use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use fltk::window::GlutWindow;

pub fn open_3d_view() {
    let mut gl = GlutWindow::new(0, 30, 1280, 690, "");
    gl.end();
    gl.show();
}