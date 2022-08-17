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
    
    /*for(int y=-1;y<=1;++y) {
        for(int x=-1;x<=1;++x) {
            vec2 offset = vec2(float(x), float(y));
            center_depth += linearize_depth(
                texture(
                    depthTexture,
                    vec2(0.5, 0.5) + offset
                ).r,
                zn,zf
            );
        }
    }
    center_depth /= 9.0;*/
    
    // Bright in center, dark in corners
    float vignette = c.x * c.x + c.y * c.y;

    // Blur more at the edges of the screen
    //int radius = abs(int(5.0 * abs(depth - center_depth)));

    vec4 blurred_color = texture( renderedTexture, gl_FragCoord.xy/resolution.xy);
    float count = 0.01;
    /*for(int y=-radius;y<=radius;++y) {
        for(int x=-radius;x<=radius;++x) {
            vec2 cur_coord = gl_FragCoord.xy - vec2(float(x), float(y));
            cur_coord /= resolution.xy;
            float cur_depth = linearize_depth(texture(depthTexture, cur_coord).r,zn,zf);
            if(cur_depth > depth || cur_depth <= 1.0) {
                blurred_color += texture( renderedTexture, cur_coord);
                count += 1.0;
            }
        }
    }
    blurred_color /= count;*/

    vec4 out_color = blurred_color;
    out_color *= (1.0 - 0.75 * vignette);

    vec3 direction = vec3(c, 1.0) + camera_forward;
    
    //out_color.a *= (1.0 - depth);

    color = out_color;
    //color = vec4(1.0);
}
