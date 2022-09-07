#version 310 es

precision mediump float;

layout (location = 0) uniform sampler2D positionTexture;
layout (location = 1) uniform sampler2D normalTexture;
layout (location = 2) uniform sampler2D colorSpecTexture;

uniform vec3 resolution;
uniform vec3 cameraPos;

layout(location = 0) out vec4 color;

#define zn 0.01
#define zf 128.0

float linearize_depth(float d,float zNear,float zFar)
{
    return zNear * zFar / (zFar + d * (zNear - zFar));
}

void main(){
    vec2 uv = gl_FragCoord.xy / resolution.xy;

    vec3 position = texture(positionTexture, uv).rgb;
    vec3 normal = texture(normalTexture, uv).rgb;
    vec3 albedo = texture(colorSpecTexture, uv).rgb;

    vec3 light = vec3(0.0, 32.0, 0.0);

    vec3 ambient = albedo * 0.1;
    vec3 dir = normalize(light - cameraPos);
    vec3 diffuse = max(dot(normal, dir), 0.0) * albedo;

    color = vec4(diffuse + ambient, 1.0);
}
