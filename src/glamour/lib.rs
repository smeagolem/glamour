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

use std::time::Instant;

use glutin::event::Event;
use glutin::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use glutin::window::WindowBuilder;
use glutin::{dpi, ContextBuilder, ContextWrapper};

use imgui;

pub struct Application {
    event_loop: EventLoop<()>,
    windowed_context: ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    imgui: imgui::Context,
    platform: imgui_winit_support::WinitPlatform,
    imgui_renderer: imgui_opengl_renderer::Renderer,
    last_frame: Instant,
    layers: Vec<Layer>,
}

impl Application {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(dpi::LogicalSize { width, height });

        let windowed_context = ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(0)
            .build_windowed(wb, &event_loop)
            .unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        let mut imgui = imgui::Context::create();

        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            windowed_context.window(),
            imgui_winit_support::HiDpiMode::Default,
        );

        // fonts
        let hidpi_factor = platform.hidpi_factor();
        let font_size = (14.0 * hidpi_factor) as f32;
        imgui.fonts().add_font(&[imgui::FontSource::TtfData {
            data: include_bytes!("../resources/SourceCodePro-Regular.ttf"),
            size_pixels: font_size,
            config: Some(imgui::FontConfig::default()),
        }]);
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        imgui.fonts().build_rgba32_texture();

        // load OpenGl
        gl::load_with(|s| windowed_context.context().get_proc_address(s) as *const _);

        // set imgui renderer to use raw OpenGL
        let imgui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
            windowed_context.context().get_proc_address(s) as _
        });

        let last_frame = Instant::now();

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        Application {
            event_loop,
            windowed_context,
            imgui,
            platform,
            imgui_renderer,
            last_frame,
            layers: Vec::new(),
        }
    }

    pub fn run(self) {
        let mut last_frame = self.last_frame;
        let mut platform = self.platform;
        let windowed_context = self.windowed_context;
        let layers = self.layers;
        let mut imgui = self.imgui;
        let imgui_renderer = self.imgui_renderer;
        self.event_loop.run(
            move |event: Event<()>,
                  _: &EventLoopWindowTarget<()>,
                  control_flow: &mut ControlFlow| {
                // println!("{:?}", event);
                *control_flow = ControlFlow::Poll;

                match event {
                    Event::NewEvents(_) => {
                        // other application-specific logic
                        last_frame = imgui.io_mut().update_delta_time(last_frame);
                    }
                    Event::MainEventsCleared => {
                        // other application-specific logic
                        platform
                            .prepare_frame(imgui.io_mut(), windowed_context.window()) // step 4
                            .expect("Failed to prepare frame");
                        windowed_context.window().request_redraw();
                    }
                    Event::LoopDestroyed => (),
                    Event::RedrawRequested(_) => {
                        // application-specific rendering *under the UI*
                        unsafe {
                            gl::ClearColor(1.0, 0.5, 0.7, 1.0);
                            gl::Clear(gl::COLOR_BUFFER_BIT);
                        }

                        for layer in &layers {
                            layer.on_update();
                        }

                        // construct the UI
                        let ui = imgui.frame();
                        imgui::Window::new(imgui::im_str!("Hello world"))
                            .size([300.0, 100.0], imgui::Condition::FirstUseEver)
                            .build(&ui, || {
                                ui.text(imgui::im_str!("Hello world!"));
                                ui.text(imgui::im_str!("こんにちは世界！"));
                                ui.text(imgui::im_str!("This...is...imgui-rs!"));
                                ui.separator();
                                let mouse_pos = ui.io().mouse_pos;
                                ui.text(format!(
                                    "Mouse Position: ({:.1},{:.1})",
                                    mouse_pos[0], mouse_pos[1]
                                ));
                            });
                        ui.show_demo_window(&mut true);
                        platform.prepare_render(&ui, windowed_context.window());
                        // render the UI with a renderer
                        imgui_renderer.render(ui);

                        // application-specific rendering *over the UI*

                        windowed_context.swap_buffers().unwrap();
                    }
                    event => {
                        platform.handle_event(imgui.io_mut(), windowed_context.window(), &event);

                        // other application-specific event handling
                        use glutin::event::{
                            ElementState, KeyboardInput, ModifiersState, VirtualKeyCode,
                            WindowEvent,
                        };
                        match event {
                            Event::WindowEvent { event, .. } => match event {
                                WindowEvent::Resized(physical_size) => {
                                    windowed_context.resize(physical_size)
                                }
                                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                                WindowEvent::KeyboardInput {
                                    input:
                                        KeyboardInput {
                                            virtual_keycode: Some(virtual_code),
                                            state,
                                            modifiers,
                                            ..
                                        },
                                    ..
                                } => match (virtual_code, state, modifiers) {
                                    (VirtualKeyCode::Q, _, ModifiersState::LOGO) => {
                                        *control_flow = ControlFlow::Exit
                                    }
                                    (VirtualKeyCode::W, ElementState::Pressed, _) => {
                                        println!("W");
                                    }
                                    _ => (),
                                },
                                _ => (),
                            },
                            _ => (),
                        }
                    }
                }
            },
        );
    }

    pub fn push_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }
}
