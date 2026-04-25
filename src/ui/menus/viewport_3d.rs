use std::sync::Arc;

use eframe::egui;
use glam::{Mat4, Quat, Vec3};
use glow::HasContext;

pub struct Viewport3DState {
    gl: Arc<glow::Context>,
    size: [i32; 2],
    camera_rotation: Quat,
    camera_distance: f32,
    camera_target: Vec3,
    last_mouse_pos: Option<egui::Pos2>,
    last_mmb_pos: Option<egui::Pos2>,
    program: glow::Program,
    vao: glow::VertexArray,
    vbo: glow::Buffer,
    vertex_count: i32,
}

impl Viewport3DState {
    pub fn new(gl: &Arc<glow::Context>) -> Self {
        unsafe {
            let program = create_shader_program(gl);
            let (vao, vbo, vertex_count) = create_grid_geometry(gl);

            Self {
                gl: Arc::clone(gl),
                size: [512, 512],
                camera_rotation: Quat::IDENTITY,
                camera_distance: 5.0,
                camera_target: Vec3::ZERO,
                last_mouse_pos: None,
                last_mmb_pos: None,
                program,
                vao,
                vbo,
                vertex_count,
            }
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

        let rotation = self.camera_rotation;
        let distance = self.camera_distance;
        let target = self.camera_target;
        let size = self.size;
        let program = self.program;
        let vao = self.vao;
        let vbo = self.vbo;
        let vertex_count = self.vertex_count;
        let gl = Arc::clone(&self.gl);

        let cb = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                let gl = painter.gl();
                unsafe {
                    gl.enable(glow::DEPTH_TEST);
                    gl.clear_color(0.2, 0.2, 0.2, 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                    gl.use_program(Some(program));

                    let offset = rotation * Vec3::new(0.0, 0.0, distance);
                    let camera_pos = target + offset;

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

        if !mb_middle {
            self.last_mmb_pos = None;
        }

        if let Some(pos) = mouse_pos {
            if let Some(last_pos) = self.last_mmb_pos {
                let delta = pos - last_pos;
                let sensitivity = 0.01;

                if ctrl {
                    self.camera_distance *= 1.0 - delta.y * sensitivity;
                    self.camera_distance = self.camera_distance.max(0.01);
                } else if shift {
                    let right = self.camera_rotation * Vec3::X;
                    let up = self.camera_rotation * Vec3::Y;
                    self.camera_target -= right * delta.x * sensitivity * self.camera_distance;
                    self.camera_target += up * delta.y * sensitivity * self.camera_distance;
                } else {
                    let yaw_rot = Quat::from_rotation_y(delta.x * sensitivity);
                    let pitch_rot = Quat::from_rotation_x(delta.y * sensitivity);
                    self.camera_rotation = (yaw_rot * self.camera_rotation * pitch_rot).normalize();
                }
            }
            if mb_middle {
                self.last_mmb_pos = Some(pos);
            }
        }

        if mb_right {
            if let Some(pos) = mouse_pos {
                if let Some(last_pos) = self.last_mouse_pos {
                    let delta = pos - last_pos;
                    let yaw_rot = Quat::from_rotation_y(delta.x * 0.01);
                    let pitch_rot = Quat::from_rotation_x(delta.y * 0.01);
                    self.camera_rotation = (yaw_rot * self.camera_rotation * pitch_rot).normalize();
                }
                self.last_mouse_pos = Some(pos);
            }
        } else {
            self.last_mouse_pos = None;
        }

        let keys_down = &input.keys_down;

        let forward = self.camera_rotation * Vec3::Z;
        let right = self.camera_rotation * Vec3::X;

        let mut move_dir = Vec3::ZERO;
        if keys_down.contains(&egui::Key::W) {
            move_dir -= forward;
        }
        if keys_down.contains(&egui::Key::S) {
            move_dir += forward;
        }
        if keys_down.contains(&egui::Key::A) {
            move_dir -= right;
        }
        if keys_down.contains(&egui::Key::D) {
            move_dir += right;
        }
        if move_dir.length_squared() > 0.0 {
            move_dir = move_dir.normalize();
            self.camera_target += move_dir * 0.1;
        }

        let scroll = input.smooth_scroll_delta.y;
        if scroll != 0.0 {
            self.camera_distance *= 1.0 - scroll * 0.05;
            self.camera_distance = self.camera_distance.max(0.01);
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

fn create_grid_geometry(gl: &glow::Context) -> (glow::VertexArray, glow::Buffer, i32) {
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
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(&vertices),
            glow::STATIC_DRAW,
        );
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 12, 0);
        gl.enable_vertex_attrib_array(0);
        gl.bind_vertex_array(None);
        (vao, vbo, vertex_count)
    }
}
