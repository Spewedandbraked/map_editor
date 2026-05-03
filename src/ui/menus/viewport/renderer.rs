use glow::HasContext;
use std::collections::HashMap;
use crate::asset::registry::MeshData;

pub struct Renderer {
    pub program: glow::Program,
    pub grid_vao: glow::VertexArray,
    pub grid_vertex_count: i32,
    pub mesh_vaos: HashMap<String, (glow::VertexArray, i32)>,
}

impl Renderer {
    pub fn new(gl: &glow::Context) -> Self {
        let program = create_shader_program(gl);
        let (grid_vao, grid_vertex_count) = create_grid_geometry(gl);
        Self {
            program,
            grid_vao,
            grid_vertex_count,
            mesh_vaos: HashMap::new(),
        }
    }

    pub fn load_mesh(&mut self, gl: &glow::Context, asset_id: &str, mesh_data: &MeshData) {
        let (vao, _) = create_mesh_geometry(gl, mesh_data);
        self.mesh_vaos.insert(asset_id.to_string(), (vao, mesh_data.index_count));
    }

    pub fn get_mesh_vao(&self, asset_id: &str) -> Option<(glow::VertexArray, i32)> {
        self.mesh_vaos.get(asset_id).copied()
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
        uniform vec3 u_color;
        out vec4 FragColor;
        void main() {
            FragColor = vec4(u_color, 1.0);
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

fn create_mesh_geometry(gl: &glow::Context, mesh_data: &MeshData) -> (glow::VertexArray, i32) {
    unsafe {
        let vao = gl.create_vertex_array().unwrap();
        let vbo = gl.create_buffer().unwrap();
        let ebo = gl.create_buffer().unwrap();
        
        gl.bind_vertex_array(Some(vao));
        
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&mesh_data.vertices), glow::STATIC_DRAW);
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 12, 0);
        gl.enable_vertex_attrib_array(0);
        
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, bytemuck::cast_slice(&mesh_data.indices), glow::STATIC_DRAW);
        
        gl.bind_vertex_array(None);
        (vao, mesh_data.vertex_count)
    }
}