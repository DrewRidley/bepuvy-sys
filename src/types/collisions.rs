use super::{
    constraints::springs::*,
    handles::{BodyHandle, StaticHandle},
    math::scalar::Vector3,
};

/// Represents how a collidable can interact and move.
#[repr(C)]
#[derive(PartialEq, Eq)]
pub enum CollidableMobility {
    /// Marks a collidable as owned by a dynamic body.
    Dynamic = 0,
    /// Marks a collidable as owned by a kinematic body.
    Kinematic = 1,
    /// Marks the collidable as an independent immobile collidable.
    Static = 2,
}

/// Uses a bitpacked representation to refer to a body or static collidable.
#[repr(C)]
pub struct CollidableReference {
    /// Bitpacked representation of the collidable reference.
    pub packed: u32,
}

impl CollidableReference {
    /// Gets the mobility state of the owner of this collidable.
    pub fn mobility(&self) -> CollidableMobility {
        unsafe { std::mem::transmute((self.packed >> 30) as u32) }
    }

    /// Gets the body handle of the owner of the collidable referred to by this instance.
    pub fn body_handle(&self) -> BodyHandle {
        assert!(
            self.mobility() == CollidableMobility::Dynamic
                || self.mobility() == CollidableMobility::Kinematic
        );
        BodyHandle {
            value: (self.packed & 0x3FFFFFFF) as i32,
        }
    }

    /// Gets the static handle of the owner of the collidable referred to by this instance.
    pub fn static_handle(&self) -> StaticHandle {
        assert!(self.mobility() == CollidableMobility::Static);
        StaticHandle {
            value: (self.packed & 0x3FFFFFFF) as i32,
        }
    }

    pub fn create_static(handle: StaticHandle) -> Self {
        Self {
            packed: ((CollidableMobility::Static as u32) << 30) | handle.value as u32,
        }
    }
    pub fn create_dynamic(handle: BodyHandle) -> Self {
        Self {
            packed: ((CollidableMobility::Dynamic as u32) << 30) | handle.value as u32,
        }
    }
    pub fn create_kinematic(handle: BodyHandle) -> Self {
        Self {
            packed: ((CollidableMobility::Kinematic as u32) << 30) | handle.value as u32,
        }
    }
}

#[repr(C)]
pub struct CollidablePair {
    pub a: CollidableReference,
    pub b: CollidableReference,
}

/// Information about a single contact in a convex collidable pair. Convex collidable pairs share one surface basis across the manifold, since the contact surface is guaranteed to be a plane.
#[repr(C)]
pub struct ConvexContact {
    /// Offset from the position of collidable A to the contact position.
    pub offset: Vector3,
    /// Penetration depth between the two collidables at this contact. Negative values represent separation.
    pub depth: f32,
    /// Id of the features involved in the collision that generated this contact. If a contact has the same feature id as in a previous frame, it is an indication that the
    /// same parts of the shape contributed to its creation. This is useful for carrying information from frame to frame.
    pub feature_id: i32,
}

/// Contains the data associated with a convex contact manifold.
#[repr(C)]
pub struct ConvexContactManifold {
    /// Offset from collidable A to collidable B.
    pub offset_b: Vector3,
    pub count: i32,
    /// Surface normal shared by all contacts. Points from collidable B to collidable A.
    pub normal: Vector3,
    pub contacts: [ConvexContact; 4],
}

impl ConvexContactManifold {
    pub fn validate_index(&self, contact_index: i32) {
        assert!(contact_index >= 0 && contact_index < self.count);
    }
}

/// Information about a single contact in a nonconvex collidable pair.
/// Nonconvex pairs can have different surface bases at each contact point, since the contact surface is not guaranteed to be a plane.
#[repr(C)]
pub struct NonconvexContact {
    /// Offset from the position of collidable A to the contact position.
    pub offset: Vector3,
    /// Penetration depth between the two collidables at this contact. Negative values represent separation.
    pub depth: f32,
    /// Surface basis of the contact. If transformed into a rotation matrix, X and Z represent tangent directions and Y represents the contact normal. Points from collidable B to collidable A.
    pub normal: Vector3,
    /// Id of the features involved in the collision that generated this contact. If a contact has the same feature id as in a previous frame, it is an indication that the
    /// same parts of the shape contributed to its creation. This is useful for carrying information from frame to frame.
    pub feature_id: i32,
}

/// Contains the data associated with a nonconvex contact manifold.
#[repr(C)]
pub struct NonconvexContactManifold {
    /// Offset from collidable A to collidable B.
    pub offset_b: Vector3,
    pub count: i32,
    pub contacts: [NonconvexContact; 4],
}

/// Material properties governing the interaction between colliding bodies. Used by the narrow phase to create constraints of the appropriate configuration.
#[repr(C)]
pub struct PairMaterialProperties {
    /// Coefficient of friction to apply for the constraint. Maximum friction force will be equal to the normal force times the friction coefficient.
    pub friction_coefficient: f32,
    /// Maximum relative velocity along the contact normal at which the collision constraint will recover from penetration. Clamps the velocity goal created from the spring settings.
    pub maximum_recovery_velocity: f32,
    /// Defines the constraint's penetration recovery spring properties.
    pub contact_spring_settings: SpringSettings,
}

impl PairMaterialProperties {
    /// Constructs a pair's material properties.
    ///
    /// # Arguments
    ///
    /// * `friction_coefficient`: Coefficient of friction to apply for the constraint. Maximum friction force will be equal to the normal force times the friction coefficient.
    /// * `maximum_recovery_velocity`: Maximum relative velocity along the contact normal at which the collision constraint will recover from penetration. Clamps the velocity goal created from the spring settings.
    /// * `spring_settings`: Defines the constraint's penetration recovery spring properties.
    pub fn new(
        friction_coefficient: f32,
        maximum_recovery_velocity: f32,
        spring_settings: SpringSettings,
    ) -> Self {
        Self {
            friction_coefficient,
            maximum_recovery_velocity,
            contact_spring_settings: spring_settings,
        }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, SpringSettings::new(0.0, 0.0))
    }
}
