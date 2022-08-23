use crate::c_str;
use std::ffi::c_void;
use std::ptr;
use std::mem::size_of;
use crate::offset_of;
use cgmath::{Vector3, Matrix4, Vector2, Matrix2, Matrix3};
use gl::types::*;
use image::{self, GenericImageView};

use super::{vertex::{Vertex3D, Vertex2D}, shader::Shader};


#[derive(Clone, Copy, Debug)]
pub struct Texture {
    pub id: u32,
}

impl Texture {
    pub fn from_id(id: u32) -> Self {
        Self {
            id,
        }
    }

    pub fn from_dynamic_image_bytes(img_bytes: &[u8], format: image::ImageFormat) -> Texture {
        let img = image::load_from_memory_with_format(img_bytes, format).unwrap().flipv();
        let format = match img {
            image::DynamicImage::ImageLuma8(_) => gl::RED,
            image::DynamicImage::ImageLumaA8(_) => gl::RG,
            image::DynamicImage::ImageRgb8(_) => gl::RGB,
            image::DynamicImage::ImageRgba8(_) => gl::RGBA,
            _ => panic!("Unknown image format"),
        };
    
        let data = img.as_bytes();
    
        let mut texture_id = 0;
        
        unsafe {
            gl::GenTextures(1, &mut texture_id);
    
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, img.width() as i32, img.height() as i32,
                0, format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
    
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            
        }
    
        Texture {
            id: texture_id,
        }
    }
}

pub struct ChunkMesh {
    pub vertices: Vec<Vertex3D>,
    pub texture: Texture,
    pub shader: Shader,
    pub vao: u32,
    vbo: u32,
}

impl ChunkMesh {
    pub fn new(vertices: Vec<Vertex3D>, texture: Texture, shader: Shader) -> ChunkMesh {
        let mut mesh = ChunkMesh {
            vertices,
            texture,
            shader,
            vao: 0, vbo: 0
        };
        
        mesh.setup_mesh();
        mesh
    }

    pub fn setup_mesh(&mut self) {
        if self.vertices.len() == 0 {
            //panic!("[ Mesh::setup_mesh() ] No vertices to setup!");
            return;
        }
        unsafe {
            if self.vao != 0 {
                gl::DeleteVertexArrays(1, &mut self.vao);
            }
            if self.vbo != 0 {
                gl::DeleteBuffers(1, &mut self.vbo);
            }
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            let size = (self.vertices.len() * size_of::<Vertex3D>()) as GLsizeiptr;
            let data = &self.vertices[0] as *const Vertex3D as *const c_void;
            gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

            let stride = size_of::<Vertex3D>() as i32;
            
            // vertex Positions
            let position_location = gl::GetAttribLocation(self.shader.id, c_str!("position").as_ptr()) as u32;
            gl::EnableVertexAttribArray(position_location);
            gl::VertexAttribPointer(position_location, 3, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex3D, position) as *const c_void);
            
            // vertex normals
            let normal_location = gl::GetAttribLocation(self.shader.id, c_str!("normal").as_ptr()) as u32;
            gl::EnableVertexAttribArray(normal_location);
            gl::VertexAttribPointer(normal_location, 3, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex3D, normal) as *const c_void);
            
            // vertex texture coords
            let tex_coords_location = gl::GetAttribLocation(self.shader.id, c_str!("tex_coords").as_ptr()) as u32;
            gl::EnableVertexAttribArray(tex_coords_location);
            gl::VertexAttribPointer(tex_coords_location, 2, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex3D, tex_coords) as *const c_void);
            
            // vertex type
            let vertex_type_location = gl::GetAttribLocation(self.shader.id, c_str!("vtype").as_ptr()) as u32;
            gl::EnableVertexAttribArray(vertex_type_location);
            gl::VertexAttribPointer(vertex_type_location, 1, gl::INT, gl::FALSE, stride, offset_of!(Vertex3D, vtype) as *const c_void);
        }
    }

    pub fn draw(&self, shader: &Shader) {
        if self.vao == 0 || self.vbo == 0 {
            return;
        }
        
        unsafe {
            let texture_map_str = c_str!("texture_map").as_ptr();
            gl::Uniform1i(gl::GetUniformLocation(shader.id, texture_map_str), 0);

            gl::BindTexture(gl::TEXTURE_2D, self.texture.id);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn cleanup_vertex_objects(&mut self) {
        if self.vao != 0 {
            unsafe {
                gl::DeleteVertexArrays(1, &mut self.vao);
            }
            self.vao = 0;
        }
        if self.vbo != 0 {
            unsafe {
                gl::DeleteBuffers(1, &mut self.vbo);
            }
            self.vbo = 0;
        }
    }

    /*pub fn draw_from_texture(&self, shader: &Shader, rgb_tex_id: u32, depth_tex_id: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, rgb_tex_id);
            let sampler = c_str!("renderTexture").as_ptr();
            gl::Uniform1i(gl::GetUniformLocation(shader.id, sampler), 0);

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, depth_tex_id);
            let sampler = c_str!("depthTexture").as_ptr();
            gl::Uniform1i(gl::GetUniformLocation(shader.id, sampler), 1);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
            gl::BindVertexArray(0);
        }
    }*/
}

impl Drop for ChunkMesh {
    fn drop(&mut self) {
        self.cleanup_vertex_objects();
    }
}

pub struct Mesh3D {
    vertices: Vec<Vertex3D>,
    texture: Texture,
    shader: Shader,

    vao: u32,
    vbo: u32,
}

impl Mesh3D {
    pub fn new(vertices: Vec<Vertex3D>, texture: Texture, shader: Shader) -> Mesh3D {

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
            
            // vertex normals
            let normal_location = gl::GetAttribLocation(shader.id, c_str!("normal").as_ptr()) as u32;
            gl::EnableVertexAttribArray(normal_location);
            gl::VertexAttribPointer(normal_location, 3, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex3D, normal) as *const c_void);
            
            // vertex texture coords
            let tex_coords_location = gl::GetAttribLocation(shader.id, c_str!("tex_coords").as_ptr()) as u32;
            gl::EnableVertexAttribArray(tex_coords_location);
            gl::VertexAttribPointer(tex_coords_location, 2, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex3D, tex_coords) as *const c_void);
        }

        Mesh3D {
            vertices,
            texture,
            shader,

            vao,
            vbo,
        }
    }

    pub fn draw(&mut self, model_matrix: &Matrix4<f32>, view_matrix: &Matrix4<f32>, perspective_matrix: &Matrix4<f32>, elapsed_time: f32) {
        unsafe {
            let texture_map_str = c_str!("texture_map").as_ptr();
            gl::Uniform1i(gl::GetUniformLocation(self.shader.id, texture_map_str), 0);

            self.shader.use_program();
            self.shader.set_mat4(c_str!("perspective_matrix"), &perspective_matrix);
            self.shader.set_mat4(c_str!("view_matrix"), &view_matrix);
            self.shader.set_mat4(c_str!("model_matrix"), &model_matrix);
            self.shader.set_float(c_str!("time"), elapsed_time);
            
            //self.shader.set_vec3(unsafe {c_str!("sunlight_direction")}, sunlight_direction);

            gl::BindTexture(gl::TEXTURE_2D, self.texture.id);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

pub struct Mesh2D {
    vertices: Vec<Vertex2D>,
    texture: Texture,
    shader: Shader,

    vao: u32,
    vbo: u32,
}

impl Mesh2D {
    pub fn new(vertices: Vec<Vertex2D>, texture: Texture, shader: Shader) -> Mesh2D {
        let mut vao = 0;
        let mut vbo = 0;
        
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            let size = (vertices.len() * size_of::<Vertex2D>()) as GLsizeiptr;
            let data = &vertices[0] as *const Vertex2D as *const c_void;
            gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

            let stride = size_of::<Vertex2D>() as i32;
            
            // vertex Positions
            let position_location = gl::GetAttribLocation(shader.id, c_str!("position").as_ptr()) as u32;
            gl::EnableVertexAttribArray(position_location);
            gl::VertexAttribPointer(position_location, 3, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex2D, position) as *const c_void);

            // vertex texture coords
            let tex_coords_location = gl::GetAttribLocation(shader.id, c_str!("tex_coords").as_ptr()) as u32;
            gl::EnableVertexAttribArray(tex_coords_location);
            gl::VertexAttribPointer(tex_coords_location, 2, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex2D, tex_coords) as *const c_void);
        }

        Mesh2D {
            vertices,
            texture,
            shader,

            vao,
            vbo,
        }
    }

    pub fn draw(&mut self, model_matrix: &Matrix3<f32>, perspective_matrix: &Matrix4<f32>) {
        self.shader.use_program();
        self.shader.set_mat4(unsafe { c_str!("perspective_matrix") }, &perspective_matrix);
        self.shader.set_mat3(unsafe { c_str!("model_matrix") }, &model_matrix);
        //self.shader.set_float(unsafe { c_str!("time") }, elapsed_time);

        unsafe {
            let texture_map_str = c_str!("texture_map").as_ptr();
            gl::Uniform1i(gl::GetUniformLocation(self.shader.id, texture_map_str), 0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture.id);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}