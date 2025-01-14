use crate::types::{collisions::*, handles::SimulationHandle};

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
