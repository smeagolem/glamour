use bracket_noise::prelude::*;
use glamour::{glm, Camera, ForwardRenderer, Layer, Transform};
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

pub struct SquareLayer {
    fr: ForwardRenderer,
    name: String,
    time: std::time::Instant,
    camera: Camera,
    // cube_transforms: Vec<Transform>,
    // rng: rand_chacha::ChaCha8Rng,
    noise: FastNoise,
}

impl SquareLayer {
    pub fn new(name: &str) -> Self {
        let seed = 912;
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        let mut noise = FastNoise::seeded(seed - 1);
        noise.set_frequency(0.1);

        let range = rand::distributions::Uniform::from(-100.0..100.0);
        let cube_count = 150_000;

        let mut cube_transforms: Vec<Transform> = (&mut rng)
            .sample_iter(range)
            .take(cube_count * 3)
            .collect::<Vec<f32>>()
            .chunks_exact(3)
            .map(glm::make_vec3)
            .map(Transform::from_pos)
            .collect();

        let mut fr = ForwardRenderer::new();

        fr.init_cubes(cube_count);
        let vertices = fr.cube_trans_vbo.vertices_mut();
        vertices
            .iter_mut()
            .zip(cube_transforms.iter_mut())
            .for_each(|(v, t)| {
                t.rotation = rng.gen::<glm::Quat>().normalize();
                let trans_mat = t.matrix();
                v.transform = trans_mat;
                v.normal = glm::mat4_to_mat3(&glm::inverse_transpose(trans_mat));
            });

        SquareLayer {
            fr,
            name: name.to_string(),
            time: std::time::Instant::now(),
            camera: Camera::new(),
            // cube_transforms,
            // rng,
            noise,
        }
    }
}

impl Layer for SquareLayer {
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

        self.fr.clear();

        // self.fr.shader().set_float4(
        //     "u_color",
        //     &glm::vec4(
        //         (time * 1.0).sin() / 2.0 + 0.5,
        //         (time * 2.0).sin() / 2.0 + 0.5,
        //         (time * 5.0).sin() / 2.0 + 0.5,
        //         1.0,
        //     ),
        // );

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

        let now = std::time::Instant::now();
        let vertices = self.fr.cube_trans_vbo.vertices_mut();
        vertices
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, vertex)| {
                vertex.transform = glm::rotate(
                    &vertex.transform,
                    glm::radians(&glm::vec1(200.0 + (index % 100) as f32)).x * delta_time,
                    &glm::vec3(0.5, 1.0, 0.0),
                    // &(self.rng.gen()),
                    // &self.rand_axes[index % 1000],
                );
                // vertex.transform = glm::scale(&vertex.transform, &glm::vec3(1.0, 1.0, 1.0));
                vertex.normal = glm::mat4_to_mat3(&glm::inverse_transpose(vertex.transform));
            });
        // println!("vertex loop: {} ms", now.elapsed().as_millis());

        // let now = std::time::Instant::now();
        // let cube_transforms = &mut self.cube_transforms;
        // cube_transforms
        //     .par_iter_mut()
        //     .enumerate()
        //     .for_each(|(index, transform)| {
        //         // transform.position = glm::vec3(
        //         //     transform.position.x,
        //         //     (time + 2.0 * index as f32).sin(),
        //         //     transform.position.z,
        //         // );
        //         transform.rotation = glm::quat_rotate(
        //             &transform.rotation,
        //             glm::radians(&glm::vec1(delta_time * 200.0 + (index % 10) as f32)).x,
        //             // &(self.rng.gen()),
        //             &glm::vec3(0.5, 1.0, 0.0),
        //         );
        //         // transform.scale = glm::vec3(1.0, (time + index as f32).sin() + 1.5, 1.0);
        //         transform.scale = glm::vec3(1.0, 1.0, 1.0);
        //         // self.fr.draw_cube(&transform);
        //     });
        // self.fr.draw_cubes(&self.cube_transforms);
        // println!("draw_cubes: {} ms", now.elapsed().as_millis());

        let distance = 50.0;
        (0..32).for_each(|index| {
            let offset = index as f32;
            self.fr.draw_light(&Transform {
                position: glm::vec3(
                    self.noise.get_noise3d(time + offset, 0.0, 0.0),
                    self.noise.get_noise3d(0.0, time + offset, 0.0),
                    self.noise.get_noise3d(0.0, 0.0, time + offset),
                ) * distance,
                rotation: glm::quat_identity(),
                scale: glm::vec3(1.0, 1.0, 1.0),
            });
        });
        self.fr.end_draw();
    }
    fn name(&self) -> &String {
        &self.name
    }
    fn on_event(&mut self, event: &glutin::event::Event<()>, _: &mut glamour::AppContext) {
        self.camera.handle_event(event);
    }
}
