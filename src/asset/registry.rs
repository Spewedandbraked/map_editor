use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AssetInfo {
    #[allow(dead_code)]
    pub id: String,
    pub name: String,
    pub path: PathBuf,
}

pub struct AssetRegistry {
    assets: HashMap<String, AssetInfo>,
    next_id: usize,
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_asset(&mut self, path: PathBuf, custom_name: Option<String>) -> String {
        let id = format!("asset_{}", self.next_id);
        self.next_id += 1;
        
        let name = custom_name.unwrap_or_else(|| {
            path.file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        });
        
        self.assets.insert(id.clone(), AssetInfo {
            id: id.clone(),
            name,
            path,
        });
        
        id
    }

    #[allow(dead_code)]
    pub fn get(&self, id: &str) -> Option<&AssetInfo> {
        self.assets.get(id)
    }

    pub fn path(&self, id: &str) -> Option<&PathBuf> {
        self.assets.get(id).map(|a| &a.path)
    }

    #[allow(dead_code)]
    pub fn name(&self, id: &str) -> Option<&String> {
        self.assets.get(id).map(|a| &a.name)
    }

    pub fn all_assets(&self) -> Vec<&AssetInfo> {
        self.assets.values().collect()
    }

    pub fn len(&self) -> usize {
        self.assets.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.assets.is_empty()
    }

    #[allow(dead_code)]
    pub fn remove_asset(&mut self, id: &str) -> Option<AssetInfo> {
        self.assets.remove(id)
    }
}