
// @see https://github.com/ogukei/filum-example/blob/master/ray-tracing/data/ray.comp

float schlick(float cosine, float ref) {
  float r0 = (1.0 - ref) / (1.0 + ref);
  r0 = r0 * r0;
  float q = 1.0 - cosine;
  return r0 + (1.0 - r0) * (q*q*q*q*q);
}

void lambertian(
  inout Random rng,
  const Hit hit,
  const Material mat, 
  out vec3 attenuation,
  out Ray scatter,
  out bool continues
) {
  attenuation = mat.albedo.xyz;
  scatter.origin = hit.position;
  scatter.direction = hit.normal + randomNextSphereUnit(rng);
  continues = true;
}

void metal(
  inout Random rng,
  const Ray ray,
  const Hit hit,
  const Material mat, 
  out vec3 attenuation,
  out Ray scatter,
  out bool continues
) {
  attenuation = mat.albedo.xyz;
  const vec3 reflected = reflect(ray.direction, hit.normal);
  scatter.origin = hit.position;
  scatter.direction = reflected + mat.albedo.w * randomNextSphereUnit(rng);
  continues = dot(scatter.direction, hit.normal) > 0;
}

void dielectric(
  inout Random rng,
  const Ray ray,
  const Hit hit,
  const Material mat, 
  out vec3 attenuation,
  out Ray scatter,
  out bool continues
) {
  vec3 outward;
  float niOverNt;
  attenuation = vec3(1, 1, 1);
  float ref = mat.albedo.w;
  float cosine;
  if (dot(ray.direction, hit.normal) > 0) {
    outward = -hit.normal;
    niOverNt = ref;
    cosine = ref * dot(ray.direction, hit.normal) / length(ray.direction);
  } else {
    outward = hit.normal;
    niOverNt = 1.0 / ref;
    cosine = -dot(ray.direction, hit.normal) / length(ray.direction);
  }
  float reflectProb;
  vec3 refracted = refract(ray.direction, outward, niOverNt);
  if (refracted != vec3(0)) {
    reflectProb = schlick(cosine, ref);
  } else {
    reflectProb = 1.0;
  }
  scatter.origin = hit.position;
  if (randomNext(rng) < reflectProb) {
    scatter.direction = reflect(ray.direction, hit.normal);
  } else {
    scatter.direction = refracted;
  }
  continues = true;
}

void scatter(
  inout Random rng,
  const Ray ray,
  const Hit hit,
  const Material mat, 
  out vec3 attenuation,
  out Ray scatter,
  out bool continues
) {
  uint model = mat.type.x;
  switch (model) {
  case 0: 
    lambertian(rng, hit, mat, attenuation, scatter, continues);
    break;
  case 1:
    metal(rng, ray, hit, mat, attenuation, scatter, continues);
    break;
  case 2:
    dielectric(rng, ray, hit, mat, attenuation, scatter, continues);
    break;
  }
}
