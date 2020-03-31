use glamour::{Application, Layer};

fn main() {
    let mut app = Application::new("It's a square!", 512, 512);
    let my_layer = Layer::new("Square Layer");
    app.push_layer(my_layer);
    app.run();
}
