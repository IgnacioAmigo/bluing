use std::ffi::CString;

use crate::resources::Resources;
use super::{data::AttributedVertex, GlProgram, buffer::VertexArray, buffer::VertexBuffer, texture::Texture, subtexture::Subtexture};

use glm::{self, Vec3};

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct SpriteVertex {
    #[location = 0]
    postex: super::data::f32_f32_f32_f32,
}

pub struct SpriteRenderer {
    program: GlProgram,
    vao: VertexArray,
    vbo: VertexBuffer,
}

impl SpriteRenderer {
    pub fn from_res(res: &Resources) -> Result<SpriteRenderer, String>{
        let vertices: Vec<SpriteVertex> = vec![
            SpriteVertex { postex: (0.0, 1.0, 0.0, 1.0,).into()}, // bottom right
            SpriteVertex { postex: (1.0, 0.0, 1.0, 0.0).into()}, // bottom left
            SpriteVertex { postex: (0.0,  0.0, 0.0, 0.0).into()},  // top
            
            SpriteVertex { postex: (0.0,  1.0, 0.0, 1.0).into()},  
            SpriteVertex { postex: (1.0,  1.0, 1.0, 1.0).into()},  
            SpriteVertex { postex: (1.0,  0.0, 1.0, 0.0).into()},  
            ];
            
                
        let program = GlProgram::from_res(res,"shaders/texture2d")?;
        let vbo: VertexBuffer = VertexBuffer::new();
        let vao = VertexArray::new();
        program.set_used();

        vbo.bind();
        vbo.upload_data_static_draw(&vertices);
        vbo.unbind();

        vao.bind();
        vbo.bind();
        SpriteVertex::vertex_attrib_pointers();
        vbo.unbind();
        vao.unbind();


        program.set_mat4("projection\0".as_ptr(), glm::ortho(0.0, 800.0, 600.0, 0.0, -1.0, 1.0));
        //program.set_integer(&CString::new("image"), 0);

        Ok(SpriteRenderer{program, vao, vbo})
    }

    pub fn draw_subtexture(&self, subtexture: &Subtexture, position: glm::Vec2) {
        self.draw_quad(subtexture.texture(), position.x, position.y, 0.0, glm::vec3(1.0,1.0,1.0), 1.0, subtexture.get_normalized_rect());
    }

    pub fn draw_quad(&self, texture: &Texture, x: f32, y: f32, rotation_angle: f32, color: glm::Vec3, scale: f32, sub_tex_rect: glm::Vec4) {
        self.program.set_used();
        let model = glm::Mat4::identity();
        let model = glm::translate(&model,&glm::vec3(x,y,0.0));
        let model = glm::translate(&model,&glm::vec3(texture.width_f() * scale* 0.5, texture.height_f() * scale* 0.5,0.0));

        let radians = f32::to_radians(rotation_angle);
        let model = glm::rotate(&model,radians,&glm::vec3(0.0, 0.0, 1.0));
        let model = glm::translate(&model,&glm::vec3(-texture.width_f()* scale * 0.5, -texture.height_f() * scale * 0.5,0.0));
        let model = glm::scale(&model,
            &glm::vec3(texture.width_f() * scale * (sub_tex_rect.z) ,
                            texture.height_f() * scale * (sub_tex_rect.w),
                            0.0)
        );
        
        self.program.set_mat4("model\0".as_ptr(), model);
        self.program.set_vector4f("subTexCoords\0".as_ptr(), sub_tex_rect);
        self.program.set_vector3f("spriteColor\0".as_ptr(), color);

        
        texture.bind();
        self.vao.bind();
        unsafe {gl::ActiveTexture(gl::TEXTURE0);}
        unsafe { gl::DrawArrays(gl::TRIANGLES,0,6); }
        self.vao.unbind();
    }

    pub fn init(&self) {

    }
}
