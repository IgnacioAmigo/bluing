use crate::resources::Resources;
use super::{data::AttributedVertex, GlProgram, buffer::{VertexArray, ElementBuffer}, buffer::VertexBuffer, texture::Texture, subtexture::Subtexture};

pub mod batch_renderer;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct SpriteVertex {
    #[location = 0]
    postex: glm::Vec4,
}

pub struct SpriteRenderer {
    program: GlProgram,
    vao: VertexArray,
    vbo: VertexBuffer,

    quad_veo: ElementBuffer,
    circle_program: GlProgram,
    ortho_matrix: glm::Mat4
}

impl SpriteRenderer {
    pub fn from_res(res: &Resources, screen_dimensions: glm::Vec2) -> Result<SpriteRenderer, String>{
        let vertices: Vec<SpriteVertex> = vec![
                SpriteVertex { postex: glm::vec4(0.0,  0.0, 0.0, 0.0) },  // top
                SpriteVertex { postex: glm::vec4(0.0, 1.0, 0.0, 1.0,) }, // bottom right
                SpriteVertex { postex: glm::vec4(1.0, 0.0, 1.0, 0.0) }, // bottom left
                
                SpriteVertex { postex: glm::vec4(0.0,  1.0, 0.0, 1.0) },  
                SpriteVertex { postex: glm::vec4(1.0,  1.0, 1.0, 1.0) },  
                SpriteVertex { postex: glm::vec4(1.0,  0.0, 1.0, 0.0) },  
        ];
                
        let program = GlProgram::from_res(res,"shaders/texture2d.glsl")?;
        let circle_program = GlProgram::from_res(res,"shaders/shapes/circle2d.glsl")?;
        let vbo: VertexBuffer = VertexBuffer::new();
        let vao = VertexArray::new();

        // let's see if we can make an EBO for rectangle rendering
        // note: it would have to be rendered with line_strip or loop

        let rect_indices: Vec<u32> = vec![0,2,4,1];
        let quad_veo = ElementBuffer::new();
        program.set_used();

        quad_veo.bind();
        quad_veo.upload_data_static_draw( &rect_indices);
        quad_veo.unbind();

        vbo.bind();
        vbo.upload_data_static_draw(&vertices);
        vbo.unbind();

        vao.bind();
        quad_veo.bind();
        vbo.bind();
        SpriteVertex::vertex_attrib_pointers();
        vbo.unbind();
        vao.unbind();

        let ortho_matrix = glm::ortho(0.0, screen_dimensions.x, screen_dimensions.y, 0.0, -1.0, 1.0);
        program.set_mat4("projection\0".as_ptr(), ortho_matrix);
        
        circle_program.set_used();
        circle_program.set_mat4("projection\0".as_ptr(), ortho_matrix);

        Ok(SpriteRenderer{program, circle_program, vao, vbo, quad_veo, ortho_matrix})
    }

    pub fn draw_subtexture(&self, subtexture: &Subtexture, position: glm::Vec2) {
        self.draw_quad(subtexture.texture(), position.x, position.y, 0.0, glm::vec3(1.0,1.0,1.0), 3.0, subtexture.get_normalized_rect());
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
        

        self.program.set_float("displacement\0".as_ptr(), 0.0);
        self.program.set_mat4("model\0".as_ptr(), model);
        self.program.set_vector4f("subTexCoords\0".as_ptr(), sub_tex_rect);
        self.program.set_vector3f("spriteColor\0".as_ptr(), color);

        
        texture.bind();
        self.vao.bind();
       // unsafe {gl::ActiveTexture(gl::TEXTURE0);}
        unsafe { gl::DrawArrays(gl::TRIANGLES,0,6); }
        self.vao.unbind();
    }

    pub fn draw_rect(&self, rect: glm::Vec4, color: glm::Vec3) {
        let model = glm::Mat4::identity();
        let model = glm::translate(&model,&glm::vec3(rect.x,rect.y,0.0));

        let model = glm::scale(&model,
            &glm::vec3( rect.z ,
                            rect.w,
                            0.0)
        );
        
        self.program.set_mat4("model\0".as_ptr(), model);
        self.program.set_vector3f("spriteColor\0".as_ptr(), color);
        self.program.set_float("displacement\0".as_ptr(), 0.5);

        self.vao.bind();
        unsafe { gl::DrawElements(gl::LINE_LOOP,4,gl::UNSIGNED_INT,0 as *const _); }
        self.vao.unbind();
    }

    pub fn draw_circle(&self, rect: glm::Vec4, color: glm::Vec3) {
        let model = glm::Mat4::identity();
        let model = glm::translate(&model,&glm::vec3(rect.x,rect.y,0.0));

        let model = glm::scale(&model,
            &glm::vec3( rect.z ,
                            rect.w,
                            0.0)
        );
        self.circle_program.set_used();
        self.circle_program.set_mat4("model\0".as_ptr(), model);

        self.vao.bind();
        unsafe { gl::DrawArrays(gl::TRIANGLES,0,6); }
        self.vao.unbind();
    }
}
