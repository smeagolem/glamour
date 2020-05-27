use crate::{TestOutput, TestResult, TestRun, TestRunResult};
use bracket_noise::prelude::*;
use glamour::{glm, Camera, Layer, Renderer, Transform};
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

pub struct DossierLayer {
    renderer: Renderer,
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
    test_run_set: Vec<TestRun>,
    test_run_timer: std::time::Instant,
    test_run_length: std::time::Duration,
    test_run_index: usize,
    test_run_fps_timings: Vec<u128>,
    test_run_output: TestOutput,
}

impl DossierLayer {
    pub fn new(name: &str, resolution: (u32, u32), test_run_set: Vec<TestRun>) -> Self {
        let max_cubes = 200_000;
        let max_lights = 1019;
        let mut renderer = Renderer::new(resolution, max_cubes, max_lights);

        let seed = 912;
        let rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        let mut noise = FastNoise::seeded(seed - 1);
        noise.set_frequency(0.1);

        let test_run_index = 0;
        let test_run = test_run_set[test_run_index];

        renderer.set_deferred(test_run.deferred);

        Self {
            renderer,
            name: name.to_string(),
            time: std::time::Instant::now(),
            camera: Camera::new(),
            cube_count: test_run.cubes as usize,
            cube_distribution: rand::distributions::Uniform::from(-100.0..100.0),
            cube_transforms: Vec::new(),
            light_count: test_run.lights as usize,
            light_transforms: Vec::new(),
            rng,
            noise,
            test_run_set,
            test_run_timer: std::time::Instant::now(),
            test_run_length: std::time::Duration::from_secs(20),
            test_run_index,
            test_run_fps_timings: Vec::new(),
            test_run_output: TestOutput {
                time: 0,
                data: Vec::new(),
            },
        }
    }

    fn test_run(&self) -> TestRun {
        self.test_run_set[self.test_run_index]
    }
}

impl Layer for DossierLayer {
    fn init(&mut self, app_context: &mut glamour::AppContext) {
        let size = app_context.windowed_context().window().inner_size();
        self.renderer.resize(size.width, size.height);
    }
    fn on_fixed_update(&mut self, app_context: &mut glamour::AppContext) {
        if self.test_run_timer.elapsed() > self.test_run_length {
            let min = *self.test_run_fps_timings.iter().min().unwrap();
            let max = *self.test_run_fps_timings.iter().max().unwrap();
            let avg = self.test_run_fps_timings.iter().sum::<u128>()
                / self.test_run_fps_timings.len() as u128;
            self.test_run_output.data.push(TestRunResult {
                run: self.test_run(),
                result: TestResult { min, max, avg },
            });

            self.test_run_fps_timings.clear();
            self.test_run_timer = std::time::Instant::now();
            self.test_run_index += 1;
            if self.test_run_index < self.test_run_set.len() {
                let test_run = self.test_run();
                self.cube_count = test_run.cubes as _;
                self.light_count = test_run.lights as _;
                self.renderer.set_deferred(test_run.deferred);
            } else {
                use std::io::prelude::*;
                let id = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                self.test_run_output.time = id;
                let serialized = serde_json::to_string(&self.test_run_output).unwrap();
                let filename = format!("{}.json", id);
                let mut file = std::fs::File::create(filename).expect("create failed");
                file.write_all(serialized.as_bytes()).expect("write failed");
                {
                    let mut wtr = csv::Writer::from_path(format!("{}.csv", id)).unwrap();
                    self.test_run_output.data.iter().for_each(|x| {
                        wtr.serialize(x.result).unwrap();
                    });
                    wtr.flush().unwrap();
                }
                println!("data written to file");
                app_context.exit();
            }
        }
    }
    fn on_frame_update(&mut self, app_context: &mut glamour::AppContext) {
        let delta_time = app_context.delta_time().as_secs_f32();
        let time = self.time.elapsed().as_secs_f32();

        self.test_run_fps_timings
            .push(app_context.delta_time().as_nanos());

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

        self.renderer.begin_draw(&self.camera);

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
        self.renderer.set_cubes(&self.cube_transforms);

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
        self.renderer.set_lights(&self.light_transforms);

        self.renderer.end_draw();
    }
    fn name(&self) -> &String {
        &self.name
    }
    fn on_event(&mut self, event: &glutin::event::Event<()>, _: &mut glamour::AppContext) {
        self.camera.handle_event(event);
        self.renderer.handle_event(event);
    }
}
