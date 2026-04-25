mod ui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Map Editor",
        options,
        Box::new(|_cc| Ok(Box::new(ui::EditorApp::new()))),
    )
    .unwrap();
}