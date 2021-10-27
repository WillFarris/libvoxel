use super::shader;

pub fn init_engine() {
    let block_shader = shader::Shader::new(include_str!("../../shaders/block_vertex.glsl"), include_str!("../../shaders/block_fragment.glsl"));
    let grass_shader = shader::Shader::new(include_str!("../../shaders/grass_vertex.glsl"), include_str!("../../shaders/block_fragment.glsl"));
    let leaves_shader = shader::Shader::new(include_str!("../../shaders/leaves_vertex.glsl"), include_str!("../../shaders/block_fragment.glsl"));

    let gui_shader = shader::Shader::new(include_str!("../../shaders/gui_vertex.glsl"), include_str!("../../shaders/gui_fragment.glsl"));
    //let block_icon_shader = shader::Shader::new(include_str!("../../shaders/gui_vertex.glsl"), include_str!("../../shaders/gui_fragment.glsl"));


}