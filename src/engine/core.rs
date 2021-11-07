use cgmath::{Matrix4, Vector3};

use crate::{c_str, engine::{camera::perspective_matrix, player, world}, graphics::{mesh::{self, Texture}, shader::Shader}, physics::vectormath::dda};

use super::{player::Player, world::World};

#[derive(PartialEq, Eq)]
enum EngineState {
    Running,
    Paused,
}

pub(crate) struct Engine {
    state: EngineState,
    world: Option<World>,
    pub player: Option<Player>,
    sunlight_direction: Vector3<f32>,
    elapsed_time: f32,
}

pub(crate) static mut ENGINE: Engine = Engine {
    state: EngineState::Paused,
    world: None,
    player: None,
    sunlight_direction: Vector3 { x: -0.701, y: 0.701, z: -0.701 },
    elapsed_time: 0.0
};


impl Engine {

    pub fn start_engine(&mut self) -> Result<(), String> {
        gl::load_with(|s| unsafe { std::mem::transmute(egli::egl::get_proc_address(s)) });
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            
            //gl::Enable(gl::CULL_FACE);
            //gl::CullFace(gl::BACK);
            
            gl::Disable(gl::CULL_FACE);
            
            gl::FrontFace(gl::CW);
    
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }


        let world_shader = match Shader::new(include_str!("../../shaders/block_vertex.glsl"), include_str!("../../shaders/block_fragment.glsl")) {
            Ok(shader) => shader,
            Err(error) => return Err(error),
        };
        let gui_shader = Shader::new(include_str!("../../shaders/gui_vertex.glsl"), include_str!("../../shaders/gui_fragment.glsl"));

        let terrain_texture_id = mesh::texture_from_dynamic_image_bytes(include_bytes!("../../terrain.png"), image::ImageFormat::Png);
        let seed = rand::random();
        self.world = Some(world::World::new(
            Texture{id: terrain_texture_id}, 
            world_shader,
            seed
        ));
        self.player = Some(player::Player::new(Vector3::new(5.0, 70.0, 4.5), Vector3::new(1.0, 0.0, 1.0)));

        self.state = EngineState::Paused;
        Ok(())
    }

    pub fn tick(&mut self, elapsed_time: f32) {
        if self.state == EngineState::Running {
            let delta_time = elapsed_time - self.elapsed_time;
            self.player.as_mut().unwrap().update(self.world.as_ref().unwrap(), (delta_time as f32) * 0.000000001);
            self.elapsed_time = elapsed_time;
        }
    }

    pub fn render(&mut self, elapsed_time: f32) {
        self.player.as_mut().unwrap().update(self.world.as_ref().unwrap(), elapsed_time - self.elapsed_time);

        unsafe {          
            gl::ClearColor(0.1, 0.4, 0.95, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let projection: Matrix4<f32> = perspective_matrix();
            let view = self.player.as_ref().unwrap().camera.view_matrix();

            if let Some(world) = self.world.as_mut() {
                let block_shader = &mut world.world_shader;
                block_shader.use_program();
                block_shader.set_mat4(c_str!("perspective_matrix"), &projection);
                block_shader.set_mat4(c_str!("view_matrix"), &view);
                block_shader.set_vec3(c_str!("sunlight_direction"), &self.sunlight_direction);
                block_shader.set_float(c_str!("time"), elapsed_time);
                world.render_world(self.player.as_ref().unwrap().position, self.player.as_ref().unwrap().camera.forward);
                
                /*let gui_shader = &mut self.gui_shader;
                gui_shader.use_program();
                gui_shader.set_float(c_str!("selected"), (self.player.as_ref().unwrap().inventory.selected % 10) as f32);
                gui_mesh.draw(&gui_shader);*/

                //gl::Disable(gl::CULL_FACE);
                //block_icon_shader.use_program();
                //block_icon_mesh.draw(&block_icon_shader);
                //gl::Enable(gl::CULL_FACE);

            }

            self.elapsed_time = elapsed_time;
        }
    }

    pub fn break_block(&mut self) {
        if let Some((_, world_index)) = dda(self.world.as_ref().unwrap(), &self.player.as_ref().unwrap().camera.position, &self.player.as_ref().unwrap().camera.forward, 6.0) {
            let block_id = self.world.as_ref().unwrap().block_at_global_pos(world_index);
            self.player.as_mut().unwrap().inventory.add_to_inventory(block_id);
            self.world.as_mut().unwrap().destroy_at_global_pos(world_index);
        }
    }

}


