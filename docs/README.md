---
home: true
heroImage: /crystal-big.svg
heroText: Glamour
tagline: 🦀 Rust & OpenGL 🦀
actionText: Technical Report →
actionLink: /tech/
features:
- title: Graphics Pipelines
  details: How are graphics APIs utilised to create high-performing real-time rendering pipelines? Design, implement, analyse, and discuss forward rendering and deferred rendering techniques.
- title: Forward Rendering
  details: Each object to be rendered gets rasterised, shaded, and drawn to the screen sequentially. Shading is calculated for each object individually.
- title: Deferred Shading
  details: Rasterise each object's geometry into buffers; position, normal, albedo, etc.. By sampling each of the geometry buffers, shading is calculated for the frame as a whole.
---

## Test the sandbox

```sh
# checkout repo
git clone https://github.com/denovodavid/glamour.git

# navigate to directory
cd glamour

# build and run
cargo run
```

::: tip
Use [rustup](https://www.rust-lang.org/tools/install) to install the latest **Rust** toolchain.
:::

<div class="footer">
Icons made by <a href="https://www.flaticon.com/authors/eucalyp" title="Eucalyp">Eucalyp</a> from <a href="https://www.flaticon.com/" title="Flaticon"> www.flaticon.com</a>
</div>