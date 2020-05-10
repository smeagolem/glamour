# Foundation

## Crates
Rust's package manager, [Cargo](https://doc.rust-lang.org/cargo/), allows code dependencies to be easily required by specifying them in a `Cargo.toml` file. In Rust-land, packages are known as **Crates**.

Many crates are used in the application, with the most fundamental ones being outlined below.

## [gl](https://crates.io/crates/gl)
Unsafe bindings to raw OpenGL calls. This is what allows the application to interop with OpenGL on the system. If this crate didn't exist, then this application would have been written with C++ because generating function pointers for system APIs is not within the scope this project.

## [glutin](https://crates.io/crates/glutin)
Manages windowing, events, and the OpenGL context. Another excellent create for cross-platform windowing and IO. The functions from the **gl** crate get loaded by the OpenGL context that **glutin** creates.

## [imgui](https://crates.io/crates/imgui)
Immediate Mode Graphical User Interface. Creating UI is out of the scope of this project, IMGUI is an incredibly popular rendering-API-agnostic library for creating development tooling. This crate provides some safe abstraction as well as bindings to every IMGUI function.

## [nalgebra-glm](https://crates.io/crates/nalgebra-glm)
This crate is a subset of linear algebra tools from the [nalgebra](https://crates.io/crates/nalgebra) crate specifically for math used in graphics programming.
