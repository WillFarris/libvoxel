use cgmath::{Matrix4, Vector3};

use crate::{c_str, engine::{camera::perspective_matrix, player, world}, graphics::{gui::Gui, mesh::{self, Texture}, shader::Shader, render_texture::RenderTexture, postprocess::PostProcessMesh}, physics::vectormath::dda};

use super::{player::Player, world::World};

#[derive(PartialEq, Eq)]
pub enum EngineState {
    Running,
    Paused,
}

pub struct Engine {
    dimensions: (i32, i32),
    pub state: EngineState,
    pub world: Option<World>,
    pub player: Option<Player>,
    pub gui: Option<Gui>,
    sunlight_direction: Vector3<f32>,
    elapsed_time: f32,
    pub should_break_block: bool,
    pub should_interact: bool,
    framebuffer_id: i32,
    render_target: RenderTexture,
    postprocess_mesh: PostProcessMesh,
}

pub static mut ENGINE: Engine = Engine {
    dimensions: (0, 0),
    state: EngineState::Paused,
    world: None,
    player: None,
    gui: None,
    sunlight_direction: Vector3 { x: -0.701, y: 0.701, z: -0.701 },
    elapsed_time: 0.0,
    should_break_block: false,
    should_interact: true,
    framebuffer_id: 0,
    render_target: RenderTexture {
        framebuffer_id: 0,
        depthbuffer_id: 0,
        dimensions: (0, 0),
        rgb_texture_id: 0,
        depth_texture_id: 0,
    },
    postprocess_mesh: PostProcessMesh {
        mesh: None,
        shader: None,
        dimensions: (0, 0),
    }
};

impl Engine {

    pub fn gl_setup(&mut self, width: i32, height: i32) -> Result<(), String> {
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
        self.dimensions = (width, height);
        #[cfg(target_os = "android")] {
            debug!("Finished gl_setup()");
        }
        Ok(())
    }

    pub fn initialize(&mut self, seed: u32, world_radius: isize) -> Result<(), String> {
        let terrain_texture_id = mesh::texture_from_dynamic_image_bytes(include_bytes!("../../assets/terrain.png"), image::ImageFormat::Png);
        let crosshair_texture_id = mesh::texture_from_dynamic_image_bytes(include_bytes!("../../assets/crosshair.png"), image::ImageFormat::Png);
        let gui_texture_id = mesh::texture_from_dynamic_image_bytes(include_bytes!("../../assets/gui.png"), image::ImageFormat::Png);
        #[cfg(target_os = "android")]
        {
            debug!("Created textures");
        }

        let world_shader = match Shader::new(include_str!("../../shaders/block_vertex.glsl"), include_str!("../../shaders/block_fragment.glsl")) {
            Ok(shader) => shader,
            Err(error) => return Err(error),
        };
        let crosshair_shader = match Shader::new(include_str!("../../shaders/crosshair_vertex.glsl"), include_str!("../../shaders/crosshair_fragment.glsl")) {
            Ok(shader) => shader,
            Err(error) => return Err(error),
        };
        let inventory_shader = match Shader::new(include_str!("../../shaders/inventory_vertex.glsl"), include_str!("../../shaders/inventory_fragment.glsl")) {
            Ok(shader) => shader,
            Err(error) => return Err(error),
        };
        let postprocess_shader = match Shader::new(include_str!("../../shaders/postprocess_vertex.glsl"), include_str!("../../shaders/postprocess_fragment.glsl")) {
            Ok(shader) => shader,
            Err(error) => return Err(error),
        };
        #[cfg(target_os = "android")]
        {
            debug!("Created shaders");
        }
        println!("Created shaders");

        unsafe {
            gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut self.framebuffer_id);
        }
        self.render_target.init(self.dimensions.0, self.dimensions.1);
        #[cfg(target_os = "android")]
        {
            debug!("Created render target for postprocessing");
        }

        self.world = Some(world::World::new(
            Texture{id: terrain_texture_id}, 
            world_shader,
            seed,
            world_radius,
        ));
        self.player = Some(player::Player::new(Vector3::new(0f32, (world_radius * 8  + 1) as f32, 0f32), Vector3::new(1.0, 0.0, 1.0)));
        self.gui = Some(Gui::new(crosshair_shader, Texture {id: crosshair_texture_id}, inventory_shader, Texture {id: gui_texture_id }));
        #[cfg(target_os = "android")]
        {
            debug!("Created world, player, gui");
        }
        println!("Created world, player, gui");
        
        self.postprocess_mesh.init(postprocess_shader, self.render_target.rgb_texture_id, self.dimensions);

        self.state = EngineState::Running;
        Ok(())
    }

    pub fn update(&mut self, elapsed_time: f32) {
        if self.state == EngineState::Running {
            if self.should_break_block {
                if let Some((_, world_index)) = dda(self.world.as_ref().unwrap(), &self.player.as_ref().unwrap().camera.position, &self.player.as_ref().unwrap().camera.forward, 6.0) {
                    let block_id = self.world.as_ref().unwrap().block_at_global_pos(world_index);
                    self.player.as_mut().unwrap().inventory.add_to_inventory(block_id);
                    self.world.as_mut().unwrap().destroy_at_global_pos(world_index);
                }
                self.should_break_block = false;
            }

            if self.should_interact {
                #[cfg(target_os = "android")]
                {
                    debug!("Interacted");
                }
                if let Some((_, world_index)) = dda(self.world.as_ref().unwrap(), &self.player.as_ref().unwrap().camera.position, &self.player.as_ref().unwrap().camera.forward, 6.0) {
                    self.world.as_mut().unwrap().interact_at_global_pos(world_index);
                }
                self.should_interact = false;
            }

            let delta_time = elapsed_time - self.elapsed_time;
            #[cfg(target_os = "android")]
            #[cfg(feature = "debug")]
            {
                //debug!("dt={}", delta_time);
            }
            self.player.as_mut().unwrap().update(self.world.as_ref().unwrap(), delta_time);
            
            self.render(elapsed_time);
        }
        self.elapsed_time = elapsed_time;
    }

    pub fn render(&mut self, elapsed_time: f32) {
        let player = match self.player.as_ref() {
            Some(player) => player,
            None => return,
        };

        let perspective_matrix: Matrix4<f32> = perspective_matrix(self.dimensions.0, self.dimensions.1);
        let view_matrix: Matrix4<f32> = player.camera.view_matrix();

        self.render_target.set_as_target_and_clear(0.1, 0.6, 1.0, 1.0);
        self.render_preprocess(&view_matrix, &perspective_matrix, elapsed_time);


        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_id as u32);
            gl::Viewport(0,0,self.dimensions.0,self.dimensions.1);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.render_postprocess(&view_matrix, &perspective_matrix, elapsed_time);
    }

    fn render_preprocess(&mut self, view_matrix: &Matrix4<f32>, perspective_matrix: &Matrix4<f32>, elapsed_time: f32) {
        if let Some(world) = self.world.as_mut() {
            let block_shader = &mut world.world_shader;
            block_shader.use_program();
            block_shader.set_mat4(unsafe {c_str!("perspective_matrix")}, &perspective_matrix);
            block_shader.set_mat4(unsafe {c_str!("view_matrix")}, &view_matrix);
            block_shader.set_vec3(unsafe {c_str!("sunlight_direction")}, &self.sunlight_direction);
            block_shader.set_float(unsafe {c_str!("time")}, elapsed_time);
            world.render_world();
        }
    }

    fn render_postprocess(&mut self, _view_matrix: &Matrix4<f32>, perspective_matrix: &Matrix4<f32>, elapsed_time: f32) {
        self.postprocess_mesh.render(elapsed_time, &self.render_target);

        if let Some(gui) = self.gui.as_mut() {
            let inventory = &self.player.as_ref().unwrap().inventory;
            gui.render(inventory, &perspective_matrix, self.world.as_ref().unwrap().texture);
        }
    }

}


