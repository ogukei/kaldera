
struct RayPayload {
  vec3 hitValue;
  Random random;
  Ray scatter;
  bool continues;
  bool hits;
  float hitT;
};
