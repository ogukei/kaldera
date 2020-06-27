#version 460
#extension GL_EXT_ray_tracing : enable
#extension GL_EXT_nonuniform_qualifier : enable

// https://github.com/nvpro-samples/vk_raytracing_tutorial_KHR/blob/master/ray_tracing__simple/shaders/raytrace.rchit
// https://github.com/SaschaWillems/Vulkan-Samples/tree/fc55746e485fbaa1aa0ecafd388759e6c6d00bf5/samples/extensions/raytracing_basic


struct Vertex {
  vec4 position;
  vec4 normal;
};

layout(location = 0) rayPayloadInEXT vec3 hitValue;
layout(binding = 3) buffer Vertices { Vertex v[]; } vertices[];
layout(binding = 4) buffer Indices { uint i[]; } indices[];
hitAttributeEXT vec3 attribs;

void main() {
  const vec3 barycentrics = vec3(1.0f - attribs.x - attribs.y, attribs.x, attribs.y);
  // Object of this instance
  uint objId = 0;
  // Indices of the triangle
  ivec3 ind = ivec3(indices[nonuniformEXT(objId)].i[3 * gl_PrimitiveID + 0],   //
                    indices[nonuniformEXT(objId)].i[3 * gl_PrimitiveID + 1],   //
                    indices[nonuniformEXT(objId)].i[3 * gl_PrimitiveID + 2]);  //
  // Vertex of the triangle
  Vertex v0 = vertices[nonuniformEXT(objId)].v[ind.x];
  Vertex v1 = vertices[nonuniformEXT(objId)].v[ind.y];
  Vertex v2 = vertices[nonuniformEXT(objId)].v[ind.z];
  vec3 normal = v0.normal.xyz * barycentrics.x + v1.normal.xyz * barycentrics.y + v2.normal.xyz * barycentrics.z;
  hitValue = abs(normal);
}
