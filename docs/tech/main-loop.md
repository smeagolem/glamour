# Main Loop

## Initialising
The `main` function in Sandbox only contains 4 lines:
```rs
fn main() {
    let mut app = glamour::App::new("Glamour Sandbox", 1920, 1080);
    let sandbox_layer = SandboxLayer::new("SandboxLayer");
    app.push_layer(Box::new(sandbox_layer));
    app.run();
}
```

Calling `glamour::App::new(title: &str, width: u32, height: u32)` will do the following:
- Create a new event loop from the `glutin` crate to handle OS I/O
- Build a window with the supplied `title`, `width`, and `height`
- Load OpenGL within that window's context
- Load imgui
- Initialise the Layer stack.

After this point, layers can be pushed to the app, and then run.

## Layer
A `Layer` is a basic trait to handle events of distinct parts of the application. Layers don't communicate with each other through the app, they're meant to be separate. There are only two layers used in the app:
- `PerfMetricsLayer` for reporting performance metrics
- `SandboxLayer` is a place to store all Sandbox related things.

`Layer` provides a few funtions that an implementation can use:
- `init()`
- `on_event()`
- `on_fixed_update()`
- `on_frame_update()`
- `on_imgui_update()`

Hopfully, those should be fairly self documentating.

## Event Loop
Calling `app.run()` kickstarts the main event loop, which is responsible for
- Polling for events
- Managing fixed and frame updates
- Updating and rendering imgui
- Calling the each layer's `Layer` trait functions at the appropriate point
- Swapping the frame buffer to actual show the rendered image on the screen.

## AppContext
`App` also maintains a struct that holds the context of the application. This is `AppContext`, it provides some convenience to access things such as the window context and frame timings. This is passed to each `Layer` function.
