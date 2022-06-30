#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct SpriteVertex {
    #[location = 0]
    postex: data::f32_f32_f32_f32,
    #[location = 1]
    color: data::f32_f32_f32,
}


pub struct sprite_renderer {
    program: GlProgram
}

impl sprite_renderer {
    pub fn render(&self, texture: texture::Texture, x: f32, y: f32, rotation_angle: f32) {
        self.set_used();
        let model = glm::Mat4::new();
        model = glm::translate(&model,&glm::vec3(x,y,0.0));
        model = glm::translate(&model,&glm::vec3(texture.width_f() * 0.5, texture.height_f() * 0.5,0.0));
        model = glm::rotate(&model,&glm::radians(rotation_angle),glm::vec3(0.0, 0.0, 1.0));
        model = glm::translate(&model,&glm::vec3(-texture.width_f() * 0.5,- texture.height_f() * 0.5,0.0));
    }

    pub fn init(&self) {

    }
}
