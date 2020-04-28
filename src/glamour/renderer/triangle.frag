#version 410 core

in vec3 frag_pos;
in vec3 normal;
in vec2 tex_coords;

uniform vec4 u_color;
uniform vec3 u_view_pos;
uniform sampler2D u_tex;

struct PointLight {
  vec3 position;
};
#define NR_POINT_LIGHTS 32
uniform PointLight u_point_lights[NR_POINT_LIGHTS];

out vec4 out_color;

vec3 calc_point_light(PointLight light, vec3 normal, vec3 frag_pos,
                      vec3 view_dir);

void main() {
  vec3 norm = normalize(normal);
  vec3 view_dir = normalize(u_view_pos - frag_pos);

  vec3 lighting = vec3(0.0, 0.0, 0.0);
  for (int i = 0; i < NR_POINT_LIGHTS; i++) {
    lighting += calc_point_light(u_point_lights[i], norm, frag_pos, view_dir);
  }

  vec4 albedo = texture(u_tex, tex_coords) * u_color;

  out_color = vec4(lighting, 1.0) * albedo;
}

vec3 calc_point_light(PointLight light, vec3 norm, vec3 frag_pos,
                      vec3 view_dir) {
  vec3 light_color = vec3(1.0, 1.0, 1.0);

  float ambient_strength = 0.05;
  vec3 ambient_light = ambient_strength * light_color;

  vec3 light_dir = normalize(light.position - frag_pos);

  float diffuse = max(dot(norm, light_dir), 0.0);
  vec3 diffuse_light = diffuse * light_color;

  float specular_strength = 0.5;
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
