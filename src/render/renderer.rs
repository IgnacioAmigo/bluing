use std::ffi::CString;

use crate::resources::Resources;
use super::{data::AttributedVertex, GlProgram, buffer::VertexArray, buffer::VertexBuffer, texture::Texture};

use nalgebra_glm as glm;

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

            SpriteVertex { postex: (0.0,  1.0, 0.0, 1.0).into()},  // top
            SpriteVertex { postex: (1.0,  1.0, 1.0, 1.0).into()},  // top
            SpriteVertex { postex: (1.0,  0.0, 1.0, 0.0).into()},  // top
        ];

        let vbo: VertexBuffer = VertexBuffer::new();
        let vao = VertexArray::new();

        vbo.bind();
        vbo.upload_data_static_draw(&vertices);

        vao.bind();
        SpriteVertex::vertex_attrib_pointers();
        vbo.unbind();
        vao.unbind();

        let program = GlProgram::from_res(res,"assets/shaders/texture2d")?;

        Ok(SpriteRenderer{program, vao, vbo})
    }

    pub fn render(&self, texture: Texture, x: f32, y: f32, rotation_angle: f32) {
        self.program.set_used();
        let model = glm::Mat4::new();
        model = glm::translate(&model,&glm::vec3(x,y,0.0));
        model = glm::translate(&model,&glm::vec3(texture.width_f() * 0.5, texture.height_f() * 0.5,0.0));
       // model = glm::rotate(&model,glm::radians(rotation_angle),glm::vec3(0.0, 0.0, 1.0));
        //model = glm::translate(&model,&glm::vec3(-texture.width_f() * 0.5,- texture.height_f() * 0.5,0.0));

        self.program.set_mat4("asd"., model);
        self.program.SetVector3f("spriteColor", color);
    
    }

    pub fn init(&self) {

    }
}
