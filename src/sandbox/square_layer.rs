use bracket_noise::prelude::*;
use glamour::{glm, Camera, ForwardRenderer, Layer, Transform};
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::ffi::CString;

pub struct SquareLayer {
    fr: ForwardRenderer,
    name: String,
    time: std::time::Instant,
    camera: Camera,
    cube_count: usize,
    cube_distribution: rand::distributions::Uniform<f32>,
    cube_transforms: Vec<Transform>,
    light_transforms: Vec<Transform>,
    rng: rand_chacha::ChaCha8Rng,
    noise: FastNoise,
}

impl SquareLayer {
    pub fn new(name: &str) -> Self {
        let fr = ForwardRenderer::new();

        let seed = 912;
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        let mut noise = FastNoise::seeded(seed - 1);
        noise.set_frequency(0.1);

        let mut light_transforms: Vec<Transform> = Vec::new();
        light_transforms.resize_with(32, std::default::Default::default);

        SquareLayer {
            fr,
            name: name.to_string(),
            time: std::time::Instant::now(),
            camera: Camera::new(),
            cube_count: 50_000,
            cube_distribution: rand::distributions::Uniform::from(-100.0..100.0),
            cube_transforms: Vec::new(),
            light_transforms,
            rng,
            noise,
        }
    }
}

impl Layer for SquareLayer {
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
            // self.camera.fov = 90.0 + time.sin() * 30.0;
        }

        // self.camera.position = glm::vec3(0.0, 0.0, 10.0);

        // self.fr.clear();

        // self.fr.shader().set_float4(
        //     "u_color",
        //     &glm::vec4(
        //         (time * 1.0).sin() / 2.0 + 0.5,
        //         (time * 2.0).sin() / 2.0 + 0.5,
        //         (time * 5.0).sin() / 2.0 + 0.5,
        //         1.0,
        //     ),
        // );

        let rng = &mut self.rng;
        let range = &self.cube_distribution;

        self.cube_transforms.resize_with(self.cube_count, || {
            Transform::from_pos(glm::vec3(
                rng.sample(range),
                rng.sample(range),
                rng.sample(range),
            ))
        });

        self.fr.begin_draw(&self.camera);
        // self.fr.draw_cube(&Transform {
        //     position: glm::vec3(0.0, -5.0, 0.0),
        //     rotation: glm::quat_identity(),
        //     scale: glm::vec3(10.0, 1.0, 10.0),
        // });
        // self.fr.draw_quad(&Transform {
        //     position: glm::vec3(-2.0, 1.0, 2.0),
        //     rotation: glm::quat_identity(),
        //     scale: glm::vec3(4.0, 4.0, 1.0),
        // });
        // self.fr.draw_triangle(&Transform {
        //     position: glm::vec3(-2.0, -3.0, 2.0),
        //     rotation: glm::quat_identity(),
        //     scale: glm::vec3(4.0, 4.0, 1.0),
        // });

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
                let mut cube_count = self.cube_count as i32;
                unsafe {
                    if imgui::sys::igSliderInt(
                        CString::new("Cube Count").unwrap().as_ptr(),
                        &mut cube_count,
                        0,
                        200_000,
                        CString::new("%d").unwrap().as_ptr(),
                    ) {
                        self.cube_count = cube_count.max(0).min(200_000) as usize;
                    }
                }
            });
    }
}
