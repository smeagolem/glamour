use crate::{
    glm, IndexBuf, ShaderBuilder, ShaderProgram, Texture, Vert, VertArray, VertAttr, VertAttrType,
    VertBuf, VertLayout,
};

pub struct Cube {
    pub shader: ShaderProgram,
    pub vert_arr: VertArray,
    pub texture: Texture,
}

impl Cube {
    pub fn new() -> Self {
        let shader = ShaderBuilder::new(
            include_str!("renderer/triangle.vert"),
            include_str!("renderer/triangle.frag"),
        )
        .with_float4("u_color", glm::vec4(1.0, 1.0, 1.0, 1.0))
        .build();
        let vertices = vec![
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
        ];
        let vert_layout = VertLayout::new(vec![
            VertAttr::new(VertAttrType::Float3, false),
            VertAttr::new(VertAttrType::Float2, false),
        ]);
        let vbo = VertBuf::new(&vertices, vert_layout);

        let indices: Vec<u32> = (0..36).collect();
        let ibo = IndexBuf::new(&indices);

        let vao = VertArray::new(&[vbo], ibo);

        let img_path = crate::assets_path().join("container.jpg");
        let tex = Texture::new(&img_path);

        Cube {
            shader,
            vert_arr: vao,
            texture: tex,
        }
    }
}
