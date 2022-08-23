use cgmath::{Vector3, Matrix4, Quaternion, Rotation3, Deg};

use super::renderer::{mesh::{Texture, Mesh3D}, meshgen::DEFAULT_CUBE, shader::Shader};

pub struct GameObject {
    mesh: Mesh3D,
    position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    scale: Vector3<f32>,
}

impl GameObject {
    pub fn cube(position: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>, shader: Shader, texture: Texture) -> GameObject {
        GameObject {
            mesh: Mesh3D::new(Vec::from(DEFAULT_CUBE), texture, shader),
            position,
            rotation,
            scale,
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