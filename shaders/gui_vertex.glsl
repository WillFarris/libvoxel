#version 310 es

precision mediump float;

in vec2 position;
in vec2 tex_coords;

uniform mat4 perspective_matrix;
uniform mat3 model_matrix;
uniform float gui_scale;

out vec3 v_position;
out vec2 v_tex_coords;

void main() {
    vec4 position = perspective_matrix * vec4(model_matrix * vec3(position, 0.0), 1.0);//(0.625 * gui_scale * (perspective_matrix * vec4(position, 0.0, 1.0)) + vec4(translation, 0.0, 0.0));

    v_position = position.xyz;
    v_tex_coords = tex_coords;

    gl_Position = vec4(position.xyz, 1.0);
}