
use std::convert;

use crate::{resources::Resources, render::{GlProgram, buffer::{VertexArray, ElementBuffer}, buffer::VertexBuffer, data::*, texture::Texture}};
const MAX_QUADS: usize = 1024;
        
struct BatchStats {
    draw_calls: i32,
    quads_rendered: i32,
    texture_slots_used: i32,
}

impl Default for BatchStats {
    fn default() -> BatchStats {
        BatchStats { draw_calls: (0), quads_rendered: (0), texture_slots_used: (0) }
    }
}

pub struct BatchRenderer {
    program: GlProgram,
    vao: VertexArray,
    vbo: VertexBuffer,
    vertices: Vec<QuadVertex>,

    index_buffer: ElementBuffer,
    ortho_matrix: glm::Mat4,

    white_texture: Texture,

    stats: BatchStats
}

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
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

impl BatchRenderer {
    pub fn from_res(res: &Resources, screen_dimensions: glm::Vec2, _max_quads: u32) -> Result<BatchRenderer, String> {        
        let vertices: Vec<QuadVertex> = Vec::with_capacity(MAX_QUADS * 4);
                
        let program = GlProgram::from_res(res,"shaders/batch_texture2d.glsl")?;

        program.set_used();
        let vbo: VertexBuffer = VertexBuffer::new();
        let vao = VertexArray::new();

        // let's see if we can make an EBO for rectangle rendering
        // note: it would have to be rendered with line_strip or loop

        // we create the indices array manually and upload it to the GPU
        // since there are 6 indices per quad, we need to make sure we make enough
     
        let rect_indices: Vec<gl::types::GLuint> = (0..MAX_QUADS * 6).step_by(4)
                                                       .flat_map(|x| [0 + x as u32, 1 + x  as u32, 2 + x  as u32, 2 + x  as u32, 3 + x  as u32, 0 + x as u32]).collect(); 
        let index_buffer = ElementBuffer::new();

        vbo.bind();
        vbo.upload_data_dynamic_draw( &vertices);
        vbo.unbind();
        
        index_buffer.bind();
        index_buffer.upload_data_static_draw( &rect_indices);
        index_buffer.unbind();

        println!("sizeof f32: {}", ::std::mem::size_of::<f32>());
        println!("glm::vec2: {}", ::std::mem::size_of::<glm::Vec2>());
        println!("glm::vec3: {}", ::std::mem::size_of::<glm::Vec3>());
        println!("glm::vec4: {}", ::std::mem::size_of::<glm::Vec4>());
        println!("sizeof vector mio: {}", ::std::mem::size_of::<QuadVertex>());

        vao.bind();
        index_buffer.bind();
        vbo.bind();
        QuadVertex::vertex_attrib_pointers();
        vbo.unbind();
        vao.unbind();

        let white_texture = Texture::with_white_new().expect("Error creating white texture");

        let ortho_matrix = glm::ortho(0.0, screen_dimensions.x, screen_dimensions.y, 0.0, -1.0, 1.0);
        program.set_mat4("projection\0".as_ptr(), ortho_matrix);
        
        Ok(BatchRenderer{program, vao, vbo, index_buffer, white_texture, ortho_matrix, vertices, stats: BatchStats::default()})
    }

    pub fn begin_scene(&mut self) {
        self.stats = BatchStats::default();
        self.vertices.clear();
    }
    
    pub fn end_scene(&self) {
        self.program.set_used();

        self.vbo.bind();

        self.vbo.upload_subdata_dynamic_draw(&self.vertices, (self.stats.quads_rendered * 4) as usize);
        
        self.index_buffer.bind();
        self.vao.bind();

        unsafe { gl::DrawElements(gl::TRIANGLES, self.stats.quads_rendered * 6, gl::UNSIGNED_INT, 0 as *const _); }
        self.vao.unbind();
    
    }

    pub fn draw_quad(&mut self, texture: &Texture, position: glm::Vec3, color: glm::Vec4, scale: f32, _sub_tex_rect: glm::Vec4) {
        let quad_positions = [
            glm::vec3(0.0, 0.0, 0.0), 
            glm::vec3(1.0, 0.0, 0.0) * scale, 
            glm::vec3(1.0, 1.0, 0.0) * scale, 
            glm::vec3(0.0, 1.0, 0.0) * scale
        ];
        
        let model = glm::Mat4::identity();
        let model = glm::translate(&model,&position);
        //let model = glm::scale(&model,&glm::vec3(scale, scale,0.0));
        
/*         let model = glm::scale(&model,
            &glm::vec3(texture.width_f() * scale * (sub_tex_rect.z) ,
                            texture.height_f() * scale * (sub_tex_rect.w),
                            0.0)
        ); */

        let real_position = model * glm::vec4(position.x, position.y, position.z, 0.0);
        for i in quad_positions {
            let new_quad_vertex = QuadVertex {
                pos: position + i,//glm::vec3(real_position.x * i.x, real_position.y * i.y, real_position.z * i.z),
                color: color,
                tex_coords: i.xy(),
                texture_id: 0.0,
            };

            self.vertices.push(new_quad_vertex);
        }
        
        self.stats.quads_rendered += 1; 
    }

}
