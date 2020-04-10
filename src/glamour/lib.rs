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

    pub fn on_fixed_update(&self) {
        // caculate physics lol
    }

    pub fn on_frame_update(&self) {
        self.fr
            .render()
            .expect(format!("Failed to render layer: {}", self.name).as_str());
    }

    pub fn on_imgui_update(&self, ui: &imgui::Ui) {
        // ui.text("Hello!");
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

use std::{
    ffi::{CStr, CString},
    time::{Duration, Instant},
};

use glutin::event::Event;
use glutin::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use glutin::window::WindowBuilder;
use glutin::{dpi, ContextBuilder, ContextWrapper};

use imgui;

pub struct Application {
    event_loop: EventLoop<()>,
    windowed_context: ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    imgui: imgui::Context,
    imgui_platform: imgui_winit_support::WinitPlatform,
    imgui_renderer: imgui_opengl_renderer::Renderer,
    layers: Vec<Layer>,
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

        Application {
            event_loop,
            windowed_context,
            imgui,
            imgui_platform,
            imgui_renderer,
            layers: Vec::new(),
        }
    }

    pub fn run(self) {
        let Application {
            windowed_context,
            mut imgui,
            mut imgui_platform,
            imgui_renderer,
            layers,
            ..
        } = self;

        let mut last_event_poll = Instant::now();
        let mut event_poll_time = Duration::from_secs(0);

        let fixed_update_rate = 120.0;
        let fixed_timestep = Duration::from_secs_f64(1.0 / fixed_update_rate);
        let mut next_fixed_update = Instant::now();
        let mut last_fixed_update = Instant::now();
        let mut fixed_delta_time = Duration::from_secs(0);

        let mut max_frame_rate: f32 = 60.0;
        let mut min_frame_timestep = Duration::from_secs_f32(1.0 / max_frame_rate);
        let mut next_frame_update = Instant::now();
        let mut last_frame_update = Instant::now();
        let mut delta_time = Duration::from_secs(0);
        let fps_timings_max_capacity = 300;
        let mut fps_timings = Vec::<f32>::with_capacity(fps_timings_max_capacity);
        // fps_timings.insert(0, element);
        // fps_timings.truncate(fps_timings_max_capacity);

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
                        event_poll_time = now - last_event_poll;
                        imgui.io_mut().update_delta_time(last_event_poll);
                        last_event_poll = now;
                    }
                    Event::MainEventsCleared => {
                        // other application-specific logic

                        // asap update

                        let now = Instant::now();
                        if now >= next_fixed_update {
                            // fixed update
                            next_fixed_update = next_fixed_update + fixed_timestep;
                            fixed_delta_time = now - last_fixed_update;
                            last_fixed_update = now;

                            for layer in &layers {
                                layer.on_fixed_update();
                            }
                        }

                        let now = Instant::now();
                        if now < next_fixed_update {
                            if now >= next_frame_update {
                                // frame update (with render)
                                next_frame_update = now + min_frame_timestep;
                                delta_time = now - last_frame_update;
                                last_frame_update = now;

                                fps_timings.insert(0, 1.0 / delta_time.as_secs_f32());
                                fps_timings.truncate(fps_timings_max_capacity);

                                imgui_platform
                                    .prepare_frame(imgui.io_mut(), windowed_context.window())
                                    .expect("Failed to prepare frame");
                                windowed_context.window().request_redraw();
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

                        for layer in &layers {
                            layer.on_frame_update();
                        }

                        // construct the UI
                        let ui = imgui.frame();
                        for layer in &layers {
                            layer.on_imgui_update(&ui);
                        }

                        imgui::Window::new(imgui::im_str!("Performance Metrics"))
                            .size([340.0, 250.0], imgui::Condition::FirstUseEver)
                            .always_auto_resize(true)
                            .position_pivot([1.0, 0.0])
                            .position(
                                [
                                    windowed_context.window().inner_size().width as f32
                                        / imgui_platform.hidpi_factor() as f32,
                                    0.0,
                                ],
                                imgui::Condition::FirstUseEver,
                            )
                            .save_settings(false)
                            .build(&ui, || {
                                ui.text(format!(
                                    "Event Poll Time: {:06.3} ms",
                                    event_poll_time.as_secs_f64() * 1_000.0
                                ));
                                ui.text(format!(
                                    "Fixed Delta Time: {:06.3} ms (Timestep: {:06.3})",
                                    fixed_delta_time.as_secs_f64() * 1_000.0,
                                    fixed_timestep.as_secs_f64() * 1_000.0,
                                ));
                                ui.text(format!(
                                    "Delta Time: {:06.3} ms (Max Frame Rate: {:03.0} FPS)",
                                    delta_time.as_secs_f64() * 1_000.0,
                                    max_frame_rate,
                                ));
                                if ui
                                    .drag_float(
                                        imgui::im_str!("Max Frame Rate"),
                                        &mut max_frame_rate,
                                    )
                                    .min(30.0)
                                    .max(300.0)
                                    .display_format(imgui::im_str!("%g"))
                                    .build()
                                {
                                    min_frame_timestep =
                                        Duration::from_secs_f32(1.0 / max_frame_rate);
                                };

                                // calling before a ui frame is created causes UB
                                // unsafe {
                                //     if imgui::sys::igSliderFloat(
                                //         CString::new("Test Slider").unwrap().as_ptr(),
                                //         &mut max_frame_rate,
                                //         30.0,
                                //         300.0,
                                //         CString::new("%g").unwrap().as_ptr(),
                                //         1.0,
                                //     ) {
                                //         min_frame_timestep =
                                //             Duration::from_secs_f32(1.0 / max_frame_rate);
                                //     }
                                // }

                                let min_fps =
                                    fps_timings.iter().fold(std::f32::MAX, |min, &val| {
                                        if val < min {
                                            val
                                        } else {
                                            min
                                        }
                                    });
                                let max_fps = fps_timings.iter().fold(0.0f32, |max, &val| {
                                    if val > max {
                                        val
                                    } else {
                                        max
                                    }
                                });
                                let ave_fps: f32 =
                                    fps_timings.iter().sum::<f32>() / fps_timings.len() as f32;
                                let overlay_text = imgui::im_str!(
                                    "Min: {:06.3} | Max: {:06.3} | Ave: {:06.3}",
                                    min_fps,
                                    max_fps,
                                    ave_fps,
                                );
                                ui.plot_lines(imgui::im_str!("FPS"), &fps_timings)
                                    .graph_size([300.0, 100.0])
                                    .overlay_text(overlay_text.as_ref())
                                    .build();
                            });
                        // ui.show_demo_window(&mut true);
                        imgui_platform.prepare_render(&ui, windowed_context.window());
                        imgui_renderer.render(ui);

                        // application-specific rendering *over the UI*

                        // let before_swap = Instant::now();
                        windowed_context.swap_buffers().unwrap();
                        // let elapsed = before_swap.elapsed();
                        // if elapsed > min_frame_timestep {
                        //     println!(
                        //         "Slow Swap Buffers Time: {:07.3}",
                        //         elapsed.as_secs_f64() * 1_000.0
                        //     );
                        // }
                    }
                    event => {
                        imgui_platform.handle_event(
                            imgui.io_mut(),
                            windowed_context.window(),
                            &event,
                        );

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
                                } => match (virtual_code, state, modifiers) {
                                    (VirtualKeyCode::Q, _, ModifiersState::LOGO) => {
                                        *control_flow = ControlFlow::Exit
                                    }
                                    (VirtualKeyCode::V, ElementState::Pressed, _) => {
                                        // TODO: figure out how to control vsync at runtime.

                                        use std::io::prelude::*;
                                        let serialized =
                                            serde_json::to_string(&fps_timings).unwrap();
                                        // println!("serialized = {}", serialized);
                                        let my_json = format!(r#"{{"fps": {}}}"#, serialized);
                                        let mut file = std::fs::File::create("data.json")
                                            .expect("create failed");
                                        file.write_all(my_json.as_bytes()).expect("write failed");
                                        println!("data written to file");
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
