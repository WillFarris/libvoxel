pub(crate) use std::{mem::size_of, ffi::c_void};
use std::ptr;
use cgmath::{Vector3, Vector2};
use gl::types::GLsizeiptr;


use crate::{offset_of, c_str};

use super::{shader::Shader, vertex::Vertex3D, mesh::Texture};

pub const POSTPROCESS_VERTICES: [Vertex3D; 6] = [
    Vertex3D { position: Vector3::new( 1.0, -1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex3D { position: Vector3::new(-1.0, -1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-left
    Vertex3D { position: Vector3::new(-1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left

    Vertex3D { position: Vector3::new( 1.0, -1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex3D { position: Vector3::new(-1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left
    Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 1.0), vtype: 0  }     // Back-top-right
];

pub(crate) struct PostProcessRenderMesh {
    vertices: Vec<Vertex3D>,
    shader: Shader,
    texture: Texture,
    dimensions: (i32, i32),

    vao: u32,
    vbo: u32,
}

impl PostProcessRenderMesh {

    pub(crate) fn new(vertex_src: &str, fragment_src: &str, rendertexture_id: u32, dimensions: (i32, i32)) -> PostProcessRenderMesh {
        let vertices = POSTPROCESS_VERTICES.to_vec();
        let texture = Texture::from_id(rendertexture_id);
        let shader = Shader::new(vertex_src, fragment_src).unwrap();

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            let size = (vertices.len() * size_of::<Vertex3D>()) as GLsizeiptr;
            let data = &vertices[0] as *const Vertex3D as *const c_void;
            gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

            let stride = size_of::<Vertex3D>() as i32;
            
            // vertex Positions
            let position_location = gl::GetAttribLocation(shader.id, c_str!("position").as_ptr()) as u32;
            gl::EnableVertexAttribArray(position_location);
            gl::VertexAttribPointer(position_location, 3, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex3D, position) as *const c_void);
            
            // Vertex texture coords
            let tex_coords_location = gl::GetAttribLocation(shader.id, c_str!("tex_coords").as_ptr()) as u32;
            gl::EnableVertexAttribArray(tex_coords_location);
            gl::VertexAttribPointer(tex_coords_location, 2, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex3D, tex_coords) as *const c_void);
        }

        Self {
            vertices, 
            shader,
            texture,
            dimensions,

            vao,
            vbo,
        }
    }

    pub(crate) fn render(&mut self, elapsed_time: f32) {      
        self.shader.use_program();
        unsafe {
            self.shader.set_float(crate::c_str!("time"), elapsed_time);
            self.shader.set_vec3(crate::c_str!("resolution"), &Vector3::new(self.dimensions.0 as f32, self.dimensions.1 as f32, 0.0));

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture.id);
            let sampler = c_str!("renderTexture").as_ptr();
            gl::Uniform1i(gl::GetUniformLocation(self.shader.id, sampler), 0);

            /*gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, depth_tex_id);
            let sampler = c_str!("depthTexture").as_ptr();
            gl::Uniform1i(gl::GetUniformLocation(shader.id, sampler), 1);*/

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
            gl::BindVertexArray(0);
        }
    }
}