use glamour::Application;

mod square_layer;

fn main() {
    let mut app = Application::new("lit", 512, 490);
    // let mut app = Application::new("lit", 1280, 720);
    let square_layer = square_layer::SquareLayer::new("SquareLayer");
    app.push_layer(Box::new(square_layer));
    app.run();
}
