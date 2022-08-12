use crate::c_str;
use std::ffi::c_void;
use std::ptr;
use std::mem::size_of;
use crate::offset_of;
use gl::types::*;
use image::{self, GenericImageView};

use super::{vertex::Vertex, shader::Shader};


#[derive(Clone, Copy, Debug)]
pub struct Texture {
    pub id: u32,
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub texture: Texture,
    pub vao: u32,
    vbo: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, texture: &Texture, shader: &Shader) -> Self {
        let mut mesh = Mesh {
            vertices, texture: texture.clone(),
            vao: 0, vbo: 0
        };
        
        mesh.setup_mesh(shader);
        mesh
    }

    pub fn setup_mesh(&mut self, shader: &Shader) {
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

            let size = (self.vertices.len() * size_of::<Vertex>()) as GLsizeiptr;
            let data = &self.vertices[0] as *const Vertex as *const c_void;
            gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

            //TODO: do the same as above if indices are desired later

            let size = size_of::<Vertex>() as i32;
            
            
            // vertex Positions
            let position_location = gl::GetAttribLocation(shader.id, c_str!("position").as_ptr()) as u32;
            gl::EnableVertexAttribArray(position_location);
            gl::VertexAttribPointer(position_location, 3, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, position) as *const c_void);
            
            // vertex normals
            let normal_location = gl::GetAttribLocation(shader.id, c_str!("normal").as_ptr()) as u32;
            gl::EnableVertexAttribArray(normal_location);
            gl::VertexAttribPointer(normal_location, 3, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, normal) as *const c_void);
            
            // vertex texture coords
            let tex_coords_location = gl::GetAttribLocation(shader.id, c_str!("tex_coords").as_ptr()) as u32;
            gl::EnableVertexAttribArray(tex_coords_location);
            gl::VertexAttribPointer(tex_coords_location, 2, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, tex_coords) as *const c_void);
            
            let vertex_type_location = gl::GetAttribLocation(shader.id, c_str!("vtype").as_ptr()) as u32;
            gl::EnableVertexAttribArray(vertex_type_location);
            gl::VertexAttribPointer(vertex_type_location, 1, gl::INT, gl::FALSE, size, offset_of!(Vertex, vtype) as *const c_void);
            //gl::VertexAttrib1f(3, offset_of!(Vertex, vtype));
        }
    }

    pub fn draw(&self, shader: &Shader) {
        unsafe {
            let sampler = c_str!("texture_map").as_ptr();
            gl::Uniform1i(gl::GetUniformLocation(shader.id, sampler), 0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture.id);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
            gl::BindVertexArray(0);
        }
    }

    pub fn draw_from_texture(&self, shader: &Shader, rgb_tex_id: u32, depth_tex_id: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, rgb_tex_id);
            let sampler = c_str!("renderTexture").as_ptr();
            gl::Uniform1i(gl::GetUniformLocation(shader.id, sampler), 0);

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, depth_tex_id);
            let sampler = c_str!("depthTexture").as_ptr();
            gl::Uniform1i(gl::GetUniformLocation(shader.id, sampler), 1);

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
            gl::BindVertexArray(0);
            gl::ActiveTexture(gl::TEXTURE0);
        }
    }
}

pub fn texture_from_dynamic_image_bytes(img_bytes: &[u8], format: image::ImageFormat) -> Texture {
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