use std::sync::Arc;

use eframe::egui;
use glam::{Mat4, Quat, Vec3};
use glow::HasContext;

const ROTATE_SENSITIVITY: f32 = 0.003;
const PAN_SENSITIVITY: f32 = 0.003;
const ZOOM_SENSITIVITY: f32 = 0.01;
const SCROLL_SENSITIVITY: f32 = 0.01;
const MOVE_SPEED: f32 = 0.1;

struct Camera {
    rotation: Quat,
    distance: f32,
    target: Vec3,
}

impl Camera {
    fn new() -> Self {
        Self {
            rotation: Quat::IDENTITY,
            distance: 5.0,
            target: Vec3::ZERO,
        }
    }

    fn forward(&self) -> Vec3 {
        self.rotation * Vec3::Z
    }

    fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        let yaw = Quat::from_rotation_y(delta_x * ROTATE_SENSITIVITY);
        let pitch = Quat::from_rotation_x(delta_y * ROTATE_SENSITIVITY);
        self.rotation = (yaw * self.rotation * pitch).normalize();
    }

    fn pan(&mut self, delta_x: f32, delta_y: f32) {
        let right = self.right();
        let up = self.up();
        self.target -= right * delta_x * PAN_SENSITIVITY * self.distance;
        self.target += up * delta_y * PAN_SENSITIVITY * self.distance;
    }

    fn zoom_distance(&mut self, factor: f32) {
        self.distance *= 1.0 - factor * ZOOM_SENSITIVITY;
        self.distance = self.distance.max(0.01);
    }

    fn apply_scroll(&mut self, scroll: f32) {
        self.distance *= 1.0 - scroll * SCROLL_SENSITIVITY;
        self.distance = self.distance.max(0.01);
    }
}

pub struct Viewport3DState {
    camera: Camera,
    program: glow::Program,
    vao: glow::VertexArray,
    vertex_count: i32,
    size: [i32; 2],
    last_mouse_pos: Option<egui::Pos2>,
    last_mmb_pos: Option<egui::Pos2>,
}

impl Viewport3DState {
    pub fn new(gl: &Arc<glow::Context>) -> Self {
        let program = create_shader_program(gl);
        let (vao, vertex_count) = create_grid_geometry(gl);

        Self {
            camera: Camera::new(),
            program,
            vao,
            vertex_count,
            size: [512, 512],
            last_mouse_pos: None,
            last_mmb_pos: None,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, _gl: &glow::Context) {
        let available_size = ui.available_size();
        let width = available_size.x.round() as i32;
        let height = available_size.y.round() as i32;
        if width != self.size[0] || height != self.size[1] {
            self.size = [width.max(1), height.max(1)];
        }

        self.handle_input(ui);

        let (rect, _response) = ui.allocate_exact_size(
            egui::vec2(width as f32, height as f32),
            egui::Sense::click_and_drag(),
        );

        let program = self.program;
        let vao = self.vao;
        let vertex_count = self.vertex_count;
        let rotation = self.camera.rotation;
        let distance = self.camera.distance;
        let target = self.camera.target;
        let size = self.size;

        let cb = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                let gl = painter.gl();
                unsafe {
                    gl.enable(glow::DEPTH_TEST);
                    gl.clear_color(0.2, 0.2, 0.2, 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                    gl.use_program(Some(program));

                    let camera_pos = target + rotation * Vec3::new(0.0, 0.0, distance);
                    let up = rotation * Vec3::Y;
                    let view = Mat4::look_at_rh(camera_pos, target, up);
                    let proj = Mat4::perspective_rh(
                        60.0f32.to_radians(),
                        size[0] as f32 / size[1] as f32,
                        0.1,
                        100.0,
                    );
                    let mvp = proj * view;

                    gl.uniform_matrix_4_f32_slice(
                        gl.get_uniform_location(program, "u_mvp").as_ref(),
                        false,
                        &mvp.to_cols_array(),
                    );

                    gl.bind_vertex_array(Some(vao));
                    gl.draw_arrays(glow::LINES, 0, vertex_count);

                    gl.disable(glow::DEPTH_TEST);
                }
            })),
        };

        ui.painter().add(cb);
        ui.ctx().request_repaint();
    }

    fn handle_input(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx();
        let input = ctx.input(|i| i.clone());
        let rect = ui.available_rect_before_wrap();

        let mouse_pos = input.pointer.hover_pos();
        let mouse_over = mouse_pos.map_or(false, |pos| rect.contains(pos));

        if !mouse_over {
            self.last_mouse_pos = None;
            self.last_mmb_pos = None;
            return;
        }

        let mb_middle = input.pointer.button_down(egui::PointerButton::Middle);
        let mb_right = input.pointer.button_down(egui::PointerButton::Secondary);
        let shift = input.modifiers.shift;
        let ctrl = input.modifiers.ctrl;

        let rotate = mb_right || (mb_middle && !shift && !ctrl);

        if rotate {
            if let Some(pos) = mouse_pos {
                if let Some(last_pos) = self.last_mouse_pos {
                    let delta = pos - last_pos;
                    self.camera.rotate(delta.x, delta.y);
                }
                self.last_mouse_pos = Some(pos);
            }
        } else {
            self.last_mouse_pos = None;
        }

        if !mb_middle {
            self.last_mmb_pos = None;
        }

        if mb_middle && (shift || ctrl) {
            if let Some(pos) = mouse_pos {
                if let Some(last_pos) = self.last_mmb_pos {
                    let delta = pos - last_pos;
                    if ctrl {
                        self.camera.zoom_distance(delta.y);
                    } else if shift {
                        self.camera.pan(delta.x, delta.y);
                    }
                }
                self.last_mmb_pos = Some(pos);
            }
        }

        let keys_down = &input.keys_down;
        let forward = self.camera.forward();
        let right = self.camera.right();

        let mut move_dir = Vec3::ZERO;
        if keys_down.contains(&egui::Key::W) { move_dir -= forward; }
        if keys_down.contains(&egui::Key::S) { move_dir += forward; }
        if keys_down.contains(&egui::Key::A) { move_dir -= right; }
        if keys_down.contains(&egui::Key::D) { move_dir += right; }
        if move_dir.length_squared() > 0.0 {
            move_dir = move_dir.normalize();
            self.camera.target += move_dir * MOVE_SPEED;
        }

        let scroll = input.smooth_scroll_delta.y;
        if scroll != 0.0 {
            self.camera.apply_scroll(scroll);
        }
    }
}

fn create_shader_program(gl: &glow::Context) -> glow::Program {
    let vertex_source = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        uniform mat4 u_mvp;
        void main() {
            gl_Position = u_mvp * vec4(aPos, 1.0);
        }
    "#;
    let fragment_source = r#"
        #version 330 core
        out vec4 FragColor;
        void main() {
            FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;
    unsafe {
        let vert = gl.create_shader(glow::VERTEX_SHADER).unwrap();
        gl.shader_source(vert, vertex_source);
        gl.compile_shader(vert);
        let frag = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
        gl.shader_source(frag, fragment_source);
        gl.compile_shader(frag);
        let program = gl.create_program().unwrap();
        gl.attach_shader(program, vert);
        gl.attach_shader(program, frag);
        gl.link_program(program);
        gl.delete_shader(vert);
        gl.delete_shader(frag);
        program
    }
}

fn create_grid_geometry(gl: &glow::Context) -> (glow::VertexArray, i32) {
    let mut vertices: Vec<f32> = Vec::new();
    let grid_size = 10;
    for i in -grid_size..=grid_size {
        let i = i as f32;
        vertices.extend_from_slice(&[i, 0.0, -grid_size as f32, i, 0.0, grid_size as f32]);
        vertices.extend_from_slice(&[-grid_size as f32, 0.0, i, grid_size as f32, 0.0, i]);
    }
    let vertex_count = (vertices.len() / 3) as i32;
    unsafe {
        let vao = gl.create_vertex_array().unwrap();
        let vbo = gl.create_buffer().unwrap();
        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&vertices), glow::STATIC_DRAW);
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 12, 0);
        gl.enable_vertex_attrib_array(0);
        gl.bind_vertex_array(None);
        (vao, vertex_count)
    }
}