    use gl;
    use std;
    use std::ffi::{CString, CStr};
    use std::iter::Iterator;
    use crate::resources::{self, Resources};
    use crate::render::shader::Shader;
    use nalgebra_glm as glm;

    mod shader;
    mod texture;
    pub mod data;
    pub mod buffer;

    const EXTENSIONS: [(&str,gl::types::GLenum); 2] = [(".vert", gl::VERTEX_SHADER),(".frag", gl::FRAGMENT_SHADER)];

        pub struct GlProgram {
            id: gl::types::GLuint,
        }

        impl GlProgram {
            pub fn from_shaders(shaders: &[Shader]) -> Result<GlProgram, String> {
                let program_id = unsafe { gl::CreateProgram() };
        
                for shader in shaders {
                    unsafe { gl::AttachShader(program_id, shader.id()); }
                }
        
                unsafe { gl::LinkProgram(program_id); }

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
                            error.as_ptr() as *mut gl::types::GLchar
                        );
                    }

                    return Err(error.to_string_lossy().into_owned());
                }

                for shader in shaders {
                    unsafe { gl::DetachShader(program_id, shader.id()); }
                }
        
                Ok(GlProgram { id: program_id })
            }
        
            pub fn id(&self) -> gl::types::GLuint {
                self.id
            }
            
            pub fn from_res(res: &resources::Resources, name: &str) -> Result<GlProgram,String> {
                let file_extensions = EXTENSIONS.map(|x| x.0);
                
                let shaders = file_extensions.map(|x|{
                    Shader::from_res(res,&format!("{}{}",name,x)) 
                }).into_iter()
                .collect::<Result<Vec<Shader>, String>>()?;

                GlProgram::from_shaders(&shaders)
            }

            pub fn set_used(&self) {
                unsafe {
                    gl::UseProgram(self.id);
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