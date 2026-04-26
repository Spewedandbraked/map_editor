use glam::{Quat, Vec3};

const ROTATE_SENSITIVITY: f32 = 0.003;
const PAN_SENSITIVITY: f32 = 0.003;
const ZOOM_SENSITIVITY: f32 = 0.01;
const SCROLL_SENSITIVITY: f32 = 0.01;

pub struct Camera {
    pub rotation: Quat,
    pub distance: f32,
    pub target: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            rotation: Quat::IDENTITY,
            distance: 5.0,
            target: Vec3::ZERO,
        }
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::Z
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    pub fn position(&self) -> Vec3 {
        self.target + self.rotation * Vec3::new(0.0, 0.0, self.distance)
    }

    pub fn view_matrix(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(self.position(), self.target, self.up())
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        let yaw = Quat::from_rotation_y(delta_x * ROTATE_SENSITIVITY);
        let pitch = Quat::from_rotation_x(delta_y * ROTATE_SENSITIVITY);
        self.rotation = (yaw * self.rotation * pitch).normalize();
    }

    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        let right = self.right();
        let up = self.up();
        self.target -= right * delta_x * PAN_SENSITIVITY * self.distance;
        self.target += up * delta_y * PAN_SENSITIVITY * self.distance;
    }

    pub fn zoom_distance(&mut self, factor: f32) {
        self.distance *= 1.0 - factor * ZOOM_SENSITIVITY;
        self.distance = self.distance.max(0.01);
    }

    pub fn apply_scroll(&mut self, scroll: f32) {
        self.distance *= 1.0 - scroll * SCROLL_SENSITIVITY;
        self.distance = self.distance.max(0.01);
    }
}