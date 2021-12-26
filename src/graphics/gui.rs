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

const ASPECT_RATIO: f32 = 16f32 / 9f32;
const HOTBAR_FACE: [Vertex; 6] = [
    Vertex { position: Vector3::new( -0.5,  -1.0 + (ASPECT_RATIO * 0.120879),  0.0), normal: Vector3::new( 0.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.9140625), vtype: 0},
    Vertex { position: Vector3::new( 0.5, -1.0 + (ASPECT_RATIO * 0.120879), 0.0), normal: Vector3::new( 0.0,  0.0, 0.0), tex_coords: Vector2::new(0.7109375, 0.9140625), vtype: 0 },
    Vertex { position: Vector3::new( 0.5,  -1.0, 0.0), normal: Vector3::new( 0.0,  0.0, 0.0), tex_coords: Vector2::new(0.7109375, 1.0), vtype: 0 },

    Vertex { position: Vector3::new( -0.5, -1.0 + (ASPECT_RATIO * 0.120879),  0.0), normal: Vector3::new( 0.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.9140625), vtype: 0 },
    Vertex { position: Vector3::new( 0.5, -1.0, 0.0), normal: Vector3::new( 0.0,  0.0, 0.0), tex_coords: Vector2::new(0.7109375, 1.0), vtype: 0 },
    Vertex { position: Vector3::new( -0.5,  -1.0,  0.0), normal: Vector3::new( 0.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0), vtype: 0 },
];

pub struct Gui {
    pub crosshair_mesh: Option<Mesh>,
    pub hotbar_mesh: Option<Mesh>,
    pub selected_meshes: Vec<Mesh>,
    pub crosshair_shader: Shader,
    pub inventory_shader: Shader,
}

impl Gui {
    pub fn new(crosshair_shader: Shader, crosshair_texture: Texture, inventory_shader: Shader, inventory_texture: Texture) -> Self {
        let crosshair_mesh = Mesh::new(
            CROSSHAIR_FACE.to_vec(),
            &crosshair_texture,
            &crosshair_shader,
        );

        let hotbar_mesh = Mesh::new(
            HOTBAR_FACE.to_vec(),
            &inventory_texture,
            &inventory_shader,
        );

        Self {
            crosshair_mesh: Some(crosshair_mesh),
            hotbar_mesh: Some(hotbar_mesh),
            selected_meshes: Vec::with_capacity(9),
            crosshair_shader,
            inventory_shader,
        }
    }
}