---
title: Finding Problems
---

<h-date>16/04/2020</h-date>
# Finding Problems

In the past week, I was looking forward to getting really stuck into the rendering side of things, and finally get something more _interesting_ on the screen. I was following along with some [tutorials](https://learnopengl.com/), [documentation](http://docs.gl/gl4/glVertexAttribPointer), and [examples](https://github.com/TheCherno/Hazel) to further the progress. I started with the goal of loading and displaying a **texture** — _a simple task_, I thought, _couldn't take more than an hour_ — then to encounter a few roadblocks/sub-problems/side-missions which stood in my way, of which, I will explain now:

## Resource handling with Cargo
Since this is a [Rust](https://www.rust-lang.org/) project, the idiom is to use [Cargo](https://doc.rust-lang.org/cargo/) as the build tool. **Cargo** has been great up to this point, but I found it lacking for this seemingly trivial task. I wanted to load a texture in my application at runtime, meaning, when I build my app I want that texture asset to be copied relative to the executable's output directory.

I thought this would be a single config variable in `Cargo.toml`, i.e., `resource-dir = "src/assets"`. Woe is me, tis not. You see, **Cargo** only really deals with **`code`**, and after some [🦆quacking](https://duckduckgo.com/?q=rust+build+resource+folder) I found out that if you want to do anything else, well you'll need a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html). And thus, `build.rs` was born.

```rust
// build.rs

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // get the source assets directory, which is found relative to cargo.toml
    let src_assets_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "src", "assets"]
        .iter()
        .collect();

    // locate target directory by walking upwards from out directory
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_dir: Option<&Path> = {
        let mut cwd = Some(out_dir.as_path());
        loop {
            match cwd {
                // it's probably the right directory if it ends with target
                Some(dir) if dir.ends_with("target") => break,
                Some(dir) => cwd = dir.parent(),
                None => break,
            }
        }
        cwd
    };

    // locate the destination asset directory, which is in the current build
    // profile directory, in the target directory
    let dest_assets_dir = PathBuf::from(target_dir.unwrap())
        .join(env::var("PROFILE").unwrap())
        .join("assets");

    // no need to explain this 🔥
    if dest_assets_dir.exists() {
        std::fs::remove_dir_all(dest_assets_dir.as_path()).unwrap();
    }

    // finally, straight up recursively copy every file using the copy_dir crate
    copy_dir::copy_dir(src_assets_dir, dest_assets_dir).expect("😢");
}
```

Now, this is a rudimentary way of solving this issue. In the future, if I have hundreds of megabytes of assets, I probably shouldn't delete everything and copy it all again on every build (especially when the vs code rust-analyzer plugin seems to run the script too). I may eventually have to walk the destination directory myself and check if each file was modified, and if so, replace it. But we'll burn that bridge as we cross it.

## Refactoring vertex arrays
So, now I had my texture asset loading from the correct directory and sent to OpenGL for rendering. _Problem solved_, nope, not problem solved. I still had to tell my sqaure shader to use the texture, meaning I had to **bind the texture** before drawing, which requires the **fragment shader** to have a **sampler2D** and **texture coordinates**, requiring the **vertex shader** to output **texture coordinates** to the **fragment shader**, requiring the **vertex array object** to have a **vertex buffer** with a **layout** including an **attribute** set to the correct **indexed location** and **data type** (and element count, stride, offset, and if to normalize)... which requires the **vertex data** to actually have **texture coordinates** in the first place.

As can be seen, that's a long string of dependent intrinsics to deal with. So, I had to refactor and encapsulate that code, otherwise I would lose my mind having to remember all of that when I add normals to the vertex data or try and load some arbitrary mesh. 

My vertex data is now like this:
```rust
struct Vert {
    position: glm::Vec3,
    tex_coords: glm::Vec2,
}
```

And, I can easily reflect that layout:
```rust
let vert_layout = VertLayout::new(vec![
    VertAttr::new(VertAttrType::Float3, false),
    VertAttr::new(VertAttrType::Float2, false),
]);
```

This can then be passed to a **vertex buffer** and then a **vertex array**, which can calculate 90% of the data from that `VertAttrType` enum I'm using, e.g., a `Float3` is 3, 32-bit floats, which gives me the count, size (in bytes), and type, then I use its index in the vector for the location, and calculate the offset from the size of the attributes before it. The stride is the sum of all attribute sizes.

In retrospect, this was a very necessary fundamental step, although annoying at the time because I just wanted to render a texture. 🤷

Full code [here](https://github.com/denovodavid/glamour/commit/775b0cd3b3535cf7ab8e424932f3170215ea8e37).

## Some other wins
- Refactored layers and the main loop to cleft some responsibility in twain — [commit](https://github.com/denovodavid/glamour/commit/5d6db2cd2a3a09e2215937184da03a38f2dd9067).
- Added a shader builder to simplify compiling shaders — [commit](https://github.com/denovodavid/glamour/commit/4d60d17651c46628802c6c1589d7e65ca8d6d030).
