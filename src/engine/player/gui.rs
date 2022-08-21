use cgmath::{Matrix4, Vector2, Vector3};

use crate::{c_str, engine::{renderer::{mesh::{ChunkMesh, Texture, Mesh2D}, shader::Shader, vertex::{Vertex3D, Vertex2D}}, world::block::{TextureType, BLOCKS}}};

use super::inventory::Inventory;

const CROSSHAIR_FACE: [Vertex2D; 6] = [
    Vertex2D { position: Vector2::new( 0.15, -0.15), tex_coords: Vector2::new(1.0, 0.0) },   // Back-bottom-right
    Vertex2D { position: Vector2::new(-0.15, -0.15), tex_coords: Vector2::new(0.0, 0.0) },   // Back-bottom-left
    Vertex2D { position: Vector2::new(-0.15,  0.15), tex_coords: Vector2::new(0.0, 1.0) },   // Back-top-left

    Vertex2D { position: Vector2::new( 0.15, -0.15), tex_coords: Vector2::new(1.0, 0.0) },   // Back-bottom-right
    Vertex2D { position: Vector2::new(-0.15,  0.15), tex_coords: Vector2::new(0.0, 1.0) },   // Back-top-left
    Vertex2D { position: Vector2::new( 0.15,  0.15), tex_coords: Vector2::new(1.0, 1.0) }    // Back-top-right
];

const ASPECT_RATIO: f32 = 16f32 / 9f32;

const HOTBAR_FACE: [Vertex2D; 6] = [
    Vertex2D { position: Vector2::new( -0.5,  -1.0 + (ASPECT_RATIO * 0.120879)), tex_coords: Vector2::new(0.0, 0.9140625) },
    Vertex2D { position: Vector2::new( 0.5, -1.0 + (ASPECT_RATIO * 0.120879)), tex_coords: Vector2::new(0.7109375, 0.9140625) },
    Vertex2D { position: Vector2::new( 0.5,  -1.0), tex_coords: Vector2::new(0.7109375, 1.0) },

    Vertex2D { position: Vector2::new( -0.5, -1.0 + (ASPECT_RATIO * 0.120879)), tex_coords: Vector2::new(0.0, 0.9140625) },
    Vertex2D { position: Vector2::new( 0.5, -1.0), tex_coords: Vector2::new(0.7109375, 1.0) },
    Vertex2D { position: Vector2::new( -0.5,  -1.0), tex_coords: Vector2::new(0.0, 1.0) },
];

pub struct Gui {
    pub crosshair_mesh: Mesh2D,
    pub hotbar_mesh: Mesh2D,
    pub selected_meshes: Vec<Mesh2D>,

    pub gui_scale: f32,
    //pub crosshair_shader: Shader,
    //pub inventory_shader: Shader,
}

impl Gui {
    pub fn new(gui_scale: f32, crosshair_shader: Shader, crosshair_texture: Texture, inventory_shader: Shader, inventory_texture: Texture) -> Self {
        
        
        let crosshair_mesh = Mesh2D::new(Vector2::new(0.0, 0.0), CROSSHAIR_FACE.to_vec(), crosshair_texture, crosshair_shader);
        let hotbar_mesh = Mesh2D::new(Vector2::new(0.0, 0.0),HOTBAR_FACE.to_vec(), inventory_texture, inventory_shader);

        Self {
            crosshair_mesh,
            hotbar_mesh,
            selected_meshes: Vec::with_capacity(9),

            gui_scale,
        }
    }

    pub fn render(&mut self, inventory: &Inventory, perspective_matrix: &Matrix4<f32>, terrain_texture: Texture, resolution: (i32, i32)) {
        let resolution = (resolution.0 as f32, resolution.1 as f32);
        let aspect_ratio = resolution.0 / resolution.1;

        
        self.crosshair_mesh.draw(self.gui_scale, perspective_matrix);
        self.hotbar_mesh.draw(self.gui_scale, perspective_matrix);

        for i in 0..inventory.items.len() {
            let item = inventory.items[i];
            if let Some((id, _quantity)) = item {
                let tex_coords = if let Some(tex_type) = BLOCKS[id].texture_map {
                    match tex_type {
                        TextureType::Single(x, y) => (x, y),
                        TextureType::TopAndSide(_top, side) => side,
                        TextureType::TopSideBottom(_top, side, _bottom) => side,
                        TextureType::TopSideFrontActivatable(front_inactive, _front_active, _side, _top) => front_inactive,
                    }
                } else {
                    continue;
                };

                let mut vertices = crate::engine::renderer::meshgen::CUBE_FACES[5].clone();
                for v in 0..vertices.len() {

                    let scale = 0.07;

                    vertices[v].position.x *= scale;
                    vertices[v].position.y *= scale * aspect_ratio;
                    
                    vertices[v].position.x -= 0.475;

                    vertices[v].position.x += 0.11 * (i as f32);
                    vertices[v].position.y -= 0.95;

                    vertices[v].position.z = -1.0;

                    vertices[v].tex_coords.x = vertices[v].tex_coords.x * 0.0625 + 0.0625 * tex_coords.0 as f32;
                    vertices[v].tex_coords.y = vertices[v].tex_coords.y * 0.0625 + 0.0625 * tex_coords.1 as f32;
                }

                //let mesh = ChunkMesh::new(vertices.to_vec(), terrain_texture.clone(), self.crosshair_shader.clone());
                //mesh.draw(&self.crosshair_shader);
            }
        }
    }
}