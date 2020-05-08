#version 410 core

layout(location = 0) out vec3 out_pos;
layout(location = 1) out vec3 out_norm;
layout(location = 2) out vec4 out_alb_spec;

in vec3 frag_pos;
in vec3 normal;
in vec2 tex_coords;

uniform vec4 u_color;
uniform sampler2D u_tex;

void main() {
  out_pos = frag_pos;
  out_norm = normalize(normal);
  out_alb_spec.rgb = (texture(u_tex, tex_coords) * u_color).rgb;
  // specular is hard coded atm
  out_alb_spec.a = 0.5;

  //   out_pos = out_alb_spec.rgb;
}
