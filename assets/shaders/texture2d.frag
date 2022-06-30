#version 330 core
in vec2 TexCoords;
out vec4 Color;

uniform sampler2d image;
uniform vec3 spriteColor;

void main()
{    
    color = vec4(spriteColor, 1.0) * texture(image, TexCoords);
}  
