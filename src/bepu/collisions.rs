use glam::Vec3;

use super::{handles::{BodyHandle, StaticHandle, SimulationHandle}, constraints::SpringSettings};


#[derive(Eq, PartialEq)]
#[repr(u32)]
pub enum CollidableMobility {
    Dynamic = 0,
    Kinematic = 1,
    Static = 2,
}

#[repr(C)]
pub struct CollidableReference {
    packed: u32,
}

impl CollidableReference {
    pub fn get_mobility(&self) -> CollidableMobility {
        unsafe { std::mem::transmute(self.packed >> 30) }
    }

    pub fn get_body_handle(&self) -> BodyHandle {
        assert!(
            self.get_mobility() == CollidableMobility::Dynamic
                || self.get_mobility() == CollidableMobility::Kinematic
        );
        BodyHandle(self.packed as i32)
    }

    pub fn get_static_handle(&self) -> StaticHandle {
        assert!(self.get_mobility() == CollidableMobility::Static);
        StaticHandle(self.packed as i32)
    }

    pub fn get_raw_handle_value(&self) -> i32 {
        (self.packed & 0x3FFFFFFF) as i32
    }

    fn create_static(handle: StaticHandle) -> CollidableReference {
        CollidableReference {
            packed: ((CollidableMobility::Static as u32) << 30) | (handle.0 as u32),
        }
    }

    fn create_dynamic(handle: BodyHandle) -> CollidableReference {
        CollidableReference {
            packed: ((CollidableMobility::Dynamic as u32) << 30) | (handle.0 as u32),
        }
    }

    fn create_kinematic(handle: BodyHandle) -> CollidableReference {
        CollidableReference {
            packed: ((CollidableMobility::Kinematic as u32) << 30) | (handle.0 as u32),
        }
    }
}

#[repr(C)]
pub struct CollidablePair {
    a: CollidableReference,
    b: CollidableReference,
}

#[repr(C)]
pub struct ConvexContact {
    offset: Vec3,
    depth: f32,
    feature_id: i32,
}

#[repr(C)]
pub struct ConvexContactManifold {
    offset_b: Vec3,
    count: i32,
    normal: Vec3,
    contacts: [ConvexContact; 4],
}

#[repr(C)]
pub struct NonconvexContact {
    offset: Vec3,
    depth: f32,
    normal: Vec3,
    feature_id: i32,
}

#[repr(C)]
pub struct NonconvexContactManifold {
    offset_b: Vec3,
    count: i32,
    contacts: [NonconvexContact; 4],
}

#[repr(C)]
pub struct PairMaterialProperties {
    friction_coefficient: f32,
    maximum_recovery_velocity: f32,
    contact_spring_settings: SpringSettings,
}

impl PairMaterialProperties {
    pub fn new(
        friction_coefficient: f32,
        maximum_recovery_velocity: f32,
        contact_spring_settings: SpringSettings,
    ) -> PairMaterialProperties {
        PairMaterialProperties {
            friction_coefficient,
            maximum_recovery_velocity,
            contact_spring_settings,
        }
    }
}

#[repr(C)]
pub struct NarrowPhaseCallbacks {
    pub initialize_function: Option<extern "C" fn(SimulationHandle)>,
    pub dispose_function: Option<extern "C" fn(SimulationHandle)>,
    pub allow_contact_generation_function: extern "C" fn(
        SimulationHandle,
        i32,
        CollidableReference,
        CollidableReference,
        *mut f32,
    ) -> bool,
    pub allow_contact_generation_between_children_function: extern "C" fn(
        SimulationHandle,
        i32,
        CollidablePair,
        i32,
        i32,
    ) -> bool,
    pub configure_convex_contact_manifold_function: extern "C" fn(
        SimulationHandle,
        i32,
        CollidablePair,
        *mut ConvexContactManifold,
        *mut PairMaterialProperties,
    ) -> bool,
    pub configure_nonconvex_contact_manifold_function: extern "C" fn(
        SimulationHandle,
        i32,
        CollidablePair,
        *mut NonconvexContactManifold,
        *mut PairMaterialProperties,
    ) -> bool,

    pub configure_child_contact_manifold_fn: extern "C" fn(
        SimulationHandle,
        i32,
        CollidablePair,
        i32,
        i32,
        *mut ConvexContactManifold,
    ) -> bool
}
