mod asset;
mod editor;
mod scene;
mod ui;

use editor::Editor;
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
            Ok(Box::new(Editor::new(cc, asset_registry)))
        }),
    )
    .unwrap();
}