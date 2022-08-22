use cgmath::{Matrix4, Vector3};
use noise::utils::PlaneMapBuilder;

use crate::physics::vectormath;

use super::entity::GameObject;
use super::player::camera::perspective_matrix;
use super::player::gui::Gui;
use super::renderer::mesh::Texture;
use super::renderer::shader::Shader;
use super::renderer::{Renderer, mesh};
use super::world::World;
use super::player::Player;


#[derive(PartialEq, Eq)]
pub enum PlayState {
    Running,
    Paused,
}

pub enum PlayerMovement {
    Look(f32, f32),
    Walk(f32, f32, f32),
    Inventory(usize),
    Interact(bool, bool),
    Jump,
    Stop,
}

pub struct Engine {
    terrain: World,
    player: Player,
    entities: Vec<GameObject>,
    renderer: Renderer,
    gui: Gui,

    dimensions: (i32, i32),
    elapsed_time: f32,
    pub play_state: PlayState,
}

enum WorldType {
    New(u32, isize),
    LoadFrom(String),
}

impl Engine {

    pub fn new(width: i32, height: i32, seed: u32, chunk_radius: isize) -> Self {

        let dimensions = (width, height);

        let renderer = Renderer::create_and_init_gl(width, height);

        let terrain_texture = Texture::from_dynamic_image_bytes(include_bytes!("../../assets/terrain.png"), image::ImageFormat::Png);
        let world_shader = Shader::new(include_str!("../../shaders/block_vertex.glsl"), include_str!("../../shaders/block_fragment.glsl")).unwrap();
        let terrain = World::new(terrain_texture, world_shader, seed, chunk_radius);
        
        let player = Player::new(Vector3::new(0.0, 16.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let mut entities: Vec<GameObject> = Vec::new();

        let cube1_texture = Texture::from_dynamic_image_bytes(include_bytes!("../../assets/cube_test.png"), image::ImageFormat::Png);
        let cube1_shader = Shader::new(include_str!("../../shaders/cube_vertex.glsl"), include_str!("../../shaders/cube_fragment.glsl")).unwrap();
        let cube1_pos = Vector3::<f32>::new(0.324, 12.0, 0.55);
        let cube1_rot = Vector3::<f32>::new(0.0, 0.0, 0.0);
        let cube1_scale = Vector3::<f32>::new(1.0, 1.0, 1.0);
        let cube1 = GameObject::cube(cube1_pos, cube1_rot, cube1_scale, cube1_shader, cube1_texture);
        entities.push(cube1);

        let crosshair_texture = Texture::from_dynamic_image_bytes(include_bytes!("../../assets/crosshair.png"), image::ImageFormat::Png);
        let crosshair_shader = Shader::new(include_str!("../../shaders/crosshair_vertex.glsl"), include_str!("../../shaders/crosshair_fragment.glsl")).unwrap();
        let inventory_texture = Texture::from_dynamic_image_bytes(include_bytes!("../../assets/gui.png"), image::ImageFormat::Png);
        let inventory_shader = Shader::new(include_str!("../../shaders/crosshair_vertex.glsl"), include_str!("../../shaders/crosshair_fragment.glsl")).unwrap();
        let gui = Gui::new(0.5, crosshair_shader, crosshair_texture, inventory_shader, inventory_texture);
        
        Self {
            renderer,
            terrain,
            player,
            entities,
            gui,
            dimensions,
            elapsed_time: 0.0,
            play_state: PlayState::Running,
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        if self.play_state == PlayState::Running {
            for i in 0..self.entities.len() {
                let entity = &mut self.entities[i];
                entity.update(delta_time);
            }

            self.player.update(&self.terrain, delta_time);
            self.elapsed_time += delta_time;
        }
    }

    pub fn render(&mut self) {
        let perspective_matrix: Matrix4<f32> = perspective_matrix(self.dimensions.0, self.dimensions.1);
        let view_matrix: Matrix4<f32> = self.player.camera.view_matrix();

        
        
        self.renderer.select_rendertexture();
        self.terrain.render(&view_matrix, &perspective_matrix, self.elapsed_time);
        for i in 0..self.entities.len() {
            let entity = &mut self.entities[i];
            entity.draw(&perspective_matrix, &view_matrix, self.elapsed_time);
        }
        self.renderer.render_postprocess(self.elapsed_time);

        self.gui.render(&self.player.inventory, &perspective_matrix, self.dimensions);
    }

    pub fn pause(&mut self) {
        self.play_state = PlayState::Paused;
    }

    pub fn resume(&mut self) {
        self.play_state = PlayState::Running;
    }

    pub fn player_movement(&mut self, movement: PlayerMovement) {
        if self.play_state == PlayState::Running {
            match movement {
                PlayerMovement::Look(dx, dy) => {
                    self.player.camera.rotate_on_x_axis(f32::from(dx));
                    self.player.camera.rotate_on_y_axis(f32::from(dy));
                },
                PlayerMovement::Walk(dx, dy, dz) => {
                    self.player.move_direction(Vector3::new(dx, dy, dz));
                },
                PlayerMovement::Jump => {
                    self.player.jump();
                },
                PlayerMovement::Stop => {
                    self.player.stop_move();
                }
                PlayerMovement::Inventory(selected) => {
                    self.player.inventory.selected = selected;
                },
                PlayerMovement::Interact(left_hand, right_hand) => {
                    if right_hand {
                        if let Some((_, world_index)) = vectormath::dda(&self.terrain, &self.player.camera.position, &self.player.camera.forward, 6.0) {
                            let block_id = self.terrain.block_at_global_pos(world_index);
                            self.player.inventory.add_to_inventory(block_id);
                            self.terrain.destroy_at_global_pos(world_index);
                        }
                    }
                    if left_hand {
                        if let Some((_, world_index)) = vectormath::dda(&self.terrain, &self.player.camera.position, &self.player.camera.forward, 6.0) {
                            self.terrain.interact_at_global_pos(world_index);
                        }
                    }
                }
            }
        }
    }
}


