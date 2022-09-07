
pub(crate) struct GBuffer {
    pub(crate) framebuffer_id: u32,
    
    pub(crate) position_texture_id: u32,
    pub(crate) normal_texture_id: u32,
    pub(crate) color_spec_texture_id: u32,
    pub(crate) depth_texture_id: u32,
    pub(crate) depthbuffer_id: u32,

    pub(crate) dimensions: (i32, i32),
}

impl GBuffer {

    pub fn new(width: i32, height: i32) -> Self {

        let mut framebuffer_id = 0;
        let mut position_texture_id = 0;
        let mut normal_texture_id = 0;
        let mut color_spec_texture_id = 0;
        let mut depth_texture_id = 0;
        let mut depthbuffer_id = 0;

        unsafe {
            gl::GenFramebuffers(1, &mut framebuffer_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer_id);

            // Position
            gl::GenTextures(1, &mut position_texture_id);
            gl::BindTexture(gl::TEXTURE_2D, position_texture_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB16F as i32, width, height, 0, gl::RGB, gl::FLOAT, 0 as *const std::ffi::c_void);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, position_texture_id, 0);

            // Normal
            gl::GenTextures(1, &mut normal_texture_id);
            gl::BindTexture(gl::TEXTURE_2D, normal_texture_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB16F as i32, width, height, 0, gl::RGB, gl::FLOAT, 0 as *const std::ffi::c_void);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT1, gl::TEXTURE_2D, normal_texture_id, 0);

            // Color + Specular
            gl::GenTextures(1, &mut color_spec_texture_id);
            gl::BindTexture(gl::TEXTURE_2D, color_spec_texture_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, 0 as *const std::ffi::c_void);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT2, gl::TEXTURE_2D, color_spec_texture_id, 0);

            let draw_buffers = [gl::COLOR_ATTACHMENT0, gl::COLOR_ATTACHMENT1, gl::COLOR_ATTACHMENT2];
            gl::DrawBuffers(3, &draw_buffers as *const u32);

            // Depth buffer
            gl::GenRenderbuffers(1, &mut depthbuffer_id);
            gl::BindRenderbuffer(gl::RENDERBUFFER, depthbuffer_id);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT24, width, height);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, depthbuffer_id);

            let fb_status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
            if fb_status != gl::FRAMEBUFFER_COMPLETE {
                #[cfg(target_os = "android")] {
                    debug!("Could not setup framebuffer: glCheckFramebufferStatus() returned {}", fb_status);
                }
                panic!("Could not setup framebuffer! (error: {})", fb_status);
            }

            //gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
        println!("Generated framebuffer (id: {}), position texture (id: {}), normal texture (id: {}), color + spec texture (id: {}), depth texture (id: {}), depth buffer (id: {})", framebuffer_id, position_texture_id, normal_texture_id, color_spec_texture_id, depth_texture_id, depthbuffer_id);
        #[cfg(target_os = "android")] {
            //debug!("Generated framebuffer {}, RGB texture {}, depth buffer {}, depth texture {}", framebuffer_id, rgb_texture_id, depthbuffer_id, depth_texture_id);
        }
        
        Self {
            framebuffer_id,
            position_texture_id,
            normal_texture_id,
            color_spec_texture_id,
            depth_texture_id,
            depthbuffer_id,
            dimensions: (width, height),
        }
    }

    pub(crate) fn bind_gbuffer_fbo(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_id);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);           
        }
    }
}