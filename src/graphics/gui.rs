use std::os::unix::prelude::OsStringExt;

use cgmath::{Vector2, Vector3};

use crate::graphics::{mesh::Texture, vertex::Vertex};

use super::{mesh::Mesh, shader::Shader};

const CROSSHAIR_FACE: [Vertex; 6] = [
    Vertex { position: Vector3::new( 0.15, -0.15, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex { position: Vector3::new(-0.15, -0.15, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-left
    Vertex { position: Vector3::new(-0.15,  0.15, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left

    Vertex { position: Vector3::new( 0.15, -0.15, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex { position: Vector3::new(-0.15,  0.15, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left
    Vertex { position: Vector3::new( 0.15,  0.15, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 1.0), vtype: 0  }     // Back-top-right
];

pub(crate) struct Gui {
    pub(crate) crosshair_mesh: Option<Mesh>,
    pub(crate) selected_meshes: Vec<Mesh>,
    pub(crate) gui_shader: Shader,
}

impl Gui {
    pub(crate) fn new(gui_shader: Shader, texture: Texture) -> Self {
        let crosshair_mesh = Mesh::new(
            CROSSHAIR_FACE.to_vec(),
            &texture,
            &gui_shader
        );

        Self {
            crosshair_mesh: Some(crosshair_mesh),
            selected_meshes: Vec::with_capacity(9),
            gui_shader,
        }
    }
}