# Technology
## Rust
[Rust](https://www.rust-lang.org/) is a systems prorgramming language that guarantees memeroy safety at compile time. Rust doesn't need a garbage collector so the performance is excellent with little to no overhead. There is zero-cost bidirectional interopability with C via the FFI; a basic requirement for communicating with OpenGL on the system. The modern Rust toolchain makes it trivial to have a fast and friendly developer experience.

## OpenGL
[OpenGL](https://www.opengl.org/) is the go-to cross-platform graphics API with a wealth of learning resources available. Using OpenGL and Rust enables the application to run on Windows, Mac OS, and Linux without much platform specific thought. OpenGL `4.1` is minumum target version as that is the latest version supported on Mac OS. Vulkan was considered, but due the increased complexity of properly implementing a Vulkan based application, OpenGL was the simpler choice.

## VS Code
Continuing with cross-platform tools, [VS Code](https://code.visualstudio.com/) is the text-editor/IDE of choice. Using the excellent [rust-analyzer](https://rust-analyzer.github.io/) extension, it's a decent Rust IDE. With various other extensions, writing shader code and documentation is also pleasant.

## GitHub
Several services on [GitHub](https://github.com/) are used:
- It's the main git remote repository for the code
- Issues are tracked on there
- Projects, labels, and milestones help triage those issues
- Actions are used to build and test the code (CI)
- Pages is used to host this website which is deployed from Actions (CD).

## Vuepress
[Vuepress](https://vuepress.vuejs.org/) is a static site generator particularly designed for writing technical documentation. It is what powers this website.
