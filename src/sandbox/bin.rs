fn main() {
    let mut app = glamour::Application::new("It's a square!", 512, 512);
    let my_layer = glamour::Layer::new("Square Layer");
    app.push_layer(my_layer);
    app.run();
}
