#[macro_use]
pub mod gl_call;

mod renderer;
pub use renderer::*;
pub use shader::*;
pub use texture::*;
pub use vertex::*;
pub use vertex_array::*;

mod layer;
pub use layer::*;

mod asset;
pub use asset::*;

pub use nalgebra_glm as glm;

mod perf_metrics_layer;

use std::time::{Duration, Instant};

use glutin::event::Event;
use glutin::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use glutin::window::WindowBuilder;
use glutin::{dpi, ContextBuilder, ContextWrapper};

use imgui;

pub struct AppContext {
    fixed_timestep: std::time::Duration,
    max_frame_rate: f32,
    min_frame_timestep: std::time::Duration,
    windowed_context: ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    imgui_platform: imgui_winit_support::WinitPlatform,
    event_poll_time: std::time::Duration,
    delta_time: std::time::Duration,
}

impl AppContext {
    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }
}

pub struct Application {
    event_loop: EventLoop<()>,
    windowed_context: ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    imgui: imgui::Context,
    imgui_platform: imgui_winit_support::WinitPlatform,
    imgui_renderer: imgui_opengl_renderer::Renderer,
    layers: Vec<Box<dyn Layer>>,
}

impl Application {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let logical_size = dpi::LogicalSize { width, height };
        let wb = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(logical_size);

        let windowed_context = ContextBuilder::new()
            .with_vsync(false)
            .with_multisampling(0)
            .with_double_buffer(Some(true))
            .build_windowed(wb, &event_loop)
            .unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        let mut imgui = imgui::Context::create();

        let mut imgui_platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        imgui_platform.attach_window(
            imgui.io_mut(),
            windowed_context.window(),
            imgui_winit_support::HiDpiMode::Default,
        );

        // fonts
        let hidpi_factor = imgui_platform.hidpi_factor();
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

        let physical_size = logical_size.to_physical::<u32>(hidpi_factor);
        unsafe {
            gl::Viewport(
                0,
                0,
                physical_size.width as i32,
                physical_size.height as i32,
            );
        }

        let mut layers: Vec<Box<dyn Layer>> = Vec::new();
        let perf_metrics_layer = perf_metrics_layer::PerfMetricsLayer::new();
        layers.push(Box::new(perf_metrics_layer));

        Application {
            event_loop,
            windowed_context,
            imgui,
            imgui_platform,
            imgui_renderer,
            layers,
        }
    }

    pub fn run(self) {
        let Application {
            windowed_context,
            mut imgui,
            imgui_platform,
            imgui_renderer,
            mut layers,
            ..
        } = self;

        let mut last_event_poll = Instant::now();

        let fixed_update_rate = 120.0;
        let mut next_fixed_update = Instant::now();

        let max_frame_rate: f32 = 60.0;
        let min_frame_timestep = Duration::from_secs_f32(1.0 / max_frame_rate);

        let mut app_context = AppContext {
            fixed_timestep: Duration::from_secs_f64(1.0 / fixed_update_rate),
            max_frame_rate,
            min_frame_timestep,
            windowed_context,
            imgui_platform,
            event_poll_time: Duration::from_secs(0),
            delta_time: Duration::from_secs(0),
        };

        let mut next_frame_update = Instant::now();
        let mut last_frame_update = Instant::now();

        // let mut ahead_frame_skip_count: u64 = 0;
        // let mut behind_frame_skip_count: u64 = 0;

        self.event_loop.run(
            move |event: Event<()>,
                  _: &EventLoopWindowTarget<()>,
                  control_flow: &mut ControlFlow| {
                // println!("{:?}", event);
                *control_flow = ControlFlow::Poll;
                match event {
                    Event::NewEvents(_) => {
                        // other application-specific logic
                        let now = Instant::now();
                        app_context.event_poll_time = now - last_event_poll;
                        imgui.io_mut().update_delta_time(last_event_poll);
                        last_event_poll = now;
                    }
                    Event::MainEventsCleared => {
                        // other application-specific logic

                        // asap update

                        let now = Instant::now();
                        if now >= next_fixed_update {
                            // fixed update
                            next_fixed_update = next_fixed_update + app_context.fixed_timestep;

                            for layer in &mut layers {
                                layer.on_fixed_update(&mut app_context);
                            }
                        }

                        let now = Instant::now();
                        if now < next_fixed_update {
                            if now >= next_frame_update {
                                // frame update (with render)
                                next_frame_update = now + app_context.min_frame_timestep;
                                app_context.delta_time = now - last_frame_update;
                                last_frame_update = now;

                                app_context
                                    .imgui_platform
                                    .prepare_frame(
                                        imgui.io_mut(),
                                        app_context.windowed_context.window(),
                                    )
                                    .expect("Failed to prepare frame");
                                app_context.windowed_context.window().request_redraw();
                            } else {
                                // ahead_frame_skip_count += 1;
                                // println!("Ahead Frames Skipped: {}", ahead_frame_skip_count);
                            }
                        } else {
                            // behind_frame_skip_count += 1;
                            // println!("BEHIND FRAMES SKIPPED! {}", behind_frame_skip_count);
                        }
                    }
                    Event::LoopDestroyed => (),
                    Event::RedrawRequested(_) => {
                        // application-specific rendering *under the UI*
                        unsafe {
                            gl::ClearColor(1.0, 0.5, 0.7, 1.0);
                            gl::Clear(gl::COLOR_BUFFER_BIT);
                        }

                        for layer in &mut layers {
                            layer.on_frame_update(&mut app_context);
                        }

                        // construct the UI
                        let ui = imgui.frame();
                        for layer in &mut layers {
                            layer.on_imgui_update(&ui, &mut app_context);
                        }

                        // ui.show_demo_window(&mut true);
                        app_context
                            .imgui_platform
                            .prepare_render(&ui, app_context.windowed_context.window());
                        imgui_renderer.render(ui);

                        // application-specific rendering *over the UI*

                        // let before_swap = Instant::now();
                        app_context.windowed_context.swap_buffers().unwrap();
                        // let elapsed = before_swap.elapsed();
                        // if elapsed > min_frame_timestep {
                        //     println!(
                        //         "Slow Swap Buffers Time: {:07.3}",
                        //         elapsed.as_secs_f64() * 1_000.0
                        //     );
                        // }
                    }
                    event => {
                        app_context.imgui_platform.handle_event(
                            imgui.io_mut(),
                            app_context.windowed_context.window(),
                            &event,
                        );

                        // other application-specific event handling
                        use glutin::event::{
                            KeyboardInput, ModifiersState, VirtualKeyCode, WindowEvent,
                        };
                        match &event {
                            Event::WindowEvent { event, .. } => match event {
                                WindowEvent::Resized(physical_size) => {
                                    app_context.windowed_context.resize(*physical_size);
                                    // FIXME: this probably is unsafe... maybe
                                    unsafe {
                                        gl::Viewport(
                                            0,
                                            0,
                                            physical_size.width as i32,
                                            physical_size.height as i32,
                                        );
                                    }
                                }
                                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                                // FIXME: remove this as `modifiders` is deprecated.
                                #[allow(deprecated)]
                                WindowEvent::KeyboardInput {
                                    input:
                                        KeyboardInput {
                                            virtual_keycode: Some(virtual_code),
                                            state,
                                            modifiers,
                                            ..
                                        },
                                    ..
                                } => match (virtual_code, state, *modifiers) {
                                    (VirtualKeyCode::Q, _, ModifiersState::LOGO) => {
                                        *control_flow = ControlFlow::Exit
                                    }
                                    _ => (),
                                },
                                _ => (),
                            },
                            _ => (),
                        }

                        for layer in &mut layers {
                            layer.on_event(&event, &mut app_context);
                        }
                    }
                }
            },
        );
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
    }
}
