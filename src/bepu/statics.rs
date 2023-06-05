use glam::{Vec3, Quat};

use super::{math::RigidPose, handles::TypedIndex, continuity::ContinuousDetection};



#[repr(C)]
pub struct StaticDescription {
    pose : RigidPose,
    shape: TypedIndex,
    continuity: ContinuousDetection
}

impl StaticDescription {
    pub fn new(pose: RigidPose, shape: TypedIndex, continuity: ContinuousDetection) -> Self {
        Self {
            pose,
            shape,
            continuity
        }
    }

    pub fn create(pos: Vec3, orientation: Quat, shape: TypedIndex) -> Self {
        StaticDescription {
            pose: RigidPose::new(
                orientation,
                pos,
            ),
            shape,
            continuity: ContinuousDetection::discrete(),
        }
    }
}