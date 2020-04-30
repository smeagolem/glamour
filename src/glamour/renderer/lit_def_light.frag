#version 410 core

in vec2 tex_coords;

uniform sampler2D u_tex_pos;
uniform sampler2D u_tex_norm;
uniform sampler2D u_tex_alb_spec;

uniform vec3 u_view_pos;

struct PointLight {
  vec3 position;
};
#define NR_POINT_LIGHTS 32
uniform PointLight u_point_lights[NR_POINT_LIGHTS];

out vec4 out_color;

vec3 calc_point_light(PointLight light, vec3 normal, vec3 frag_pos,
                      vec3 view_dir, float specular_strength);

void main() {
  vec3 frag_pos = texture(u_tex_pos, tex_coords).rgb;
  vec3 norm = texture(u_tex_norm, tex_coords).rgb;
  vec4 alb_spec = texture(u_tex_alb_spec, tex_coords);
  vec4 albedo = vec4(alb_spec.rgb, 1.0);
  float specular_strength = alb_spec.a;

  vec3 view_dir = normalize(u_view_pos - frag_pos);

  vec3 lighting = vec3(0.0, 0.0, 0.0);
  for (int i = 0; i < NR_POINT_LIGHTS; i++) {
    lighting += calc_point_light(u_point_lights[i], norm, frag_pos, view_dir,
                                 specular_strength);
  }

  out_color = vec4(lighting, 1.0) * albedo;
  //   out_color = vec4(frag_pos, 1.0);
  //   out_color = vec4(norm, 1.0);
  //   out_color = albedo;
  //   out_color = vec4(specular_strength, 0.0, 0.0, 1.0);
}

vec3 calc_point_light(PointLight light, vec3 norm, vec3 frag_pos, vec3 view_dir,
                      float specular_strength) {
  vec3 light_color = vec3(1.0, 1.0, 1.0);

  float ambient_strength = 0.05;
  vec3 ambient_light = ambient_strength * light_color;

  vec3 light_dir = normalize(light.position - frag_pos);

  float diffuse = max(dot(norm, light_dir), 0.0);
  vec3 diffuse_light = diffuse * light_color;

  float shininess = 256.0;
  vec3 halfway_dir = normalize(light_dir + view_dir);
  float specular = pow(max(dot(norm, halfway_dir), 0.0), shininess);
  vec3 specular_light = specular_strength * specular * light_color;

  float dist = length(light.position - frag_pos);
  float atten_constant = 1.0;
  float atten_linear = 0.14;
  float atten_quadratic = 0.07;
  float attenuation = 1.0 / (atten_constant + atten_linear * dist +
                             atten_quadratic * (dist * dist));

  return (ambient_light + diffuse_light + specular_light) * attenuation;
}
