

use nalgebra_glm as glm;

use crate::vk::{Mat4};
use crate::cores::InputEvent;

pub struct OrbitalCamera {
    quat: glm::Quat,
    quat_target: glm::Quat,
    inv_view: glm::Mat4,
    inv_proj: glm::Mat4,
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
        let distance: f32 = 5.0;
        let quat = orbital_quat(rotation_x, rotation_y);
        Self {
            quat: quat,
            quat_target: quat,
            inv_view: glm::inverse(&view(&quat, distance)),
            inv_proj: inv_proj,
            rotation_x,
            rotation_y,
            distance,
        }
    }

    pub fn view_inverse(&self) -> Mat4 {
        self.inv_view.into()
    }

    pub fn projection_inverse(&self) -> Mat4 {
        self.inv_proj.into()
    }

    pub fn apply(&mut self, input: InputEvent) {
        match input {
            InputEvent::MoveDelta(x, y) => self.rotate(x, y),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // TODO(ogukei): adjust speed by over time
        self.quat = glm::quat_slerp(&self.quat, &self.quat_target, 0.5);
        let view = view(&self.quat, self.distance);
        self.inv_view = glm::inverse(&view);
    }

    fn rotate(&mut self, x: f32, y: f32) {
        self.rotation_x += (x / 400.0) * glm::pi::<f32>();
        self.rotation_y += (y / 400.0) * glm::pi::<f32>();
        self.quat_target = orbital_quat(self.rotation_x, self.rotation_y);
    }
}

fn orbital_quat(rotation_x: f32, rotation_y: f32) -> glm::Quat {
    // rotation
    let right = glm::vec3(1.0, 0.0, 0.0);
    let upward = glm::vec3(0.0, 1.0, 0.0);
    let quat = glm::quat_rotate(&glm::quat_identity(), rotation_y, &right);
    let quat = glm::quat_rotate(&quat, rotation_x, &upward);
    quat
}

fn view(quat: &glm::Quat, radius: f32) -> glm::Mat4 {
    let dir = glm::vec3(0.0, 0.0, -radius);
    let translation = glm::translate(&glm::identity(), &dir);
    let center = glm::translate(&glm::identity(), &glm::vec3(0.0, -2.0, 0.0));
    let rotation = glm::quat_to_mat4(&quat);
    translation * rotation * center
}
