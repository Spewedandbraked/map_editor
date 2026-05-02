use crate::scene::SceneGraph;
use crate::asset::registry::AssetRegistry;

pub struct SceneManager {
    scene_graph: SceneGraph,
    asset_registry: AssetRegistry,
    selected_entity_id: Option<usize>,
}

impl SceneManager {
    pub fn new(asset_registry: AssetRegistry) -> Self {
        let mut manager = Self {
            scene_graph: SceneGraph::new(),
            asset_registry,
            selected_entity_id: None,
        };
        manager.reset();
        manager
    }

    pub fn reset(&mut self) {
        self.scene_graph.clear();
        self.selected_entity_id = None;
        self.scene_graph.add_entity("Default Cube".to_string(), "default_cube".to_string());
    }

    pub fn scene_graph(&self) -> &SceneGraph {
        &self.scene_graph
    }

    pub fn asset_registry(&self) -> &AssetRegistry {
        &self.asset_registry
    }

    pub fn asset_registry_mut(&mut self) -> &mut AssetRegistry {
        &mut self.asset_registry
    }

    pub fn selected_entity_id(&self) -> Option<usize> {
        self.selected_entity_id
    }

    pub fn select_entity(&mut self, id: usize) {
        self.selected_entity_id = Some(id);
    }
}