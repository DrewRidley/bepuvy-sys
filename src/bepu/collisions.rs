use super::constraints::SpringSettings;
use super::handles::*;
use super::interop_math::*;

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

/// Defines the callbacks invoked during narrow phase collision detection execution.
#[repr(C)]
pub struct NarrowPhaseCallbacks {
    /// Called after the simulation is created. Can be null.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation owning these callbacks.
    pub initialize_function: Option<unsafe extern "C" fn(simulation_handle: SimulationHandle)>,
    /// Called when the simulation is being torn down. Can be null.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation owning these callbacks.
    pub dispose_function: Option<unsafe extern "C" fn(simulation_handle: SimulationHandle)>,
    /// Called for each pair of collidables with overlapping bounding boxes found by the broad phase.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation owning these callbacks.
    /// * `worker_index`: Index of the worker within the thread dispatcher that's running this callback.
    /// * `a`: First collidable in the pair.
    /// * `b`: Second collidable in the pair.
    /// * `speculative_margin`: Speculative contact margin for the pair. Calculated ahead of time, but can be overridden.
    ///
    /// returns: True if the collision detection should run for this pair, false otherwise.
    pub allow_contact_generation_function: Option<
        unsafe extern "C" fn(
            simulation_handle: SimulationHandle,
            worker_index: i32,
            a: CollidableReference,
            b: CollidableReference,
            speculative_margin: *mut f32,
        ) -> bool,
    >,
    /// For pairs involving compound collidables (any type that has children, e.g. Compound, BigCompound, and Mesh), this is invoked for each pair of children with overlapping bounds.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation owning these callbacks.
    /// * `worker_index`: Index of the worker within the thread dispatcher that's running this callback.
    /// * `collidable_pair`: References to the parent collidables in this pair.
    /// * `child_index_a`: Index of the child belonging to the first collidable in the pair.
    /// * `child_index_b`: Index of the child belonging to the second collidable in the pair.
    ///
    /// returns: True if the collision detection should run for these children, false otherwise.
    pub allow_contact_generation_between_children_function: Option<
        unsafe extern "C" fn(
            simulation_handle: SimulationHandle,
            worker_index: i32,
            collidable_pair: CollidablePair,
            child_index_a: i32,
            child_index_b: i32,
        ) -> bool,
    >,
    /// Called after contacts have been found for a collidable pair that resulted in a convex manifold.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation owning these callbacks.
    /// * `worker_index`: Index of the worker within the thread dispatcher that's running this callback.
    /// * `collidable_pair`: References to the parent collidables in this pair.
    /// * `contact_manifold`: Contacts identified between the pair.
    /// * `material_properties`: Contact constraint material properties to use for the constraint, if any.
    ///
    /// returns: True if a contact constraint should be created for this contact manifold, false otherwise.
    pub configure_convex_contact_manifold_function: Option<
        unsafe extern "C" fn(
            simulation_handle: SimulationHandle,
            worker_index: i32,
            collidable_pair: CollidablePair,
            contact_manifold: *mut ConvexContactManifold,
            material_properties: *mut PairMaterialProperties,
        ) -> bool,
    >,
    /// Called after contacts have been found for a collidable pair that resulted in a nonconvex manifold.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation owning these callbacks.
    /// * `worker_index`: Index of the worker within the thread dispatcher that's running this callback.
    /// * `collidable_pair`: References to the parent collidables in this pair.
    /// * `contact_manifold`: Contacts identified between the pair.
    /// * `material_properties`: Contact constraint material properties to use for the constraint, if any.
    ///
    /// returns: True if a contact constraint should be created for this contact manifold, false otherwise.
    pub configure_nonconvex_contact_manifold_function: Option<
        unsafe extern "C" fn(
            simulation_handle: SimulationHandle,
            worker_index: i32,
            collidable_pair: CollidablePair,
            contact_manifold: *mut NonconvexContactManifold,
            material_properties: *mut PairMaterialProperties,
        ) -> bool,
    >,
    /// Called for contacts identified between children in a compound-involving pair prior to being processed into the top level contact manifold.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation owning these callbacks.
    /// * `worker_index`: Index of the worker within the thread dispatcher that's running this callback.
    /// * `collidable_pair`: References to the parent collidables in this pair.
    /// * `child_index_a`: Index of the child belonging to the first collidable in the pair.
    /// * `child_index_b`: Index of the child belonging to the second collidable in the pair.
    /// * `contact_manifold`: Contacts identified between the pair.
    ///
    /// returns: True if the contacts in this child pair should be considered for constraint generation, false otherwise.
    ///
    /// # Remarks
    ///
    /// Note that all children are required to be convex, so there is no nonconvex version of this callback.
    pub configure_child_contact_manifold_function: Option<
        unsafe extern "C" fn(
            simulation_handle: SimulationHandle,
            worker_index: i32,
            collidable_pair: CollidablePair,
            child_index_a: i32,
            child_index_b: i32,
            contact_manifold: *mut ConvexContactManifold,
        ) -> bool,
    >,
}
