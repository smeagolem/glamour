#version 410 core

layout(location = 0) in vec3 in_pos;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_tex_coords;

out vec3 frag_pos;
out vec3 normal;
out vec2 tex_coords;

uniform mat4 u_view_projection;

void main() {
  frag_pos = in_pos;
  gl_Position = u_view_projection * vec4(in_pos, 1.0);
  normal = in_normal;
  tex_coords = in_tex_coords;
}
