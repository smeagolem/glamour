use glamour::{glm, ForwardRenderer, Layer};
use glutin::event::{Event, WindowEvent};

pub struct SquareLayer {
    fr: ForwardRenderer,
    name: String,
    time: std::time::Instant,
}

impl SquareLayer {
    pub fn new(name: &str) -> Self {
        let mut fr = ForwardRenderer::new().expect("Failed to create forward renderer.");
        let cube_positions = vec![
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(2.0, 5.0, -15.0),
            glm::vec3(-1.5, -2.2, -2.5),
            glm::vec3(-3.8, -2.0, -12.3),
            glm::vec3(2.4, -0.4, -3.5),
            glm::vec3(-1.7, 3.0, -7.5),
            glm::vec3(1.3, -2.0, -2.5),
            glm::vec3(1.5, 2.0, -2.5),
            glm::vec3(1.5, 0.2, -1.5),
            glm::vec3(-1.3, 1.0, -1.5),
        ];
        fr.cube_transforms = cube_positions
            .iter()
            .map(|p| {
                let model: glm::Mat4 = glm::identity();
                glm::translate(&model, p)
            })
            .collect();
        SquareLayer {
            fr,
            name: name.to_string(),
            time: std::time::Instant::now(),
        }
    }
}

impl Layer for SquareLayer {
    fn on_frame_update(&mut self, app_context: &mut glamour::AppContext) {
        let delta_time = app_context.delta_time().as_secs_f32();
        let time = self.time.elapsed().as_secs_f32();
        self.fr.shader_program.set_mat4("u_view", {
            let radius = 5.0;
            let cam_x = time.sin() * radius;
            let cam_z = time.cos() * radius;
            &glm::look_at(
                &glm::vec3(cam_x, 0.0, cam_z),
                &glm::vec3(0.0, 0.0, 0.0),
                &glm::vec3(0.0, 1.0, 0.0),
            )
        });
        self.fr.shader_program.set_float4(
            "u_color",
            &glm::vec4(
                (time * 1.0).sin() / 2.0 + 0.5,
                (time * 2.0).sin() / 2.0 + 0.5,
                (time * 5.0).sin() / 2.0 + 0.5,
                1.0,
            ),
        );
        for (index, transform) in self.fr.cube_transforms.iter_mut().enumerate() {
            *transform = glm::rotate(
                transform,
                glm::radians(&glm::vec1((index + 1) as f32 * delta_time * 20.0)).x,
                &glm::vec3(0.5, 1.0, 0.0),
            );
            *transform = glm::translate(
                transform,
                &glm::vec3(
                    0.0,
                    ((index + 1) as f32 * time).sin() * delta_time * 0.5,
                    0.0,
                ),
            );
        }
        self.fr
            .render()
            .expect(format!("Failed to render layer: {}", self.name).as_str());
    }
    fn name(&self) -> &String {
        &self.name
    }
    fn on_event(&mut self, event: &glutin::event::Event<()>, _: &mut glamour::AppContext) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                let projection: glm::Mat4 = glm::perspective(
                    physical_size.width as f32 / physical_size.height as f32,
                    glm::radians(&glm::vec1(90.0)).x,
                    0.1,
                    100.0,
                );
                self.fr.shader_program.set_mat4("u_projection", &projection);
            }
            _ => (),
        }
    }
}
