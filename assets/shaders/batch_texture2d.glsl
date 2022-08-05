!frag

#version 400 core

in vec4 Color;
out vec4 o_color;

//uniform sampler2D image;

void main()
{
    o_color = Color;// * texture2D(image, TexCoords);
} 


!vert

/*
struct QuadVertex {
    #[location = 0]
    pos: glm::Vec3,
    #[location = 1]
    color: glm::Vec4,
    #[location = 2]
    tex_coords: glm::Vec2,
    #[location = 3]
    texture_id: f32,
}
*/

#version 400 core
layout (location = 0) in vec3 v_Position; 
layout (location = 1) in vec4 v_Color; 
layout (location = 2) in vec2 v_TexCoords; 
layout (location = 3) in float v_TexId; 

//uniform mat4 model; // i believe this should be gone
uniform mat4 projection;
//uniform sampler2D u_Textures[2];

out vec4 Color;

void main()
{
    Color = v_Color;
    vec4 modelPos =  vec4(v_Position.xy, 0.0, 1.0);
    gl_Position = projection * modelPos;
    
}