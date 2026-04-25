pub mod functions;
pub mod menus;

use fltk::enums::Shortcut;
use fltk::menu::{MenuBar, MenuFlag};
use fltk::prelude::{MenuExt, WidgetBase, WidgetExt};
use fltk::window::Window;

pub fn build_menu(win: &mut Window, mut gl_win: fltk::window::GlWindow) {
    gl_win.hide();

    let mut menubar = MenuBar::new(0, 0, win.w(), 30, "");

    let gl_new = gl_win.clone();
    menubar.add(
        "File/New Project\t",
        Shortcut::None,
        MenuFlag::Normal,
        move |_| functions::new_project(gl_new.clone()),
    );
    menubar.add(
        "File/Open Project\t",
        Shortcut::None,
        MenuFlag::Normal,
        |_| functions::open_project(),
    );
    menubar.add(
        "File/Save Project\t",
        Shortcut::None,
        MenuFlag::Normal,
        |_| functions::save_project(),
    );
    menubar.add(
        "File/Export Project\t",
        Shortcut::None,
        MenuFlag::Normal,
        |_| functions::export_project(),
    );
    menubar.add(
        "File/Quit\t",
        Shortcut::Ctrl | 'q',
        MenuFlag::Normal,
        |_| std::process::exit(0),
    );

    let gl_view = gl_win.clone();
    menubar.add(
        "View/3D View\t",
        Shortcut::None,
        MenuFlag::Normal,
        move |_| functions::open_3d_view(gl_view.clone()),
    );
    menubar.add(
        "View/Tools\t",
        Shortcut::None,
        MenuFlag::Normal,
        |_| menus::tools_menu::show(),
    );

    win.resize_callback(move |w, _, _, w_w, _h| {
        menubar.set_size(w_w, 30);
        w.redraw();
    });
}