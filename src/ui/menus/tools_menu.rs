use fltk::{prelude::*, window::Window, group::Flex, button::Button, enums::Align};
use crate::ui::menus::instruments;

pub fn show() {
    let mut win = Window::new(200, 100, 300, 400, "Tools");
    let mut col = Flex::new(0, 0, 300, 400, "");
    col.set_type(fltk::group::FlexType::Column);

    let mut add_cube_btn = Button::new(0, 0, 0, 0, "Add Cube");
    add_cube_btn.set_callback(|_| instruments::add_cube::execute());
    add_cube_btn.set_align(Align::Inside);

    col.end();
    win.end();
    win.show();

    while win.shown() {
        fltk::app::wait();
    }
}