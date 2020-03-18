use crate::vertex_buffer::VertexBuffer;
use anyhow::Result;
use gl;

#[allow(dead_code)]
pub struct VertexArray {
    renderer_id: u32,
}

impl VertexArray {
    pub fn new() -> Result<VertexArray> {
        let mut id: u32 = 0;
        gl_call!(gl::GenVertexArrays(1, &mut id))?;
        VertexArray { renderer_id: id }
    }

    pub fn add_vertex_buffer(vbo: &VertexBuffer, layout: &VertexLayout) -> Result<()> {}
}

pub struct VertexLayout {}

pub struct VertexLayoutAttribute {}
