pub(crate) use cgmath::{Vector3, Matrix4, Quaternion, Rotation3, Deg};

use crate::{renderer::{mesh::{Mesh3D, Texture}, meshgen::DEFAULT_CUBE, shader::Shader}, physics::collision::{Rect3, rect_vs_rect, Collider}};

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
        let rot_speed = 10.;
        self.rotation.x += rot_speed * delta_time;
        self.rotation.y += rot_speed * delta_time;
        self.rotation.z += rot_speed * delta_time;
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
    fn check_collision(&mut self, delta: Vector3<f32>, other: &mut impl Collider, delta_time: f32) -> Vector3<f32> {
        let other_bounding_box = other.bounding_box();

        let collision = rect_vs_rect(&self.bounding_box(), &other_bounding_box);
        if collision {
            println!("Collision!");
        }

        [0.0, 0.0, 0.0].into()
    }

    fn bounding_box(&self) -> crate::physics::collision::Rect3 {
        let mut bounding_box = self.collision_box.clone();
        bounding_box.pos += self.position;
        bounding_box
    }
}