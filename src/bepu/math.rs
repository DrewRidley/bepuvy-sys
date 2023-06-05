use glam::{Quat, Vec3};



#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct RigidPose {
    pub orientation: Quat,
    pub position: Vec3,
    pad: i32
}

impl RigidPose {
    pub fn new(orientation: Quat, position: Vec3) -> Self {
        Self {
            orientation,
            position,
            pad: 0
        }
    }
    
    pub fn from_position(position: Vec3) -> Self {
        Self {
            orientation: Quat::IDENTITY,
            position,
            pad: 0
        }
    }
}



#[derive(Debug, Clone)]
#[repr(C, align(4))]
pub struct Symmetric3x3 {
    pub xx: f32,
    pub yx: f32,
    pub yy: f32,
    pub zx: f32,
    pub zy: f32,
    pub zz: f32,
}

impl Symmetric3x3 {
    pub fn uniform(value: f32) -> Self {
        Self {
            xx: value,
            yx: value,
            yy: value,
            zx: value,
            zy: value,
            zz: value,
        }
    }
}