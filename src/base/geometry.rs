
use nalgebra_glm as glm;

use crate::vk::{Mat4, Vec4};

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
