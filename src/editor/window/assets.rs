use egui_dock::DockState;
use eframe::egui;
use rfd::FileDialog;
use super::PanelWindow;
use crate::editor::dock_manager::Tab;
use crate::editor::scene_manager::SceneManager;

// Структура окна
pub struct AssetsWindow {
    pub open: bool,
}

impl AssetsWindow {
    pub fn new() -> Self {
        Self { open: false }
    }
}

impl PanelWindow for AssetsWindow {
    fn is_open(&self) -> bool { self.open }

    fn toggle(&mut self, dock_state: &mut DockState<Tab>) {
        self.open = !self.open;
        if self.open {
            dock_state.push_to_focused_leaf(Tab::Assets);
        } else if let Some(path) = dock_state.find_tab(&Tab::Assets) {
            dock_state.remove_tab(path);
        }
    }

    fn on_close(&mut self) {
        self.open = false;
    }
}

// Функция отображения содержимого
pub fn show_assets(ui: &mut egui::Ui, scene_manager: &mut SceneManager) {
    // Кнопка в стиле меню (как в File)
    egui::Panel::top("assets_menu_bar")
        .frame(egui::Frame::default().inner_margin(egui::Margin {
            bottom: 6,
            ..Default::default()
        }))
        .show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("View", |ui| {
                    if ui.button("Add File from Explorer").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("All files", &["*"])
                            .pick_file()
                        {
                            let asset_id = scene_manager.asset_registry_mut().add_asset(path, None);
                            println!("✅ Asset added: {}", asset_id);
                        }
                        ui.close();
                    }
                });
            })
        });

    // Список ассетов с кнопкой добавления в сцену
    // Сначала собираем ID ассетов, чтобы избежать проблем с заимствованием
    let assets: Vec<(String, String)> = scene_manager
        .asset_registry()
        .all_assets()
        .iter()
        .map(|a| (a.id.clone(), a.name.clone()))
        .collect();

    if assets.is_empty() {
        ui.label("No assets loaded.");
    } else {
        for (asset_id, asset_name) in assets {
            ui.horizontal(|ui| {
                ui.label(format!("{}", asset_name));
                
                // Кнопка для добавления ассета в сцену
                if ui.button("➕ Add to Scene").clicked() {
                    let entity_name = format!("{}_entity", asset_name);
                    let entity_id = scene_manager.scene_graph_mut().add_entity(entity_name, asset_id.clone());
                    println!("✅ Entity added: {} (id: {})", asset_name, entity_id);
                }
            });
        }
    }
}