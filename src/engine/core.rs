use cgmath::{Matrix4, Vector3};

use crate::{c_str, engine::{camera::perspective_matrix, player, world}, graphics::{mesh::{self, Texture}, shader::Shader}};

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
    elapsed_time: i64,
}

pub(crate) static mut ENGINE: Engine = Engine {
    state: EngineState::Paused,
    world: None,
    player: None,
    sunlight_direction: Vector3 { x: -0.701, y: 0.701, z: -0.701 },
    elapsed_time: 0
};


impl Engine {

    pub fn start_engine(&mut self) -> Result<(), String> {
        gl::load_with(|s| unsafe { std::mem::transmute(egli::egl::get_proc_address(s)) });
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::FrontFace(gl::CW);
    
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }


        let block_shader = match Shader::new(include_str!("../../shaders/block_vertex.glsl"), include_str!("../../shaders/block_fragment.glsl")) {
            Ok(shader) => shader,
            Err(error) => return Err(error),
        };
        let grass_shader = match Shader::new(include_str!("../../shaders/grass_vertex.glsl"), include_str!("../../shaders/block_fragment.glsl")) {
            Ok(shader) => shader,
            Err(error) => return Err(error),
        };
        let leaves_shader = match Shader::new(include_str!("../../shaders/leaves_vertex.glsl"), include_str!("../../shaders/block_fragment.glsl")) {
            Ok(shader) => shader,
            Err(error) => return Err(error),
        };
        //let _gui_shader = Shader::new(include_str!("../../shaders/gui_vertex.glsl"), include_str!("../../shaders/gui_fragment.glsl"));

        let terrain_texture_id = mesh::texture_from_dynamic_image_bytes(include_bytes!("../../terrain.png"), image::ImageFormat::Png);
        let seed = rand::random();
        self.world = Some(world::World::new(
            Texture{id: terrain_texture_id}, 
            block_shader, 
            grass_shader,
            leaves_shader,
            seed
        ));
        self.player = Some(player::Player::new(Vector3::new(5.0, 65.0, 4.5), Vector3::new(1.0, 0.0, 1.0)));

        self.state = EngineState::Paused;
        Ok(())
    }

    pub fn tick(&mut self, elapsed_time: i64) {
        if self.state == EngineState::Running {
            let delta_time = elapsed_time - self.elapsed_time;
            self.player.as_mut().unwrap().update(self.world.as_ref().unwrap(), (delta_time as f32) * 0.000000001);
            self.elapsed_time = elapsed_time;
        }
    }

    pub fn render(&mut self) {
        self.player.as_mut().unwrap().update(self.world.as_ref().unwrap(), 0.01);

        let elapsed_time = (self.elapsed_time as f32) * 0.000000001;
        unsafe {          
            gl::ClearColor(0.1, 0.4, 0.95, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let projection: Matrix4<f32> = perspective_matrix();
            let view = self.player.as_ref().unwrap().camera.view_matrix();

            if let Some(world) = self.world.as_mut() {
                let block_shader = &mut world.block_shader;
                block_shader.use_program();
                block_shader.set_mat4(c_str!("perspective_matrix"), &projection);
                block_shader.set_mat4(c_str!("view_matrix"), &view);
                block_shader.set_vec3(c_str!("sunlight_direction"), &self.sunlight_direction);
                block_shader.set_float(c_str!("time"), elapsed_time);
                world.render_solid(self.player.as_ref().unwrap().position, self.player.as_ref().unwrap().camera.forward);

                let grass_shader = &mut world.grass_shader;
                grass_shader.use_program();
                grass_shader.set_mat4(c_str!("perspective_matrix"), &projection);
                grass_shader.set_mat4(c_str!("view_matrix"), &view);
                grass_shader.set_vec3(c_str!("sunlight_direction"), &self.sunlight_direction);
                grass_shader.set_float(c_str!("time"), elapsed_time);
                world.render_grass();

                let leaves_shader = &mut world.leaves_shader;
                leaves_shader.use_program();
                leaves_shader.set_mat4(c_str!("perspective_matrix"), &projection);
                leaves_shader.set_mat4(c_str!("view_matrix"), &view);
                leaves_shader.set_vec3(c_str!("sunlight_direction"), &self.sunlight_direction);
                leaves_shader.set_float(c_str!("time"), elapsed_time);
                world.render_leaves();

                /*gui_shader.use_program();
                gui_shader.set_float(c_str!("selected"), (player.inventory.selected % 10) as f32);
                gui_mesh.draw(&gui_shader);*/

                //gl::Disable(gl::CULL_FACE);
                //block_icon_shader.use_program();
                //block_icon_mesh.draw(&block_icon_shader);
                //gl::Enable(gl::CULL_FACE);

            }
        }
    }

}


