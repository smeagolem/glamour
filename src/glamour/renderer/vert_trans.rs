use crate::{glm, Vert, VertAttr, VertAttrType, VertLayout};
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct VertTrans {
    pub transform: glm::Mat4,
    pub normal: glm::Mat3,
}

impl Vert for VertTrans {
    fn layout() -> crate::VertLayout {
        VertLayout::new(vec![
            VertAttr::new(VertAttrType::Mat4, false),
            VertAttr::new(VertAttrType::Mat3, false),
        ])
    }
}
