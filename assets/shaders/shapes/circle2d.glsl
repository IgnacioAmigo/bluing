!frag
#version 330 core

out vec4 color;
uniform mat4 projection;
uniform mat4 model;
uniform vec4 spriteColor;

void main()
{
    vec2 uv = (gl_FragCoord.xy)/vec2(1600,900);
    float dist = distance(vec2(0.5, 0.5), uv);
    
    float fade = 0.0;
    float thickness = 0.1/2.0;
    float inv_thickness = 1 - thickness;
    float fade_factor = smoothstep(0.5, 0.5 + fade, 1 - dist);
    fade_factor*= smoothstep(0.5 - thickness, 0.5 - thickness + fade, dist);
    
    color = vec4(0.1, 0.2, 0.7, fade_factor);
} 

!vert
#version 330 core
layout (location = 0) in vec2 vertex; 

uniform mat4 model;
uniform mat4 projection;
uniform float displacement;

void main()
{
    vec4 modelPos =  model * vec4(vertex.xy, 0.0, 1.0);

    // Pixel centers are on half-integer boundaries. Add 0.5 for pixel-perfect corners.
    modelPos.xy += 0.5;
    gl_Position = projection * modelPos;
}