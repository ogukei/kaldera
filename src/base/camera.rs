

use nalgebra_glm as glm;

use crate::vk::{Mat4};
use crate::cores::InputEvent;

pub struct OrbitalCamera {
    view: glm::Mat4x4,
    inv_proj: glm::Mat4x4,
    rotation_x: f32,
    rotation_y: f32,
    distance: f32,
}

impl OrbitalCamera {
    pub fn new() -> Self {
        let perspective = glm::perspective_fov(glm::radians(&glm::vec1(60.0)).x, 1.0, 1.0, 0.1, 100.0);
        let inv_proj = glm::inverse(&perspective);
        let rotation_x: f32 = -0.24;
        let rotation_y: f32 = 0.36;
        let distance: f32 = 3.0;
        Self {
            view: orbital(distance, rotation_x, rotation_y),
            inv_proj: inv_proj,
            rotation_x,
            rotation_y,
            distance,
        }
    }

    pub fn view_inverse(&self) -> Mat4 {
        // view
        let view = self.view;
        let inv_view = glm::inverse(&view);
        inv_view.into()
    }

    pub fn projection_inverse(&self) -> Mat4 {
        self.inv_proj.into()
    }

    pub fn apply(&mut self, input: InputEvent) {
        match input {
            InputEvent::MoveDelta(x, y) => self.rotate(x, y),
        }
    }

    fn rotate(&mut self, x: f32, y: f32) {
        self.rotation_x += (x / 400.0) * glm::pi::<f32>();
        self.rotation_y += (y / 400.0) * glm::pi::<f32>();
        self.view = orbital(self.distance, self.rotation_x, self.rotation_y);
    }
}

fn orbital(radius: f32, rotation_x: f32, rotation_y: f32) -> glm::Mat4 {
    // rotation
    let right = glm::vec3(1.0, 0.0, 0.0);
    let upward = glm::vec3(0.0, 1.0, 0.0);
    let quat = glm::quat_rotate(&glm::quat_identity(), rotation_y, &right);
    let quat = glm::quat_rotate(&quat, rotation_x, &upward);
    let rotation = glm::quat_to_mat4(&quat);
    // translation
    let dir = glm::vec3(0.0, 0.0, -radius);
    let translation = glm::translate(&glm::identity(), &dir);
    let center = glm::translate(&glm::identity(), &glm::vec3(0.0, 0.0, 0.0));
    translation * rotation * center
}
