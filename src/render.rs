
use crate::render::shader::Shader;
use crate::resources::{self};



use std::ffi::{CString};
use std::iter::Iterator;

pub mod buffer;
pub mod data;
pub mod renderer;
mod shader;
pub mod subtexture;
pub mod texture;
pub mod animation;
//    pub mod line_segment_renderer;

const EXTENSIONS: [(&str, gl::types::GLenum); 2] =
    [(".vert", gl::VERTEX_SHADER), (".frag", gl::FRAGMENT_SHADER)];

pub struct GlProgram {
    id: gl::types::GLuint,
}

impl GlProgram {
    pub fn with_shaders(shaders: &[Shader]) -> Result<GlProgram, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;

        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(GlProgram { id: program_id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn from_res(res: &resources::Resources, name: &str) -> Result<GlProgram, String> {
        // if file extension is glsl, assume both shaders in the same file
        if name.ends_with(".glsl") {
            println!("reading shaders from single file");
            let shaders = Shader::from_single_source(res, name)?;
            return GlProgram::with_shaders(&shaders);
        }

        let file_extensions = EXTENSIONS.map(|x| x.0);

        let shaders = file_extensions
            .map(|x| Shader::from_res(res, &format!("{}{}", name, x)))
            .into_iter()
            .collect::<Result<Vec<Shader>, String>>()?;

        GlProgram::with_shaders(&shaders)
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_float(&self, name: *const u8, value: f32) {
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.id, name as *const i8), value);
        }
    }

    pub fn set_integer(&self, name: *const u8, value: i32) {
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.id, name as *const i8), value);
        }
    }

    // todo: value by ref or copy?
    pub fn set_vector3f(&self, name: *const u8, value: glm::Vec3) {
        unsafe {
            gl::Uniform3f(
                gl::GetUniformLocation(self.id, name as *const i8),
                value.x,
                value.y,
                value.z,
            );
        }
    }

    pub fn set_vector4f(&self, name: *const u8, value: glm::Vec4) {
        unsafe {
            gl::Uniform4f(
                gl::GetUniformLocation(self.id, name as *const i8),
                value.x,
                value.y,
                value.z,
                value.w,
            );
        }
    }

    // todo: arguments here should be reviewed (value_ptr overhead? how does it relate to transpose arg?)
    pub fn set_mat4(&self, name: *const u8, matrix: glm::Mat4) {
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.id, name as *const i8),
                1,
                gl::FALSE,
                glm::value_ptr(&matrix).as_ptr(),
            );
        }
    }
}

impl Drop for GlProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);

    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));

    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
