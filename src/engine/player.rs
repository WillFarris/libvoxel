use cgmath::Vector3;

use crate::engine::{block::BLOCKS, camera::Camera, world::World};
use crate::physics::collision;
use crate::physics::{collision::rect_vs_rect, vectormath::{Y_VECTOR, normalize, q_rsqrt}};
use crate::engine::inventory::Inventory;

const GRAVITY: Vector3<f32> = Vector3 {x: 0.0, y: -9.81, z: 0.0};

pub struct Player {
    pub camera: Camera,
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub acceleration: Vector3<f32>,
    move_speed: f32,
    grounded: bool,
    walking: bool,
    height: f32,

    pub inventory: Inventory,
}

impl Player {
    pub fn new(position: Vector3<f32>, forward: Vector3<f32>) -> Self {
        Self {
            camera: Camera::new(position, forward),
            position,
            velocity: Vector3::new(0f32, 0f32, 0f32),
            acceleration: Vector3::new(0f32, 0f32, 0f32),
            move_speed: 3.0,
            grounded: false,
            walking: false,
            height: 1.6,
            inventory: Inventory::new(),
        }
    }

    pub fn update(&mut self, world: &World, delta_time: f32) {

        if !self.grounded {
            self.acceleration.y = GRAVITY.y;
            self.velocity.y += self.acceleration.y * delta_time;
        }

        if !self.walking {
            self.velocity.x *= 1.0 - 10.0 * delta_time;
            self.velocity.z *= 1.0 - 10.0 * delta_time;
        }

        self.velocity += self.acceleration * delta_time;

        let forward = normalize(&Vector3::new(self.camera.forward.x, 0.0, self.camera.forward.z));
        let delta = delta_time * Vector3 {
            x: (self.move_speed * self.camera.right.x * self.velocity.x as f32) + (self.move_speed * forward.x * self.velocity.z as f32),
            y: self.velocity.y as f32,
            z: (self.move_speed * self.camera.right.z * self.velocity.x as f32) + (self.move_speed * forward.z * self.velocity.z as f32),
        };


        let collision_box_dimensions = (0.5, 1.8);

        self.position.x += delta.x;
        let mut player_bounding_box = collision::Rect3 {
            pos: Vector3::new(
                self.position.x - (collision_box_dimensions.0/2.0),
                self.position.y,
                self.position.z - (collision_box_dimensions.0/2.0)),
            size: Vector3::new(
                collision_box_dimensions.0,
                collision_box_dimensions.1,
                collision_box_dimensions.0
            )
        };
        for block_x in (self.position.x.floor() as isize - 1) ..= (self.position.x.floor() as isize + 1) {
            for block_y in (self.position.y.floor() as isize - 1) ..= (self.position.y.floor() as isize + 2) {
                for block_z in (self.position.z.floor() as isize - 1) ..= (self.position.z.floor() as isize + 1) {
                    if !BLOCKS[world.block_at_global_pos(Vector3::new(block_x, block_y, block_z))].solid {
                        continue;
                    }
                    let block_bounding_box = collision::Rect3 {
                        pos: Vector3::new(block_x as f32, block_y as f32, block_z as f32),
                        size: Vector3::new(1.0, 1.0, 1.0)
                    };
                    if rect_vs_rect(&player_bounding_box, &block_bounding_box) {
                        let x_overlap = if player_bounding_box.pos.x > block_bounding_box.pos.x {
                            (block_bounding_box.pos.x + 1.0) - player_bounding_box.pos.x 
                        } else {
                            -1.0 * (player_bounding_box.pos.x + player_bounding_box.size.x - block_bounding_box.pos.x)
                        };
                        self.position.x += x_overlap;
                        player_bounding_box.pos.x += x_overlap;
                    }
                }
            }
        }

        self.position.y += delta.y;
        player_bounding_box = collision::Rect3 {
            pos: Vector3::new(
                self.position.x - (collision_box_dimensions.0/2.0),
                self.position.y,
                self.position.z - (collision_box_dimensions.0/2.0)),
            size: Vector3::new(
                collision_box_dimensions.0,
                collision_box_dimensions.1,
                collision_box_dimensions.0
            )
        };
        for block_x in (self.position.x.floor() as isize - 1) ..= (self.position.x.floor() as isize + 1) {
            for block_y in (self.position.y.floor() as isize - 1) ..= (self.position.y.floor() as isize + 2) {
                for block_z in (self.position.z.floor() as isize - 1) ..= (self.position.z.floor() as isize + 1) {
                    if !BLOCKS[world.block_at_global_pos(Vector3::new(block_x, block_y, block_z))].solid {
                        continue;
                    }
                    let block_bounding_box = collision::Rect3 {
                        pos: Vector3::new(block_x as f32, block_y as f32, block_z as f32),
                        size: Vector3::new(1.0, 1.0, 1.0)
                    };
                    if rect_vs_rect(&player_bounding_box, &block_bounding_box) {
                        let y_overlap = if player_bounding_box.pos.y > block_bounding_box.pos.y {
                            (block_bounding_box.pos.y + 1.0) - player_bounding_box.pos.y 
                        } else {
                            -1.0 * (player_bounding_box.pos.y + player_bounding_box.size.y - block_bounding_box.pos.y)
                        };

                        self.position.y += y_overlap;
                        player_bounding_box.pos.y += y_overlap;
                        if y_overlap.abs() > 0.0 {
                            self.velocity.y = 0f32;
                            if y_overlap > 0.0 {
                                self.grounded = true;
                            }
                        } else {
                            self.grounded = false;
                        }
                    }
                }
            }
        }

        self.position.z += delta.z;
        player_bounding_box = collision::Rect3 {
            pos: Vector3::new(
                self.position.x - (collision_box_dimensions.0/2.0),
                self.position.y,
                self.position.z - (collision_box_dimensions.0/2.0)),
            size: Vector3::new(
                collision_box_dimensions.0,
                collision_box_dimensions.1,
                collision_box_dimensions.0
            )
        };
        for block_x in (self.position.x.floor() as isize - 1) ..= (self.position.x.floor() as isize + 1) {
            for block_y in (self.position.y.floor() as isize - 1) ..= (self.position.y.floor() as isize + 2) {
                for block_z in (self.position.z.floor() as isize - 1) ..= (self.position.z.floor() as isize + 1) {
                    if !BLOCKS[world.block_at_global_pos(Vector3::new(block_x, block_y, block_z))].solid {
                        continue;
                    }
                    let block_bounding_box = collision::Rect3 {
                        pos: Vector3::new(block_x as f32, block_y as f32, block_z as f32),
                        size: Vector3::new(1.0, 1.0, 1.0)
                    };
                    if rect_vs_rect(&player_bounding_box, &block_bounding_box) {
                        let z_overlap = if player_bounding_box.pos.z > block_bounding_box.pos.z {
                            (block_bounding_box.pos.z + 1.0) - player_bounding_box.pos.z 
                        } else {
                            -1.0 * (player_bounding_box.pos.z + player_bounding_box.size.z - block_bounding_box.pos.z)
                        };
                        self.position.z += z_overlap;
                        player_bounding_box.pos.z += z_overlap;
                    }
                }
            }
        }
        self.camera.translate(self.position + self.height * Y_VECTOR);
    }

    pub fn move_direction(&mut self, direction: Vector3<f32>) {
        self.walking = true;
        self.velocity.x += direction.x;
        self.velocity.z += direction.z;
        self.velocity.x *= q_rsqrt(self.velocity.x * self.velocity.x + self.velocity.z * self.velocity.z);
        self.velocity.z *= q_rsqrt(self.velocity.x * self.velocity.x + self.velocity.z * self.velocity.z);
    }

    pub fn jump(&mut self) {
        if self.grounded {
            self.velocity.y += 8f32;
            self.grounded = false;
        }
    }

    pub fn stop_move(&mut self) {
        self.walking = false;
    }
}