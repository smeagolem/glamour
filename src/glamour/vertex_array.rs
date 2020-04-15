use std::convert::TryFrom;

pub struct VertArray {
    id: u32,
    index_buf: IndexBuf,
}

impl VertArray {
    pub fn new(vert_bufs: &[VertBuf], index_buf: IndexBuf) -> Self {
        assert!(vert_bufs.len() > 0, "no vertex buffers!");
        let mut id: u32 = 0;
        gl_call!(gl::GenVertexArrays(1, &mut id));
        gl_call!(gl::BindVertexArray(id));
        for vert_buf in vert_bufs {
            vert_buf.set_bind();
            for (index, attr) in vert_buf.layout.attrs.iter().enumerate() {
                gl_call!(gl::EnableVertexAttribArray(index as u32));
                gl_call!(gl::VertexAttribPointer(
                    index as u32,
                    attr.count() as i32,
                    attr.gl_data_type(),
                    if attr.normalized { gl::TRUE } else { gl::FALSE },
                    vert_buf.layout.stride as i32,
                    attr.offset as *const gl::types::GLvoid,
                ));
            }
        }
        VertArray { id, index_buf }
    }
    pub fn set_bind(&self) {
        gl_call!(gl::BindVertexArray(self.id));
        self.index_buf.set_bind();
    }
    pub fn idx_buf(&self) -> &IndexBuf {
        &self.index_buf
    }
}

pub struct IndexBuf {
    id: u32,
    len: usize,
}

impl IndexBuf {
    pub fn new(indices: &Vec<u32>) -> IndexBuf {
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
        IndexBuf {
            id: id,
            len: indices.len(),
        }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn set_bind(&self) {
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id));
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

pub struct VertBuf {
    id: u32,
    layout: VertLayout,
}

impl VertBuf {
    pub fn new<T>(vertices: &[T], layout: VertLayout) -> Self {
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
        VertBuf { id, layout }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn layout(&self) -> &VertLayout {
        &self.layout
    }
    pub fn set_bind(&self) {
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, self.id))
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
