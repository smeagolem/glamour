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
    cube_trans_vbo: VertBuf<VertTrans>,
    cube_tex: Texture,
    light_shader: ShaderProgram,
    light_vao: VertArray,
    light_vbo: VertBuf<VertBasic>,
    light_trans_vbo: VertBuf<VertTrans>,
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
        let cube_vbo = VertBuf::<VertBasic>::new(tex_cube_verts());
        let cube_trans_vbo = VertBuf::<VertTrans>::new(Vec::with_capacity(max_cubes));
        let ibo = IndexBuf::new(tex_cube_inds());
        let cube_vao = VertArray::new(&[&cube_vbo, &cube_trans_vbo], ibo);

        let light_shader =
            ShaderBuilder::new(include_str!("light.vert"), include_str!("light.frag"))
                .with_float4("u_color", glm::vec4(1.0, 1.0, 1.0, 1.0))
                .build();
        // TODO: check in draw functions if overflowing buffer, if so, draw (flush and reset).
        let max_lights = 32;
        let light_vbo = VertBuf::<VertBasic>::new(tex_cube_verts());
        let light_trans_vbo = VertBuf::<VertTrans>::new(Vec::with_capacity(max_lights));
        let ibo = IndexBuf::new(tex_cube_inds());
        let light_vao = VertArray::new(&[&light_vbo, &light_trans_vbo], ibo);

        ForwardRenderer {
            cube_shader,
            cube_vao,
            cube_vbo,
            cube_trans_vbo,
            cube_tex,
            light_shader,
            light_vao,
            light_vbo,
            light_trans_vbo,
        }
    }

    pub fn cube_shader(&self) -> &ShaderProgram {
        &self.cube_shader
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
        self.draw_cubes();
        self.draw_lights();
    }

    pub fn set_vert_trans(vertices: &mut Vec<VertTrans>, transforms: &[Transform]) {
        vertices.resize_with(transforms.len(), std::default::Default::default);
        vertices
            .par_iter_mut()
            .zip(transforms.par_iter())
            .for_each(|(v, t)| v.set(t));
    }

    // ~25% slower
    #[deprecated]
    pub fn set_vert_trans2(vertices: &mut Vec<VertTrans>, transforms: &[Transform]) {
        vertices.clear();
        vertices.append(
            &mut transforms
                .par_iter()
                .map(VertTrans::from_transform)
                .collect(),
        );
    }

    pub fn set_cubes(&mut self, transforms: &[Transform]) {
        let vertices = self.cube_trans_vbo.vertices_mut();
        ForwardRenderer::set_vert_trans(vertices, transforms);
    }

    pub fn set_lights(&mut self, transforms: &[Transform]) {
        let vertices = self.light_trans_vbo.vertices_mut();
        ForwardRenderer::set_vert_trans(vertices, transforms);
        transforms.iter().enumerate().for_each(|(i, t)| {
            self.cube_shader
                .set_float3(&format!("u_point_lights[{}].position", i), &t.position);
        });
    }

    fn draw_cubes(&self) {
        self.cube_trans_vbo.set_data();
        self.cube_shader.bind();
        self.cube_tex.bind();
        self.cube_vao.bind();
        gl_call!(gl::DrawElementsInstanced(
            gl::TRIANGLES,
            self.cube_vao.index_buf().len() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
            self.cube_trans_vbo.vertices().len() as i32,
        ));
        self.cube_vao.unbind();
        self.cube_tex.unbind();
        self.cube_shader.unbind();
    }

    fn draw_lights(&self) {
        self.light_trans_vbo.set_data();
        self.light_shader.bind();
        self.light_vao.bind();
        gl_call!(gl::DrawElementsInstanced(
            gl::TRIANGLES,
            self.light_vao.index_buf().len() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
            self.light_trans_vbo.vertices().len() as i32,
        ));
        self.light_vao.unbind();
        self.light_shader.unbind();
    }

    pub fn draw_quad(&mut self, _transform: &Transform) {
        todo!()
    }

    pub fn draw_triangle(&mut self, _transform: &Transform) {
        todo!()
    }
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
