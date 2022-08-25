use cgmath::{Matrix4, Vector2, Matrix2, SquareMatrix, Matrix3, Zero, Vector3};

use crate::{renderer::{mesh::{Texture, Mesh2D}, shader::Shader, vertex::Vertex2D}, world::block::{TextureType, BLOCKS}};

use super::inventory::Inventory;

const SQUARE_VERTICES: [Vertex2D; 6] = [
    Vertex2D { position: Vector2::new( 1.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) },   // Back-bottom-right
    Vertex2D { position: Vector2::new(-1.0, -1.0), tex_coords: Vector2::new(0.0, 0.0) },   // Back-bottom-left
    Vertex2D { position: Vector2::new(-1.0,  1.0), tex_coords: Vector2::new(0.0, 1.0) },   // Back-top-left

    Vertex2D { position: Vector2::new( 1.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) },   // Back-bottom-right
    Vertex2D { position: Vector2::new(-1.0,  1.0), tex_coords: Vector2::new(0.0, 1.0) },   // Back-top-left
    Vertex2D { position: Vector2::new( 1.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) }    // Back-top-right
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

pub struct Sprite2D {
    position: Vector2<f32>,
    rotation_deg: f32,
    scale: Vector2<f32>,

    mesh: Mesh2D,
}

impl Sprite2D {
    pub fn square(position: Vector2<f32>, rotation_deg: f32, scale: Vector2<f32>, texture: Texture, shader: Shader) -> Sprite2D {
        let mesh = Mesh2D::new(SQUARE_VERTICES.to_vec(), texture, shader);
        Sprite2D {
            position,
            rotation_deg,
            scale,

            mesh
        }
    }

    pub fn from_verts(vertices: Vec<Vertex2D>, position: Vector2<f32>, rotation_deg: f32, scale: Vector2<f32>, texture: Texture, shader: Shader) -> Sprite2D {
        let mesh = Mesh2D::new(vertices, texture, shader);
        Sprite2D {
            position,
            rotation_deg,
            scale,

            mesh
        }
    }

    pub fn draw(&mut self, perspective_matrix: &Matrix4<f32>) {
        let scale_matrix = Matrix3::from_nonuniform_scale(self.scale.x, self.scale.y);
        //let rotation = Quaternion::from_angle_x(Deg(self.rotation.x)) * Quaternion::from_angle_y(Deg(self.rotation.y)) * Quaternion::from_angle_z(Deg(self.rotation.z));
        //let rotation_matrix = Matrix4::from(rotation);
        let translation_matrix = Matrix3::from_translation(self.position);
        let model_matrix = translation_matrix * /*rotation_matrix * */ scale_matrix;

        self.mesh.draw(&model_matrix, perspective_matrix);
    }
}

pub struct Gui {
    pub crosshair: Sprite2D,
    pub hotbar: Sprite2D,

    pub selected_meshes: Vec<Mesh2D>,

    pub gui_scale: f32,
}

impl Gui {
    pub fn new(gui_scale: f32, crosshair_shader: Shader, crosshair_texture: Texture, inventory_shader: Shader, inventory_texture: Texture) -> Self {
        
        
        let crosshair = Sprite2D::square( [0.0, 0.0].into(), 0.0, [0.075, 0.075].into(), crosshair_texture, crosshair_shader);
        let hotbar = Sprite2D::from_verts(HOTBAR_FACE.to_vec(), [0.0, 0.0].into(),0.0, [1.0, 1.0].into(), inventory_texture, inventory_shader);

        Self {
            crosshair,
            hotbar,
            selected_meshes: Vec::with_capacity(9),

            gui_scale,
        }
    }

    pub fn render(&mut self, inventory: &Inventory, perspective_matrix: &Matrix4<f32>, resolution: (i32, i32)) {
        let resolution = (resolution.0 as f32, resolution.1 as f32);
        let aspect_ratio = resolution.0 / resolution.1;

        self.crosshair.draw(perspective_matrix);
        self.hotbar.draw(perspective_matrix);

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

                let mut vertices = crate::renderer::meshgen::CUBE_FACES[5].clone();
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