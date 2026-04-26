use glow::HasContext;

pub struct Renderer {
    pub program: glow::Program,
    pub vao: glow::VertexArray,
    pub vertex_count: i32,
}

impl Renderer {
    pub fn new(gl: &glow::Context) -> Self {
        let program = create_shader_program(gl);
        let (vao, vertex_count) = create_grid_geometry(gl);
        Self {
            program,
            vao,
            vertex_count,
        }
    }

    pub fn render(&self, gl: &glow::Context, camera: &super::camera::Camera, size: [i32; 2]) {
        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.clear_color(0.2, 0.2, 0.2, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            gl.use_program(Some(self.program));

            let view = camera.view_matrix();
            let proj = glam::Mat4::perspective_rh(
                60.0f32.to_radians(),
                size[0] as f32 / size[1] as f32,
                0.1,
                100.0,
            );
            let mvp = proj * view;

            gl.uniform_matrix_4_f32_slice(
                gl.get_uniform_location(self.program, "u_mvp").as_ref(),
                false,
                &mvp.to_cols_array(),
            );

            gl.bind_vertex_array(Some(self.vao));
            gl.draw_arrays(glow::LINES, 0, self.vertex_count);
            gl.disable(glow::DEPTH_TEST);
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