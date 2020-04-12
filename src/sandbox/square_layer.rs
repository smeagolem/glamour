use glamour::{ForwardRenderer, Layer};

pub struct SquareLayer {
    fr: ForwardRenderer,
    name: String,
}

impl SquareLayer {
    pub fn new(name: &str) -> Self {
        let fr = ForwardRenderer::new().expect("Failed to create forward renderer.");
        SquareLayer {
            fr,
            name: name.to_string(),
        }
    }
}

impl Layer for SquareLayer {
    fn on_frame_update(&mut self, _: &mut glamour::AppContext) {
        self.fr
            .render()
            .expect(format!("Failed to render layer: {}", self.name).as_str());
    }
    fn name(&self) -> &String {
        &self.name
    }
}
