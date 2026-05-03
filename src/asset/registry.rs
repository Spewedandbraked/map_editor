use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AssetInfo {
    #[allow(dead_code)]
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub asset_type: AssetType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssetType {
    Model,
    Texture,
    Sound,
    Other,
}

#[derive(Debug, Clone)]
pub struct MeshData {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub vertex_count: i32,
    pub index_count: i32,
}

impl MeshData {
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>) -> Self {
        let vertex_count = (vertices.len() / 3) as i32;
        let index_count = indices.len() as i32;
        Self {
            vertices,
            indices,
            vertex_count,
            index_count,
        }
    }
}

pub struct AssetRegistry {
    assets: HashMap<String, AssetInfo>,
    meshes: HashMap<String, Arc<MeshData>>,
    next_id: usize,
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            meshes: HashMap::new(),
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
        
        let asset_type = Self::detect_asset_type(&path);
        
        self.assets.insert(id.clone(), AssetInfo {
            id: id.clone(),
            name,
            path,
            asset_type,
        });
        
        id
    }

    pub fn add_mesh(&mut self, asset_id: &str, mesh_data: MeshData) {
        self.meshes.insert(asset_id.to_string(), Arc::new(mesh_data));
    }

    pub fn get_mesh(&self, asset_id: &str) -> Option<Arc<MeshData>> {
        self.meshes.get(asset_id).cloned()
    }

    #[allow(dead_code)]
    pub fn get(&self, id: &str) -> Option<&AssetInfo> {
        self.assets.get(id)
    }

    pub fn path(&self, id: &str) -> Option<&PathBuf> {
        self.assets.get(id).map(|a| &a.path)
    }

    pub fn all_assets(&self) -> Vec<&AssetInfo> {
        self.assets.values().collect()
    }

    pub fn len(&self) -> usize {
        self.assets.len()
    }

    fn detect_asset_type(path: &PathBuf) -> AssetType {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("gltf") | Some("glb") | Some("obj") | Some("fbx") => AssetType::Model,
            Some("png") | Some("jpg") | Some("jpeg") | Some("tga") | Some("bmp") => AssetType::Texture,
            Some("wav") | Some("mp3") | Some("ogg") => AssetType::Sound,
            _ => AssetType::Other,
        }
    }
}