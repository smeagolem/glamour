#version 410 core

// https : // github.com/OsnaCS/plantex/wiki/Shader-code-conventions

// TODO: https://www.khronos.org/opengl/wiki/Interface_Block_(GLSL)
// need different gl call to set uniforms in blocks
// uniform Uniforms { vec4 color; }
// u;

in vec2 tex_coords;

uniform vec4 u_color;
uniform sampler2D u_tex;

out vec4 out_color;

void main() { out_color = texture(u_tex, tex_coords) * u_color; }
