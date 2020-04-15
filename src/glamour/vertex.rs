use crate::glm;

#[repr(C)]
pub struct Vert {
    pub position: glm::Vec3,
    // pub normal: glm::Vec3,
    pub tex_coords: glm::Vec2,
}

impl Vert {
    pub fn from_pos(x: f32, y: f32, z: f32) -> Vert {
        Vert {
            position: glm::vec3(x, y, z),
            // normal: glm::vec3(0.0, 0.0, 0.0),
            tex_coords: glm::vec2(0.0, 0.0),
        }
    }
}
