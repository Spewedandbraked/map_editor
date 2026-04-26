use std::collections::HashMap;

pub struct AssetRegistry {
    pub assets: HashMap<String, String>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        let mut assets = HashMap::new();
        assets.insert("default_cube".to_string(), "assets/cube.glb".to_string());
        Self { assets }
    }

    pub fn path(&self, id: &str) -> Option<&String> {
        self.assets.get(id)
    }
}