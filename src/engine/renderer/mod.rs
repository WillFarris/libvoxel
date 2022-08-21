use cgmath::{Matrix4, Vector3};
use render_texture::RenderTexture;
use postprocess::PostProcessRenderMesh;

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
    postprocess_target: PostProcessRenderMesh,
    dimensions: (i32, i32),
}

impl Renderer {
    pub fn create_and_init_gl(width: i32, height: i32) -> Self {
        #[cfg(target_os = "android")] {
            gl::load_with(|s| unsafe { std::mem::transmute(egli::egl::get_proc_address(s)) });
            
            debug!("Loaded GL pointer");
        }
        unsafe {
            gl::ClearDepthf(1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);

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

        let postprocess_vertex_src = include_str!("../../../shaders/postprocess_vertex.glsl");
        let postprocess_fragment_src = include_str!("../../../shaders/postprocess_fragment.glsl");
        let postprocess_target = PostProcessRenderMesh::new(postprocess_vertex_src, postprocess_fragment_src, postprocess_rgb_texture_id, dimensions);

        #[cfg(target_os = "android")] {
            debug!("Setup Renderer");
        }

        Self {
            framebuffer_id,
            render_target,
            dimensions,
            postprocess_target,
        }
    }

    pub(crate) fn select_rendertexture(&mut self) {
        self.render_target.set_as_target_and_clear(0.1, 0.6, 1.0, 1.0);
    }

    pub(crate) fn render_postprocess(&mut self, player: &Player, elapsed_time: f32){
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_id as u32);
            gl::Viewport(0,0,self.dimensions.0,self.dimensions.1);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.postprocess_target.render(elapsed_time);
    }
}