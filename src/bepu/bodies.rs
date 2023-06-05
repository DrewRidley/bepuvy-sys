use glam::Vec3;

use super::{math::{RigidPose, Symmetric3x3}, continuity::ContinuousDetection, handles::TypedIndex};

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct BodyVelocity {
    pub linear: Vec3,
    pad1: i32,
    pub angular: Vec3,
    pad2: i32,
}

impl BodyVelocity {
    pub fn new(linear: Vec3, angular: Vec3) -> Self {
        Self {
            linear,
            pad1: 0,
            angular,
            pad2: 0,
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct MotionState {
    pose: RigidPose,
    vel: BodyVelocity
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct BodyInertia 
{
    inverse_tensor: Symmetric3x3,
    inverse_mass: f32,
    pad: u32
}

impl BodyInertia {
    pub fn from_symmetric(inverse_tensor: Symmetric3x3, inverse_mass: f32) -> Self {
        Self {
            inverse_tensor,
            inverse_mass,
            pad: 0
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct CollidableDescription {
    pub shape: TypedIndex,
    pub continuity: ContinuousDetection,
    pub min_spec_margin: f32,
    pub max_spec_margin: f32,
}

impl CollidableDescription {
    pub fn new(shape: TypedIndex)  -> Self{
        Self {
            shape,
            continuity: ContinuousDetection::discrete(),
            min_spec_margin: 0.0,
            max_spec_margin: std::f32::MAX,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct 
BodyDescription {
    pub pose: RigidPose,
    pub vel: BodyVelocity,
    pub local_inertia: BodyInertia,
    pub collidable: CollidableDescription,
    pub activity: BodyActivityDescription,
}

#[derive(Clone)]
#[repr(C)]
pub struct BodyActivityDescription {
    pub sleep_thres: f32,
    pub min_timesteps_to_sleep: u8
}

impl BodyActivityDescription {
    pub fn new(sleep_thres: f32) -> Self {
        Self {
            sleep_thres,
            min_timesteps_to_sleep: 32
        }
    }
}

impl BodyDescription {
    pub fn new(
        pose: RigidPose,
        vel: BodyVelocity,
        local_inertia: BodyInertia,
        collidable: CollidableDescription,
        activity: BodyActivityDescription,
    ) -> Self {
        Self {
            pose,
            vel,
            local_inertia,
            collidable,
            activity,
        }
    }

    pub fn new_dynamic(pose: RigidPose, inertia: BodyInertia, desc: CollidableDescription, activity: BodyActivityDescription) -> BodyDescription {
        BodyDescription {
            pose,
            vel: BodyVelocity::default(),
            local_inertia: inertia,
            collidable: desc,
            activity,
        }
    }
}


#[derive(Debug)]
#[repr(C)]
pub struct BodyInertias {
    pub local: BodyInertia,
    pub world: BodyInertia
}

#[derive(Debug)]
#[repr(C)]
pub struct BodyDynamics {
    pub motion: MotionState,
    pub inertia: BodyInertias
}