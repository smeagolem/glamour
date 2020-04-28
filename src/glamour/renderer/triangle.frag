#version 410 core

// https : // github.com/OsnaCS/plantex/wiki/Shader-code-conventions

// TODO: https://www.khronos.org/opengl/wiki/Interface_Block_(GLSL)
// need different gl call to set uniforms in blocks
// uniform Uniforms { vec4 color; }
// u;

in vec3 frag_pos;
in vec3 normal;
in vec2 tex_coords;

uniform vec4 u_color;
uniform vec3 u_light_pos;
uniform vec3 u_view_pos;
uniform sampler2D u_tex;

out vec4 out_color;

void main() {
  vec3 light_color = vec3(1.0, 1.0, 1.0);

  // attenuation
  float atten_constant = 1.0;
  float atten_linear = 0.045;
  float atten_quadratic = 0.0075;

  float ambient_strength = 0.1;
  vec3 ambient_light = ambient_strength * light_color;

  vec3 light_dir = normalize(u_light_pos - frag_pos);

  vec3 norm = normalize(normal);
  float diffuse = max(dot(norm, light_dir), 0.0);
  vec3 diffuse_light = diffuse * light_color;

  float specular_strength = 0.5;
  vec3 view_dir = normalize(u_view_pos - frag_pos);
  vec3 halfway_dir = normalize(light_dir + view_dir);
  float specular = pow(max(dot(norm, halfway_dir), 0.0), 256);
  vec3 specular_light = specular_strength * specular * light_color;

  float dist = length(u_light_pos - frag_pos);
  float attenuation = 1.0 / (atten_constant + atten_linear * dist +
                             atten_quadratic * (dist * dist));

  vec4 albedo = texture(u_tex, tex_coords) * u_color;

  out_color = vec4(ambient_light + diffuse_light + specular_light, 1.0) *
              attenuation * albedo;
}
