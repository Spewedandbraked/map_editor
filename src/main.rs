mod ui;

use fltk::{app, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut win = Window::default()
        .with_size(1280, 720)
        .with_label("Map Editor");
    win.make_resizable(true);
    win.size_range(600, 400, 0, 0);
    ui::build_menu(&mut win);
    win.end();
    win.show();
    app.run().unwrap();
}