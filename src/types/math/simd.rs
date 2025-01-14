use crate::types::WIDEST_LANE;
use std::simd::Simd;

#[repr(C)]
pub struct Vector3Wide {
    pub x: Simd<f32, WIDEST_LANE>,
    pub y: Simd<f32, WIDEST_LANE>,
    pub z: Simd<f32, WIDEST_LANE>,
}

#[repr(C)]
pub struct QuaternionWide {
    pub x: Simd<f32, WIDEST_LANE>,
    pub y: Simd<f32, WIDEST_LANE>,
    pub z: Simd<f32, WIDEST_LANE>,
    pub w: Simd<f32, WIDEST_LANE>,
}

#[repr(C)]
pub struct RigidPoseWide {
    position: Vector3Wide,
    orientation: QuaternionWide,
}

#[repr(C)]
pub struct Symmetric3x3Wide {
    /// First row, first column of the matrix.
    pub xx: Simd<f32, WIDEST_LANE>,
    /// Second row, first column of the matrix.
    pub yx: Simd<f32, WIDEST_LANE>,
    /// Second row, second column of the matrix.
    pub yy: Simd<f32, WIDEST_LANE>,
    /// Third row, first column of the matrix.
    pub zx: Simd<f32, WIDEST_LANE>,
    /// Third row, second column of the matrix.
    pub zy: Simd<f32, WIDEST_LANE>,
    /// Third row, third column of the matrix.
    pub zz: Simd<f32, WIDEST_LANE>,
}

#[repr(C)]
pub struct BodyInertiaWide {
    inverse_tensor: Symmetric3x3Wide,
    inverse_mass: Simd<f32, WIDEST_LANE>,
}

#[repr(C)]
pub struct BodyVelocityWide {
    linear: Vector3Wide,
    angular: Vector3Wide,
}
