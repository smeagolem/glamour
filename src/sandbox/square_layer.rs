use glamour::{glm, ForwardRenderer, Layer};

pub struct SquareLayer {
    fr: ForwardRenderer,
    name: String,
    time: std::time::Instant,
}

impl SquareLayer {
    pub fn new(name: &str) -> Self {
        let fr = ForwardRenderer::new().expect("Failed to create forward renderer.");
        SquareLayer {
            fr,
            name: name.to_string(),
            time: std::time::Instant::now(),
        }
    }
}

impl Layer for SquareLayer {
    fn on_frame_update(&mut self, _: &mut glamour::AppContext) {
        self.fr.shader_program.set_float4(
            "u_Color",
            glm::vec4(
                1.0,
                (self.time.elapsed().as_secs_f32() * 10.0).sin() / 2.0 + 0.5,
                0.2,
                1.0,
            ),
        );
        self.fr
            .render()
            .expect(format!("Failed to render layer: {}", self.name).as_str());
    }
    fn name(&self) -> &String {
        &self.name
    }
}
