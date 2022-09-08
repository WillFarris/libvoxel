#version 310 es

precision mediump float;

in vec3 v_position;
in vec3 v_normal;
in vec2 v_tex_coords;

uniform vec3 camera_position;
uniform vec3 sunlight_direction;
uniform sampler2D texture_map;

layout (location = 0) out vec4 gPosition;
layout (location = 1) out vec4 gNormal;
layout (location = 2) out vec4 gColorSpec;

void main() {
    vec4 tex_color = texture(texture_map, v_tex_coords).rgba;
    if(tex_color.a < 0.5) { discard; }

    gPosition = vec4(v_position, 1.0);
    gNormal = vec4(normalize(v_normal), 1.0);
    gColorSpec.rgb = tex_color.rgb;
    gColorSpec.a = 1.0;
}