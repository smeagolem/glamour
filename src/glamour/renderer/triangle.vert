#version 410 core

layout(location = 0) in vec3 in_pos;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_tex_coords;
// mat4 takes up 4 locations since size is limited to 4 bytes
layout(location = 3) in mat4 in_model_mat;
// mat3 takes up 3 locations since size is limited to 4 bytes
layout(location = 7) in mat3 in_norm_mat;
// layout(location = 10) in vec3 in_something;

out vec3 frag_pos;
out vec3 normal;
out vec2 tex_coords;

uniform mat4 u_view_projection;

void main() {
  vec4 model = in_model_mat * vec4(in_pos, 1.0);
  frag_pos = model.xyz;
  gl_Position = u_view_projection * model;
  //   normal = mat3(transpose(inverse(in_model_mat))) * in_normal;
  // normal = (in_norm_mat * vec4(in_normal, 1.0)).xyz;
  normal = in_norm_mat * in_normal;
  // normal = in_normal;
  tex_coords = in_tex_coords;
}
