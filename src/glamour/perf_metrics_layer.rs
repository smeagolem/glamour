use crate::{AppContext, Layer};

pub struct PerfMetricsLayer {
    name: String,
    fixed_delta_time: std::time::Duration,
    last_fixed_update: std::time::Instant,
    fps_timings_max_capacity: usize,
    fps_timings: Vec<f32>,
    frames_to_skip: u32,
    skipped_frames: u32,
    max_frame_rate: f32,
}

impl PerfMetricsLayer {
    pub fn new() -> Self {
        let fps_timings_max_capacity = 300;
        PerfMetricsLayer {
            name: "PerfMetricsLayer".to_string(),
            fixed_delta_time: std::time::Duration::from_secs(0),
            last_fixed_update: std::time::Instant::now(),
            fps_timings_max_capacity,
            fps_timings: Vec::<f32>::with_capacity(fps_timings_max_capacity),
            frames_to_skip: 20,
            skipped_frames: 0,
            max_frame_rate: 60.0,
        }
    }
}

impl Layer for PerfMetricsLayer {
    fn name(&self) -> &String {
        return &self.name;
    }

    fn init(&mut self, app_context: &mut AppContext) {
        self.max_frame_rate = app_context.max_frame_rate();
    }

    fn on_fixed_update(&mut self, _: &mut AppContext) {
        let now = std::time::Instant::now();
        self.fixed_delta_time = now - self.last_fixed_update;
        self.last_fixed_update = now;
    }

    fn on_frame_update(&mut self, app_context: &mut AppContext) {
        if self.skipped_frames < self.frames_to_skip {
            self.skipped_frames += 1;
            return;
        }
        self.fps_timings
            .insert(0, 1.0 / app_context.delta_time().as_secs_f32());
        self.fps_timings.truncate(self.fps_timings_max_capacity);
    }

    fn on_imgui_update(&mut self, ui: &imgui::Ui, app_context: &mut AppContext) {
        imgui::Window::new(imgui::im_str!("Performance Metrics"))
            .size([340.0, 250.0], imgui::Condition::FirstUseEver)
            .always_auto_resize(true)
            .position_pivot([1.0, 0.0])
            .position(
                [
                    app_context.windowed_context().window().inner_size().width as f32
                        / app_context.imgui_platform().hidpi_factor() as f32,
                    0.0,
                ],
                imgui::Condition::FirstUseEver,
            )
            .save_settings(false)
            .collapsed(true, imgui::Condition::FirstUseEver)
            .build(&ui, || {
                ui.text(format!(
                    "Event Poll Time: {:06.3} ms",
                    app_context.event_poll_time().as_secs_f64() * 1_000.0
                ));
                ui.text(format!(
                    "Fixed Delta Time: {:06.3} ms (Timestep: {:06.3})",
                    self.fixed_delta_time.as_secs_f64() * 1_000.0,
                    app_context.fixed_timestep().as_secs_f64() * 1_000.0,
                ));
                ui.text(format!(
                    "Delta Time: {:06.3} ms (Max Frame Rate: {:03.0} FPS)",
                    app_context.delta_time().as_secs_f64() * 1_000.0,
                    app_context.max_frame_rate(),
                ));
                if ui
                    .drag_float(imgui::im_str!("Max Frame Rate"), &mut self.max_frame_rate)
                    .min(30.0)
                    .max(300.0)
                    .display_format(imgui::im_str!("%g"))
                    .build()
                {
                    app_context.set_min_frame_timestep(std::time::Duration::from_secs_f32(
                        1.0 / self.max_frame_rate,
                    ));
                };

                let min_fps =
                    self.fps_timings.iter().fold(
                        std::f32::MAX,
                        |min, &val| {
                            if val < min {
                                val
                            } else {
                                min
                            }
                        },
                    );
                let max_fps =
                    self.fps_timings
                        .iter()
                        .fold(0.0f32, |max, &val| if val > max { val } else { max });
                let ave_fps: f32 =
                    self.fps_timings.iter().sum::<f32>() / self.fps_timings.len() as f32;
                let overlay_text = imgui::im_str!(
                    "Min: {:06.3} | Max: {:06.3} | Ave: {:06.3}",
                    min_fps,
                    max_fps,
                    ave_fps,
                );
                ui.plot_lines(imgui::im_str!("FPS"), &self.fps_timings)
                    .graph_size([300.0, 100.0])
                    .overlay_text(overlay_text.as_ref())
                    .build();
            });
    }

    fn on_event(&mut self, event: &glutin::event::Event<()>, _: &mut AppContext) {
        use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

        // TODO: should probs have a fn or macro to make this shorter...
        match event {
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::V),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                use std::io::prelude::*;
                let serialized = serde_json::to_string(&self.fps_timings).unwrap();
                let my_json = format!(r#"{{"fps": {}}}"#, serialized);
                let mut file = std::fs::File::create("data.json").expect("create failed");
                file.write_all(my_json.as_bytes()).expect("write failed");
                println!("data written to file");
            }
            _ => (),
        }
    }
}
