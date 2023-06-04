use glam::Vec3;

#[repr(C)]
pub struct BodyVelocity {
    linear: Vec3,
    pad1: i32,
    angular: Vec3,
    pad2: i32,
}