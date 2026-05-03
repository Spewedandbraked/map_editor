use glam::{Quat, Vec3, Mat4};

#[derive(Debug, Clone)]
pub struct SceneEntity {
    pub id: usize,
    pub name: String,
    pub asset_id: String,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl SceneEntity {
    #[allow(dead_code)]
    pub fn model_matrix(&self) -> Mat4 {
        Mat4::from_translation(self.translation)
            * Mat4::from_quat(self.rotation)
            * Mat4::from_scale(self.scale)
    }
}

pub struct SceneGraph {
    pub entities: Vec<SceneEntity>,
    next_id: usize,
}

impl SceneGraph {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            next_id: 0,
        }
    }

    pub fn add_entity(&mut self, name: String, asset_id: String) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.entities.push(SceneEntity {
            id,
            name,
            asset_id,
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        });
        id
    }

    pub fn get(&self, id: usize) -> Option<&SceneEntity> {
        self.entities.iter().find(|e| e.id == id)
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, id: usize) -> Option<&mut SceneEntity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }

    pub fn clear(&mut self) {
        self.entities.clear();
    }
}