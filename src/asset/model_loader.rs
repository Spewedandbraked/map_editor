use std::path::Path;
use crate::asset::registry::MeshData;

pub fn load_model(path: &Path) -> Option<MeshData> {
    let extension = path.extension()?.to_str()?;
    
    match extension {
        "gltf" | "glb" => load_gltf(path),
        "obj" => load_obj(path),
        _ => {
            println!("Unsupported model format: {}", extension);
            None
        }
    }
}

fn load_gltf(path: &Path) -> Option<MeshData> {
    println!("Loading GLTF: {:?}", path);
    
    let (gltf, buffers, _) = match gltf::import(path) {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to load GLTF: {:?}", e);
            return None;
        }
    };
    
    for mesh in gltf.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            
            let positions = match reader.read_positions() {
                Some(positions) => positions.collect::<Vec<[f32; 3]>>(),
                None => {
                    println!("No positions found in mesh");
                    continue;
                }
            };
            
            let indices = match reader.read_indices() {
                Some(indices) => indices.into_u32().collect::<Vec<u32>>(),
                None => {
                    (0..positions.len() as u32).collect()
                }
            };
            
            let vertices: Vec<f32> = positions.iter().flat_map(|p| p.iter().copied()).collect();
            
            println!("Loaded mesh: {} vertices, {} indices", vertices.len() / 3, indices.len());
            
            return Some(MeshData::new(vertices, indices));
        }
    }
    
    println!("No mesh data found in GLTF file");
    None
}

fn load_obj(path: &Path) -> Option<MeshData> {
    println!("Loading OBJ: {:?}", path);
    
    let (models, _) = match tobj::load_obj(path, &tobj::LoadOptions::default()) {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to load OBJ: {:?}", e);
            return None;
        }
    };
    
    for model in models {
        let mesh = model.mesh;
        
        if mesh.positions.is_empty() {
            continue;
        }
        
        let vertices: Vec<f32> = mesh.positions;
        let indices: Vec<u32> = mesh.indices;
        
        println!("Loaded OBJ mesh: {} vertices, {} indices", vertices.len() / 3, indices.len());
        
        return Some(MeshData::new(vertices, indices));
    }
    
    None
}