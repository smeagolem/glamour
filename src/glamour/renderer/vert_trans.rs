use crate::{glm, Transform, Vert, VertAttr, VertAttrType, VertLayout};
use std::default::Default;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct VertTrans {
    pub transform: glm::Mat4,
    pub normal: glm::Mat3,
}

impl VertTrans {
    pub fn from_transform(transform: &Transform) -> Self {
        let matrix = transform.matrix();
        VertTrans {
            transform: matrix,
            normal: Transform::normal_matrix(&matrix),
        }
    }
    pub fn set(&mut self, transform: &Transform) {
        let matrix = transform.matrix();
        self.transform = matrix;
        self.normal = Transform::normal_matrix(&matrix);
    }
}

impl Vert for VertTrans {
    fn layout() -> crate::VertLayout {
        VertLayout::new(vec![
            VertAttr::new(VertAttrType::Mat4, false),
            VertAttr::new(VertAttrType::Mat3, false),
        ])
    }
}

impl Default for VertTrans {
    fn default() -> Self {
        VertTrans {
            transform: glm::identity(),
            normal: glm::identity(),
        }
    }
}
