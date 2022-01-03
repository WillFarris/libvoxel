#version 310 es

precision mediump float;

uniform sampler2D renderedTexture;
uniform sampler2D depthTexture;

uniform float time;
uniform vec3 resolution;

layout(location = 0) out vec4 color;


void main(){
    
    vec2 uv = gl_FragCoord.xy / resolution.xy;

    // Center of screen
    vec2 c = uv-0.5;

    // Bright in center, dark in corners
    float vignette = c.x * c.x + c.y * c.y;

    // Blur more at the edges of the screen
    int radius = int(100.0 * vignette * vignette);
    vec3 blurred_color = vec3(0.0);
    float count = 0.0;
    for(int y=-radius;y<=radius;++y) {
        for(int x=-radius;x<=radius;++x) {
            vec2 cur_coord = gl_FragCoord.xy - vec2(float(x), float(y));
            cur_coord /= resolution.xy;
            blurred_color += texture( renderedTexture, cur_coord).xyz;
            count += 1.0;
        }
    }
    blurred_color /= count;

    float depth = texture(depthTexture, uv).r;

    vec4 out_color = vec4(blurred_color, 1.0);

    // Darken fragment based on distance from center
    out_color *= (1.0 - 0.75 * vignette);

    //out_color *= min(1.0, 1.1 - depth);

    color = out_color;
}
