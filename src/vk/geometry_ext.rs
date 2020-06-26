
use nalgebra_glm as glm;

use super::geometry::*;

impl From<glm::Mat4x4> for Mat4 {
    fn from(v: glm::Mat4x4) -> Self {
        Mat4 {
            columns: [
                Vec4 {
                    x: v.m11, 
                    y: v.m21, 
                    z: v.m31, 
                    w: v.m41,
                },
                Vec4 {
                    x: v.m12, 
                    y: v.m22, 
                    z: v.m32, 
                    w: v.m42,
                },
                Vec4 {
                    x: v.m13, 
                    y: v.m23, 
                    z: v.m33, 
                    w: v.m43,
                },
                Vec4 {
                    x: v.m14, 
                    y: v.m24, 
                    z: v.m34, 
                    w: v.m44,
                },
            ],
        }
    }
}

pub struct Camera {
    inv_view: Mat4,
    inv_proj: Mat4,
}

impl Camera {
    pub fn new() -> Self {
        let look_at = glm::look_at(&glm::vec3(0.0, 0.0, 10.0), &glm::vec3(0.0, 0.0, 0.0), &glm::vec3(0.0, 1.0, 0.0));
        let perspective = glm::perspective_fov(glm::radians(&glm::vec1(60.0)).x, 1.0, 1.0, 0.1, 100.0);
        let inv_view = glm::inverse(&look_at);
        let inv_proj = glm::inverse(&perspective);
        Self {
            inv_view: inv_view.into(),
            inv_proj: inv_proj.into(),
        }
    }

    pub fn inv_view(&self) -> Mat4 {
        self.inv_view
    }

    pub fn inv_proj(&self) -> Mat4 {
        self.inv_proj
    }
}
