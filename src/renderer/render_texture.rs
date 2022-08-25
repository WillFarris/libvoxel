pub(crate) struct RenderTexture {
    pub(crate) framebuffer_id: u32,
    pub(crate) rgb_texture_id: u32,
    pub(crate) depthbuffer_id: u32,
    pub(crate) depth_texture_id: u32,
    pub(crate) dimensions: (i32, i32),
}

impl RenderTexture {

    pub fn new(width: i32, height: i32) -> Self {

        let mut framebuffer_id = 0;
        let mut rgb_texture_id = 0;

        let mut depthbuffer_id = 0;
        let mut depth_texture_id = 0;

        unsafe {
            gl::GenFramebuffers(1, &mut framebuffer_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer_id);

            // RGB texture
            gl::GenTextures(1, &mut rgb_texture_id);
            gl::BindTexture(gl::TEXTURE_2D, rgb_texture_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width, height, 0, gl::RGB, gl::UNSIGNED_BYTE, 0 as *const std::ffi::c_void);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, rgb_texture_id, 0);

            // Depth buffer
            gl::GenRenderbuffers(1, &mut depthbuffer_id);
            gl::BindRenderbuffer(gl::RENDERBUFFER, depthbuffer_id);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT24, width, height);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, depthbuffer_id);

            // Depth texture
            gl::GenTextures(1, &mut depth_texture_id);
            gl::BindTexture(gl::TEXTURE_2D, depth_texture_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT as i32, width, height, 0, gl::DEPTH_COMPONENT, gl::FLOAT, 0 as *const std::ffi::c_void);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32); 
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_COMPARE_FUNC, gl::LEQUAL as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_COMPARE_MODE, gl::NONE as i32);
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, depth_texture_id, 0);

            let draw_buffers = [gl::COLOR_ATTACHMENT0];
            gl::DrawBuffers(2, &draw_buffers as *const u32);

            let fb_status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
            if fb_status != gl::FRAMEBUFFER_COMPLETE {
                #[cfg(target_os = "android")] {
                    debug!("Could not setup framebuffer: glCheckFramebufferStatus() returned {}", fb_status);
                }
                panic!("Could not setup framebuffer!")
            }
        }
        println!("Generated framebuffer {}, RGB texture {}, depth buffer {}, depth texture {}", framebuffer_id, rgb_texture_id, depthbuffer_id, depth_texture_id);
        #[cfg(target_os = "android")] {
            debug!("Generated framebuffer {}, RGB texture {}, depth buffer {}, depth texture {}", framebuffer_id, rgb_texture_id, depthbuffer_id, depth_texture_id);
        }
        Self {
            framebuffer_id,
            rgb_texture_id,
            depthbuffer_id,
            depth_texture_id,
            dimensions: (width, height),
        }
    }

    pub(crate) fn set_as_target_and_clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_id);
            gl::Viewport(0, 0, self.dimensions.0, self.dimensions.1);
            gl::ClearColor(r, g,b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}