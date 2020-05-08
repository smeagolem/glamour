use bracket_noise::prelude::*;
use glamour::{glm, Camera, ForwardRenderer, Layer, Transform};
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::ffi::CString;

pub struct SandboxLayer {
    max_cubes: usize,
    max_lights: usize,
    fr: ForwardRenderer,
    name: String,
    time: std::time::Instant,
    camera: Camera,
    cube_count: usize,
    cube_distribution: rand::distributions::Uniform<f32>,
    cube_transforms: Vec<Transform>,
    light_count: usize,
    light_transforms: Vec<Transform>,
    rng: rand_chacha::ChaCha8Rng,
    noise: FastNoise,
    selected_resolution: usize,
}

impl SandboxLayer {
    pub fn new(name: &str) -> Self {
        let max_cubes = 200_000;
        let max_lights = 1019;
        let fr = ForwardRenderer::new(max_cubes, max_lights);

        let seed = 912;
        let rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        let mut noise = FastNoise::seeded(seed - 1);
        noise.set_frequency(0.1);

        Self {
            max_cubes,
            max_lights,
            fr,
            name: name.to_string(),
            time: std::time::Instant::now(),
            camera: Camera::new(),
            cube_count: 50_000,
            cube_distribution: rand::distributions::Uniform::from(-100.0..100.0),
            cube_transforms: Vec::new(),
            light_count: 32,
            light_transforms: Vec::new(),
            rng,
            noise,
            selected_resolution: 0,
        }
    }
}

impl Layer for SandboxLayer {
    fn init(&mut self, app_context: &mut glamour::AppContext) {
        let size = app_context.windowed_context().window().inner_size();
        self.fr.resize(size.width, size.height);
    }

    fn on_frame_update(&mut self, app_context: &mut glamour::AppContext) {
        let delta_time = app_context.delta_time().as_secs_f32();
        let time = self.time.elapsed().as_secs_f32();

        // animate camera
        {
            let speed = 0.1;
            let radius = 10.1;
            let cam_x = (time * speed).sin() * radius;
            let cam_z = (time * speed).cos() * radius;
            self.camera.position = glm::vec3(cam_x, 0.0, cam_z);
            self.camera.target = glm::vec3(0.0, 0.0, 0.0);
        }

        let rng = &mut self.rng;
        let range = &self.cube_distribution;

        self.cube_transforms.resize_with(self.cube_count, || {
            Transform::from_pos(glm::vec3(
                rng.sample(range),
                rng.sample(range),
                rng.sample(range),
            ))
        });

        self.light_transforms.resize_with(self.light_count, || {
            Transform::from_pos(glm::vec3(
                rng.sample(range),
                rng.sample(range),
                rng.sample(range),
            ))
        });

        self.fr.begin_draw(&self.camera);

        self.cube_transforms
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, transform)| {
                transform.rotation = glm::quat_rotate(
                    &transform.rotation,
                    glm::radians(&glm::vec1(200.0 + (index % 100) as f32)).x * delta_time,
                    &glm::vec3(0.5, 1.0, 0.0),
                );
            });
        self.fr.set_cubes(&self.cube_transforms);

        let distance = 50.0;
        let noise = &self.noise;
        self.light_transforms
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, transform)| {
                let offset = index as f32;
                transform.position = glm::vec3(
                    noise.get_noise3d(time + offset, 0.0, 0.0),
                    noise.get_noise3d(0.0, time + offset, 0.0),
                    noise.get_noise3d(0.0, 0.0, time + offset),
                ) * distance
            });
        self.fr.set_lights(&self.light_transforms);

        self.fr.end_draw();
    }
    fn name(&self) -> &String {
        &self.name
    }
    fn on_event(&mut self, event: &glutin::event::Event<()>, _: &mut glamour::AppContext) {
        self.camera.handle_event(event);
        self.fr.handle_event(event);
    }
    fn on_imgui_update(&mut self, ui: &imgui::Ui, app_context: &mut glamour::AppContext) {
        imgui::Window::new(imgui::im_str!("Cubes!"))
            .size([340.0, 250.0], imgui::Condition::FirstUseEver)
            .always_auto_resize(true)
            .save_settings(false)
            .collapsed(false, imgui::Condition::FirstUseEver)
            .build(&ui, || {
                // cube slider
                {
                    let mut cube_count = self.cube_count as i32;
                    unsafe {
                        if imgui::sys::igSliderInt(
                            CString::new("Cube Count").unwrap().as_ptr(),
                            &mut cube_count,
                            0,
                            self.max_cubes as i32,
                            CString::new("%d").unwrap().as_ptr(),
                        ) {
                            self.cube_count = cube_count.max(0).min(self.max_cubes as i32) as usize;
                        }
                    }
                }
                // light slider
                {
                    let mut light_slider = self.light_count as i32;
                    unsafe {
                        if imgui::sys::igSliderInt(
                            CString::new("Light Count").unwrap().as_ptr(),
                            &mut light_slider,
                            0,
                            self.max_lights as i32,
                            CString::new("%d").unwrap().as_ptr(),
                        ) {
                            self.light_count =
                                light_slider.max(0).min(self.max_lights as i32) as usize;
                        }
                    }
                }
                // resolution combo box
                let resolutions: Vec<(u32, u32)> =
                    vec![(1280, 720), (1920, 1080), (2560, 1440), (3840, 2160)];
                {
                    if imgui::ComboBox::new(imgui::im_str!("Resolution")).build_simple(
                        ui,
                        &mut self.selected_resolution,
                        &resolutions,
                        &|r| {
                            std::borrow::Cow::from(imgui::ImString::new(format!(
                                "{} x {}",
                                r.0, r.1
                            )))
                        },
                    ) {
                        let res = resolutions[self.selected_resolution];
                        app_context
                            .windowed_context()
                            .window()
                            .set_inner_size(glutin::dpi::PhysicalSize::new(res.0, res.1));
                    }
                }
            });
    }
}
