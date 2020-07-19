#version 460
#extension GL_EXT_ray_tracing : enable
#extension GL_GOOGLE_include_directive : enable

#include "ray.common.glsl"
#include "ray.common.payload.glsl"

layout(location = 0) rayPayloadInEXT RayPayload payload;

void main() {
  // applies sky color (Ray Tracing in One Weekend, 4.2)
  const vec3 direction = normalize(gl_WorldRayDirectionEXT);
  const float t = 0.5 * (direction.y + 1.0);
  const vec3 skyColor = vec3(0.5, 0.7, 1.0);
  const vec3 bottomColor = vec3(1.0);
  const vec3 diffuseColor = mix(bottomColor, skyColor, t);
  payload.hitValue = diffuseColor;
}
