use crate::IndexBuffer;
use crate::Vertex;
use crate::VertexBuffer;
use crate::{Program, Shader};
use anyhow::Result;
use gl;
use std::ffi::CString;

const VERTEX_SHADER_SOURCE: &str = include_str!("renderer/triangle.vert");
const FRAGMENT_SHADER_SOURCE: &str = include_str!("renderer/triangle.frag");

// TODO: make Renderer trait to implement on Forward and Deferred.

#[allow(dead_code)]
pub struct ForwardRenderer {
    shader_program: Program,
    vao: gl::types::GLuint,
    ibo: gl::types::GLuint,
    indices_len: gl::types::GLsizei,
}

impl ForwardRenderer {
    pub fn new() -> Result<ForwardRenderer> {
        // set up shader program
        // vertex shader gets called for each vertex in our buffer, it tells opengl where the vertex will be in screen space. Takes in all vertex attributes, like position, and can output data to consecutive shaders (fragment shader).
        // fragment shader gets called for each (potential) pixel that needs to be filled in. Determines the color of the pixel.
        let vert_shader = Shader::new(&CString::new(VERTEX_SHADER_SOURCE)?, gl::VERTEX_SHADER);
        let frag_shader = Shader::new(&CString::new(FRAGMENT_SHADER_SOURCE)?, gl::FRAGMENT_SHADER);
        let shader_program = Program::new(&[vert_shader, frag_shader])?;

        shader_program.set_use();

        // if -1, could not find uniform, might be fine if unused and stripped by shader compilation.
        let u_color = unsafe {
            gl::GetUniformLocation(shader_program.id(), CString::new("u_Color")?.as_ptr())
        };

        unsafe {
            gl::Uniform4f(u_color, 1.0, 0.0, 0.2, 1.0);
        }

        // set up vertex buffer object
        let vertices: Vec<Vertex> = vec![
            Vertex::from_pos(-0.5, -0.5, 0.0),
            Vertex::from_pos(0.5, -0.5, 0.0),
            Vertex::from_pos(0.5, 0.5, 0.0),
            Vertex::from_pos(-0.5, 0.5, 0.0),
        ];
        // generating 1 array buffer to be our vertex buffer object
        let vbo = VertexBuffer::new::<Vertex>(&vertices);

        // set up index buffer object
        // triangle
        // let indices: Vec<u32> = vec![0, 1, 2];
        // square
        let indices: Vec<u32> = vec![0, 1, 2, 2, 3, 0];
        let ibo = IndexBuffer::new(&indices);

        // set up vertex array object
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        // vertex array object binds the vertex buffer and layout config (VertexAttribPointer)
        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo.renderer_id());
            gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader

            // tell opengl how to interpret the vertex data
            gl::VertexAttribPointer(
                0,         // index of the generic vertex attribute ("layout (location = 0)")
                3, // the number of components per generic vertex attribute, 3 floats representing vertex positions
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                std::mem::size_of::<Vertex>() as gl::types::GLint, // stride (byte offset between consecutive attributes)
                Vertex::offset_of_position() as *const gl::types::GLvoid, // offset of the position member of Vertex struct, which is currently 0
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Ok(ForwardRenderer {
            shader_program,
            vao,
            ibo: ibo.renderer_id(),
            indices_len: indices.len() as gl::types::GLsizei,
        })
    }

    pub fn render(&self) -> Result<()> {
        // render
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // draw triangle
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::DrawElements(
                gl::TRIANGLES,    // mode
                self.indices_len, // number of indices
                gl::UNSIGNED_INT, // type of an index
                std::ptr::null(), // pointer to indices, nullptr if already bound.
            );
        }

        Ok(())
    }
}
