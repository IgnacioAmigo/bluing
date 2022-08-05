use gl;
use std;
use std::ffi::{CString, CStr};
use crate::resources::{Resources};
use glm;

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_single_source(res: &Resources, name: &str) -> Result<Vec<Shader>, String> {
        let source = res.load_cstring(name).expect("Error loading shader file");
        let str_source = source.to_str().unwrap();

        let frag_directive = "!frag";
        let geo_directive = "!geo";
        let vertex_directive = "!vert";

        let frag_index = match str_source.find(frag_directive) {
            Some(index) => index,
            None => panic!("Error loading fragment shader code for {}, directive not found", name),
        };

        let _geo_index = match str_source.find(geo_directive) {
            Some(index) => index,
            None => {println!("Geo shader code for {}, directive not found", name);0},
        };

        let vertex_index = match str_source.find(vertex_directive) {
            Some(index) => index,
            None => panic!("Error loading vertex shader code for {}; directive not found", name),
        };

        // returns next ocurrence of _index_ or string length if none found
        fn find_after(source: &str, index: usize) -> usize {
            source.match_indices("\n!").find_map(|(i, _)| (i > index).then(|| i)).or(Some(source.as_bytes().len())).unwrap()
        }

        // todo: this is all very inefficient; no need for CString at beginning of function and then this
        let vert_shader_string = str_source[vertex_index + vertex_directive.len()..find_after(&str_source, vertex_index)].to_string();
        let frag_shader_string = str_source[frag_index + frag_directive.len()..find_after(&str_source, frag_index)].to_string();

        let vertex_shader = Shader::vert_shader_from_source(&CString::new(vert_shader_string).unwrap()).unwrap();
        let frag_shader = Shader::frag_shader_from_source(&CString::new(frag_shader_string).unwrap()).unwrap();

        Ok(vec![vertex_shader,frag_shader])
    } 

    fn from_source(source: &CStr, kind: u32) -> Result<Shader, String> {
        let shader_id = shader_from_source(source, kind)?;
        Ok(Shader { id: shader_id })
    }
    pub fn vert_shader_from_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    } 
    
    pub fn frag_shader_from_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    } 

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn from_res(res: &Resources, name: &str) -> Result<Shader, String> {
        
        // TODO: this should not panic, and instead print to console or debug log
        println!("about to do shader {}", name);

        let shader_type = super::EXTENSIONS.iter()
        .find(|&&x|
            name.ends_with(x.0)
        ).map(|&x| x.1).expect(&format!("Couldn't determine shader type for {}",name));
        let source = res.load_cstring(name).expect("Error loading shader");
        println!("returned shader {}", source.to_string_lossy());
        Shader::from_source(&source, shader_type)
    }

}
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        
        // todo: review this, looks pretty dumb
        let error: CString = super::create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
            id,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar
            );
        }
    
        return Err(error.to_string_lossy().into_owned());
    }
    
    Ok(id)
}