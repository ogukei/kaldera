#version 460
#extension GL_EXT_ray_tracing : enable
#extension GL_EXT_nonuniform_qualifier : enable
#extension GL_EXT_scalar_block_layout : enable

// https://github.com/nvpro-samples/vk_raytracing_tutorial_KHR/blob/master/ray_tracing__simple/shaders/raytrace.rchit
// https://github.com/SaschaWillems/Vulkan-Samples/tree/fc55746e485fbaa1aa0ecafd388759e6c6d00bf5/samples/extensions/raytracing_basic

struct MeshPrimitiveDescription {
  uint vertexOffset;
  uint indexOffset;
  uint textureIndex;
  uint reserved;
};

layout(location = 0) rayPayloadInEXT vec3 hitValue;
layout(binding = 3) readonly buffer Vertices { float vertices[]; };
layout(binding = 4) readonly buffer Indices { uint indices[]; };
layout(binding = 5) readonly buffer Normals { float normals[]; };
layout(binding = 6) readonly buffer Descriptions { MeshPrimitiveDescription descriptions[]; };
layout(binding = 7) readonly buffer Texcoords { float texcoords[]; };
layout(binding = 8) uniform sampler2D textures[];

hitAttributeEXT vec3 attribs;

float lightDiffuse(vec3 lightPosition, vec3 position, vec3 normal) {
  // Vector toward the light
  vec3 lDir      = lightPosition - position;
  float lightDistance = length(lDir);
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
  const vec3 objectNormal = n0 * barycentrics.x + n1 * barycentrics.y + n2 * barycentrics.z;
  const vec3 worldNormal = normalize(vec3(objectNormal * gl_WorldToObjectEXT));
  // Texture
  const vec2 uv0 = texcoordAt(triangleIndex.x);
  const vec2 uv1 = texcoordAt(triangleIndex.y);
  const vec2 uv2 = texcoordAt(triangleIndex.z);
  const vec2 texcoord0 = uv0 * barycentrics.x + uv1 * barycentrics.y + uv2 * barycentrics.z;
  const vec3 textureDiffuse = texture(textures[nonuniformEXT(desc.textureIndex)], texcoord0).xyz;
  // Diffuse
  const vec3 light = vec3(lightDiffuse(vec3(0.0f, 5.0f, 0.0f), worldPosition, worldNormal));
  const vec3 finalColor = textureDiffuse * light;
  hitValue = finalColor;
}
