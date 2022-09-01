#version 310 es

precision mediump float;

uniform sampler2D renderedTexture;
uniform sampler2D depthTexture;

uniform float time;
uniform vec3 resolution;
uniform vec3 camera_forward;
uniform vec3 camera_right;

layout(location = 0) out vec4 color;

#define zn 0.01
#define zf 128.0

float linearize_depth(float d,float zNear,float zFar)
{
    return zNear * zFar / (zFar + d * (zNear - zFar));
}

void main(){
    
    vec2 uv = gl_FragCoord.xy / resolution.xy;

    // Center of screen
    vec2 c = uv-0.5;

    float depth = linearize_depth(texture(depthTexture, uv).r,zn,zf) / zf;
    float center_depth = linearize_depth(texture(depthTexture, vec2(0.5, 0.5)).r,zn,zf);
    
    // Bright in center, dark in corners
    float vignette = c.x * c.x + c.y * c.y;

    vec4 out_color = (1.0 - depth) * texture( renderedTexture, gl_FragCoord.xy/resolution.xy);
    out_color *= (1.0 - 0.25 * vignette);

    color = out_color;
}
