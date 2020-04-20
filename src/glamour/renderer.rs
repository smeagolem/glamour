use crate::{
    glm, Cube, IndexBuf, ShaderBuilder, ShaderProgram, Texture, Vert, VertArray, VertAttr,
    VertAttrType, VertBuf, VertLayout,
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
    tex: Texture,
    pub cube_transforms: Vec<glm::Mat4>,
}

impl ForwardRenderer {
    pub fn new() -> Result<ForwardRenderer> {
        let model: glm::Mat4 = glm::identity();
        let model = glm::rotate_x(&model, glm::radians(&glm::vec1(-55.0)).x);
        let view: glm::Mat4 = glm::identity();
        let view = glm::translate(&view, &glm::vec3(0.0, 0.0, -2.0));
        let projection: glm::Mat4 =
            glm::perspective(1.0, glm::radians(&glm::vec1(90.0)).x, 0.1, 100.0);

        // set up shader program
        // vertex shader gets called for each vertex in our buffer, it tells opengl where the vertex will be in screen space. Takes in all vertex attributes, like position, and can output data to consecutive shaders (fragment shader).
        // fragment shader gets called for each (potential) pixel that needs to be filled in. Determines the color of the pixel.
        let shader_program = ShaderBuilder::new(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)
            .with_mat4("u_model", model)
            .with_mat4("u_view", view)
            .with_mat4("u_projection", projection)
            .with_float4("u_color", glm::vec4(1.0, 1.0, 1.0, 1.0))
            .build();
        shader_program.set_float4("u_color", &glm::vec4(1.0, 0.0, 0.2, 1.0));

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
        let vbo = VertBuf::new(&vertices, vert_layout);

        // triangle
        // let indices: Vec<u32> = vec![0, 1, 2];
        // square
        let indices: Vec<u32> = vec![0, 1, 2, 2, 3, 0];
        let ibo = IndexBuf::new(&indices);

        let vao = VertArray::new(&[vbo], ibo);

        let img_path = crate::assets_path().join("container.jpg");
        let tex = Texture::new(&img_path);

        let cube = Cube::new();

        cube.shader.set_mat4("u_model", &model);
        cube.shader.set_mat4("u_view", &view);
        cube.shader.set_mat4("u_projection", &projection);

        Ok(ForwardRenderer {
            shader_program: cube.shader,
            vao: cube.vert_arr,
            tex: cube.texture,
            cube_transforms: Vec::new(),
        })
    }

    pub fn render(&self) -> Result<()> {
        // render
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // draw triangle
        self.vao.set_bind();
        self.tex.set_bind();
        for transform in &self.cube_transforms {
            self.shader_program.set_mat4("u_model", transform);
            unsafe {
                gl::DrawElements(
                    gl::TRIANGLES,                   // mode
                    self.vao.idx_buf().len() as i32, // number of indices
                    gl::UNSIGNED_INT,                // type of an index
                    std::ptr::null(), // pointer to indices, nullptr if already bound.
                );
            }
        }

        Ok(())
    }
}
