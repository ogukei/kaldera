#version 460
#extension GL_EXT_ray_tracing : enable
#extension GL_EXT_nonuniform_qualifier : enable
#extension GL_EXT_scalar_block_layout : enable
#extension GL_GOOGLE_include_directive : enable

#include "ray.common.glsl"
#include "ray.common.payload.glsl"

// https://github.com/nvpro-samples/vk_raytracing_tutorial_KHR/blob/master/ray_tracing__simple/shaders/raytrace.rchit
// https://github.com/SaschaWillems/Vulkan-Samples/tree/fc55746e485fbaa1aa0ecafd388759e6c6d00bf5/samples/extensions/raytracing_basic

struct MeshPrimitiveDescription {
  uint vertexOffset;
  uint indexOffset;
  uint materialIndex;
  uint reserved;
};

struct MaterialDescription {
  int colorTextureIndex;
  int normalTextureIndex;
};

layout(location = 0) rayPayloadInEXT RayPayload payload;
layout(binding = 3) readonly buffer Vertices { float vertices[]; };
layout(binding = 4) readonly buffer Indices { uint indices[]; };
layout(binding = 5) readonly buffer Normals { float normals[]; };
layout(binding = 6) readonly buffer Descriptions { MeshPrimitiveDescription descriptions[]; };
layout(binding = 7) readonly buffer Texcoords { float texcoords[]; };
layout(binding = 8) uniform sampler2D textures[];
layout(binding = 11) readonly buffer Materials { MaterialDescription materials[]; };
layout(binding = 12) readonly buffer Tangents { float tangents[]; };

hitAttributeEXT vec3 attribs;

float lightDiffuse(vec3 lightPosition, vec3 position, vec3 normal) {
  // Vector toward the light
  vec3 lDir      = lightPosition - position;
  vec3  L = normalize(lDir);
  float dotNL = max(dot(normal, L), 0.2);
  return dotNL;
}

vec3 vertexAt(uint index) {
  return vec3(vertices[nonuniformEXT(3 * index + 0)],
              vertices[nonuniformEXT(3 * index + 1)],
              vertices[nonuniformEXT(3 * index + 2)]);
}

vec3 normalAt(uint index) {
  return vec3(normals[nonuniformEXT(3 * index + 0)],
              normals[nonuniformEXT(3 * index + 1)],
              normals[nonuniformEXT(3 * index + 2)]);
}

vec2 texcoordAt(uint index) {
  return vec2(texcoords[nonuniformEXT(2 * index + 0)],
              texcoords[nonuniformEXT(2 * index + 1)]);
}

vec4 tangentAt(uint index) {
  return vec4(tangents[nonuniformEXT(4 * index + 0)],
              tangents[nonuniformEXT(4 * index + 1)],
              tangents[nonuniformEXT(4 * index + 2)],
              tangents[nonuniformEXT(4 * index + 3)]);
}

void main() {
  const vec3 barycentrics = vec3(1.0f - attribs.x - attribs.y, attribs.x, attribs.y);
  const MeshPrimitiveDescription desc = descriptions[gl_InstanceCustomIndexEXT];
  const uint indexOffset  = desc.indexOffset + (3 * gl_PrimitiveID);
  const uint vertexOffset = desc.vertexOffset;
  const ivec3 triangleIndex = ivec3(indices[nonuniformEXT(indexOffset + 0)],
                              indices[nonuniformEXT(indexOffset + 1)],
                              indices[nonuniformEXT(indexOffset + 2)]) + ivec3(vertexOffset);
  // Vertex of the triangle
  const vec3 v0 = vertexAt(triangleIndex.x);
  const vec3 v1 = vertexAt(triangleIndex.y);
  const vec3 v2 = vertexAt(triangleIndex.z);
  const vec3 objectPosition = v0 * barycentrics.x + v1 * barycentrics.y + v2 * barycentrics.z;
  const vec3 worldPosition = vec3(gl_ObjectToWorldEXT * vec4(objectPosition, 1.0));
  // Normal of the triangle
  const vec3 n0 = normalAt(triangleIndex.x);
  const vec3 n1 = normalAt(triangleIndex.y);
  const vec3 n2 = normalAt(triangleIndex.z);
  const vec3 objectNormal = normalize(n0 * barycentrics.x + n1 * barycentrics.y + n2 * barycentrics.z);
  vec3 worldNormal = normalize(vec3(objectNormal * gl_WorldToObjectEXT));
  const vec3 objectGeometricNormal = normalize(cross(v1 - v0, v2 - v0));
  const vec3 worldGeometricNormal = normalize(vec3(objectGeometricNormal * gl_WorldToObjectEXT));
  // Texture
  const vec2 uv0 = texcoordAt(triangleIndex.x);
  const vec2 uv1 = texcoordAt(triangleIndex.y);
  const vec2 uv2 = texcoordAt(triangleIndex.z);
  const vec2 texcoord0 = uv0 * barycentrics.x + uv1 * barycentrics.y + uv2 * barycentrics.z;
  const MaterialDescription material = materials[nonuniformEXT(desc.materialIndex)];
  const vec3 textureDiffuse = texture(textures[nonuniformEXT(material.colorTextureIndex)], texcoord0).xyz;
  // Normal Mapping
  if (material.normalTextureIndex >= 0) {
    const vec4 t0 = tangentAt(triangleIndex.x);
    const vec4 t1 = tangentAt(triangleIndex.y);
    const vec4 t2 = tangentAt(triangleIndex.z);
    const vec4 tangent = t0 * barycentrics.x + t1 * barycentrics.y + t2 * barycentrics.z;
    const vec3 bitangent = normalize(cross(objectNormal, tangent.xyz)) * tangent.w;
    const vec3 textureNormal = texture(textures[nonuniformEXT(material.normalTextureIndex)], texcoord0).xyz;
    const vec3 tangentNormal = (textureNormal * 2.0) - 1.0;
    const mat3 TBN = mat3(tangent.xyz, bitangent, objectNormal);
    const vec3 objectPNormal = normalize(TBN * tangentNormal);
    const vec3 worldPnormal = normalize(vec3(objectPNormal * gl_WorldToObjectEXT));
    worldNormal = worldPnormal;
  }
  // Diffuse
  const vec3 light = vec3(lightDiffuse(vec3(0.0f, 5.0f, 0.0f), worldPosition, worldNormal));
  const vec3 finalColor = textureDiffuse * light;
  payload.hitValue = finalColor;
}
