use crate::{
    glm, IndexBuf, ShaderBuilder, ShaderProgram, Vert, VertArray, VertAttr, VertAttrType, VertBuf,
    VertLayout,
};
use anyhow::Result;
use gl;

const VERTEX_SHADER_SOURCE: &str = include_str!("renderer/triangle.vert");
const FRAGMENT_SHADER_SOURCE: &str = include_str!("renderer/triangle.frag");

// TODO: make Renderer trait to implement on Forward and Deferred.

#[allow(dead_code)]
pub struct ForwardRenderer {
    pub shader_program: ShaderProgram,
    vao: VertArray,
}

impl ForwardRenderer {
    pub fn new() -> Result<ForwardRenderer> {
        // set up shader program
        // vertex shader gets called for each vertex in our buffer, it tells opengl where the vertex will be in screen space. Takes in all vertex attributes, like position, and can output data to consecutive shaders (fragment shader).
        // fragment shader gets called for each (potential) pixel that needs to be filled in. Determines the color of the pixel.
        let shader_program = ShaderBuilder::new(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)
            .with_float4("u_Color", glm::vec4(1.0, 1.0, 1.0, 1.0))
            .build();
        shader_program.set_float4("u_Color", glm::vec4(1.0, 0.0, 0.2, 1.0));

        let vertices: Vec<Vert> = vec![
            Vert {
                position: glm::vec3(-0.5, -0.5, 0.0),
                tex_coords: glm::vec2(0.0, 0.0),
            },
            Vert {
                position: glm::vec3(0.5, -0.5, 0.0),
                tex_coords: glm::vec2(1.0, 0.0),
            },
            Vert {
                position: glm::vec3(0.5, 0.5, 0.0),
                tex_coords: glm::vec2(1.0, 1.0),
            },
            Vert {
                position: glm::vec3(-0.5, 0.5, 0.0),
                tex_coords: glm::vec2(0.0, 1.0),
            },
        ];
        let vert_layout = VertLayout::new(vec![
            VertAttr::new(VertAttrType::Float3, false),
            VertAttr::new(VertAttrType::Float2, false),
        ]);
        let vbo = VertBuf::new::<Vert>(&vertices, vert_layout);

        // triangle
        let indices: Vec<u32> = vec![0, 1, 2];
        // square
        // let indices: Vec<u32> = vec![0, 1, 2, 2, 3, 0];
        let ibo = IndexBuf::new(&indices);

        let vao = VertArray::new(&[vbo], ibo);

        Ok(ForwardRenderer {
            shader_program,
            vao,
        })
    }

    pub fn render(&self) -> Result<()> {
        // render
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // draw triangle
        self.vao.set_bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,                   // mode
                self.vao.idx_buf().len() as i32, // number of indices
                gl::UNSIGNED_INT,                // type of an index
                std::ptr::null(),                // pointer to indices, nullptr if already bound.
            );
        }

        Ok(())
    }
}
