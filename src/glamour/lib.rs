#[macro_use]
pub mod gl_call;

mod renderer;
pub use renderer::*;

mod shader;
pub use shader::*;

mod vertex;
pub use vertex::*;

mod vertex_buffer;
pub use vertex_buffer::*;

mod index_buffer;
pub use index_buffer::*;

pub struct Layer {
    fr: ForwardRenderer,
    name: String,
}

impl Layer {
    pub fn new(name: &str) -> Self {
        let fr = ForwardRenderer::new().expect("Failed to create forward renderer.");
        Layer {
            fr,
            name: name.to_string(),
        }
    }

    pub fn on_update(&self) {
        self.fr
            .render()
            .expect(format!("Failed to render layer: {}", self.name).as_str());
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

pub struct Application {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    layers: Vec<Layer>,
}

impl Application {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        use glfw::Context;
        // TODO: abstract to Window struct
        // TODO: create window gl context
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        // opengl is state machine, you set up data in vram, then tell it to draw whatever is in there.
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        Application {
            glfw,
            window,
            events,
            layers: Vec::new(),
        }
    }

    pub fn push_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn run(&mut self) {
        use glfw::Context;

        while !self.window.should_close() {
            // input
            let events = glfw::flush_messages(&self.events).collect::<Vec<_>>();
            for (_, event) in events {
                self.on_event(event);
            }

            // TODO: calculate delta_time
            // TODO: update layers
            for layer in self.layers.iter() {
                layer.on_update();
            }

            // update window
            self.window.swap_buffers();
            self.glfw.poll_events();
        }

        // self.window.set_should_close(true);
    }

    fn on_event(&mut self, event: glfw::WindowEvent) {
        if matches!(
            event,
            glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _)
        ) {
            self.window.set_should_close(true);
        }

        // TODO: loop through all layers and match events to on_event?
    }
}
