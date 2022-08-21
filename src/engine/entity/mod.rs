use cgmath::{Vector3, Matrix4, SquareMatrix, Vector4, Quaternion};

use crate::physics::vectormath::rotation_matrix;

use super::renderer::{mesh::{Texture, Mesh3D}, meshgen::DEFAULT_CUBE, shader::Shader};

pub struct GameObject {
    mesh: Mesh3D,
    texture: Texture,
    position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    scale: Vector3<f32>,
}

impl GameObject {
    pub fn cube(position: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>, shader: Shader, texture: Texture) -> GameObject {
        GameObject {
            mesh: Mesh3D::new(Vec::from(DEFAULT_CUBE), texture, shader),
            texture,
            position,
            rotation,
            scale,
        }
    }

    pub fn draw(&mut self, perspective_matrix: &Matrix4<f32>, view_matrix: &Matrix4<f32>, elapsed_time: f32) {
        //shader.set_vec3(unsafe {c_str!("transform_position")}, &self.position);
        //shader.set_float(unsafe {c_str!("time")}, elapsed_time);

        let scale_matrix = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

        let rotation_matrix = Matrix4::from(Quaternion::from_sv(1.0, self.rotation));

        let translation_matrix = Matrix4::from_translation(self.position);

        let model_matrix = translation_matrix * rotation_matrix * scale_matrix;

        self.mesh.draw(&model_matrix, view_matrix, perspective_matrix);
    }
}