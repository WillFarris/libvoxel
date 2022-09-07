use cgmath::Vector3;

use crate::c_str;

use self::{screen_quad::ScreenFillQuad, gbuffer::GBuffer, shader::Shader};


pub(crate) mod mesh;
pub(crate) mod meshgen;
pub(crate) mod shader;
pub(crate) mod vertex;
pub(crate) mod gbuffer;
mod screen_quad;

pub struct Renderer {
    screen_framebuffer_id: i32,
    gbuffer: GBuffer,
    screen_fill_quad: ScreenFillQuad,
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
        let mut screen_framebuffer_id = 0;
        unsafe {
            gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut screen_framebuffer_id);
        }

        
        let gbuffer = GBuffer::new(dimensions.0, dimensions.1);

        let quad_vertex_src = include_str!("../../shaders/screen_fill_quad_vertex.glsl");
        let quad_fragment_src = include_str!("../../shaders/gbuffer_lighting_fragment.glsl");
        let quad_shader = Shader::new(quad_vertex_src, quad_fragment_src).unwrap();
        let screen_fill_quad = ScreenFillQuad::new(quad_shader, (width, height));


        #[cfg(target_os = "android")] {
            debug!("Setup Renderer");
        }

        Self {
            screen_framebuffer_id,
            gbuffer,
            dimensions,
            screen_fill_quad,
        }
    }

    pub(crate) fn bind_gbuffer_fbo(&mut self) {
        self.gbuffer.bind_gbuffer_fbo();
    }

    pub(crate) fn bind_screen_fbo(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.screen_framebuffer_id as u32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub(crate) fn render_gbuffer_to_screen(&mut self, camera_pos: &Vector3<f32>){
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.screen_framebuffer_id as u32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.screen_fill_quad.shader.use_program();
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.gbuffer.position_texture_id);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.gbuffer.normal_texture_id);
            gl::ActiveTexture(gl::TEXTURE2);
            gl::BindTexture(gl::TEXTURE_2D, self.gbuffer.color_spec_texture_id);

            let sampler = c_str!("positionTexture").as_ptr();
            gl::Uniform1i(gl::GetUniformLocation(self.screen_fill_quad.shader.id, sampler), 0);
            let sampler = c_str!("normalTexture").as_ptr();
            gl::Uniform1i(gl::GetUniformLocation(self.screen_fill_quad.shader.id, sampler), 1);
            let sampler = c_str!("colorSpecTexture").as_ptr();
            gl::Uniform1i(gl::GetUniformLocation(self.screen_fill_quad.shader.id, sampler), 2);

            self.screen_fill_quad.shader.set_vec3(c_str!("cameraPos"), camera_pos);
        }
        self.screen_fill_quad.render();
    }
}