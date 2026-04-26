pub mod camera;
pub mod renderer;

use std::sync::Arc;
use eframe::egui;
use glam::Vec3;

use self::camera::Camera;
use self::renderer::Renderer;
use glow::HasContext;

const MOVE_SPEED: f32 = 0.1;

pub struct Viewport3DState {
    camera: Camera,
    renderer: Renderer,
    size: [i32; 2],
    last_mouse_pos: Option<egui::Pos2>,
    last_mmb_pos: Option<egui::Pos2>,
}

impl Viewport3DState {
    pub fn new(gl: &Arc<glow::Context>) -> Self {
        Self {
            camera: Camera::new(),
            renderer: Renderer::new(gl),
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

        let program = self.renderer.program;
        let vao = self.renderer.vao;
        let vertex_count = self.renderer.vertex_count;
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
                    let view = glam::Mat4::look_at_rh(camera_pos, target, up);
                    let proj = glam::Mat4::perspective_rh(
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