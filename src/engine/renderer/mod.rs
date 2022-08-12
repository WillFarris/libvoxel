use cgmath::{Matrix4, Vector3};
use render_texture::RenderTexture;
use postprocess::PostProcessTarget;

use self::shader::Shader;

use super::{world::World, player::Player};

pub mod mesh;
pub mod meshgen;
pub mod shader;
pub mod vertex;
pub mod render_texture;
pub mod postprocess;


pub struct Renderer {
    framebuffer_id: i32,
    render_target: RenderTexture,
    postprocess_target: PostProcessTarget,
    dimensions: (i32, i32),
}

impl Renderer {
    pub fn create_and_init_gl(width: i32, height: i32) -> Self {
        #[cfg(target_os = "android")] {
            gl::load_with(|s| unsafe { std::mem::transmute(egli::egl::get_proc_address(s)) });
            
            debug!("Loaded GL pointer");
        }
        unsafe {
            gl::Enable(gl::DEPTH_TEST);

            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            
            gl::FrontFace(gl::CW);
    
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        #[cfg(target_os = "android")] {
            debug!("Set OpenGL parameters");
        }

        let dimensions = (width, height);
        let mut framebuffer_id = 0;
        unsafe {
            gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut framebuffer_id);
        }
        let render_target = RenderTexture::new(dimensions.0, dimensions.1);
        let postprocess_rgb_texture_id = render_target.rgb_texture_id;

        let postprocess_shader = Shader::new(include_str!("../../../shaders/postprocess_vertex.glsl"), include_str!("../../../shaders/postprocess_fragment.glsl")).unwrap();

        #[cfg(target_os = "android")] {
            debug!("Setup Renderer");
        }

        Self {
            framebuffer_id,
            render_target,
            dimensions,
            postprocess_target: PostProcessTarget::create(postprocess_shader, postprocess_rgb_texture_id, dimensions),
        }
    }

    pub fn render_preprocess(&mut self, world: &World, view_matrix: &Matrix4<f32>, perspective_matrix: &Matrix4<f32>, sunlight_direction: &Vector3<f32>, elapsed_time: f32) {
        self.render_target.set_as_target_and_clear(0.1, 0.6, 1.0, 1.0);

        let block_shader = &world.world_shader;
        block_shader.use_program();
        block_shader.set_mat4(unsafe {c_str!("perspective_matrix")}, &perspective_matrix);
        block_shader.set_mat4(unsafe {c_str!("view_matrix")}, &view_matrix);
        block_shader.set_vec3(unsafe {c_str!("sunlight_direction")}, sunlight_direction);
        block_shader.set_float(unsafe {c_str!("time")}, elapsed_time);
        world.render_world();
    }

    pub fn render_postprocess(&mut self, player: &Player, elapsed_time: f32){
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_id as u32);
            gl::Viewport(0,0,self.dimensions.0,self.dimensions.1);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.postprocess_target.render(elapsed_time, &self.render_target, &player.camera.forward, &player.camera.right);
    }
}