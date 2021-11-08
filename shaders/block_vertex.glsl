#version 310 es

precision mediump float;

in vec3 position;
in vec3 normal;
in vec2 tex_coords;
in int vtype;

uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform mat4 perspective_matrix;
uniform float time;


out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;

#define WIND_SPEED 5.0

void main() {
    mat4 camera_matrix = perspective_matrix * view_matrix;
    vec4 pos4 = vec4(position, 1.0);
    if(vtype == 1) {
        pos4.xz += mod(position.y, 1.0) * 0.03 * sin(WIND_SPEED * (time + position.y));
    } else if(vtype == 2) {
        pos4.xz += 0.03 * sin(WIND_SPEED * (time + position.y + 0.1415));
    }
    vec4 pos4_new = camera_matrix * model_matrix * pos4;
    
    
    v_position = pos4_new.xyz;
    v_normal = transpose(inverse(mat3(model_matrix))) * normal;
    v_tex_coords = tex_coords;

    gl_Position = pos4_new;
}