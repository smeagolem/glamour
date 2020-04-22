#version 410 core

layout(location = 0) in vec3 in_position;

uniform mat4 u_view_projection;

void main() { gl_Position = u_view_projection * vec4(in_position, 1.0); }
