use crate::glm;
use glutin::event::{Event, WindowEvent};

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
    pub fn view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &self.target, &glm::vec3(0.0, 1.0, 0.0))
    }
    pub fn projection_matrix(&self) -> glm::Mat4 {
        glm::perspective(
            self.aspect,
            glm::radians(&glm::vec1(self.fov)).x,
            0.1,
            400.0,
        )
    }
    pub fn view_projection_matrix(&self) -> glm::Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
    pub fn handle_event(&mut self, event: &Event<()>) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => self.aspect = physical_size.width as f32 / physical_size.height as f32,
            _ => (),
        }
    }
}
