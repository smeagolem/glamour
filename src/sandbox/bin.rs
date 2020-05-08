use glamour::App;

mod sandbox_layer;
use sandbox_layer::SandboxLayer;

fn main() {
    let mut app = App::new("Glamour Sandbox", 512, 490);
    let sandbox_layer = SandboxLayer::new("SandboxLayer");
    app.push_layer(Box::new(sandbox_layer));
    app.run();
}
