use crate::{
    glm, Camera, IndexBuf, ShaderBuilder, ShaderProgram, Texture, Transform, VertArray, VertBasic,
    VertBuf, VertTrans,
};
use gl;
use rayon::prelude::*;

// TODO: make Renderer trait to implement on Forward and Deferred.

#[allow(dead_code)]
pub struct ForwardRenderer {
    cube_shader: ShaderProgram,
    cube_vao: VertArray,
    cube_vbo: VertBuf<VertBasic>,
    pub cube_trans_vbo: VertBuf<VertTrans>,
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
        let img_path = crate::assets_path().join("tile_bookcaseFull.png");
        let cube_tex = Texture::new(&img_path);
        // TODO: check in draw functions if overflowing buffer, if so, draw (flush and reset).
        let max_cubes = 1_000_000;
        let cube_vertices = tex_cube_verts();
        let cube_indices = tex_cube_inds();
        let cube_vbo = VertBuf::<VertBasic>::new(cube_vertices);
        let cube_trans_vbo = VertBuf::<VertTrans>::new(Vec::with_capacity(max_cubes));
        let ibo = IndexBuf::new(cube_indices);
        let cube_vao = VertArray::new(&[&cube_vbo, &cube_trans_vbo], ibo);

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
            cube_tex,
            light_shader,
            light_vao,
            light_vbo,
        }
    }

    pub fn shader(&self) -> &ShaderProgram {
        &self.cube_shader
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
        gl_call!(gl::ClearColor(
            20.0 / 255.0,
            24.0 / 255.0,
            82.0 / 255.0,
            1.0
        ));
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
        let now = std::time::Instant::now();
        self.cube_trans_vbo.set_data();
        // println!("set_data: {} ms", now.elapsed().as_millis());

        self.cube_shader.bind();
        self.cube_tex.bind();
        self.cube_vao.bind();
        gl_call!(gl::DrawElementsInstanced(
            gl::TRIANGLES,                               // mode
            self.cube_vao.index_buf().len() as i32,      // number of indices
            gl::UNSIGNED_INT,                            // type of an index
            std::ptr::null(), // pointer to indices, nullptr if already bound.
            self.cube_trans_vbo.vertices().len() as i32, // number of instances
        ));
        self.cube_vao.unbind();
        self.cube_tex.unbind();
        self.cube_shader.unbind();
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

    pub fn init_cubes(&mut self, count: usize) {
        let vertices = self.cube_trans_vbo.vertices_mut();
        let mut verts: Vec<VertTrans> = (0..count)
            .map(|_| VertTrans {
                transform: glm::identity(),
                normal: glm::identity(),
            })
            .collect();
        vertices.clear();
        vertices.append(&mut verts);
    }

    pub fn draw_cubes(&mut self, transforms: &[Transform]) {
        let vertices = self.cube_trans_vbo.vertices_mut();
        vertices
            .par_iter_mut()
            .zip(transforms.par_iter())
            .for_each(|(v, t)| {
                let trans_mat = t.matrix();
                v.transform = trans_mat;
                v.normal = glm::mat4_to_mat3(&glm::inverse_transpose(trans_mat));
            });
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

fn tex_cube_verts() -> Vec<VertBasic> {
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
        // 2
        VertBasic {
            position: glm::vec3(-0.5, 0.5, -0.5),
            normal: glm::vec3(0.0, 0.0, -1.0),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        // 0
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
        // 6
        VertBasic {
            position: glm::vec3(-0.5, 0.5, 0.5),
            normal: glm::vec3(0.0, 0.0, 1.0),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        // 4
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
        // 10
        VertBasic {
            position: glm::vec3(-0.5, -0.5, 0.5),
            normal: glm::vec3(-1.0, 0.0, 0.0),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        // 8
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
        // 14
        VertBasic {
            position: glm::vec3(0.5, -0.5, 0.5),
            normal: glm::vec3(1.0, 0.0, 0.0),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        // 12
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
        // 18
        VertBasic {
            position: glm::vec3(-0.5, -0.5, 0.5),
            normal: glm::vec3(0.0, -1.0, 0.0),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        // 16
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
        // 22
        VertBasic {
            position: glm::vec3(-0.5, 0.5, 0.5),
            normal: glm::vec3(0.0, 1.0, 0.0),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        // 20
    ]
}

fn tex_cube_inds() -> Vec<u32> {
    vec![
        0, 1, 2, //
        2, 3, 0, //
        4, 5, 6, //
        6, 7, 4, //
        8, 9, 10, //
        10, 11, 8, //
        12, 13, 14, //
        14, 15, 12, //
        16, 17, 18, //
        18, 19, 16, //
        20, 21, 22, //
        22, 23, 20, //
    ]
}
