pub mod shader;
pub mod texture;
pub mod vertex;
pub mod vertex_array;

use crate::{glm, IndexBuf, ShaderBuilder, ShaderProgram, Texture, Vert, VertArray, VertBuf};
use gl;

// TODO: make Renderer trait to implement on Forward and Deferred.

#[allow(dead_code)]
pub struct ForwardRenderer {
    shader: ShaderProgram,
    vao: VertArray,
    tex: Texture,
}

impl ForwardRenderer {
    pub fn new() -> ForwardRenderer {
        gl_call!(gl::Enable(gl::DEPTH_TEST));

        let shader =
            ShaderBuilder::new(include_str!("triangle.vert"), include_str!("triangle.frag"))
                .with_float4("u_color", glm::vec4(1.0, 1.0, 1.0, 1.0))
                .build();

        let img_path = crate::assets_path().join("container.jpg");
        let tex = Texture::new(&img_path);

        // TODO: check in draw functions if overflowing buffer, if so, draw (flush and reset).
        let max_vertices = 100_000;
        let vbo = VertBuf::new(Vec::with_capacity(max_vertices), Vert::layout());
        let ibo = IndexBuf::new(Vec::with_capacity(max_vertices));
        let vao = VertArray::new(vec![vbo], ibo);

        ForwardRenderer { shader, vao, tex }
    }

    pub fn shader(&self) -> &ShaderProgram {
        &self.shader
    }

    fn vert_buf(&self) -> &VertBuf {
        self.vao.vert_bufs().get(0).unwrap()
    }

    fn vert_buf_mut(&mut self) -> &mut VertBuf {
        self.vao.vert_bufs_mut().get_mut(0).unwrap()
    }

    pub fn clear(&self) {
        gl_call!(gl::ClearColor(0.3, 0.3, 0.5, 1.0));
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));
    }

    pub fn begin_draw(&self, camera: &Camera) {
        self.shader.bind();
        self.shader
            .set_mat4("u_view_projection", &camera.view_projection_matrix());

        self.tex.bind();
    }

    pub fn end_draw(&mut self) {
        self.vert_buf().set_data();
        self.vao.index_buf().set_data();

        self.vao.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES, // mode
                // 3 as i32,      // number of indices
                self.vao.index_buf().len() as i32, // number of indices
                gl::UNSIGNED_INT,                  // type of an index
                std::ptr::null(),                  // pointer to indices, nullptr if already bound.
            );
        }
        self.vao.unbind();

        self.tex.unbind();
        self.shader.unbind();

        self.vert_buf_mut().vertices_mut().clear();
        self.vao.index_buf_mut().indices_mut().clear();
    }

    pub fn draw_cube(&mut self, transform: &Transform) {
        // create new vertices from cube vertices.
        let mut verts = cube_vertices();

        // transform each vertex position by the transform.
        let trans_mat = transform.matrix();
        for vert in verts.iter_mut() {
            let pos = glm::vec4(vert.position.x, vert.position.y, vert.position.z, 1.0);
            vert.position = (trans_mat * pos).xyz();
        }

        // add vertices to vertec buffer.
        let vert_buf = self.vert_buf_mut();
        let vertices = vert_buf.vertices_mut();
        let vl_pre = vertices.len();
        vertices.append(&mut verts);
        let vl_post = vertices.len();

        // update index buffer
        let indices: &mut Vec<u32> = self.vao.index_buf_mut().indices_mut();
        let mut new_indices: Vec<u32> = (vl_pre as u32..vl_post as u32).collect();
        indices.append(&mut new_indices);
    }

    pub fn draw_quad(&mut self, transform: &Transform) {
        // create new vertices
        let mut verts = vec![
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

        // transform each vertex position by the transform.
        let trans_mat = transform.matrix();
        for vert in verts.iter_mut() {
            let pos = glm::vec4(vert.position.x, vert.position.y, vert.position.z, 1.0);
            vert.position = (trans_mat * pos).xyz();
        }

        // add vertices to vertec buffer.
        let vert_buf = self.vert_buf_mut();
        let vertices = vert_buf.vertices_mut();
        let vl = vertices.len() as u32;
        vertices.append(&mut verts);

        // update index buffer
        let mut indices: Vec<u32> = vec![vl, vl + 1, vl + 2, vl + 2, vl + 3, vl];
        self.vao.index_buf_mut().indices_mut().append(&mut indices);
    }

    pub fn draw_triangle(&mut self, transform: &Transform) {
        // create new vertices
        let mut verts = vec![
            Vert {
                position: glm::vec3(-0.5, -0.5, 0.0),
                tex_coords: glm::vec2(0.0, 0.0),
            },
            Vert {
                position: glm::vec3(0.5, -0.5, 0.0),
                tex_coords: glm::vec2(1.0, 0.0),
            },
            Vert {
                position: glm::vec3(0.0, 0.5, 0.0),
                tex_coords: glm::vec2(0.5, 1.0),
            },
        ];

        // transform each vertex position by the transform.
        let trans_mat = transform.matrix();
        for vert in verts.iter_mut() {
            let pos = glm::vec4(vert.position.x, vert.position.y, vert.position.z, 1.0);
            vert.position = (trans_mat * pos).xyz();
        }

        // add vertices to vertec buffer.
        let vert_buf = self.vert_buf_mut();
        let vertices = vert_buf.vertices_mut();
        let vl = vertices.len() as u32;
        vertices.append(&mut verts);

        // update index buffer
        let mut indices: Vec<u32> = vec![vl, vl + 1, vl + 2];
        self.vao.index_buf_mut().indices_mut().append(&mut indices);
    }
}

pub struct Camera {
    pub position: glm::Vec3,
    pub target: glm::Vec3,
    pub fov: f32,
    aspect: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: glm::vec3(0.0, 0.0, 0.0),
            target: glm::vec3(0.0, 0.0, 0.0),
            fov: 90.0,
            aspect: 1.0,
        }
    }
    pub fn view_projection_matrix(&self) -> glm::Mat4 {
        // let view = glm::translate(&glm::identity::<f32, glm::U4>(), &self.position);
        let view = glm::look_at(&self.position, &self.target, &glm::vec3(0.0, 1.0, 0.0));
        let projection: glm::Mat4 = glm::perspective(
            self.aspect,
            glm::radians(&glm::vec1(self.fov)).x,
            0.1,
            100.0,
        );
        projection * view
    }

    pub fn handle_event(&mut self, event: &glutin::event::Event<()>) {
        match event {
            glutin::event::Event::WindowEvent {
                event: glutin::event::WindowEvent::Resized(physical_size),
                ..
            } => self.aspect = physical_size.width as f32 / physical_size.height as f32,
            _ => (),
        }
    }
}

pub struct Transform {
    pub position: glm::Vec3,
    pub rotation: glm::Quat,
    pub scale: glm::Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::quat_identity(),
            scale: glm::vec3(1.0, 1.0, 1.0),
        }
    }
    pub fn matrix(&self) -> glm::Mat4 {
        glm::translation(&self.position)
            * glm::quat_cast(&self.rotation)
            * glm::scaling(&self.scale)
    }
}

fn cube_vertices() -> Vec<Vert> {
    vec![
        Vert {
            position: glm::vec3(-0.5, -0.5, -0.5),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        Vert {
            position: glm::vec3(0.5, -0.5, -0.5),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        Vert {
            position: glm::vec3(0.5, 0.5, -0.5),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        Vert {
            position: glm::vec3(0.5, 0.5, -0.5),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        Vert {
            position: glm::vec3(-0.5, 0.5, -0.5),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        Vert {
            position: glm::vec3(-0.5, -0.5, -0.5),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        Vert {
            position: glm::vec3(-0.5, -0.5, 0.5),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        Vert {
            position: glm::vec3(0.5, -0.5, 0.5),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        Vert {
            position: glm::vec3(0.5, 0.5, 0.5),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        Vert {
            position: glm::vec3(0.5, 0.5, 0.5),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        Vert {
            position: glm::vec3(-0.5, 0.5, 0.5),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        Vert {
            position: glm::vec3(-0.5, -0.5, 0.5),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        Vert {
            position: glm::vec3(-0.5, 0.5, 0.5),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        Vert {
            position: glm::vec3(-0.5, 0.5, -0.5),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        Vert {
            position: glm::vec3(-0.5, -0.5, -0.5),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        Vert {
            position: glm::vec3(-0.5, -0.5, -0.5),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        Vert {
            position: glm::vec3(-0.5, -0.5, 0.5),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        Vert {
            position: glm::vec3(-0.5, 0.5, 0.5),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        Vert {
            position: glm::vec3(0.5, 0.5, 0.5),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        Vert {
            position: glm::vec3(0.5, 0.5, -0.5),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        Vert {
            position: glm::vec3(0.5, -0.5, -0.5),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        Vert {
            position: glm::vec3(0.5, -0.5, -0.5),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        Vert {
            position: glm::vec3(0.5, -0.5, 0.5),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        Vert {
            position: glm::vec3(0.5, 0.5, 0.5),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        Vert {
            position: glm::vec3(-0.5, -0.5, -0.5),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        Vert {
            position: glm::vec3(0.5, -0.5, -0.5),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        Vert {
            position: glm::vec3(0.5, -0.5, 0.5),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        Vert {
            position: glm::vec3(0.5, -0.5, 0.5),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        Vert {
            position: glm::vec3(-0.5, -0.5, 0.5),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        Vert {
            position: glm::vec3(-0.5, -0.5, -0.5),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        Vert {
            position: glm::vec3(-0.5, 0.5, -0.5),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        Vert {
            position: glm::vec3(0.5, 0.5, -0.5),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        Vert {
            position: glm::vec3(0.5, 0.5, 0.5),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        Vert {
            position: glm::vec3(0.5, 0.5, 0.5),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        Vert {
            position: glm::vec3(-0.5, 0.5, 0.5),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        Vert {
            position: glm::vec3(-0.5, 0.5, -0.5),
            tex_coords: glm::vec2(0.0, 1.0),
        },
    ]
}
