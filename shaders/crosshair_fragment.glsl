#version 310 es

precision mediump float;

in vec3 v_position;
in vec2 v_tex_coords;

uniform float selected;
uniform mat4 perspective_matrix;
uniform sampler2D texture_map;

out vec4 color;

void main() {
    vec4 tex_color = texture(texture_map, v_tex_coords).rgba;

    float normalized_position_x = floor((v_position.x + 0.5) * 9.0);
    if(normalized_position_x == selected) {
        tex_color.rgba += 0.4;
    }

    color = tex_color;
}