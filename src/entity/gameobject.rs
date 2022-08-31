pub(crate) use cgmath::{Vector3, Matrix4, Quaternion, Rotation3, Deg};

use crate::{renderer::{mesh::{Mesh3D, Texture}, meshgen::DEFAULT_CUBE, shader::Shader}, physics::collision::{Rect3, rect_vs_rect, Collider}, world::World, player::Player};

pub struct GameObject {
    mesh: Mesh3D,
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,

    collision_box: Rect3,
}

impl GameObject {
    pub fn cube(position: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>, shader: Shader, texture: Texture) -> GameObject {
        let collision_box = Rect3 {
            pos: [-0.5, -0.5, -0.5].into(),
            size: [1.0, 1.0, 1.0].into(),
        };

        GameObject {
            mesh: Mesh3D::new(Vec::from(DEFAULT_CUBE), texture, shader),
            position,
            rotation,
            scale,
            
            collision_box,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        /*
        let rot_speed = 10.;
        self.rotation.x += rot_speed * delta_time;
        self.rotation.y += rot_speed * delta_time;
        self.rotation.z += rot_speed * delta_time;
        */

        //self.position.y -= 0.1 * delta_time;
    }

    pub fn draw(&mut self, perspective_matrix: &Matrix4<f32>, view_matrix: &Matrix4<f32>, elapsed_time: f32) {

        let scale_matrix = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        let rotation = Quaternion::from_angle_x(Deg(self.rotation.x)) * Quaternion::from_angle_y(Deg(self.rotation.y)) * Quaternion::from_angle_z(Deg(self.rotation.z));
        let rotation_matrix = Matrix4::from(rotation);
        let translation_matrix = Matrix4::from_translation(self.position);
        let model_matrix = translation_matrix * rotation_matrix * scale_matrix;
        
        self.mesh.draw(&model_matrix, view_matrix, perspective_matrix, elapsed_time);
    }
}

impl Collider for GameObject {
    fn bounding_box(&self) -> crate::physics::collision::Rect3 {
        let mut bounding_box = self.collision_box.clone();
        bounding_box.pos += self.position;
        bounding_box
    }

    fn check_overlap_x(&self, other: &impl Collider) -> f32 {
        let other_bounding_box = other.bounding_box();
        let mut self_bounding_box = self.bounding_box();

        if rect_vs_rect(&self_bounding_box, &other_bounding_box) {
            let x_overlap = if self_bounding_box.pos.x > other_bounding_box.pos.x {
                (other_bounding_box.pos.x + 1.0) - self_bounding_box.pos.x 
            } else {
                -1.0 * (self_bounding_box.pos.x + self_bounding_box.size.x - other_bounding_box.pos.x)
            };
            self_bounding_box.pos.x += x_overlap;
            return x_overlap;
        }

        0.0
    }

    fn check_overlap_y(&self, other: &impl Collider) -> f32 {
        let other_bounding_box = other.bounding_box();
        let mut self_bounding_box = self.bounding_box();

        if rect_vs_rect(&self_bounding_box, &other_bounding_box) {
            let y_overlap = if self_bounding_box.pos.y > other_bounding_box.pos.y {
                (other_bounding_box.pos.y + 1.0) - self_bounding_box.pos.y
            } else {
                -1.0 * (self_bounding_box.pos.y + self_bounding_box.size.y - other_bounding_box.pos.y)
            };
            self_bounding_box.pos.y += y_overlap;
            return y_overlap;
        }

        0.0
    }

    fn check_overlap_z(&self, other: &impl Collider) -> f32 {
        let other_bounding_box = other.bounding_box();
        let mut self_bounding_box = self.bounding_box();

        if rect_vs_rect(&self_bounding_box, &other_bounding_box) {
            let z_overlap = if self_bounding_box.pos.z > other_bounding_box.pos.z {
                (other_bounding_box.pos.z + 1.0) - self_bounding_box.pos.z 
            } else {
                -1.0 * (self_bounding_box.pos.z + self_bounding_box.size.z - other_bounding_box.pos.z)
            };
            self_bounding_box.pos.z += z_overlap;
            return z_overlap;
        }

        0.0
    }
}