use gl;

pub type VertexBuffer = Buffer<{gl::ARRAY_BUFFER}>;
pub type ElementBuffer = Buffer<{gl::ELEMENT_ARRAY_BUFFER}>;
pub type SSVertexBuffer = Buffer<{gl::SHADER_STORAGE_BUFFER}>;

pub struct Buffer<const B: gl::types::GLuint> {
    vbo: gl::types::GLuint,
}

impl<const B: gl::types::GLuint> Buffer<B> {
    pub fn new() -> Buffer<{B}> {
        let mut vbo: gl::types::GLuint = 0;
        println!("creado vbo {}", B);
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        println!("creado vbo id {}", vbo );

        Buffer { vbo }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(B, self.vbo); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(B, 0); }
    }

    pub fn upload_data_static_draw<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData( 
                B,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            )
        }
    }
}
/* 
impl<const B:gl::types::GLuint> Drop for Buffer<B> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
} */

pub struct VertexArray {
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vao: gl::types::GLuint = 0;
        unsafe {gl::GenVertexArrays(1, &mut vao);}

        VertexArray { vao }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.vao); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0); }
    }
}

/* impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
    }
} */
