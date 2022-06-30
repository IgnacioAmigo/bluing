use std::ffi::c_void;

use gl;
// https://learnopengl.com/code_viewer_gh.php?code=src/7.in_practice/3.2d_game/0.full_source/texture.h 
pub struct Texture {
    id: gl::types::GLuint,
    width: usize,
    height: usize,
}

impl Texture {
    fn new( width: usize, height: usize) -> Texture {
        let mut id = 0;
        unsafe {gl::GenTextures(1,&mut id);}

        Texture{id, width, height}
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }


    pub fn width_f(&self) -> f32 { self.width as f32 }
    pub fn height_f(&self) -> f32 { self.height as f32 }


    pub fn from_data(data: Vec<u8>, width: usize, height: usize) -> Result<Texture, String> {
        let mut texture = Texture::new(width, height);
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width as i32, height as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, data.as_ptr() as *const gl::types::GLvoid);

            // set Texture wrap and filter modes
            gl::TextureParameterIuiv(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, &gl::CLAMP_TO_EDGE);
            gl::TextureParameterIuiv(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, &gl::CLAMP_TO_EDGE);
            gl::TextureParameterIuiv(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER,& gl::LINEAR);
            gl::TextureParameterIuiv(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, &gl::LINEAR);

            // unbind texture
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(texture)
    }


}