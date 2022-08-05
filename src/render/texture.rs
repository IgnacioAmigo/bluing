use std::ffi::c_void;
use gl::types::{GLint, GLuint};

use gl;
// https://learnopengl.com/code_viewer_gh.php?code=src/7.in_practice/3.2d_game/0.full_source/texture.h 
pub struct Texture {
    id: gl::types::GLuint,
    width: usize,
    height: usize,
}

impl Texture {
    pub fn new(width: usize, height: usize) -> Texture {
        let mut id = 0;
        unsafe {gl::GenTextures(1,&mut id);}

        Texture{id, width, height}
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn width_f(&self) -> f32 { self.width as f32 }
    pub fn height_f(&self) -> f32 { self.height as f32 }

    pub fn id(&self ) -> gl::types::GLuint { self.id }

    pub fn from_data(data: Vec<u8>, width: usize, height: usize) -> Result<Texture, String> {
        let texture = Texture::new(width, height);
        print!("id of texture is {}", texture.id);
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            
            gl::ActiveTexture(gl::TEXTURE0);
            //https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameter.xhtml#:~:text=to%20GL_REPEAT.-,Notes,-Suppose%20that%20a
            // todo: image data should be freed here, probably
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width as i32, height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, data.as_ptr() as *const gl::types::GLvoid);
            
            // set Texture wrap and filter modes
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER,gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0 as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

            //gl::GenerateMipmap(gl::TEXTURE_2D);
            // unbind texture
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(texture)
    }
    
    pub fn with_white_new() -> Result<Texture, String>{
        // todo: maybe we should assert that it was not created before?
        return Texture::from_data(vec![0x0], 1, 1);
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0); }
    }

}