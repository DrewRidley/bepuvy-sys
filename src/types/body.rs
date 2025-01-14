use super::{
    ccd::ContinuousDetection,
    handles::{ConstraintHandle, TypedIndex},
    math::scalar::{RigidPose, Symmetric3x3, Vector3},
};

/// Description of a collidable used by a body living in the broad phase and able to generate collision pairs.
/// Collidables with a `ShapeIndex` that points to nothing (a default constructed `TypedIndex`) are not capable of colliding with anything.
/// This can be used for a body which needs no collidable representation.
#[repr(C)]
pub struct Collidable {
    /// Index of the shape used by the body. While this can be changed, any transition from shapeless->shapeful or shapeful->shapeless must be reported to the broad phase.
    /// If you need to perform such a transition, consider using `Bodies::set_shape` or `Bodies::apply_description`; those functions update the relevant state.
    pub shape: TypedIndex,
    /// Continuous collision detection settings for this collidable. Includes the collision detection mode to use and tuning variables associated with those modes.
    pub continuity: ContinuousDetection,
    /// Lower bound on the value of the speculative margin used by the collidable.
    ///
    /// # Remarks
    ///
    /// 0 tends to be a good default value. Higher values can be chosen if velocity magnitude is a poor proxy for speculative margins, but these cases are rare.
    /// In those cases, try to use the smallest value that still satisfies requirements to avoid creating unnecessary contact constraints.
    pub minimum_speculative_margin: f32,
    /// Upper bound on the value of the speculative margin used by the collidable.
    ///
    /// # Remarks
    ///
    /// `f32::MAX` tends to be a good default value for discrete or passive mode collidables.
    /// The speculative margin will increase in size proportional to velocity magnitude, so having an unlimited maximum won't cost extra if the body isn't moving fast.
    ///
    /// Smaller values can be useful for improving performance in chaotic situations where missing a collision is acceptable. When using `ContinuousDetectionMode::Continuous`, a speculative margin larger than the velocity magnitude will result in the sweep test being skipped, so lowering the maximum margin can help avoid ghost collisions.
    pub maximum_speculative_margin: f32,
    /// Automatically computed size of the margin around the surface of the shape in which contacts can be generated. These contacts will have negative depth and only contribute if the frame's velocities
    /// would push the shapes of a pair into overlap.
    ///
    /// # Remarks
    ///
    /// This is automatically set by bounding box prediction each frame, and is bound by the collidable's `minimum_speculative_margin` and `maximum_speculative_margin` values.
    /// The effective speculative margin for a collision pair can also be modified from `INarrowPhaseCallbacks` callbacks.
    ///
    /// This should be positive to avoid jittering.
    ///
    /// It can also be used as a form of continuous collision detection, but excessively high values combined with fast motion may result in visible 'ghost collision' artifacts.
    /// For continuous collision detection with less chance of ghost collisions, use `ContinuousDetectionMode::Continuous`.
    ///
    /// If using `ContinuousDetectionMode::Continuous`, consider setting `maximum_speculative_margin` to a smaller value to help filter ghost collisions.
    ///
    /// For more information, see the ContinuousCollisionDetection.md documentation.
    pub speculative_margin: f32,
    /// Index of the collidable in the broad phase. Used to look up the target location for bounding box scatters. Under normal circumstances, this should not be set externally.
    pub broad_phase_index: i32,
}

/// Linear and angular velocity for a body.
#[repr(C)]
pub struct BodyVelocity {
    /// Linear velocity associated with the body.
    pub linear: Vector3,
    pub pad0: i32,
    /// Angular velocity associated with the body.
    pub angular: Vector3,
    pub pad1: i32,
}

impl BodyVelocity {
    pub fn new(linear: Vector3, angular: Vector3) -> Self {
        Self {
            linear,
            pad0: 0,
            angular,
            pad1: 0,
        }
    }

    pub fn from_linear(linear: Vector3) -> Self {
        Self::new(linear, Vector3::zero())
    }

    pub fn zero() -> Self {
        Self::new(Vector3::zero(), Vector3::zero())
    }
}

/// Describes the pose and velocity of a body.
#[repr(C)]
pub struct MotionState {
    /// Pose of the body.
    pub pose: RigidPose,
    /// Linear and angular velocity of the body.
    pub velocity: BodyVelocity,
}

/// Stores the inertia for a body.
///
/// # Remarks
///
/// This representation stores the inverse mass and inverse inertia tensor. Most of the high frequency use cases in the engine naturally use the inverse.
#[repr(C)]
pub struct BodyInertia {
    /// Inverse of the body's inertia tensor.
    pub inverse_inertia_tensor: Symmetric3x3,
    /// Inverse of the body's mass.
    pub inverse_mass: f32,
    pub pad: u32,
}

/// Stores the local and world views of a body's inertia, packed together for efficient access.
#[repr(C)]
pub struct BodyInertias {
    /// Local inertia of the body.
    pub local: BodyInertia,
    /// Transformed world inertia of the body. Note that this is only valid between the velocity integration that updates it and the pose integration that follows.
    /// Outside of that execution window, this should be considered undefined.
    ///
    /// # Remarks
    ///
    /// We cache this here because velocity integration wants both the local and world inertias, and any integration happening within the solver will do so without the benefit of sequential loads.
    /// In that context, being able to load a single cache line to grab both local and world inertia helps quite a lot.
    pub world: BodyInertia,
}

/// Stores all body information needed by the solver together.
#[repr(C)]
pub struct BodyDynamics {
    /// Pose and velocity information for the body.
    pub motion: MotionState,
    /// Inertia information for the body.
    pub inertia: BodyInertias,
}

/// Describes how a body sleeps, and its current state with respect to sleeping.
#[repr(C)]
pub struct BodyActivity {
    /// Threshold of squared velocity under which the body is allowed to go to sleep. This is compared against dot(linearVelocity, linearVelocity) + dot(angularVelocity, angularVelocity).
    /// Setting this to a negative value guarantees the body cannot go to sleep without user action.
    pub sleep_threshold: f32,
    /// The number of time steps that the body must be under the sleep threshold before the body becomes a sleeping candidate.
    /// Note that the body is not guaranteed to go to sleep immediately after meeting this minimum.
    pub minimum_timesteps_under_threshold: u8,
    /// If the body is awake, this is the number of time steps that the body has had a velocity below the sleep threshold.
    ///
    /// # Remarks
    ///
    /// Note that all values beyond this point are runtime set. The user should virtually never need to modify them.
    /// We do not constrain write access by default, instead opting to leave it open for advanced users to mess around with.
    /// TODO: If people misuse these, we should internalize them in a case by case basis.
    pub timesteps_under_threshold_count: u8,
    /// True if this body is a candidate for being slept. If all the bodies that it is connected to by constraints are also candidates, this body may go to sleep.
    ///
    /// # Remarks
    ///
    /// Note that this flag is held alongside the other sleeping data, despite the fact that the traversal only needs the SleepCandidate state.
    /// This is primarily for simplicity, but also note that the dominant accessor of this field is actually the sleep candidacy computation. Traversal doesn't visit every
    /// body every frame, but sleep candidacy analysis does.
    /// The reason why this flag exists at all is just to prevent traversal from being aware of the logic behind candidacy managemnt.
    /// It doesn't cost anything extra to store this; it fits within the 8 byte layout.
    pub sleep_candidate: bool,
}

/// Describes a collidable and how it should handle continuous collision detection.
#[repr(C)]
pub struct CollidableDescription {
    /// Shape of the collidable.
    pub shape: TypedIndex,
    /// Continuous collision detection settings used by the collidable.
    pub continuity: ContinuousDetection,
    /// Lower bound on the value of the speculative margin used by the collidable.
    ///
    /// # Remarks
    ///
    /// 0 tends to be a good default value. Higher values can be chosen if velocity magnitude is a poor proxy for speculative margins, but these cases are rare.
    /// In those cases, try to use the smallest value that still satisfies requirements to avoid creating unnecessary contact constraints.
    pub minimum_speculative_margin: f32,
    /// Upper bound on the value of the speculative margin used by the collidable.
    ///
    /// # Remarks
    ///
    /// `f32::MAX` tends to be a good default value for discrete or passive mode collidables.
    /// The speculative margin will increase in size proportional to velocity magnitude, so having an unlimited maximum won't cost extra if the body isn't moving fast.
    ///
    /// Smaller values can be useful for improving performance in chaotic situations where missing a collision is acceptable. When using `ContinuousDetectionMode::Continuous`, a speculative margin larger than the velocity magnitude will result in the sweep test being skipped, so lowering the maximum margin can help avoid ghost collisions.
    pub maximum_speculative_margin: f32,
}

impl CollidableDescription {
    /// Constructs a new collidable description.
    ///
    /// # Arguments
    ///
    /// * `shape`: Shape used by the collidable.
    /// * `minimum_speculative_margin`: Lower bound on the value of the speculative margin used by the collidable.
    /// * `maximum_speculative_margin`: Upper bound on the value of the speculative margin used by the collidable.
    /// * `continuity`: Continuous collision detection settings for the collidable.
    pub fn new(
        shape: TypedIndex,
        minimum_speculative_margin: f32,
        maximum_speculative_margin: f32,
        continuity: ContinuousDetection,
    ) -> Self {
        Self {
            shape,
            minimum_speculative_margin,
            maximum_speculative_margin,
            continuity,
        }
    }
    /// Constructs a new collidable description with `ContinuousDetectionMode::Discrete`.
    ///
    /// # Arguments
    ///
    /// * `shape`: Shape used by the collidable.
    /// * `minimum_speculative_margin`: Lower bound on the value of the speculative margin used by the collidable.
    /// * `maximum_speculative_margin`: Upper bound on the value of the speculative margin used by the collidable.
    pub fn with_discrete(
        shape: TypedIndex,
        minimum_speculative_margin: f32,
        maximum_speculative_margin: f32,
    ) -> Self {
        Self {
            shape,
            minimum_speculative_margin,
            maximum_speculative_margin,
            continuity: ContinuousDetection::discrete(),
        }
    }
    /// Constructs a new collidable description. Uses 0 for the `minimum_speculative_margin`.
    ///
    /// # Arguments
    ///
    /// * `shape`: Shape used by the collidable.
    /// * `maximum_speculative_margin`: Upper bound on the value of the speculative margin used by the collidable.
    /// * `continuity`: Continuous collision detection settings for the collidable.
    pub fn with_minimum_speculative_margin(
        shape: TypedIndex,
        maximum_speculative_margin: f32,
        continuity: ContinuousDetection,
    ) -> Self {
        Self {
            shape,
            minimum_speculative_margin: 0.0,
            maximum_speculative_margin,
            continuity,
        }
    }
    /// Constructs a new collidable description. Uses 0 for the `minimum_speculative_margin` and `f32::MAX` for the `maximum_speculative_margin`.
    ///
    /// # Arguments
    ///
    /// * `shape`: Shape used by the collidable.
    /// * `continuity`: Continuous collision detection settings for the collidable.
    pub fn with_max_speculative_margin(shape: TypedIndex, continuity: ContinuousDetection) -> Self {
        Self {
            shape,
            minimum_speculative_margin: 0.0,
            maximum_speculative_margin: f32::MAX,
            continuity,
        }
    }
    /// Constructs a new collidable description with `ContinuousDetectionMode::Passive`. Will use a `minimum_speculative_margin` of 0 and a `maximum_speculative_margin` of `f32::MAX`.
    ///
    /// # Arguments
    ///
    /// * `shape`: Shape used by the collidable.
    ///
    /// # Remarks
    ///
    /// `ContinuousDetectionMode::Passive` and `ContinuousDetectionMode::Discrete` are equivalent in behavior when the `maximum_speculative_margin` is `f32::MAX` since they both result in the same (unbounded) expansion of body bounding boxes in response to velocity.
    pub fn passive(shape: TypedIndex) -> Self {
        Self {
            shape,
            minimum_speculative_margin: 0.0,
            maximum_speculative_margin: f32::MAX,
            continuity: ContinuousDetection::passive(),
        }
    }
    /// Constructs a new collidable description with `ContinuousDetectionMode::Discrete`. Will use a minimum speculative margin of 0 and the given maximumSpeculativeMargin.
    ///
    /// # Arguments
    ///
    /// * `shape`: Shape used by the collidable.
    /// * `maximum_speculative_margin`: Maximum speculative margin to be used with the discrete continuity configuration.
    pub fn with_max_speculative_margin_discrete(
        shape: TypedIndex,
        maximum_speculative_margin: f32,
    ) -> Self {
        Self {
            shape,
            minimum_speculative_margin: 0.0,
            maximum_speculative_margin,
            continuity: ContinuousDetection::discrete(),
        }
    }
}

/// Describes the thresholds for a body going to sleep.
#[repr(C)]
pub struct BodyActivityDescription {
    /// Threshold of squared velocity under which the body is allowed to go to sleep. This is compared against dot(linearVelocity, linearVelocity) + dot(angularVelocity, angularVelocity).
    pub sleep_threshold: f32,
    /// The number of time steps that the body must be under the sleep threshold before the body becomes a sleep candidate.
    /// Note that the body is not guaranteed to go to sleep immediately after meeting this minimum.
    pub minimum_timestep_count_under_threshold: u8,
}

impl BodyActivityDescription {
    /// Creates a body activity description.
    ///
    /// # Arguments
    ///
    /// * `sleep_threshold`: Threshold of squared velocity under which the body is allowed to go to sleep. This is compared against dot(linearVelocity, linearVelocity) + dot(angularVelocity, angularVelocity).
    /// * `minimum_timestep_count_under_threshold`: The number of time steps that the body must be under the sleep threshold before the body becomes a sleep candidate.
    /// Note that the body is not guaranteed to go to sleep immediately after meeting this minimum.
    pub fn new(sleep_threshold: f32, minimum_timestep_count_under_threshold: u8) -> Self {
        Self {
            sleep_threshold,
            minimum_timestep_count_under_threshold,
        }
    }
}

/// Describes a body's state.
#[repr(C)]
pub struct BodyDescription {
    /// Position and orientation of the body.
    pub pose: RigidPose,
    /// Linear and angular velocity of the body.
    pub velocity: BodyVelocity,
    /// Mass and inertia tensor of the body.
    pub local_inertia: BodyInertia,
    /// Shape and collision detection settings for the body.
    pub collidable: CollidableDescription,
    /// Sleeping settings for the body.
    pub activity: BodyActivityDescription,
}

impl BodyDescription {
    /// Creates a dynamic body description.
    ///
    /// # Arguments
    ///
    /// * `pose`: Pose of the body.
    /// * `velocity`: Initial velocity of the body.
    /// * `inertia`: Local inertia of the body.
    /// * `collidable`: Collidable to associate with the body.
    /// * `activity`: Activity settings for the body.
    ///
    /// returns: Constructed description for the body.
    pub fn create_dynamic(
        pose: RigidPose,
        velocity: BodyVelocity,
        inertia: BodyInertia,
        collidable: CollidableDescription,
        activity: BodyActivityDescription,
    ) -> Self {
        Self {
            pose,
            velocity,
            local_inertia: inertia,
            collidable,
            activity,
        }
    }
    /// Creates a dynamic body description with zero initial velocity.
    ///
    /// # Arguments
    ///
    /// * `pose`: Pose of the body.
    /// * `inertia`: Local inertia of the body.
    /// * `collidable`: Collidable to associate with the body.
    /// * `activity`: Activity settings for the body.
    ///
    /// returns: Constructed description for the body.
    pub fn create_dynamic_at_rest(
        pose: RigidPose,
        inertia: BodyInertia,
        collidable: CollidableDescription,
        activity: BodyActivityDescription,
    ) -> Self {
        Self {
            pose,
            velocity: BodyVelocity::zero(),
            local_inertia: inertia,
            collidable,
            activity,
        }
    }
    /// Creates a kinematic body description.
    ///
    /// # Arguments
    ///
    /// * `pose`: Pose of the body.
    /// * `velocity`: Initial velocity of the body.
    /// * `collidable`: Collidable to associate with the body.
    /// * `activity`: Activity settings for the body.
    ///
    /// returns: Constructed description for the body.
    pub fn create_kinematic(
        pose: RigidPose,
        velocity: BodyVelocity,
        collidable: CollidableDescription,
        activity: BodyActivityDescription,
    ) -> Self {
        Self {
            pose,
            velocity,
            local_inertia: BodyInertia {
                inverse_inertia_tensor: Symmetric3x3::zero(),
                inverse_mass: 0.0,
                pad: 0,
            },
            collidable,
            activity,
        }
    }
    /// Creates a kinematic body description with zero initial velocity.
    ///
    /// # Arguments
    ///
    /// * `pose`: Pose of the body.
    /// * `collidable`: Collidable to associate with the body.
    /// * `activity`: Activity settings for the body.
    ///
    /// returns: Constructed description for the body.
    pub fn create_kinematic_at_rest(
        pose: RigidPose,
        collidable: CollidableDescription,
        activity: BodyActivityDescription,
    ) -> Self {
        Self {
            pose,
            velocity: BodyVelocity::zero(),
            local_inertia: BodyInertia {
                inverse_inertia_tensor: Symmetric3x3::zero(),
                inverse_mass: 0.0,
                pad: 0,
            },
            collidable,
            activity,
        }
    }
}

#[repr(C)]
pub struct BodyConstraintReference {
    pub connecting_constraint_handle: ConstraintHandle,
    pub body_index_in_constraint: i32,
}
