use super::{
    ccd::ContinuousDetection,
    handles::TypedIndex,
    math::scalar::{Quaternion, RigidPose, Vector3},
};

/// Describes the properties of a static object. When added to a simulation, static objects can collide but have no velocity and will not move in response to forces.
#[repr(C)]
pub struct StaticDescription {
    /// Position and orientation of the static.
    pub pose: RigidPose,
    /// Shape of the static.
    pub shape: TypedIndex,
    /// Continuous collision detection settings for the static.
    pub continuity: ContinuousDetection,
}

impl StaticDescription {
    /// Builds a new static description.
    ///
    /// # Arguments
    ///
    /// * `pose`: Pose of the static collidable.
    /// * `shape`: Shape of the static.
    /// * `continuity`: Continuous collision detection settings for the static.
    pub fn create(pose: RigidPose, shape: TypedIndex, continuity: ContinuousDetection) -> Self {
        Self {
            pose,
            shape,
            continuity,
        }
    }

    /// Builds a new static description with `ContinuousDetectionMode::Discrete` continuity.
    ///
    /// # Arguments
    ///
    /// * `pose`: Pose of the static collidable.
    /// * `shape`: Shape of the static.
    pub fn create_discrete(pose: RigidPose, shape: TypedIndex) -> Self {
        Self {
            pose,
            shape,
            continuity: ContinuousDetection::discrete(),
        }
    }

    /// Builds a new static description.
    ///
    /// # Arguments
    ///
    /// * `position`: Position of the static.
    /// * `orientation`: Orientation of the static.
    /// * `shape`: Shape of the static.
    /// * `continuity`: Continuous collision detection settings for the static.
    pub fn create_with_position_orientation(
        position: Vector3,
        orientation: Quaternion,
        shape: TypedIndex,
        continuity: ContinuousDetection,
    ) -> Self {
        Self {
            pose: RigidPose::new(position, orientation),
            shape,
            continuity,
        }
    }

    /// Builds a new static description with `ContinuousDetectionMode::Discrete` continuity.
    ///
    /// # Arguments
    ///
    /// * `position`: Position of the static.
    /// * `orientation`: Orientation of the static.
    /// * `shape`: Shape of the static.
    pub fn create_with_position_orientation_discrete(
        position: Vector3,
        orientation: Quaternion,
        shape: TypedIndex,
    ) -> Self {
        Self {
            pose: RigidPose::new(position, orientation),
            shape,
            continuity: ContinuousDetection::discrete(),
        }
    }
}

/// Stores data for a static collidable in the simulation. Statics can be posed and collide, but have no velocity and no dynamic behavior.
///
/// # Remarks
///
/// Unlike bodies, statics have a very simple access pattern. Most data is referenced together and there are no extreme high frequency data accesses like there are in the solver.
/// Everything can be conveniently stored within a single location contiguously.
#[repr(C)]
pub struct Static {
    /// Pose of the static collidable.
    pub pose: RigidPose,
    /// Continuous collision detection settings for this collidable. Includes the collision detection mode to use and tuning variables associated with those modes.
    ///
    /// # Remarks
    ///
    /// Note that statics cannot move, so there is no difference between `ContinuousDetectionMode::Discrete` and `ContinuousDetectionMode::Passive` for them.
    /// Enabling `ContinuousDetectionMode::Continuous` will still require that pairs associated with the static use swept continuous collision detection.
    pub continuity: ContinuousDetection,
    /// Index of the shape used by the static. While this can be changed, any transition from shapeless->shapeful or shapeful->shapeless must be reported to the broad phase.
    /// If you need to perform such a transition, consider using `Statics::set_shape` or `Statics::apply_description`; those functions update the relevant state.
    pub shape: TypedIndex,
    /// Index of the collidable in the broad phase. Used to look up the target location for bounding box scatters. Under normal circumstances, this should not be set externally.
    pub broad_phase_index: i32,
}
