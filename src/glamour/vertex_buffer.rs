use gl;
use std::convert::TryFrom;

#[allow(dead_code)]
pub struct VertexBuffer {
    renderer_id: u32,
}

impl VertexBuffer {
    pub fn new<T>(vertices: &Vec<T>) -> VertexBuffer {
        let mut id = 0;
        gl_call!(gl::GenBuffers(1, &mut id));
        // select the buffer as an simple array
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, id));
        let size =
            gl::types::GLsizeiptr::try_from(vertices.len() * std::mem::size_of::<T>()).unwrap();
        let ptr = vertices.as_ptr() as *const gl::types::GLvoid;
        // fill selected buffer with data
        gl_call!(gl::BufferData(
            gl::ARRAY_BUFFER, // target buffer type
            size,             // size of data in bytes
            ptr,              // pointer to data
            gl::STATIC_DRAW,  // usage hint
        ));
        VertexBuffer { renderer_id: id }
    }

    pub fn renderer_id(&self) -> u32 {
        self.renderer_id
    }
}
