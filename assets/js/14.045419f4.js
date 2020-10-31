(window.webpackJsonp=window.webpackJsonp||[]).push([[14],{334:function(e,t,r){"use strict";r.r(t);var a=r(6),n=Object(a.a)({},(function(){var e=this,t=e.$createElement,r=e._self._c||t;return r("ContentSlotsDistributor",{attrs:{"slot-key":e.$parent.slotKey}},[r("h-date",[e._v("20/04/2020")]),e._v(" "),r("h1",{attrs:{id:"the-crit-path-to-deferred-shading"}},[r("a",{staticClass:"header-anchor",attrs:{href:"#the-crit-path-to-deferred-shading"}},[e._v("#")]),e._v(" The Crit Path to Deferred Shading")]),e._v(" "),r("p",[e._v("I need to get this "),r("strong",[e._v("deferred shading")]),e._v(" thing under wraps, so I've constructed the "),r("strong",[e._v("MVP Critical Pathway to Success")]),e._v("! "),r("em",[e._v("Ooh, fancy")]),e._v(" 😮")]),e._v(" "),r("p",[e._v("According to "),r("a",{attrs:{href:"https://learnopengl.com/Advanced-Lighting/Deferred-Shading",target:"_blank",rel:"noopener noreferrer"}},[e._v("Learn OpenGL"),r("OutboundLink")],1),e._v(", I only require the following data to light a fragment with forward rendering, verbatim:")]),e._v(" "),r("ul",[r("li",[e._v("A 3D world-space "),r("strong",[e._v("position")]),e._v(" vector to calculate the (interpolated) fragment position variable used for "),r("code",[e._v("lightDir")]),e._v(" and "),r("code",[e._v("viewDir")]),e._v(".")]),e._v(" "),r("li",[e._v("An RGB diffuse "),r("strong",[e._v("color")]),e._v(" vector also known as "),r("em",[e._v("albedo")]),e._v(".")]),e._v(" "),r("li",[e._v("A 3D "),r("strong",[e._v("normal")]),e._v(" vector for determining a surface's slope.")]),e._v(" "),r("li",[e._v("A "),r("strong",[e._v("specular intensity")]),e._v(" float.")]),e._v(" "),r("li",[e._v("All light source position and color vectors.")]),e._v(" "),r("li",[e._v("The player or viewer's position vector.")])]),e._v(" "),r("p",[e._v("I can use that exact same data to create a few "),r("strong",[e._v("goemetry buffers")]),e._v(" containing the rasterised position, normal, albedo, and specular data for the whole scene. Using the same lighting calculation with the new "),r("strong",[e._v("G-buffer")]),e._v(" input data, I can shade the whole scene, render it to a texture, and display it on a quad. That's the goal. To get there from where I currently am, I'll need a "),r("strong",[e._v("critical path")]),e._v(".")]),e._v(" "),r("h2",{attrs:{id:"critical-path"}},[r("a",{staticClass:"header-anchor",attrs:{href:"#critical-path"}},[e._v("#")]),e._v(" Critical Path")]),e._v(" "),r("ol",[r("li",[r("strong",[e._v("Coordinate System")]),e._v(" "),r("ul",[r("li",[r("strong",[e._v("Entity")]),e._v(" object to encapsulate a "),r("strong",[e._v("Transform")]),e._v(" component and "),r("strong",[e._v("Renderer")]),e._v(" data.")]),e._v(" "),r("li",[r("strong",[e._v("Camera")]),e._v(" component to handle view projection transforms, and move around the scene.")]),e._v(" "),r("li",[e._v("The "),r("a",{attrs:{href:"https://crates.io/crates/nalgebra-glm",target:"_blank",rel:"noopener noreferrer"}},[e._v("nalgebra-glm"),r("OutboundLink")],1),e._v(" crate has all the required math funcionality.")])])]),e._v(" "),r("li",[r("strong",[e._v("Forward Lighting")]),e._v(" "),r("ul",[r("li",[r("strong",[e._v("Light")]),e._v(" component to with color, strength, and type (directional or spotlight).")]),e._v(" "),r("li",[r("strong",[e._v("Material")]),e._v(" component to encapsulate ambient, diffues, specular, and shininess values.")]),e._v(" "),r("li",[e._v("Create a "),r("strong",[e._v("GLSL function")]),e._v(" to calculate lighting from a "),r("strong",[e._v("material's")]),e._v(" values and all the "),r("strong",[e._v("light")]),e._v(" source values.")])])]),e._v(" "),r("li",[r("strong",[e._v("Geometry")]),e._v(" "),r("ul",[r("li",[e._v("I'm going to skip model loading for now, and just use some pre-defined cube vertex data.")]),e._v(" "),r("li",[e._v("Maybe I'll render the light sources as little spheres.")])])]),e._v(" "),r("li",[r("strong",[e._v("Deferred Shading")]),e._v(" "),r("ul",[r("li",[e._v("I will need to learn how to use "),r("strong",[e._v("framebuffers")]),e._v(".")]),e._v(" "),r("li",[e._v("I might need to do something with the "),r("strong",[e._v("Depth buffer")]),e._v(".")]),e._v(" "),r("li",[e._v("I will need to learn about "),r("strong",[e._v("Multiple Render Targets")]),e._v(" (MRT).")]),e._v(" "),r("li",[e._v("Update the lighting calculation to take input from the "),r("strong",[e._v("G-buffer")]),e._v(" framebuffer.")])])]),e._v(" "),r("li",[r("strong",[e._v("Profit?")]),e._v(" "),r("ul",[r("li",[e._v("Well, no, but, minimum viable product? Yes.")])])])])],1)}),[],!1,null,null,null);t.default=n.exports}}]);