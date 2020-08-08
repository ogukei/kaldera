#version 460
#extension GL_EXT_ray_tracing : require
#extension GL_EXT_nonuniform_qualifier : enable
#extension GL_EXT_scalar_block_layout : enable
#extension GL_GOOGLE_include_directive : enable

#include "ray.common.glsl"
#include "ray.common.payload.glsl"
#include "ray.common.random.glsl"
#include "ray.common.scatter.glsl"

hitAttributeEXT vec3 attribs;

layout(location = 0) rayPayloadInEXT RayPayload payload;
layout(binding = 9) readonly buffer Spheres { vec4 spheres[]; };
layout(binding = 10) readonly buffer SphereMaterials { Material materials[]; };

void main() {
  const vec4 sphere = spheres[gl_PrimitiveID];
  const vec3 center = sphere.xyz;
  const vec3 worldPosition = gl_WorldRayOriginEXT + gl_WorldRayDirectionEXT * gl_HitTEXT;
  const vec3 worldNormal = normalize(worldPosition - center);

  const Hit hit = Hit(worldPosition, worldNormal);
  const Ray ray = Ray(gl_WorldRayOriginEXT, gl_WorldRayDirectionEXT);
  const Material mat = materials[gl_PrimitiveID];

  scatter(payload.random, ray, hit, mat, payload.hitValue, payload.scatter, payload.continues);
}
