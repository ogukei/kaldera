
use nalgebra_glm as glm;

use std::sync::Arc;
use std::sync::Mutex;

use super::image_provider::ImageProvider;
use crate::vk::*;
use crate::ffi::vk::*;

use super::material::*;
use super::asset::*;
use super::mesh::*;
use super::buffer::*;
use super::material_repository::*;

pub struct SceneBuilder {
    asset: Arc<SceneAsset>,
}

impl SceneBuilder {
    pub fn new(asset: &Arc<SceneAsset>) -> Self {
        Self {
            asset: Arc::clone(asset),
        }
    }

    pub fn build(self, command_pool: &Arc<CommandPool>) -> Scene {
        let asset = &self.asset;
        log_debug!("start scene builder");
        let table = MeshTable::new(asset);
        log_debug!("iterating nodes");
        let scene = asset.document()
            .scenes()
            .nth(0)
            .unwrap();
        let nodes: Vec<_> = scene.nodes()
            .map(|v| FlattenNode::flatten(v))
            .flatten()
            .map(|v| MeshNode::new(v, &table))
            .filter_map(|v| v)
            .flatten()
            .collect();
        log_debug!("iterating nodes complete");
        log_debug!("iterating materials");
        let materials: Vec<_> = asset.document().materials()
            .into_iter()
            .map(|v| Material::new(v))
            .collect();
        log_debug!("iterating materials complete");
        Scene::new(asset, &table, &nodes, &materials, command_pool)
    }
}

#[allow(dead_code)]
pub struct Scene {
    asset: Arc<SceneAsset>,
    command_pool: Arc<CommandPool>,
    primitives: Vec<Arc<SceneMeshPrimitive>>,
    staging_buffers: Arc<SceneStagingBuffers>,
    top_level_acceleration_structure: Arc<TopLevelAccelerationStructure>,
    material_repository: Arc<MaterialRepository>,
    state: Mutex<SceneState>,
}

impl Scene {
    fn new(asset: &Arc<SceneAsset>, table: &MeshTable, nodes: &[MeshNode], materials: &[Material], command_pool: &Arc<CommandPool>) -> Self {
        let primitives = table.mesh_primitives();
        log_debug!("creating material images");
        let image_provider = ImageProvider::new(asset);
        let descriptions_textures = MaterialDescriptionsTextures::new(
            materials,
            &image_provider,
            command_pool);
        let material_repository = MaterialRepository::new(descriptions_textures);
        log_debug!("creating material images complete");
        log_debug!("creating staging buffers");
        let staging_buffers = SceneStagingBuffers::new(command_pool, primitives, material_repository.state().descriptions());
        log_debug!("creating staging buffers complete");
        log_debug!("building blas");
        let scene_mesh_primitive_geometries: Vec<_> = table.mesh_primitives().iter()
            .map(|v| SceneMeshPrimitiveGeometry::new(v, &staging_buffers, command_pool))
            .collect();
        let geometries: Vec<_> = scene_mesh_primitive_geometries.iter()
            .map(|v| v.structure_geometry())
            .map(Arc::clone)
            .collect();
        let queries: Vec<_> = geometries.into_iter()
            .map(|v| BottomLevelAccelerationStructureBuildQuery::new(vec![v]))
            .collect();
        let builder = BottomLevelAccelerationStructuresBuilder::new(command_pool, queries);
        let structures = builder.build();
        let scene_mesh_primitives: Vec<_> = structures.into_iter()
            .zip(scene_mesh_primitive_geometries.into_iter())
            .map(|(structure, geometry)| SceneMeshPrimitive::new(geometry, structure))
            .collect();
        log_debug!("building blas complete");
        log_debug!("building tlas");
        let node_scale: f32 = 1.0;
        let node_scale = glm::scaling(&glm::vec3(node_scale, node_scale, node_scale));
        let node_translate = glm::translation(&glm::vec3(0.0, 0.0, 0.0));
        let node_instances = nodes.into_iter()
            .map(|node| {
                let index = node.primitive().index();
                let mesh_primitive = scene_mesh_primitives.get(index).unwrap();
                let transform = node_translate * node_scale * node.transform();
                let transform = VkTransformMatrixKHR {
                    matrix: [
                        [transform.m11, transform.m12, transform.m13, transform.m14],
                        [transform.m21, transform.m22, transform.m23, transform.m24],
                        [transform.m31, transform.m32, transform.m33, transform.m34],
                    ]
                };
                let instance = TopLevelAccelerationStructureInstance::new(
                    index as u32,
                    transform,
                    0,
                    mesh_primitive.bottom_level_acceleration_structure(),
                ).unwrap();
                instance
            });
        let instances = node_instances
            .collect();
        let top_level_acceleration_structure = TopLevelAccelerationStructure::new(command_pool, instances)
            .unwrap();
        log_debug!("building tlas complete");
        log_debug!("scene building complete");
        Self {
            asset: Arc::clone(asset),
            command_pool: Arc::clone(command_pool),
            primitives: scene_mesh_primitives,
            staging_buffers,
            top_level_acceleration_structure,
            material_repository,
            state: Mutex::new(SceneState::new())
        }
    }

    pub fn top_level_acceleration_structure(&self) -> &Arc<TopLevelAccelerationStructure> {
        &self.top_level_acceleration_structure
    }

    pub fn index_staging_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.staging_buffers.index_buffer()
    }

    pub fn vertex_staging_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.staging_buffers.vertex_buffer()
    }

    pub fn normal_staging_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.staging_buffers.normal_buffer()
    }

    pub fn description_staging_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.staging_buffers.description_buffer()
    }

    pub fn texcoord_staging_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.staging_buffers.texcoord_buffer()
    }

    pub fn material_description_staging_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.staging_buffers.material_description_buffer()
    }

    pub fn tangent_staging_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.staging_buffers.tangent_buffer()
    }

    pub fn color_staging_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.staging_buffers.color_buffer()
    }

    pub fn textures(&self) -> Vec<Arc<Texture>> {
        // copying Vec for some convenience
        let state = self.material_repository.state();
        let textures = state.textures();
        textures.iter()
            .map(|v| Arc::clone(v))
            .collect()
    }

    pub fn update(&self, delta_time: f32, descriptor_sets: &Arc<RayTracingDescriptorSets>) {
        self.state.lock().unwrap().update(self, delta_time, descriptor_sets);
    }
}

struct SceneState {

}

impl SceneState {
    fn new() -> Self { 
        Self {
        } 
    }

    fn update(&mut self, _scene: &Scene, _delta_time: f32, _descriptor_sets: &Arc<RayTracingDescriptorSets>) {

    }
}
