use cgmath::Vector3;

use super::renderer::mesh::Mesh;

pub struct GameObject {
    mesh: Mesh,
    position: Vector3<f32>,
    rotation: Vector3<f32>,
}