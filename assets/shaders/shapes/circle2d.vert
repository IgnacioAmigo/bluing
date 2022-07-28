#version 330 core
layout (location = 0) in vec4 vertex; // <vec2 position, vec2 texCoords>

out vec2 TexCoords;

uniform mat4 model;
uniform mat4 projection;
uniform float displacement;

uniform vec4 subTexCoords;

void main()
{
    TexCoords = subTexCoords.xy + (vertex.zw * subTexCoords.zw);
    vec4 modelPos =  model * vec4(vertex.xy, 0.0, 1.0);
    // Pixel centers are on half-integer boundaries. Add 0.5 for pixel-perfect corners.
    modelPos.x += displacement;
    modelPos.y += displacement;
    gl_Position = projection * modelPos;

//this would be more performant: 
//    gl_Position = projection * model * vec4(vertex.xy, 0.0, 1.0);
}