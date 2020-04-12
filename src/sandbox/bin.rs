use glamour::Application;

mod square_layer;

fn main() {
    let mut app = Application::new("It's a square!", 1024, 1024);
    let square_layer = square_layer::SquareLayer::new("SquareLayer");
    app.push_layer(Box::new(square_layer));
    app.run();
}
