
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
