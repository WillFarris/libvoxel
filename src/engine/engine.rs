use cgmath::{Matrix4, Vector3};
use noise::utils::PlaneMapBuilder;

use crate::physics::vectormath;

use super::player::camera::perspective_matrix;
use super::player::gui::Gui;
use super::renderer::shader::Shader;
use super::renderer::{Renderer, mesh};
use super::world::World;
use super::player::Player;


#[derive(PartialEq, Eq)]
pub enum PlayState {
    Running,
    Paused,
}

#[derive(PartialEq, Eq)]
pub enum PlayerInteraction {
    None,
    RightHand,
    LeftHand,
    BothHands,
}

pub enum PlayerMovement {
    Look(f32, f32),
    Walk(f32, f32, f32),
    Inventory(usize),
    Jump,
    Stop,
}

pub struct Engine {
    terrain: World,
    player: Player,
    renderer: Renderer,
    gui: Gui,

    dimensions: (i32, i32),
    sunlight_direction: Vector3<f32>,
    elapsed_time: f32,
    pub play_state: PlayState,
    pub player_interaction: PlayerInteraction,
}

enum WorldType {
    New(u32, isize),
    LoadFrom(String),
}

impl Engine {

    pub fn new(width: i32, height: i32, seed: u32, chunk_radius: isize) -> Self {

        let dimensions = (width, height);

        let renderer = Renderer::create_and_init_gl(width, height);

        let terrain_texture = mesh::texture_from_dynamic_image_bytes(include_bytes!("../../assets/terrain.png"), image::ImageFormat::Png);
        let world_shader = Shader::new(include_str!("../../shaders/block_vertex.glsl"), include_str!("../../shaders/block_fragment.glsl")).unwrap();
        let terrain = World::new(terrain_texture, world_shader, seed, chunk_radius);
        
        let player = Player::new(Vector3::new(0.0, 32.0, 0.0), Vector3::new(0.0, 0.0, 1.0));

        let crosshair_texture = mesh::texture_from_dynamic_image_bytes(include_bytes!("../../assets/crosshair.png"), image::ImageFormat::Png);
        let crosshair_shader = Shader::new(include_str!("../../shaders/crosshair_vertex.glsl"), include_str!("../../shaders/crosshair_fragment.glsl")).unwrap();
        let inventory_texture = mesh::texture_from_dynamic_image_bytes(include_bytes!("../../assets/gui.png"), image::ImageFormat::Png);
        let inventory_shader = Shader::new(include_str!("../../shaders/inventory_vertex.glsl"), include_str!("../../shaders/inventory_fragment.glsl")).unwrap();
        let gui = Gui::new(crosshair_shader, crosshair_texture, inventory_shader, inventory_texture);
        
        Self {
            renderer,
            terrain,
            player,
            gui,
            dimensions,
            sunlight_direction: Vector3::new(0.45, 0.45, 0.45),
            elapsed_time: 0.0,
            play_state: PlayState::Running,
            player_interaction: PlayerInteraction::None,
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        if self.play_state == PlayState::Running {
            
            match self.player_interaction {
                PlayerInteraction::None => {},
                PlayerInteraction::LeftHand => {
                    #[cfg(target_os = "android")]
                    {
                        debug!("Interacted");
                    }
                    if let Some((_, world_index)) = vectormath::dda(&self.terrain, &self.player.camera.position, &self.player.camera.forward, 6.0) {
                        self.terrain.interact_at_global_pos(world_index);
                    }
                },
                PlayerInteraction::RightHand => {
                    if let Some((_, world_index)) = vectormath::dda(&self.terrain, &self.player.camera.position, &self.player.camera.forward, 6.0) {
                        let block_id = self.terrain.block_at_global_pos(world_index);
                        self.player.inventory.add_to_inventory(block_id);
                        self.terrain.destroy_at_global_pos(world_index);
                    }
                }
                PlayerInteraction::BothHands => todo!(),
            }
            self.player_interaction = PlayerInteraction::None;

            self.player.update(&self.terrain, delta_time);
            self.elapsed_time += delta_time;
        }
    }

    pub fn render(&mut self) {
        let perspective_matrix: Matrix4<f32> = perspective_matrix(self.dimensions.0, self.dimensions.1);
        let view_matrix: Matrix4<f32> = self.player.camera.view_matrix();

        
        self.renderer.render_preprocess(&self.terrain, &view_matrix, &perspective_matrix, &self.sunlight_direction, self.elapsed_time);
        self.renderer.render_postprocess(&self.player, self.elapsed_time);


        self.gui.render(&self.player.inventory, &perspective_matrix, self.terrain.texture, self.dimensions);
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
            }
        }
    }
}


