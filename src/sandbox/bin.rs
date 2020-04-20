use glamour::Application;

mod square_layer;

fn main() {
    let mut app = Application::new("It's cubes now!", 510, 488);
    let square_layer = square_layer::SquareLayer::new("SquareLayer");
    app.push_layer(Box::new(square_layer));
    app.run();
}
