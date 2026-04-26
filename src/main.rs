mod ui;
mod scene;
mod asset;

use asset::registry::AssetRegistry;

fn main() {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Map Editor",
        options,
        Box::new(|cc| {
            let asset_registry = AssetRegistry::new();
            Ok(Box::new(ui::EditorApp::new(cc, asset_registry)))
        }),
    )
    .unwrap();
}