use crate::glm;
use memoffset::offset_of;

#[allow(dead_code)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
    pub tex_coords: glm::Vec2,
}

impl Vertex {
    pub fn from_pos(x: f32, y: f32, z: f32) -> Vertex {
        Vertex {
            position: glm::vec3(x, y, z),
            normal: glm::vec3(0.0, 0.0, 0.0),
            tex_coords: glm::vec2(0.0, 0.0),
        }
    }

    pub fn offset_of_position() -> usize {
        offset_of!(Vertex, position)
    }
}
