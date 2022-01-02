#version 310 es

precision mediump float;

uniform sampler2D renderedTexture;
uniform float time;
uniform vec3 resolution;

layout(location = 0) out vec4 color;


void main(){
    
    vec2 uv = gl_FragCoord.xy / resolution.xy;

    vec2 c = uv-0.5;
    float vignette = sqrt(c.x * c.x + c.y * c.y);

    int radius = int(10.0 * vignette * vignette);
    vec3 average = vec3(0.0);
    float count = 0.0;
    for(int y=-radius;y<=radius;++y) {
        for(int x=-radius;x<=radius;++x) {
            vec2 cur_coord = gl_FragCoord.xy - vec2(float(x), float(y));
            cur_coord /= resolution.xy;
            average += texture( renderedTexture, cur_coord).xyz;
            count += 1.0;
        }
    }
    average /= count;

    color =  (1.0 - 0.75 * vignette * vignette) * vec4(average, 1.0);
}
