use glamour::App;

mod dossier_layer;
use dossier_layer::DossierLayer;

use serde::{Deserialize, Serialize};

#[macro_use]
extern crate lazy_static;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct TestRun {
    resolution: (u32, u32),
    deferred: bool,
    lights: u32,
    cubes: u32,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct TestResult {
    min: u128,
    max: u128,
    avg: u128,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct TestRunResult {
    run: TestRun,
    result: TestResult,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestOutput {
    time: u128,
    warmup: u32,
    length: u32,
    data: Vec<TestRunResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestSpec {
    resolution_index: usize,
    warmup: u32,
    length: u32,
}

/*
Some math:
an exponential function that passes through two points:
(0, 1) and (5, 200_000)
y = ab^x
solve the simulateneous equations (using the internet) and you get
a = 1, b = 200_000^(1/5)
so,
y = (200_000^(1/5))^x

n = number of steps
m = max value
i = current step
s = step value

I want the function to equal 0 at i = 0, so let's fudge it a bit.

[(m+1)^(1/(n-1))^i]-1=s

[(200_000+1)^(1/(5-1))^3]-1=9_456

*/

fn step_value(m: u32, n: usize, i: usize) -> u32 {
    let m = (m + 1) as f64;
    let n = (n - 1) as f64;
    m.powf(1f64 / n).powi(i as i32).round() as u32 - 1
}

const RESOLUTIONS: [(u32, u32); 5] = [
    (640, 360),
    (1280, 720),
    (1920, 1080),
    (2560, 1440),
    (3840, 2160),
];
const DEFERRED: [bool; 2] = [false, true];
const MAX_LIGHTS: u32 = 1_000;
const LIGHT_STEPS: usize = 5;
lazy_static! {
    static ref LIGHTS: [u32; LIGHT_STEPS] = {
        let mut steps = [0; LIGHT_STEPS];
        for i in 0..LIGHT_STEPS {
            steps[i] = step_value(MAX_LIGHTS, LIGHT_STEPS, i);
        }
        steps
    };
}
const CUBE_STEPS: usize = 5;
const MAX_CUBES: u32 = 200_000;
lazy_static! {
    static ref CUBES: [u32; CUBE_STEPS] = {
        let mut steps = [0; CUBE_STEPS];
        for i in 0..CUBE_STEPS {
            steps[i] = step_value(MAX_CUBES, CUBE_STEPS, i);
        }
        steps
    };
}

fn _all_test_run_set() -> Vec<TestRun> {
    RESOLUTIONS
        .iter()
        .flat_map(|r| DEFERRED.iter().map(|d| (r, d)).collect::<Vec<_>>())
        .flat_map(|r| LIGHTS.iter().map(|l| (r.0, r.1, l)).collect::<Vec<_>>())
        .flat_map(|r| CUBES.iter().map(|c| (r.0, r.1, r.2, c)).collect::<Vec<_>>())
        .map(|r| TestRun {
            resolution: *r.0,
            deferred: *r.1,
            lights: *r.2,
            cubes: *r.3,
        })
        .collect()
}

fn test_run_set_from_res(resolution: (u32, u32)) -> Vec<TestRun> {
    DEFERRED
        .iter()
        .flat_map(|r| LIGHTS.iter().map(|l| (r, l)).collect::<Vec<_>>())
        .flat_map(|r| CUBES.iter().map(|c| (r.0, r.1, c)).collect::<Vec<_>>())
        .map(|r| TestRun {
            resolution,
            deferred: *r.0,
            lights: *r.1,
            cubes: *r.2,
        })
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let spec_file_path = args[1].clone();
    let file = std::fs::File::open(spec_file_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let spec: TestSpec = serde_json::from_reader(reader).unwrap();

    let resolution = RESOLUTIONS[spec.resolution_index];
    let run_set = test_run_set_from_res(resolution);
    let mut app = App::new("Glamour Dossier", resolution.0, resolution.1);
    let dossier_layer = DossierLayer::new(
        "DossierLayer",
        resolution,
        run_set,
        spec.warmup,
        spec.length,
    );
    app.push_layer(Box::new(dossier_layer));
    app.run();
}
