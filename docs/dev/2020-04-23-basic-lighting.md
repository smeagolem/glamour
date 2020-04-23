---
title: Basic Lighting
---

<h-date>23/04/2020</h-date>
# Basic Lighting

One again, many things have happened, as seen in the tweet below. In the past three days I have added the coordinate system with camera and transformations, refactored the rendering API to a more immediate pipeline, and added basic ambient + diffuse + specular shading. All that refactoring of vertex arrays has definitely come in handy and allowed me to fix a couple of bugs as I understood more about what _exactly_ a **vertex array** is.

<blockquote class="twitter-tweet" data-conversation="none" data-theme="light"><p lang="en" dir="ltr">Ambient + diffuse + specular lighting ✨ <a href="https://twitter.com/hashtag/rustlang?src=hash&amp;ref_src=twsrc%5Etfw">#rustlang</a> <a href="https://twitter.com/hashtag/opengl?src=hash&amp;ref_src=twsrc%5Etfw">#opengl</a> <a href="https://t.co/LQV0GKX5xC">pic.twitter.com/LQV0GKX5xC</a></p>&mdash; David J Holland (@davidlemonboy) <a href="https://twitter.com/davidlemonboy/status/1252910873431011329?ref_src=twsrc%5Etfw">April 22, 2020</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

## Vertex Arrays are not Arrays of Vertices
Well, I knew that much coming into it, but I was lost on the specifics. Here's what I now understand.

::: tip Vertex Array
A **vertex array** is an object that, when currently bound, _captures the state_ of **vertex attributes** and **index buffers**. Binding it again, will _rebind_ that _captured state_.
:::

There are only 4 OpenGL functions that a currently bound **vertex array** will capture state changes of:
#### 👉 [**`glEnableVertexAttribArray(...)` / `glDisableVertexAttribArray(...)`**](http://docs.gl/gl4/glEnableVertexAttribArray)
Enable/disable **vertex attributes**, e.g., enable `location = 0` so I can send vertex positions to my vertex shader.
#### 👉 [**`glVertexAttribPointer(...)`**](http://docs.gl/gl4/glVertexAttribPointer)
Tell OpenGL what part of the **vertex buffer** is the attribute, e.g., the first 3 floats are the position.
::: warning Important! 
Calling this will cause the **vertex array** to _implicity_ capture the currently bound **vertex buffer** to know where the data is.
:::
#### 👉 [**`glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ...)`**](http://docs.gl/gl4/glBindBuffer)
Bind (or unbind) an **index buffer**.
::: warning Unbinding Matters.
Make sure to unbind the **vertex array** before unbinding the **index buffer**, otherwise (since the same function binds/unbinds), the **vertex array** will capture the _unbound_ state.
:::

Knowing all this now, I have been very careful to bind/unbind everything correctly, in order not to capture the wrong state, or leave my **vertex array** bound. 

> Please, please, this is supposed to be a _happy_ occasion, let's not _bicker_ and _argue_ about who unbound who. We are here today, to witness the **Critical Path** progress.

## Critical Path `v1.1`
1. ✅ **Coordinate System**
    - ~~**Entity** object to encapsulate a **Transform** component and **Renderer** data.~~
      - _Just have a list of transforms as the rendering data is only cubes._
    - ~~**Camera** component to handle view projection transforms, and move around the scene.~~
      - _Camera automatically orbits the scene, no manual control._
    - ~~The [nalgebra-glm](https://crates.io/crates/nalgebra-glm) crate has all the required math funcionality.~~
1. 🚧 **Forward Lighting**
    - ✅ ~~**Light** component to with color, strength, and type (directional or spotlight).~~
      - _Just using spotlights for now._
    - **Material** component to encapsulate ambient, diffues, specular, and shininess values.
    - 🚧 Create a **GLSL function** to calculate lighting from a **material's** values and all the **light** source values.
1. 🚧 **Geometry**
    - ✅ ~~I'm going to skip model loading for now, and just use some pre-defined cube vertex data.~~
    - 🚫 ~~Maybe I'll render the light sources as little spheres.~~
    - 💎 Use [instanced rendering](https://learnopengl.com/Advanced-OpenGL/Instancing) to more easily manage **vertex/index/transform data**.
1. ✋ **Deferred Shading**
    - I will need to learn how to use **framebuffers**.
    - I might need to do something with the **Depth buffer**.
    - I will need to learn about **Multiple Render Targets** (MRT).
    - Update the lighting calculation to take input from the **G-buffer** framebuffer.

| Mark | Description |       
| :--: | ----------- |
| ✅  | Done        |
| 🚧  | WIP         |
| ✋  | Blocked     |
| 🚫  | Removed     |
| 💎  | New         |
