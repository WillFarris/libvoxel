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

    vec3 light = cameraPos;//vec3(0.0, 16.0, 0.0);

    vec3 ambient = albedo * 0.01;
    
    vec3 lightDir = light - position;
    float lightDist = 0.5 * length(lightDir);
    lightDir /= lightDist;

    vec3 diffuse = max(dot(normal, lightDir), 0.0) * albedo / lightDist;

    vec3 camDir = cameraPos - position;

    vec4 shaded = vec4(0.0, 0.0, 0.0, 1.0);
    shaded.rgb = diffuse + ambient;

    /*
    vec2 cutoff = vec2(0.5, 0.5);
    if(uv.x < cutoff.x && uv.y < cutoff.y) {
        color = vec4(position, 1.0);
    } else if(uv.x < cutoff.x && uv.y >= cutoff.y) {
        color = vec4(albedo, 1.0);
    } else if(uv.y < cutoff.y) {
        color = vec4(normal, 1.0);
    } else {
        color = shaded;
    }
    */

    color = shaded;
}
