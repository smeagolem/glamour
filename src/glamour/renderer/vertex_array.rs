use crate::Vert;
use std::convert::TryFrom;

pub struct VertArray {
    id: u32,
    vert_attr_index: u32,
    index_buf: IndexBuf,
    vert_bufs: Vec<VertBuf>,
}

// https://stackoverflow.com/questions/26552642/when-is-what-bound-to-a-vao
impl VertArray {
    pub fn new(vert_bufs: Vec<VertBuf>, index_buf: IndexBuf) -> Self {
        // assert!(vert_bufs.len() > 0, "no vertex buffers!");
        let mut id: u32 = 0;
        gl_call!(gl::GenVertexArrays(1, &mut id));
        gl_call!(gl::BindVertexArray(id));
        index_buf.bind();
        let mut vert_attr_index = 0u32;
        for vert_buf in &vert_bufs {
            vert_buf.bind();
            for attr in vert_buf.layout.attrs.iter() {
                gl_call!(gl::EnableVertexAttribArray(vert_attr_index));
                gl_call!(gl::VertexAttribPointer(
                    vert_attr_index,
                    attr.count() as i32,
                    attr.gl_data_type(),
                    if attr.normalized { gl::TRUE } else { gl::FALSE },
                    vert_buf.layout.stride as i32,
                    attr.offset as *const gl::types::GLvoid,
                ));
                vert_attr_index += 1;
            }
            vert_buf.unbind();
        }
        gl_call!(gl::BindVertexArray(0));
        index_buf.unbind();
        VertArray {
            id,
            vert_attr_index,
            index_buf,
            vert_bufs,
        }
    }
    pub fn bind(&self) {
        gl_call!(gl::BindVertexArray(self.id));
    }
    pub fn unbind(&self) {
        gl_call!(gl::BindVertexArray(0));
    }
    pub fn index_buf(&self) -> &IndexBuf {
        &self.index_buf
    }
    pub fn index_buf_mut(&mut self) -> &mut IndexBuf {
        &mut self.index_buf
    }
    pub fn set_index_buf(&mut self, index_buf: IndexBuf) {
        self.bind();
        index_buf.bind();
        self.unbind();
        index_buf.unbind();
        self.index_buf = index_buf;
    }
    pub fn push_buf(&mut self, vert_buf: VertBuf) {
        self.bind();
        vert_buf.bind();
        for attr in vert_buf.layout.attrs.iter() {
            gl_call!(gl::EnableVertexAttribArray(self.vert_attr_index));
            gl_call!(gl::VertexAttribPointer(
                self.vert_attr_index,
                attr.count() as i32,
                attr.gl_data_type(),
                if attr.normalized { gl::TRUE } else { gl::FALSE },
                vert_buf.layout.stride as i32,
                attr.offset as *const gl::types::GLvoid,
            ));
            self.vert_attr_index += 1;
        }
        self.unbind();
        vert_buf.unbind();
        self.vert_bufs.push(vert_buf);
    }
    pub fn vert_bufs(&self) -> &[VertBuf] {
        &self.vert_bufs
    }
    pub fn vert_bufs_mut(&mut self) -> &mut [VertBuf] {
        &mut self.vert_bufs
    }
}

impl Drop for VertArray {
    fn drop(&mut self) {
        gl_call!(gl::DeleteVertexArrays(1, &self.id));
    }
}

pub struct IndexBuf {
    id: u32,
    indices: Vec<u32>,
}

impl IndexBuf {
    pub fn new(indices: Vec<u32>) -> IndexBuf {
        let mut id = 0;
        gl_call!(gl::GenBuffers(1, &mut id));
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id));
        let size = gl::types::GLsizeiptr::try_from(indices.capacity() * std::mem::size_of::<u32>())
            .unwrap();
        let ptr = indices.as_ptr() as *const gl::types::GLvoid;
        gl_call!(gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER, // target buffer type
            size,                     // size of data in bytes
            ptr,                      // pointer to data
            gl::STATIC_DRAW,          // usage hint
        ));
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0));
        IndexBuf { id: id, indices }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn bind(&self) {
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id));
    }
    pub fn unbind(&self) {
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0));
    }
    pub fn len(&self) -> usize {
        self.indices.len()
    }
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }
    pub fn indices_mut(&mut self) -> &mut Vec<u32> {
        &mut self.indices
    }
    pub fn set_data(&self) {
        self.bind();
        let size = gl::types::GLsizeiptr::try_from(self.indices.len() * std::mem::size_of::<u32>())
            .unwrap();
        let ptr = self.indices.as_ptr() as *const gl::types::GLvoid;
        gl_call!(gl::BufferSubData(gl::ELEMENT_ARRAY_BUFFER, 0, size, ptr));
        self.unbind();
    }
}

impl Drop for IndexBuf {
    fn drop(&mut self) {
        gl_call!(gl::DeleteBuffers(1, &self.id));
    }
}

pub struct VertBuf {
    id: u32,
    vertices: Vec<Vert>,
    layout: VertLayout,
}

impl VertBuf {
    pub fn new(vertices: Vec<Vert>, layout: VertLayout) -> Self {
        let mut id = 0;
        gl_call!(gl::GenBuffers(1, &mut id));
        // select the buffer as an simple array
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, id));
        // TODO: maybe use the layout sizes for this...
        let size =
            gl::types::GLsizeiptr::try_from(vertices.capacity() * std::mem::size_of::<Vert>())
                .unwrap();
        let ptr = vertices.as_ptr() as *const gl::types::GLvoid;
        // fill selected buffer with data
        gl_call!(gl::BufferData(
            gl::ARRAY_BUFFER, // target buffer type
            size,             // size of data in bytes
            ptr,              // pointer to data
            gl::STATIC_DRAW,  // usage hint
        ));
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, 0));
        VertBuf {
            id,
            vertices,
            layout,
        }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn layout(&self) -> &VertLayout {
        &self.layout
    }
    pub fn bind(&self) {
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, self.id))
    }
    pub fn unbind(&self) {
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, 0));
    }
    pub fn vertices(&self) -> &[Vert] {
        &self.vertices
    }
    pub fn vertices_mut(&mut self) -> &mut Vec<Vert> {
        &mut self.vertices
    }
    pub fn set_data(&self) {
        self.bind();
        let size =
            gl::types::GLsizeiptr::try_from(self.vertices.len() * std::mem::size_of::<Vert>())
                .unwrap();
        let ptr = self.vertices.as_ptr() as *const gl::types::GLvoid;
        // fill selected buffer with data
        gl_call!(gl::BufferSubData(gl::ARRAY_BUFFER, 0, size, ptr));
        self.unbind();
        // self.vertices = vertices;
    }
}

impl Drop for VertBuf {
    fn drop(&mut self) {
        gl_call!(gl::DeleteBuffers(1, &self.id));
    }
}

#[derive(Debug)]
pub struct VertLayout {
    attrs: Vec<VertAttr>,
    stride: u32,
}

impl VertLayout {
    pub fn new(mut attrs: Vec<VertAttr>) -> Self {
        let mut offset: usize = 0;
        let mut stride = 0;
        let attrs = attrs
            .iter_mut()
            .map(|vert_attr| {
                vert_attr.offset = offset;
                let size = vert_attr.size();
                offset += size;
                stride += size;
                *vert_attr
            })
            .collect();
        VertLayout {
            attrs,
            stride: stride as u32,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct VertAttr {
    attr_type: VertAttrType,
    normalized: bool,
    offset: usize,
}

impl VertAttr {
    pub fn new(attr_type: VertAttrType, normalized: bool) -> Self {
        VertAttr {
            attr_type,
            normalized,
            offset: 0,
        }
    }
    pub fn count(&self) -> usize {
        match self.attr_type {
            VertAttrType::Float2 => 2,
            VertAttrType::Float3 => 3,
        }
    }
    pub fn size(&self) -> usize {
        match self.attr_type {
            VertAttrType::Float2 => 4 * 2,
            VertAttrType::Float3 => 4 * 3,
        }
    }
    pub fn gl_data_type(&self) -> u32 {
        match self.attr_type {
            VertAttrType::Float2 => gl::FLOAT,
            VertAttrType::Float3 => gl::FLOAT,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum VertAttrType {
    Float2,
    Float3,
}
