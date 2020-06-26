

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mat4 {
    pub columns: [Vec4; 4],
}

impl Default for Mat4 {
    fn default() -> Self {
        Mat4 {
            columns: [
                Vec4 {
                    x: 1.0, 
                    y: 0.0, 
                    z: 0.0, 
                    w: 0.0,
                },
                Vec4 {
                    x: 0.0, 
                    y: 1.0, 
                    z: 0.0, 
                    w: 0.0,
                },
                Vec4 {
                    x: 0.0, 
                    y: 0.0, 
                    z: 1.0, 
                    w: 0.0,
                },
                Vec4 {
                    x: 0.0, 
                    y: 0.0, 
                    z: 0.0, 
                    w: 1.0,
                },
            ],
        }
    }
}
