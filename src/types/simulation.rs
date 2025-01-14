use super::{
    body::{BodyActivity, BodyConstraintReference, BodyDynamics, Collidable},
    handles::BodyHandle,
    utilities::{Buffer, QuickList},
};

/// Defines properties of the solver
#[repr(C)]
pub struct SolveDescription {
    /// Number of velocity iterations to use in the solver if there is no `VelocityIterationScheduler` or if it returns a non-positive value for a substep.
    pub velocity_iteration_count: i32,
    /// Number of substeps to execute each time the solver runs.
    pub substep_count: i32,
    /// Number of synchronzed constraint batches to use before using a fallback approach.
    pub fallback_batch_threshold: i32,
    /// Callback executed to determine how many velocity iterations should be used for a given substep. If null, or if it returns a non-positive value, the `velocity_iteration_count` will be used instead.
    ///
    /// # Arguments
    ///
    /// * `substep_index`: Index of the substep to schedule velocity iterations for.
    ///
    /// returns: Number of velocity iterations to run during this substep.
    pub velocity_iteration_scheduler: Option<fn(i32) -> i32>,
}

impl Default for SolveDescription {
    fn default() -> Self {
        // Refer to BepuPhysics examples to better understand what these defaults are and why they were chosen.
        Self {
            velocity_iteration_count: 8,
            // By default we don't have substepping.
            substep_count: 1,
            fallback_batch_threshold: 128,
            velocity_iteration_scheduler: None,
        }
    }
}

impl SolveDescription {
    /// Creates a solve description.
    ///
    /// # Arguments
    ///
    /// * `velocity_iteration_count`: Number of velocity iterations per substep.
    /// * `substep_count`: Number of substeps in the solve.
    /// * `fallback_batch_threshold`: Number of synchronzed constraint batches to use before using a fallback approach.
    pub fn new(
        velocity_iteration_count: i32,
        substep_count: i32,
        fallback_batch_threshold: i32,
    ) -> Self {
        Self {
            velocity_iteration_count,
            substep_count,
            fallback_batch_threshold,
            velocity_iteration_scheduler: None,
        }
    }
}

/// The common set of allocation sizes for a simulation.
#[repr(C)]
pub struct SimulationAllocationSizes {
    /// The number of bodies to allocate space for.
    pub bodies: i32,
    /// The number of statics to allocate space for.
    pub statics: i32,
    /// The number of inactive islands to allocate space for.
    pub islands: i32,
    /// Minimum number of shapes to allocate space for in each shape type batch.
    pub shapes_per_type: i32,
    /// The number of constraints to allocate bookkeeping space for. This does not affect actual type batch allocation sizes, only the solver-level constraint handle storage.
    pub constraints: i32,
    /// The minimum number of constraints to allocate space for in each individual type batch.
    /// New type batches will be given enough memory for this number of constraints, and any compaction will not reduce the allocations below it.
    /// The number of constraints can vary greatly across types- there are usually far more contacts than ragdoll constraints.
    /// Per type estimates can be assigned within the Solver.TypeBatchAllocation if necessary. This value acts as a lower bound for all types.
    pub constraints_per_type_batch: i32,
    /// The minimum number of constraints to allocate space for in each body's constraint list.
    /// New bodies will be given enough memory for this number of constraints, and any compaction will not reduce the allocations below it.
    pub constraint_count_per_body_estimate: i32,
}

impl SimulationAllocationSizes {
    /// Constructs a description of simulation allocations.
    ///
    /// # Arguments
    ///
    /// * `bodies`: The number of bodies to allocate space for.
    /// * `statics`: The number of statics to allocate space for.
    /// * `islands`: The number of inactive islands to allocate space for.
    /// * `shapes_per_type`: Minimum number of shapes to allocate space for in each shape type batch.
    /// * `constraints`: The number of constraints to allocate bookkeeping space for. This does not affect actual type batch allocation sizes, only the solver-level constraint handle storage.
    /// * `constraints_per_type_batch`: The minimum number of constraints to allocate space for in each individual type batch.
    /// New type batches will be given enough memory for this number of constraints, and any compaction will not reduce the allocations below it.
    /// The number of constraints can vary greatly across types- there are usually far more contacts than ragdoll constraints.
    /// Per type estimates can be assigned within the Solver.TypeBatchAllocation if necessary. This value acts as a lower bound for all types.
    /// * `constraint_count_per_body_estimate`: The minimum number of constraints to allocate space for in each body's constraint list.
    /// New bodies will be given enough memory for this number of constraints, and any compaction will not reduce the allocations below it.
    pub fn new(
        bodies: i32,
        statics: i32,
        islands: i32,
        shapes_per_type: i32,
        constraints: i32,
        constraints_per_type_batch: i32,
        constraint_count_per_body_estimate: i32,
    ) -> Self {
        Self {
            bodies,
            statics,
            islands,
            shapes_per_type,
            constraints,
            constraints_per_type_batch,
            constraint_count_per_body_estimate,
        }
    }
}

impl Default for SimulationAllocationSizes {
    fn default() -> Self {
        // These defaults were picked out of the bepu examples.
        // Feel free to read the documentation to better understand these values, but they are effectively just hints,
        // similar to Vec::with_capacity() to reduce re-allocs on initial creation of the simulation.
        Self {
            bodies: 4096,
            statics: 4096,
            islands: 4096,
            shapes_per_type: 128,
            constraints: 4096,
            constraints_per_type_batch: 128,
            constraint_count_per_body_estimate: 6,
        }
    }
}

/// Location of a body in memory.
#[repr(C)]
pub struct BodyMemoryLocation {
    /// Index of the set owning the body reference. If the set index is 0, the body is awake. If the set index is greater than zero, the body is asleep.
    pub set_index: i32,
    /// Index of the body within its owning set.
    pub index: i32,
}

/// Stores a group of bodies- either the set of active bodies, or the bodies involved in an inactive simulation island.
#[repr(C)]
pub struct BodySet {
    /// Remaps a body index to its handle.
    pub index_to_handle: Buffer<BodyHandle>,
    /// Stores all data involved in solving constraints for a body, including pose, velocity, and inertia.
    pub dynamics_state: Buffer<BodyDynamics>,
    /// The collidables owned by each body in the set. Speculative margins, continuity settings, and shape indices can be changed directly.
    /// Shape indices cannot transition between pointing at a shape and pointing at nothing or vice versa without notifying the broad phase of the collidable addition or removal.
    pub collidables: Buffer<Collidable>,
    /// Activity states of bodies in the set.
    pub activity: Buffer<BodyActivity>,
    /// List of constraints associated with each body in the set.
    pub constraints: Buffer<QuickList<BodyConstraintReference>>,
    /// Number of bodies in the body set.
    pub count: i32,
}

impl BodySet {
    /// Gets whether this instance is backed by allocated memory.
    pub fn is_allocated(&self) -> bool {
        self.index_to_handle.memory != std::ptr::null_mut()
    }
}
