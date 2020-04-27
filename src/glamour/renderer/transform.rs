use crate::glm;

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
        glm::scale(
            &(glm::translation(&self.position) * glm::quat_to_mat4(&self.rotation)),
            &self.scale,
        )
    }
    pub fn from_pos(position: glm::Vec3) -> Self {
        Transform {
            position,
            rotation: glm::quat_identity(),
            scale: glm::vec3(1.0, 1.0, 1.0),
        }
    }
}
