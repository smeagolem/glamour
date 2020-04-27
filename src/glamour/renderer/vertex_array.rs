use crate::glm;
use std::convert::TryFrom;

pub struct VertArray {
    id: u32,
    vert_attr_index: u32,
    index_buf: IndexBuf,
}

// https://stackoverflow.com/questions/26552642/when-is-what-bound-to-a-vao
impl VertArray {
    pub fn new(vert_bufs: &[&dyn VertBuffer], index_buf: IndexBuf) -> Self {
        let mut id: u32 = 0;
        gl_call!(gl::GenVertexArrays(1, &mut id));
        gl_call!(gl::BindVertexArray(id));
        index_buf.bind();
        let mut vert_attr_index = 0u32;
        for vert_buf in vert_bufs {
            vert_buf.bind();
            let layout = vert_buf.layout();
            // TODO: check if layout is empty and assert error if so.
            for attr in layout.attrs.iter() {
                match attr.attr_type {
                    VertAttrType::Mat4 => {
                        for index in 0..4 {
                            let attr_index = vert_attr_index + index;
                            gl_call!(gl::EnableVertexAttribArray(attr_index));
                            gl_call!(gl::VertexAttribPointer(
                                attr_index,
                                4 as i32,
                                attr.gl_data_type(),
                                if attr.normalized { gl::TRUE } else { gl::FALSE },
                                std::mem::size_of::<glm::Mat4>() as i32,
                                (attr.gl_data_type_size() * index as usize * 4)
                                    as *const gl::types::GLvoid,
                            ));
                            // FIXME: this should only run if attr is for instancing
                            gl_call!(gl::VertexAttribDivisor(attr_index, 1));
                        }
                        vert_attr_index += 4;
                    }
                    VertAttrType::Mat3 => {
                        for index in 0..3 {
                            let attr_index = vert_attr_index + index;
                            let size = if index == 0 || index == 1 { 3 } else { 1 };
                            gl_call!(gl::EnableVertexAttribArray(attr_index));
                            gl_call!(gl::VertexAttribPointer(
                                attr_index,
                                size as i32,
                                attr.gl_data_type(),
                                if attr.normalized { gl::TRUE } else { gl::FALSE },
                                std::mem::size_of::<glm::Mat3>() as i32,
                                (attr.gl_data_type_size() * index as usize * 4)
                                    as *const gl::types::GLvoid,
                            ));
                            // FIXME: this should only run if attr is for instancing
                            gl_call!(gl::VertexAttribDivisor(attr_index, 1));
                        }
                        vert_attr_index += 3;
                    }
                    _ => {
                        gl_call!(gl::EnableVertexAttribArray(vert_attr_index));
                        gl_call!(gl::VertexAttribPointer(
                            vert_attr_index,
                            attr.count() as i32,
                            attr.gl_data_type(),
                            if attr.normalized { gl::TRUE } else { gl::FALSE },
                            layout.stride as i32,
                            attr.offset as *const gl::types::GLvoid,
                        ));
                        vert_attr_index += 1;
                    }
                }
            }
            vert_buf.unbind();
        }
        gl_call!(gl::BindVertexArray(0));
        index_buf.unbind();
        VertArray {
            id,
            vert_attr_index,
            index_buf,
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
    pub fn push_buf(&mut self, vert_buf: &dyn VertBuffer) {
        self.bind();
        vert_buf.bind();
        let layout = vert_buf.layout();
        for attr in layout.attrs.iter() {
            gl_call!(gl::EnableVertexAttribArray(self.vert_attr_index));
            gl_call!(gl::VertexAttribPointer(
                self.vert_attr_index,
                attr.count() as i32,
                attr.gl_data_type(),
                if attr.normalized { gl::TRUE } else { gl::FALSE },
                layout.stride as i32,
                attr.offset as *const gl::types::GLvoid,
            ));
            self.vert_attr_index += 1;
        }
        self.unbind();
        vert_buf.unbind();
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

pub trait VertBuffer {
    fn layout(&self) -> &VertLayout;
    fn bind(&self);
    fn unbind(&self);
}

impl<T: Vert> VertBuffer for VertBuf<T> {
    fn layout(&self) -> &VertLayout {
        &self.layout
    }
    fn bind(&self) {
        self.bind();
    }
    fn unbind(&self) {
        self.unbind();
    }
}

pub struct VertBuf<T: Vert> {
    id: u32,
    vertices: Vec<T>,
    layout: VertLayout,
}

impl<T: Vert> VertBuf<T> {
    pub fn new(vertices: Vec<T>) -> Self {
        let mut id = 0;
        gl_call!(gl::GenBuffers(1, &mut id));
        // select the buffer as an simple array
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, id));
        // TODO: maybe use the layout sizes for this...
        let size = gl::types::GLsizeiptr::try_from(vertices.capacity() * std::mem::size_of::<T>())
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
            layout: T::layout(),
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
    pub fn vertices(&self) -> &[T] {
        &self.vertices
    }
    pub fn vertices_mut(&mut self) -> &mut Vec<T> {
        &mut self.vertices
    }
    pub fn set_data(&self) {
        self.bind();
        let size = gl::types::GLsizeiptr::try_from(self.vertices.len() * std::mem::size_of::<T>())
            .unwrap();
        let ptr = self.vertices.as_ptr() as *const gl::types::GLvoid;
        // fill selected buffer with data
        gl_call!(gl::BufferSubData(gl::ARRAY_BUFFER, 0, size, ptr));
        self.unbind();
        // self.vertices = vertices;
    }
}

impl<T: Vert> Drop for VertBuf<T> {
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
            VertAttrType::Mat3 => 3 * 3,
            VertAttrType::Mat4 => 4 * 4,
        }
    }
    pub fn size(&self) -> usize {
        self.gl_data_type_size() * self.count()
    }
    pub fn gl_data_type(&self) -> u32 {
        match self.attr_type {
            VertAttrType::Float2 => gl::FLOAT,
            VertAttrType::Float3 => gl::FLOAT,
            VertAttrType::Mat3 => gl::FLOAT,
            VertAttrType::Mat4 => gl::FLOAT,
        }
    }
    pub fn gl_data_type_size(&self) -> usize {
        match self.gl_data_type() {
            gl::FLOAT => 4,
            _ => panic!("unsupported data type"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum VertAttrType {
    Float2,
    Float3,
    Mat3,
    Mat4,
}

pub trait Vert {
    fn layout() -> VertLayout;
}
