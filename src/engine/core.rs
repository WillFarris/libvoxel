use std::{sync::Mutex, time::Instant};

use cgmath::{Matrix4, Vector3};

use crate::{c_str, engine::{camera::perspective_matrix, mesh, player, world}};

use super::{player::Player, shader, world::World};

enum EngineState {
    Running,
    Paused,
}

pub(crate) struct Engine<'a> {
    state: EngineState,
    world: Option<World<'a>>,
    pub player: Option<Player>,

    sunlight_direction: Vector3<f32>,

    elapsed_sec: f64,

    block_shader: Option<shader::Shader>,
    grass_shader: Option<shader::Shader>,
    leaves_shader: Option<shader::Shader>,
}

pub(crate) static mut ENGINE: Engine = Engine {
    state: EngineState::Paused,
    world: None,
    player: None,
    block_shader: None,
    grass_shader: None,
    leaves_shader: None,
    sunlight_direction: Vector3 { x: -0.701, y: 0.701, z: -0.701 },
    elapsed_sec: 0.0
};


impl<'a> Engine<'a> {

    pub fn start_engine(&'a mut self) {
        gl::load_with(|s| unsafe { std::mem::transmute(egli::egl::get_proc_address(s)) });
        
        self.block_shader = Some(shader::Shader::new(include_str!("../../shaders/block_vertex.glsl"), include_str!("../../shaders/block_fragment.glsl")));
        self.grass_shader = Some(shader::Shader::new(include_str!("../../shaders/grass_vertex.glsl"), include_str!("../../shaders/block_fragment.glsl")));
        self.leaves_shader = Some(shader::Shader::new(include_str!("../../shaders/leaves_vertex.glsl"), include_str!("../../shaders/block_fragment.glsl")));
    
        let _gui_shader = shader::Shader::new(include_str!("../../shaders/gui_vertex.glsl"), include_str!("../../shaders/gui_fragment.glsl"));

        let terrain_texture_id = mesh::texture_from_dynamic_image_bytes(include_bytes!("../../terrain.png"), image::ImageFormat::Png);
        let seed = rand::random();
        self.world = Some(world::World::new(
            mesh::Texture{id: terrain_texture_id}, 
            &self.block_shader.as_ref().unwrap(), 
            &self.grass_shader.as_ref().unwrap(),
            self.leaves_shader.as_ref().unwrap(),
            seed
        ));
        self.player = Some(player::Player::new(Vector3::new(5.0, 65.0, 4.5), Vector3::new(1.0, 0.0, 1.0)));

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::FrontFace(gl::CW);
    
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        self.state = EngineState::Running;
        self.elapsed_sec = 0.0;
    }

    pub fn tick(&mut self) {
        
    }

    pub fn render(&mut self) {
        self.player.as_mut().unwrap().update(self.world.as_ref().unwrap(), 0.01);
        unsafe {          
            gl::ClearColor(0.1, 0.4, 0.95, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            //solid_shader.use_program();
            //transparent_shader.use_program();

            let projection: Matrix4<f32> = perspective_matrix();//cgmath::perspective(cgmath::Deg(90.0), WIDTH as f32 / HEIGHT as f32, 0.1, 100.0);
            let view = self.player.as_ref().unwrap().camera.view_matrix();
            //let model = Matrix4::from_scale(1.0);

            if let Some(shader) = self.block_shader.as_ref() {
                shader.use_program();
                shader.set_mat4(c_str!("perspective_matrix"), &projection);
                shader.set_mat4(c_str!("view_matrix"), &view);
                shader.set_vec3(c_str!("sunlight_direction"), &self.sunlight_direction);
                shader.set_float(c_str!("time"), 0.0 as f32);
                self.world.as_ref().unwrap().render_solid(self.player.as_ref().unwrap().position, self.player.as_ref().unwrap().camera.forward);
            }
            
            if let Some(shader) = self.grass_shader.as_ref() {
                shader.use_program();
                shader.set_mat4(c_str!("perspective_matrix"), &projection);
                shader.set_mat4(c_str!("view_matrix"), &view);
                shader.set_vec3(c_str!("sunlight_direction"), &self.sunlight_direction);
                shader.set_float(c_str!("time"), 0.0 as f32);
                self.world.as_ref().unwrap().render_grass();
            }

            if let Some(shader) = self.leaves_shader.as_ref() {
                shader.use_program();
                shader.set_mat4(c_str!("perspective_matrix"), &projection);
                shader.set_mat4(c_str!("view_matrix"), &view);
                shader.set_vec3(c_str!("sunlight_direction"), &self.sunlight_direction);
                shader.set_float(c_str!("time"), 0.0 as f32);
                self.world.as_ref().unwrap().render_leaves();
            }

            /*gui_shader.use_program();
            gui_shader.set_float(c_str!("selected"), (player.inventory.selected % 10) as f32);
            gui_mesh.draw(&gui_shader);*/

            //gl::Disable(gl::CULL_FACE);
            //block_icon_shader.use_program();
            //block_icon_mesh.draw(&block_icon_shader);
            //gl::Enable(gl::CULL_FACE);

            //let cursor_projection: Matrix4<f32> = perspective_matrix();//cgmath::perspective(cgmath::Deg(90.0), WIDTH as f32 / HEIGHT as f32, 0.1, 100.0);
            //let view = player.camera.view_matrix();
            /*let cursor_model = Matrix4::from_translation(cursor_position);
            world_shader.set_mat4(c_str!("model_matrix"), &cursor_model);

            cursor_cube_mesh.draw();*/

        }
        /*unsafe {
            gl::ClearColor(self.x, self.y, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }*/
    }

}


