

use crate::vk::{Mat4};
use crate::cores::InputEvent;

pub trait Camera {
    fn view_inverse(&self) -> Mat4;
    fn projection_inverse(&self) -> Mat4;
    fn apply(&mut self, inputs: InputEvent, delta_time: f32);
    fn update(&mut self, delta_time: f32);
}
