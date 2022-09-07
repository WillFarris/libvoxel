pub(crate) use std::{mem::size_of, ffi::c_void};
use std::ptr;
use cgmath::{Vector3, Vector2};
use gl::types::GLsizeiptr;


use crate::{offset_of, c_str};

use super::{shader::Shader, vertex::Vertex3D, mesh::Texture};

pub const SCREEN_FILL_QUAD_VERTS: [Vertex3D; 6] = [
    Vertex3D { position: Vector3::new( 1.0, -1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex3D { position: Vector3::new(-1.0, -1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-left
    Vertex3D { position: Vector3::new(-1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left

    Vertex3D { position: Vector3::new( 1.0, -1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex3D { position: Vector3::new(-1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left
    Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 1.0), vtype: 0  }     // Back-top-right
];

pub(crate) struct ScreenFillQuad {
    vertices: Vec<Vertex3D>,
    dimensions: (i32, i32),
    pub(crate) shader: Shader,
    vao: u32,
    vbo: u32,
}

impl ScreenFillQuad {

    pub(crate) fn new(shader: Shader, dimensions: (i32, i32)) -> ScreenFillQuad {
        let vertices = SCREEN_FILL_QUAD_VERTS.to_vec();
        
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
            dimensions,
            shader,
            vao,
            vbo,
        }
    }

    pub(crate) fn render(&mut self) {
        self.shader.use_program();
        unsafe {
            self.shader.set_vec3(c_str!("resolution"), &Vector3 { x: self.dimensions.0 as f32, y: self.dimensions.1 as f32, z: 0.0 });

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
            gl::BindVertexArray(0);
        }
    }
}