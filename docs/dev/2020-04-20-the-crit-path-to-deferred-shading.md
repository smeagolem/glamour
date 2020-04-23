---
title: The Crit Path to Deferred Shading
---

<h-date>20/04/2020</h-date>
# The Crit Path to Deferred Shading
I need to get this **deferred shading** thing under wraps, so I've constructed the **MVP Critical Pathway to Success**! _Ooh, fancy_ 😮

According to [Learn OpenGL](https://learnopengl.com/Advanced-Lighting/Deferred-Shading), I only require the following data to light a fragment with forward rendering, verbatim:
- A 3D world-space **position** vector to calculate the (interpolated) fragment position variable used for `lightDir` and `viewDir`.
- An RGB diffuse **color** vector also known as _albedo_.
- A 3D **normal** vector for determining a surface's slope.
- A **specular intensity** float.
- All light source position and color vectors.
- The player or viewer's position vector.

I can use that exact same data to create a few **goemetry buffers** containing the rasterised position, normal, albedo, and specular data for the whole scene. Using the same lighting calculation with the new **G-buffer** input data, I can shade the whole scene, render it to a texture, and display it on a quad. That's the goal. To get there from where I currently am, I'll need a **critical path**.

## Critical Path
1. **Coordinate System**
    - **Entity** object to encapsulate a **Transform** component and **Renderer** data.
    - **Camera** component to handle view projection transforms, and move around the scene.
    - The [nalgebra-glm](https://crates.io/crates/nalgebra-glm) crate has all the required math funcionality.
1. **Forward Lighting**
    - **Light** component to with color, strength, and type (directional or spotlight).
    - **Material** component to encapsulate ambient, diffues, specular, and shininess values.
    - Create a **GLSL function** to calculate lighting from a **material's** values and all the **light** source values.
1. **Geometry**
    - I'm going to skip model loading for now, and just use some pre-defined cube vertex data.
    - Maybe I'll render the light sources as little spheres.
1. **Deferred Shading**
    - I will need to learn how to use **framebuffers**.
    - I might need to do something with the **Depth buffer**.
    - I will need to learn about **Multiple Render Targets** (MRT).
    - Update the lighting calculation to take input from the **G-buffer** framebuffer.
1. **Profit?**
    - Well, no, but, minimum viable product? Yes.
