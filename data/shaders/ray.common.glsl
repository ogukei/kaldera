
struct Random {
  uint seed;
};

struct Material {
  vec4 albedo;
  uvec4 type;
};

struct Ray {
  vec3 origin;
  vec3 direction;
};

struct Hit {
  vec3 position;
  vec3 normal;
};

#define M_PI 3.14159265358979323846264338327950288

#define G_LIGHT_POS (vec3(9.0, 20.0, 8.0))
