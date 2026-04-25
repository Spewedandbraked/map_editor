mod ui;

fn main() {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Map Editor",
        options,
        Box::new(|cc| Ok(Box::new(ui::EditorApp::new(cc)))),
    )
    .unwrap();
}