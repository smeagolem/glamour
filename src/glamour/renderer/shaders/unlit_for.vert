#version 410 core

layout(location = 0) in vec3 in_pos;
layout(location = 3) in mat4 in_model_mat;

uniform mat4 u_view_projection;

void main() {
  gl_Position = u_view_projection * in_model_mat * vec4(in_pos, 1.0);
}
