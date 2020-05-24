use glamour::App;

mod sandbox_layer;
use sandbox_layer::SandboxLayer;

fn main() {
    let resolution = (512, 490);
    let mut app = App::new("Glamour Sandbox", resolution.0, resolution.1);
    let sandbox_layer = SandboxLayer::new("SandboxLayer", resolution);
    app.push_layer(Box::new(sandbox_layer));
    app.run();
}
