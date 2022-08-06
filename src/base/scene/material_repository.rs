

use gltf;
use nalgebra_glm as glm;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::vk::Result;
use crate::vk::*;
use crate::ffi::vk::*;

use super::image_provider::ImageProvider;
use super::material::{
    SceneMaterialDescription,
    MaterialDescriptionsTextures,
    Material,
};

pub struct MaterialRepository {
    state: Mutex<MaterialDescriptionsTextures>,
}

impl MaterialRepository {
    pub fn new(state: MaterialDescriptionsTextures) -> Arc<Self> {
        let this = Self {
            state: Mutex::new(state),
        };
        Arc::new(this)
    }

    pub fn state<'a>(&'a self) -> MaterialRepositoryStateRef {
        let guard = self.state.lock().unwrap();
        MaterialRepositoryStateRef::new(guard)
    }
}

pub struct MaterialRepositoryStateRef<'a> {
    guard: MutexGuard<'a, MaterialDescriptionsTextures>,
}

impl<'a> MaterialRepositoryStateRef<'a> {
    fn new(guard: MutexGuard<'a, MaterialDescriptionsTextures>) -> Self {
        Self {
            guard,
        }
    }

    pub fn textures(&self) -> &Vec<Arc<Texture>> {
        &self.guard.textures
    }

    pub fn descriptions(&self) -> &Vec<SceneMaterialDescription> {
        &self.guard.descriptions
    }

    pub fn replace_material(
        &mut self,
        command_pool: &Arc<CommandPool>,
        image_provider: &ImageProvider,
        material: &Material,
        material_index: usize) 
    {
        self.guard.replace_material(command_pool, image_provider, material, material_index)
    }
}
