use gl;

pub trait AttributedVertex {
    unsafe fn vertex_attrib_pointer(stride:usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            Self::amount_of_components(), // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }

    fn amount_of_components() -> i32 {panic!("amount of components not defined for type (this is the default trait implementation)"); }
}

#[allow(non_camel_case_types)]
pub type f32x3 = f32_f32_f32;
#[allow(non_camel_case_types)]
pub type f32x4 = f32_f32_f32_f32;

// todo: this should be a macro or should be refactored somehow
impl AttributedVertex for f32x3 {
    fn amount_of_components() -> i32 {
        3
    }    
}

impl AttributedVertex for f32x4 {
    fn amount_of_components() -> i32 {
        4
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
    pub d3: f32,
}

impl f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32) -> f32x3 {
        f32x3 {
            d0, d1, d2
        }
    }
}

impl f32_f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32, d3: f32) -> f32x4 {
        f32x4 {
            d0, d1, d2, d3
        }
    }
}

impl From<(f32,f32,f32)> for f32x3 {
    fn from(a: (f32,f32,f32)) -> Self {
        f32x3::new(a.0,a.1,a.2)
    }
}

impl From<(f32,f32,f32,f32)> for f32x4 {
    fn from(a: (f32,f32,f32,f32)) -> Self {
        f32x4::new(a.0,a.1,a.2, a.3)
    }
}

//todo: rough draft of how this could work
// #[allow(non_camel_case_types)]
// #[derive(Copy, Clone, Debug)]
// #[repr(C, packed)]
// struct f32x<const AMOUNT: usize>{
//     pub inner: [u8; AMOUNT],
// }

// impl<const AMOUNT:usize> f32x<AMOUNT> {
//     pub fn new(d0: f32, d1: f32, d2: f32, d3: f32) -> f32x4 {
//         f32x4 {
//             d0, d1, d2, d3
//         }
//     }
// }

// impl<const AMOUNT:usize> AttributedVertex for f32x<AMOUNT> {
//     fn amount_of_components() -> i32 {
//         AMOUNT as i32
//     }
// }