use gl;
use std::convert::TryFrom;

#[allow(dead_code)]
pub struct IndexBuffer {
    renderer_id: u32,
}

impl IndexBuffer {
    pub fn new(indices: &Vec<u32>) -> IndexBuffer {
        let mut id = 0;
        gl_call!(gl::GenBuffers(1, &mut id));
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id));
        let size =
            gl::types::GLsizeiptr::try_from(indices.len() * std::mem::size_of::<u32>()).unwrap();
        let ptr = indices.as_ptr() as *const gl::types::GLvoid;
        gl_call!(gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER, // target buffer type
            size,                     // size of data in bytes
            ptr,                      // pointer to data
            gl::STATIC_DRAW,          // usage hint
        ));
        IndexBuffer { renderer_id: id }
    }

    pub fn renderer_id(&self) -> u32 {
        self.renderer_id
    }
}
