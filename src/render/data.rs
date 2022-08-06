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
pub type _f32x2 = glm::Vec2;
//pub struct f32x2(glm::Vec2); 
#[allow(non_camel_case_types)]
pub type _f32x3 = glm::Vec3;
#[allow(non_camel_case_types)]
pub type _f32x4 = glm::Vec4;

impl AttributedVertex for f32 {
    fn amount_of_components() -> i32 { 1 }
}

impl AttributedVertex for glm::Vec2 {
    fn amount_of_components() -> i32 { 2 }
}

impl AttributedVertex for glm::Vec3 {
    fn amount_of_components() -> i32 { 3 }
}

impl AttributedVertex for glm::Vec4 {
    fn amount_of_components() -> i32 { 4 }
}

/* 
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]

*/


//todo: rough draft of how this could work

/*  

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32x<const AMOUNT: usize>{
    pub inner: [f32; AMOUNT]
}

impl<const AMOUNT:usize> f32x<AMOUNT> {
    pub fn new(components: [f32; AMOUNT]) -> f32x<AMOUNT> {
        let ret:f32x<AMOUNT>;
        ret.inner = components;
        
        ret
    } 
}

impl<const AMOUNT:usize> AttributedVertex for f32x<AMOUNT> {
    fn amount_of_components() -> i32 {
        AMOUNT as i32
    }
}
 */