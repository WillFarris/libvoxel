use super::shader;

enum EngineState {
    Running,
    Paused,
}

pub(crate) struct Engine {
    state: EngineState,
    world: Option<u32>,
    player: Option<u32>,

    pub(crate) x: f32,
    pub(crate) y: f32,
}

pub(crate) static mut ENGINE: Engine = Engine {
    state: EngineState::Paused,
    world: None,
    player: None,
    x: 1.0,
    y: 0.0,
};

impl Engine {

    pub fn start_engine(&mut self) {
        gl::load_with(|s| unsafe { std::mem::transmute(egli::egl::get_proc_address(s)) });
        
        let block_shader = shader::Shader::new(include_str!("../../shaders/block_vertex.glsl"), include_str!("../../shaders/block_fragment.glsl"));
        let grass_shader = shader::Shader::new(include_str!("../../shaders/grass_vertex.glsl"), include_str!("../../shaders/block_fragment.glsl"));
        let leaves_shader = shader::Shader::new(include_str!("../../shaders/leaves_vertex.glsl"), include_str!("../../shaders/block_fragment.glsl"));
    
        let gui_shader = shader::Shader::new(include_str!("../../shaders/gui_vertex.glsl"), include_str!("../../shaders/gui_fragment.glsl"));

        self.state = EngineState::Running;
    }

    pub fn tick(&mut self) {
        

    }

    pub fn render(&mut self) {
        unsafe {
            gl::ClearColor(self.x, self.y, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn set_xy(&mut self, nx: f32, ny: f32) {
        self.x = nx;
        self.y = ny;
    }

}


