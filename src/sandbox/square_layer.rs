use glamour::{glm, Camera, ForwardRenderer, Layer, Transform};

pub struct SquareLayer {
    fr: ForwardRenderer,
    name: String,
    time: std::time::Instant,
    camera: Camera,
    cube_transforms: Vec<Transform>,
}

impl SquareLayer {
    pub fn new(name: &str) -> Self {
        let fr = ForwardRenderer::new();
        let cube_positions = vec![
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(2.0, 5.0, -15.0),
            glm::vec3(-2.5, -2.2, -3.5),
            glm::vec3(-4.8, -2.0, -12.3),
            glm::vec3(3.4, -0.4, -8.5),
            glm::vec3(-2.7, 3.0, -7.5),
            glm::vec3(3.3, -2.0, -3.5),
            glm::vec3(2.5, 4.0, -5.5),
            glm::vec3(4.5, 0.2, -2.5),
            glm::vec3(-2.3, 1.0, -1.5),
        ];
        let cube_transforms = cube_positions
            .iter()
            .map(|p| {
                let mut transform = Transform::new();
                transform.position = *p;
                transform
            })
            .collect();

        SquareLayer {
            fr,
            name: name.to_string(),
            time: std::time::Instant::now(),
            camera: Camera::new(),
            cube_transforms,
        }
    }
}

impl Layer for SquareLayer {
    fn on_frame_update(&mut self, app_context: &mut glamour::AppContext) {
        let delta_time = app_context.delta_time().as_secs_f32();
        let time = self.time.elapsed().as_secs_f32();

        // animate camera
        {
            let radius = 10.0;
            let cam_x = time.sin() * radius;
            let cam_z = time.cos() * radius;
            self.camera.position = glm::vec3(cam_x, 0.0, cam_z);
            self.camera.target = glm::vec3(0.0, 0.0, 0.0);
            self.camera.fov = 90.0 + time.sin() * 30.0;
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
        self.fr.draw_cube(&Transform {
            position: glm::vec3(0.0, -5.0, 0.0),
            rotation: glm::quat_identity(),
            scale: glm::vec3(10.0, 1.0, 10.0),
        });
        self.fr.draw_quad(&Transform {
            position: glm::vec3(-2.0, 1.0, 2.0),
            rotation: glm::quat_identity(),
            scale: glm::vec3(4.0, 4.0, 1.0),
        });
        self.fr.draw_triangle(&Transform {
            position: glm::vec3(-2.0, -3.0, 2.0),
            rotation: glm::quat_identity(),
            scale: glm::vec3(4.0, 4.0, 1.0),
        });
        for (index, transform) in self.cube_transforms.iter_mut().enumerate() {
            transform.position = glm::vec3(
                transform.position.x,
                (time + 2.0 * index as f32).sin(),
                transform.position.z,
            );
            transform.rotation = glm::quat_rotate(
                &transform.rotation,
                glm::radians(&glm::vec1((index + 1) as f32 * delta_time * 20.0)).x,
                &glm::vec3(0.5, 1.0, 0.0),
            );
            transform.scale = glm::vec3(1.0, (time + index as f32).sin() + 1.5, 1.0);
            self.fr.draw_cube(&transform);
        }
        self.fr.draw_light(&Transform {
            // position: glm::vec3(4.0, 4.0, 4.0),
            position: glm::vec3(4.0, (time * 2.0).sin() * 4.0, 4.0),
            rotation: glm::quat_identity(),
            scale: glm::vec3(1.0, 1.0, 1.0),
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
