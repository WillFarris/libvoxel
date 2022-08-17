use cgmath::{Vector3, Vector2};


use super::{shader::Shader, vertex::Vertex, mesh::{Mesh, Texture}, render_texture::RenderTexture};

pub const POSTPROCESS_VERTICES: [Vertex; 6] = [
    Vertex { position: Vector3::new( 1.0, -1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex { position: Vector3::new(-1.0, -1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-left
    Vertex { position: Vector3::new(-1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left

    Vertex { position: Vector3::new( 1.0, -1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex { position: Vector3::new(-1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left
    Vertex { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 1.0), vtype: 0  }     // Back-top-right
];

pub(crate) struct PostProcessTarget {
    pub(crate) mesh: Mesh,
    pub(crate) shader: Shader,
    pub(crate) dimensions: (i32, i32),
}

impl PostProcessTarget {

    pub(crate) fn create(shader: Shader, texture_id: u32, dimensions: (i32, i32)) -> Self {
        let mesh = Mesh::new(
            POSTPROCESS_VERTICES.to_vec(),
            &Texture {id: texture_id},
            &shader,
        );

        Self {
            mesh,
            shader,
            dimensions,
        }
    }

    pub(crate) fn render(&mut self, elapsed_time: f32, render_target: &RenderTexture, camera_forward: &Vector3<f32>, camera_right: &Vector3<f32>) {      
        self.shader.use_program();
        unsafe {
            //let sampler_str = crate::c_str!("renderedTexture").as_ptr();
            //gl::Uniform1i(gl::GetUniformLocation(shader.id, sampler_str), 0);
            //gl::BindTexture(gl::TEXTURE_2D, render_target.rgb_texture_id);

            self.shader.set_float(crate::c_str!("time"), elapsed_time);
            self.shader.set_vec3(crate::c_str!("resolution"), &Vector3::new(self.dimensions.0 as f32, self.dimensions.1 as f32, 0.0));
            //shader.set_vec3(crate::c_str!("camera_forward"), camera_forward);
            //shader.set_vec3(crate::c_str!("camera_right"), camera_right);
        }
        
        self.mesh.draw_from_texture(&self.shader, render_target.rgb_texture_id, render_target.depth_texture_id);
    }
}