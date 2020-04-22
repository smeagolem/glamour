#version 410 core

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec2 in_tex_coords;

out vec2 tex_coords;

uniform mat4 u_view_projection;

void main() {
  gl_Position = u_view_projection * vec4(in_position, 1.0);
  tex_coords = in_tex_coords;
}
