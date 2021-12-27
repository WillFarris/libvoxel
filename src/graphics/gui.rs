use cgmath::{Matrix4, Vector2, Vector3};

use crate::{c_str, engine::{block::BLOCKS, inventory::{Inventory}}, graphics::{mesh::Texture, vertex::Vertex}};

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

    pub fn render(&self, inventory: &Inventory, perspective_matrix: &Matrix4<f32>, terrain_texture: Texture) {
        if let Some(crosshair_mesh) = &self.crosshair_mesh {
            let crosshair_shader = &self.crosshair_shader;

            crosshair_shader.use_program();
            crosshair_shader.set_mat4(unsafe {c_str!("perspective_matrix")}, &perspective_matrix);
            crosshair_mesh.draw(&crosshair_shader);
        }

        if let Some(hotbar_mesh) = &self.hotbar_mesh {
            let hotbar_shader = &self.inventory_shader;

            hotbar_shader.use_program();
            hotbar_shader.set_float(unsafe {c_str!("selected")}, (inventory.selected % 10) as f32);
            hotbar_mesh.draw(&hotbar_shader);
        }

        for i in 0..inventory.items.len() {
            let item = inventory.items[i];
            if let Some((id, _quantity)) = item {
                let tex_coords = if let Some(tex_type) = BLOCKS[id].texture_map {
                    match tex_type {
                        crate::engine::block::TextureType::Single(x, y) => (x, y),
                        crate::engine::block::TextureType::TopAndSide(_top, side) => side,
                        crate::engine::block::TextureType::TopSideBottom(_top, side, _bottom) => side,
                        crate::engine::block::TextureType::TopSideFrontActivatable(front_inactive, _front_active, _side, _top) => front_inactive,
                    }
                } else {
                    continue;
                };

                let mut vertices = crate::graphics::meshgen::CUBE_FACES[5].clone();
                for i in 0..vertices.len() {

                    
                    vertices[i].position.x = 0.1 * (i as f32);
                    vertices[i].tex_coords.x = vertices[i].tex_coords.x * 0.0625 + 0.0625 * tex_coords.0 as f32;
                    vertices[i].tex_coords.y = vertices[i].tex_coords.y * 0.0625 + 0.0625 * tex_coords.1 as f32;
                }

                let mesh = Mesh::new(vertices.to_vec(), &terrain_texture, &self.crosshair_shader);
                mesh.draw(&self.crosshair_shader);
                println!("Drew {}", id)
            }
        }
    }
}