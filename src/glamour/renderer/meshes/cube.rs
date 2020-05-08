use crate::{glm, VertBasic};

pub fn tex_cube_verts() -> Vec<VertBasic> {
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

pub fn tex_cube_inds() -> Vec<u32> {
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
