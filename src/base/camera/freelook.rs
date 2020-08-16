


use nalgebra_glm as glm;

use crate::vk::{Mat4};
use crate::cores::{InputEvent, InputKeyEvent};
use super::camera::Camera;

// uses right-handed coordinate system, that is the same as glTF 2.0. 
// @see https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#coordinate-system-and-units

pub struct FreeLookCamera {
    inv_view: glm::Mat4,
    inv_proj: glm::Mat4,
    quat_target: glm::Quat,
    quat_smooth: glm::Quat,
    rotation_x: f32,
    rotation_y: f32,
    position_target: glm::Vec3,
    position_smooth: glm::Vec3,
}

impl FreeLookCamera {
    pub fn new(width: f32, height: f32) -> Self {
        let perspective = glm::perspective_fov(glm::radians(&glm::vec1(60.0)).x, width, height, 0.1, 100.0);
        let inv_proj = glm::inverse(&perspective);
        let view: glm::Mat4 = glm::identity();
        let quat = orbital_quat(0.0, 0.0);
        let position = glm::vec3(0.0, 0.0, 0.0);
        Self {
            inv_view: glm::inverse(&view),
            inv_proj: inv_proj,
            quat_target: quat,
            quat_smooth: quat,
            rotation_x: 0.0,
            rotation_y: 0.0,
            position_target: position,
            position_smooth: position,
        }
    }

    fn rotate(&mut self, x: f32, y: f32, delta_time: f32) {
        let x = (x / 600.0) * glm::pi::<f32>();
        let y = (y / 600.0) * glm::pi::<f32>();
        self.rotation_x += x;
        self.rotation_y += y;
        self.quat_target = orbital_quat(self.rotation_x, self.rotation_y);
    }

    fn forward(&mut self, 
        InputKeyEvent { x, y, z, is_shift, is_control }: InputKeyEvent, 
        delta_time: f32
    ) {
        let shift_modifier = if is_shift { 2.0 } else { 1.0 };
        let control_modifier = if is_control { 0.25 } else { 1.0 };
        let speed = 1.5 * shift_modifier * control_modifier;
        let forward = glm::vec3(0.0, 0.0, -1.0);
        let right = glm::vec3(1.0, 0.0, 0.0);
        let up = glm::vec3(0.0, 1.0, 0.0);
        let qi = glm::quat_inverse(&self.quat_target);
        let forward = glm::quat_rotate_vec3(&qi, &forward) * y * delta_time * speed;
        let right = glm::quat_rotate_vec3(&qi, &right) * x * delta_time * speed;
        let upward = glm::quat_rotate_vec3(&qi, &up) * z * delta_time * speed;
        self.position_target += forward + right + upward;
    }
}

impl Camera for FreeLookCamera {
    fn view_inverse(&self) -> Mat4 {
        self.inv_view.into()
    }

    fn projection_inverse(&self) -> Mat4 {
        self.inv_proj.into()
    }

    fn apply(&mut self, input: InputEvent, delta_time: f32) {
        match input {
            InputEvent::MoveDelta(x, y) => self.rotate(x, y, delta_time),
            InputEvent::Key(event) => self.forward(event, delta_time),
        }
    }

    fn update(&mut self, delta_time: f32) {
        // TODO(ogukei): adjust speed by over time
        let a = 0.6;
        self.position_smooth = glm::lerp_vec(&self.position_smooth, &self.position_target, &glm::vec3(a, a, a));
        self.quat_smooth = glm::quat_slerp(&self.quat_smooth, &self.quat_target, a);
        let translate = self.position_smooth * -1.0;
        let view = glm::quat_to_mat4(&self.quat_smooth) * glm::translation(&translate);
        self.inv_view = glm::inverse(&view);
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
