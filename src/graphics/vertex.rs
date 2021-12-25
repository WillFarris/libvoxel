use cgmath::{Vector2, Vector3};
use cgmath::prelude::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coords: Vector2<f32>,
    pub vtype: i32,
}

impl Vertex {
    pub(crate) fn _postion_only(position: Vector3<f32>, tex_coords: Vector2<f32>) -> Self {
        let mut default = Self::default();
        default.position = position;
        default.tex_coords = tex_coords;
        default
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            normal: Vector3::zero(),
            tex_coords: Vector2::zero(),
            vtype: 0,
        }
    }
}

//implement_vertex!(Vertex, position, normal, tex_coords);

#[derive(Copy, Clone, Debug)]
pub struct Vertex2D {
    pub position: (f32, f32),
    pub tex_coords: (f32, f32),
}
