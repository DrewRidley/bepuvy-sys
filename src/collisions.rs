use glam::Vec3A;

use crate::{handles::{BodyHandle, SimulationHandle, StaticHandle}, constraints::SpringSettings};

#[derive(Eq, PartialEq)]
#[repr(u32)]
enum CollidableMobility {
    Dynamic = 0,
    Kinematic = 1,
    Static = 2,
}

#[repr(C)]
pub struct CollidableReference {
    packed: u32,
}

impl CollidableReference {
    fn get_mobility(&self) -> CollidableMobility {
        unsafe { std::mem::transmute(self.packed >> 30) }
    }

    fn get_body_handle(&self) -> BodyHandle {
        assert!(
            self.get_mobility() == CollidableMobility::Dynamic
                || self.get_mobility() == CollidableMobility::Kinematic
        );
        BodyHandle(self.packed as i32)
    }

    fn get_static_handle(&self) -> StaticHandle {
        assert!(self.get_mobility() == CollidableMobility::Static);
        StaticHandle(self.packed as i32)
    }

    fn get_raw_handle_value(&self) -> i32 {
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

pub struct ConvexContact {
    offset: Vec3A,
    depth: f32,
    feature_id: i32,
}

pub struct ConvexContactManifold {
    offset_b: Vec3A,
    count: i32,
    normal: Vec3A,
    contacts: [ConvexContact; 4],
}

pub struct NonconvexContact {
    offset: Vec3A,
    depth: f32,
    normal: Vec3A,
    feature_id: i32,
}

pub struct NonconvexContactManifold {
    offset_b: Vec3A,
    count: i32,
    contacts: [NonconvexContact; 4],
}

pub struct PairMaterialProperties {
    friction_coefficient: f32,
    maximum_recovery_velocity: f32,
    contact_spring_settings: SpringSettings,
}

#[derive(Default)]
pub struct NarrowPhaseCallbacks {
    pub initialize_function: Option<fn(SimulationHandle)>,
    pub dispose_function: Option<fn(SimulationHandle)>,
    pub allow_contact_generation_function: Option<fn(
        SimulationHandle,
        i32,
        CollidableReference,
        CollidableReference,
        *mut f32,
    ) -> bool>,
    pub allow_contact_generation_between_children_function: Option<fn(
        SimulationHandle,
        i32,
        CollidablePair,
        i32,
        i32,
    ) -> bool>,
    pub configure_convex_contact_manifold_function: Option<fn(
        SimulationHandle,
        i32,
        CollidablePair,
        *mut ConvexContactManifold,
        *mut PairMaterialProperties,
    ) -> bool>,
    pub configure_nonconvex_contact_manifold_function: Option<fn(
        SimulationHandle,
        i32,
        CollidablePair,
        *mut NonconvexContactManifold,
        *mut PairMaterialProperties,
    ) -> bool>,
    pub collide_bodies: Option<fn(SimulationHandle, i32, CollidablePair)>,
    pub collidable_added: Option<fn(SimulationHandle, CollidableReference)>,
    pub collidable_removed: Option<fn(SimulationHandle, CollidableReference)>,
    pub convex_shape_added: Option<fn(SimulationHandle, i32)>,
    pub convex_shape_removed: Option<fn(SimulationHandle, i32)>,
    pub nonconvex_shape_added: Option<fn(SimulationHandle, i32)>,
    pub nonconvex_shape_removed: Option<fn(SimulationHandle, i32)>,
    pub collider_set_modified: Option<fn(SimulationHandle)>,
    pub contact_modification_callback: Option<fn(SimulationHandle, CollidablePair, i32, bool)>,
    pub contact_pair_modified_callback: Option<fn(SimulationHandle, CollidablePair, i32, bool)>,
}
