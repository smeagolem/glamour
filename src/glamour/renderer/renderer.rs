use crate::{
    glm, Camera, IndexBuf, ShaderBuilder, ShaderProgram, Texture, Transform, VertArray, VertBasic,
    VertBuf, VertTrans,
};
use gl;

// TODO: make Renderer trait to implement on Forward and Deferred.

#[allow(dead_code)]
pub struct ForwardRenderer {
    cube_shader: ShaderProgram,
    cube_vao: VertArray,
    cube_vbo: VertBuf<VertBasic>,
    pub cube_trans_vbo: VertBuf<VertTrans>,
    pub cube_norm_vbo: VertBuf<VertTrans>,
    cube_tex: Texture,
    light_shader: ShaderProgram,
    light_vao: VertArray,
    light_vbo: VertBuf<VertBasic>,
}

impl ForwardRenderer {
    pub fn new() -> ForwardRenderer {
        gl_call!(gl::Enable(gl::DEPTH_TEST));

        let cube_shader =
            ShaderBuilder::new(include_str!("triangle.vert"), include_str!("triangle.frag"))
                .with_float4("u_color", glm::vec4(1.0, 1.0, 1.0, 1.0))
                .build();
        let img_path = crate::assets_path().join("container.jpg");
        let cube_tex = Texture::new(&img_path);
        // TODO: check in draw functions if overflowing buffer, if so, draw (flush and reset).
        let max_cubes = 1_000_000;
        let cube_vertices = cube_vertices();
        let cube_vertices_len = cube_vertices.len();
        // let max_vertices = max_cubes * cube_vertices().len();
        let cube_vbo = VertBuf::<VertBasic>::new(cube_vertices);
        let cube_trans_vbo = VertBuf::<VertTrans>::new(Vec::with_capacity(max_cubes));
        let cube_norm_vbo = VertBuf::<VertTrans>::new(Vec::with_capacity(max_cubes));
        let ibo = IndexBuf::new((0..cube_vertices_len as u32).collect());
        let cube_vao = VertArray::new(&[&cube_vbo, &cube_trans_vbo, &cube_norm_vbo], ibo);

        let light_shader =
            ShaderBuilder::new(include_str!("light.vert"), include_str!("light.frag"))
                .with_float4("u_color", glm::vec4(1.0, 1.0, 1.0, 1.0))
                .build();
        // TODO: check in draw functions if overflowing buffer, if so, draw (flush and reset).
        let max_vertices = 100_000;
        let light_vbo = VertBuf::<VertBasic>::new(Vec::with_capacity(max_vertices));
        let ibo = IndexBuf::new(Vec::with_capacity(max_vertices));
        let light_vao = VertArray::new(&[&light_vbo], ibo);

        ForwardRenderer {
            cube_shader,
            cube_vao,
            cube_vbo,
            cube_trans_vbo,
            cube_norm_vbo,
            cube_tex,
            light_shader,
            light_vao,
            light_vbo,
        }
    }

    pub fn shader(&self) -> &ShaderProgram {
        &self.cube_shader
    }

    fn cube_vert_buf(&self) -> &VertBuf<VertBasic> {
        &self.cube_vbo
    }

    fn cube_vert_buf_mut(&mut self) -> &mut VertBuf<VertBasic> {
        &mut self.cube_vbo
    }

    fn light_vert_buf(&self) -> &VertBuf<VertBasic> {
        &self.light_vbo
    }

    fn light_vert_buf_mut(&mut self) -> &mut VertBuf<VertBasic> {
        &mut self.light_vbo
    }

    pub fn clear(&self) {
        gl_call!(gl::ClearColor(0.3, 0.3, 0.5, 1.0));
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));
    }

    pub fn begin_draw(&self, camera: &Camera) {
        let vp_mat = camera.view_projection_matrix();

        self.light_shader.bind();
        self.light_shader.set_mat4("u_view_projection", &vp_mat);
        self.light_shader.unbind();

        self.cube_shader.bind();
        self.cube_shader.set_mat4("u_view_projection", &vp_mat);
        self.cube_shader.set_float3("u_view_pos", &camera.position);
        self.cube_shader.unbind();
    }

    pub fn end_draw(&mut self) {
        // draw lights
        self.light_vert_buf().set_data();
        self.light_vao.index_buf().set_data();
        self.light_shader.bind();
        self.light_vao.bind();
        gl_call!(gl::DrawElements(
            gl::TRIANGLES,
            // 0 as i32,
            self.light_vao.index_buf().len() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        ));
        self.light_vao.unbind();
        self.light_shader.unbind();
        self.light_vert_buf_mut().vertices_mut().clear();
        self.light_vao.index_buf_mut().indices_mut().clear();

        // draw cubes
        // self.cube_vert_buf().set_data();
        // self.cube_vao.index_buf().set_data();

        // self.cube_trans_vbo.set_data();

        self.cube_shader.bind();
        self.cube_tex.bind();
        self.cube_vao.bind();
        // println!("{:#?}", self.cube_vao.index_buf().indices());
        // println!("{:#?}", self.cube_vbo.vertices().len());
        // println!("{:#?}", self.cube_trans_vbo.vertices().len());
        gl_call!(gl::DrawElementsInstanced(
            gl::TRIANGLES,                          // mode
            self.cube_vao.index_buf().len() as i32, // number of indices
            gl::UNSIGNED_INT,                       // type of an index
            std::ptr::null(),                       // pointer to indices, nullptr if already bound.
            self.cube_trans_vbo.vertices().len() as i32,
        ));
        self.cube_vao.unbind();
        self.cube_tex.unbind();
        self.cube_shader.unbind();

        // self.cube_trans_vbo.vertices_mut().clear();

        // self.cube_vert_buf_mut().vertices_mut().clear();
        // self.cube_vao.index_buf_mut().indices_mut().clear();
    }

    pub fn draw_light(&mut self, transform: &Transform) {
        // create new vertices from cube vertices.
        let mut verts = cube_vertices();

        // transform each vertex position by the transform.
        let trans_mat = transform.matrix();
        for vert in verts.iter_mut() {
            let pos = glm::vec4(vert.position.x, vert.position.y, vert.position.z, 1.0);
            vert.position = (trans_mat * pos).xyz();
        }

        // add vertices to vertec buffer.
        let vert_buf = self.light_vert_buf_mut();
        let vertices = vert_buf.vertices_mut();
        let vl_pre = vertices.len();
        vertices.append(&mut verts);
        let vl_post = vertices.len();

        // update index buffer
        let indices: &mut Vec<u32> = self.light_vao.index_buf_mut().indices_mut();
        let mut new_indices: Vec<u32> = (vl_pre as u32..vl_post as u32).collect();
        indices.append(&mut new_indices);

        self.cube_shader
            .set_float3("u_light_pos", &transform.position);
    }

    pub fn draw_cube(&mut self, transform: &Transform) {
        let vertices = self.cube_trans_vbo.vertices_mut();
        let trans_mat = transform.matrix();
        vertices.append(&mut vec![VertTrans {
            transform: trans_mat,
            // normal: glm::inverse_transpose(trans_mat),
            // normal: glm::mat4_to_mat3(&glm::inverse_transpose(trans_mat)),
        }]);
        self.cube_norm_vbo
            .vertices_mut()
            .append(&mut vec![VertTrans {
                transform: glm::inverse_transpose(trans_mat),
            }]);
    }

    pub fn draw_quad(&mut self, transform: &Transform) {
        // create new vertices
        let mut verts = vec![
            VertBasic {
                position: glm::vec3(-0.5, -0.5, 0.0),
                normal: glm::vec3(0.0, 0.0, 1.0),
                tex_coords: glm::vec2(0.0, 0.0),
            },
            VertBasic {
                position: glm::vec3(0.5, -0.5, 0.0),
                normal: glm::vec3(0.0, 0.0, 1.0),
                tex_coords: glm::vec2(1.0, 0.0),
            },
            VertBasic {
                position: glm::vec3(0.5, 0.5, 0.0),
                normal: glm::vec3(0.0, 0.0, 1.0),
                tex_coords: glm::vec2(1.0, 1.0),
            },
            VertBasic {
                position: glm::vec3(-0.5, 0.5, 0.0),
                normal: glm::vec3(0.0, 0.0, 1.0),
                tex_coords: glm::vec2(0.0, 1.0),
            },
        ];

        // transform each vertex position by the transform.
        let trans_mat = transform.matrix();
        let norm_mat = glm::mat4_to_mat3(&glm::inverse_transpose(trans_mat));
        for vert in verts.iter_mut() {
            let pos = glm::vec4(vert.position.x, vert.position.y, vert.position.z, 1.0);
            vert.position = (trans_mat * pos).xyz();
            vert.normal = norm_mat * vert.normal;
        }

        // add vertices to vertec buffer.
        let vert_buf = self.cube_vert_buf_mut();
        let vertices = vert_buf.vertices_mut();
        let vl = vertices.len() as u32;
        vertices.append(&mut verts);

        // update index buffer
        let mut indices: Vec<u32> = vec![vl, vl + 1, vl + 2, vl + 2, vl + 3, vl];
        self.cube_vao
            .index_buf_mut()
            .indices_mut()
            .append(&mut indices);
    }

    pub fn draw_triangle(&mut self, transform: &Transform) {
        // create new vertices
        let mut verts = vec![
            VertBasic {
                position: glm::vec3(-0.5, -0.5, 0.0),
                normal: glm::vec3(0.0, 0.0, 1.0),
                tex_coords: glm::vec2(0.0, 0.0),
            },
            VertBasic {
                position: glm::vec3(0.5, -0.5, 0.0),
                normal: glm::vec3(0.0, 0.0, 1.0),
                tex_coords: glm::vec2(1.0, 0.0),
            },
            VertBasic {
                position: glm::vec3(0.0, 0.5, 0.0),
                normal: glm::vec3(0.0, 0.0, 1.0),
                tex_coords: glm::vec2(0.5, 1.0),
            },
        ];

        // transform each vertex position by the transform.
        let trans_mat = transform.matrix();
        let norm_mat = glm::mat4_to_mat3(&glm::inverse_transpose(trans_mat));
        for vert in verts.iter_mut() {
            let pos = glm::vec4(vert.position.x, vert.position.y, vert.position.z, 1.0);
            vert.position = (trans_mat * pos).xyz();
            vert.normal = norm_mat * vert.normal;
        }

        // add vertices to vertec buffer.
        let vert_buf = self.cube_vert_buf_mut();
        let vertices = vert_buf.vertices_mut();
        let vl = vertices.len() as u32;
        vertices.append(&mut verts);

        // update index buffer
        let mut indices: Vec<u32> = vec![vl, vl + 1, vl + 2];
        self.cube_vao
            .index_buf_mut()
            .indices_mut()
            .append(&mut indices);
    }
}

fn cube_vertices() -> Vec<VertBasic> {
    vec![
        VertBasic {
            position: glm::vec3(-0.5, -0.5, -0.5),
            normal: glm::vec3(0.0, 0.0, -1.0),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(0.5, -0.5, -0.5),
            normal: glm::vec3(0.0, 0.0, -1.0),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(0.5, 0.5, -0.5),
            normal: glm::vec3(0.0, 0.0, -1.0),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(0.5, 0.5, -0.5),
            normal: glm::vec3(0.0, 0.0, -1.0),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, 0.5, -0.5),
            normal: glm::vec3(0.0, 0.0, -1.0),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, -0.5, -0.5),
            normal: glm::vec3(0.0, 0.0, -1.0),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, -0.5, 0.5),
            normal: glm::vec3(0.0, 0.0, 1.0),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(0.5, -0.5, 0.5),
            normal: glm::vec3(0.0, 0.0, 1.0),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(0.5, 0.5, 0.5),
            normal: glm::vec3(0.0, 0.0, 1.0),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(0.5, 0.5, 0.5),
            normal: glm::vec3(0.0, 0.0, 1.0),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, 0.5, 0.5),
            normal: glm::vec3(0.0, 0.0, 1.0),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, -0.5, 0.5),
            normal: glm::vec3(0.0, 0.0, 1.0),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, 0.5, 0.5),
            normal: glm::vec3(-1.0, 0.0, 0.0),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, 0.5, -0.5),
            normal: glm::vec3(-1.0, 0.0, 0.0),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, -0.5, -0.5),
            normal: glm::vec3(-1.0, 0.0, 0.0),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, -0.5, -0.5),
            normal: glm::vec3(-1.0, 0.0, 0.0),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, -0.5, 0.5),
            normal: glm::vec3(-1.0, 0.0, 0.0),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, 0.5, 0.5),
            normal: glm::vec3(-1.0, 0.0, 0.0),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(0.5, 0.5, 0.5),
            normal: glm::vec3(1.0, 0.0, 0.0),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(0.5, 0.5, -0.5),
            normal: glm::vec3(1.0, 0.0, 0.0),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(0.5, -0.5, -0.5),
            normal: glm::vec3(1.0, 0.0, 0.0),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(0.5, -0.5, -0.5),
            normal: glm::vec3(1.0, 0.0, 0.0),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(0.5, -0.5, 0.5),
            normal: glm::vec3(1.0, 0.0, 0.0),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(0.5, 0.5, 0.5),
            normal: glm::vec3(1.0, 0.0, 0.0),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, -0.5, -0.5),
            normal: glm::vec3(0.0, -1.0, 0.0),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(0.5, -0.5, -0.5),
            normal: glm::vec3(0.0, -1.0, 0.0),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(0.5, -0.5, 0.5),
            normal: glm::vec3(0.0, -1.0, 0.0),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(0.5, -0.5, 0.5),
            normal: glm::vec3(0.0, -1.0, 0.0),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, -0.5, 0.5),
            normal: glm::vec3(0.0, -1.0, 0.0),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, -0.5, -0.5),
            normal: glm::vec3(0.0, -1.0, 0.0),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, 0.5, -0.5),
            normal: glm::vec3(0.0, 1.0, 0.0),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(0.5, 0.5, -0.5),
            normal: glm::vec3(0.0, 1.0, 0.0),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(0.5, 0.5, 0.5),
            normal: glm::vec3(0.0, 1.0, 0.0),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(0.5, 0.5, 0.5),
            normal: glm::vec3(0.0, 1.0, 0.0),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, 0.5, 0.5),
            normal: glm::vec3(0.0, 1.0, 0.0),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(-0.5, 0.5, -0.5),
            normal: glm::vec3(0.0, 1.0, 0.0),
            tex_coords: glm::vec2(0.0, 1.0),
        },
    ]
}
