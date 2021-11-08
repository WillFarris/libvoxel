#version 310 es

precision mediump float;

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform mat4 perspective_matrix;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;

void main() {
    vec4 position = 0.625 *perspective_matrix * vec4(position, 1.0);


    v_position = position.xyz;
    v_normal = normal;
    v_tex_coords = tex_coords;

    gl_Position = vec4(position.xyz, 1.0);
}