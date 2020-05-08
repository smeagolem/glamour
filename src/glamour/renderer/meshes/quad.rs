use crate::{glm, VertBasic};

pub fn ndc_quad_verts() -> Vec<VertBasic> {
    vec![
        VertBasic {
            position: glm::vec3(-1.0, 1.0, 0.0),
            normal: glm::vec3(0.0, 0.0, 1.0),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(-1.0, -1.0, 0.0),
            normal: glm::vec3(0.0, 0.0, 1.0),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        VertBasic {
            position: glm::vec3(1.0, 1.0, 0.0),
            normal: glm::vec3(0.0, 0.0, 1.0),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        VertBasic {
            position: glm::vec3(1.0, -1.0, 0.0),
            normal: glm::vec3(0.0, 0.0, 1.0),
            tex_coords: glm::vec2(1.0, 0.0),
        },
    ]
}
