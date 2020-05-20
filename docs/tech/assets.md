# Assets

## Asset Management
Since Cargo only deals with code, a `build.rs` script was created to facilitate copying the assets source folder to it's destination directory relative to the built executable.

There also exists an `asset` module that exports a helper function `assets_path()` to obtain the assets directory at runtime, to make loading the assets easier.

## Including resources
Some files are loaded directly into the application binary at compile time. This includes the imgui font and the shaders. Rust has a couple of helper macros that make this trivial, namely, `include_bytes!()` and `include_str!()`. This is something that seems incredibly simple, but cannot be done with just C++ without changing the original file to make it a raw string. _50 points to Rustlepuff!_ 🧙‍♂️
