use glamour::App;

mod dossier_layer;
use dossier_layer::DossierLayer;

use serde::{Deserialize, Serialize};

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
    data: Vec<TestRunResult>,
}

const RESOLUTIONS: [(u32, u32); 5] = [
    (640, 360),
    (1280, 720),
    (1920, 1080),
    (2560, 1440),
    (3840, 2160),
];
const DEFERRED: [bool; 2] = [false, true];
const LIGHTS: [u32; 5] = [0, 10, 100, 500, 1_000];
const CUBES: [u32; 5] = [0, 1_000, 10_000, 100_000, 200_000];

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

fn test_run_set(resolution: (u32, u32)) -> Vec<TestRun> {
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
    let resolution_index: usize = args[1].parse().unwrap();
    let resolution = RESOLUTIONS[resolution_index];
    let run_set = test_run_set(resolution);
    let mut app = App::new("Glamour Dossier", resolution.0, resolution.1);
    let dossier_layer = DossierLayer::new("DossierLayer", resolution, run_set);
    app.push_layer(Box::new(dossier_layer));
    app.run();
}
