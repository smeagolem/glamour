#version 410 core

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec2 in_tex_coords;

out vec2 tex_coords;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

void main() {
  gl_Position = u_projection * u_view * u_model * vec4(in_position, 1.0);
  tex_coords = in_tex_coords;
}
