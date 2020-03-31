use std::time::Instant;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

use imgui::{Condition, Context, FontConfig, FontSource, Window};
use imgui_winit_support::{HiDpiMode, WinitPlatform};

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("A fantastic window!");

    let windowed_context = ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(2)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    let mut imgui = Context::create();

    // step 1
    let mut platform = WinitPlatform::init(&mut imgui);
    // step 2
    platform.attach_window(
        imgui.io_mut(),
        windowed_context.window(),
        HiDpiMode::Default,
    );

    // fonts
    let hidpi_factor = platform.hidpi_factor();
    let font_size = (14.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("../resources/SourceCodePro-Regular.ttf"),
        size_pixels: font_size,
        config: Some(FontConfig::default()),
    }]);
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
    imgui.fonts().build_rgba32_texture();

    // load OpenGl
    gl::load_with(|s| windowed_context.context().get_proc_address(s) as *const _);

    // set imgui renderer to use raw OpenGL
    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
        windowed_context.context().get_proc_address(s) as _
    });

    let mut last_frame = Instant::now();
    el.run(move |event, _, control_flow| {
        // println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

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
            Event::LoopDestroyed => return,
            Event::RedrawRequested(_) => {
                let ui = imgui.frame();
                // application-specific rendering *under the UI*
                // gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                unsafe {
                    gl::ClearColor(1.0, 0.5, 0.7, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

                // construct the UI
                Window::new(imgui::im_str!("Hello world"))
                    .size([300.0, 100.0], Condition::FirstUseEver)
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
                // step 5
                platform.prepare_render(&ui, windowed_context.window());
                // render the UI with a renderer
                // let draw_data = ui.render();
                // renderer.render(..., draw_data).expect("UI rendering failed");
                renderer.render(ui);

                // application-specific rendering *over the UI*

                windowed_context.swap_buffers().unwrap();
            }
            event => {
                // step 3
                platform.handle_event(imgui.io_mut(), windowed_context.window(), &event);
                // other application-specific event handling
                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::Resized(physical_size) => {
                            windowed_context.resize(physical_size)
                        }
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
    });
}
