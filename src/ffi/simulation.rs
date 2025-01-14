use crate::types::{
    body::*,
    callbacks::{narrow_phase::NarrowPhaseCallbacks, pose::PoseIntegratorCallbacks},
    handles::*,
    math::scalar::Vector3,
    simulation::*,
    statics::{Static, StaticDescription},
    utilities::{Buffer, QuickList},
};

extern "C" {
    /// Creates a new simulation.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool`: Buffer pool for the simulation's main allocations.
    /// * `narrow_phase_callbacks`: Narrow phase callbacks to be invoked by the simulation.
    /// * `pose_integrator_callbacks`: Pose integration state and callbacks to be invoked by the simulation.
    /// * `solve_description_interop`: Defines velocity iteration count and substep counts for the simulation's solver.
    /// * `initial_allocation_sizes`: Initial capacities to allocate within the simulation.
    ///
    /// # Returns
    ///
    /// The created simulation.
    #[link_name = "Simulation.Create"]
    pub fn create_simulation(
        buffer_pool: BufferPoolHandle,
        narrow_phase_callbacks: NarrowPhaseCallbacks,
        pose_integrator_callbacks: PoseIntegratorCallbacks,
        solve_description_interop: SolveDescription,
        initial_allocation_sizes: SimulationAllocationSizes,
    ) -> SimulationHandle;
    #[link_name = "Simulation.Destroy"]
    pub fn destroy_simulation(handle: SimulationHandle);
    #[link_name = "Simulation.AddBody"]
    pub fn add_body(
        simulation_handle: SimulationHandle,
        body_description: BodyDescription,
    ) -> BodyHandle;
    #[link_name = "Simulation.RemoveBody"]
    pub fn remove_body(simulation_handle: SimulationHandle, body_handle: BodyHandle);
    /// Gets a pointer to the dynamic state associated with a body. Includes pose, velocity, and inertia.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Simulation to pull a body's state from.
    /// * `body_handle`: Body handle to pull data about.
    ///
    /// # Returns
    ///
    /// Pointer to the body's dynamic state.
    ///
    /// # Remarks
    ///
    /// This is a direct pointer. The memory location associated with a body can move other bodies are removed from the simulation; do not hold a pointer beyond the point where it may be invalidated.
    #[link_name = "Simulation.GetBodyDynamics"]
    pub fn get_body_dynamics(
        simulation_handle: SimulationHandle,
        body_handle: BodyHandle,
    ) -> *mut BodyDynamics;
    /// Gets a pointer to the collidable associated with a body.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Simulation to pull a body's state from.
    /// * `body_handle`: Body handle to pull data about.
    ///
    /// # Returns
    ///
    /// Pointer to the body's collidable.
    ///
    /// # Remarks
    ///
    /// This is a direct pointer. The memory location associated with a body can move if other bodies are removed from the simulation; do not hold a pointer beyond the point where it may be invalidated.
    #[link_name = "Simulation.GetBodyCollidable"]
    pub fn get_body_collidable(
        simulation_handle: SimulationHandle,
        body_handle: BodyHandle,
    ) -> *mut Collidable;
    /// Gets a pointer to the activity state associated with a body.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Simulation to pull a body's state from.
    /// * `body_handle`: Body handle to pull data about.
    ///
    /// # Returns
    ///
    /// Pointer to the body's activity state.
    ///
    /// # Remarks
    ///
    /// This is a direct pointer. The memory location associated with a body can move if other bodies are removed from the simulation; do not hold a pointer beyond the point where it may be invalidated.
    #[link_name = "Simulation.GetBodyActivity"]
    pub fn get_body_activity(
        simulation_handle: SimulationHandle,
        body_handle: BodyHandle,
    ) -> *mut BodyActivity;
    /// Gets a pointer to the list of constraints associated with a body.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Simulation to pull a body's state from.
    /// * `body_handle`: Body handle to pull data about.
    ///
    /// # Returns
    ///
    /// Pointer to the body's constraint list.
    ///
    /// # Remarks
    ///
    /// This is a direct pointer. The memory location associated with a body can move if other bodies are removed from the simulation; do not hold a pointer beyond the point where it may be invalidated.
    #[link_name = "Simulation.GetBodyConstraints"]
    pub fn get_body_constraints(
        simulation_handle: SimulationHandle,
        body_handle: BodyHandle,
    ) -> *mut QuickList<BodyConstraintReference>;
    /// Gets a description of a body.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Simulation to pull a body's state from.
    /// * `body_handle`: Body handle to pull data about.
    ///
    /// # Returns
    ///
    /// Description of a body.
    #[link_name = "Simulation.GetBodyDescription"]
    pub fn get_body_description(
        simulation_handle: SimulationHandle,
        body_handle: BodyHandle,
    ) -> BodyDescription;
    /// Applies a description to a body.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Simulation to pull a body's state from.
    /// * `body_handle`: Body handle to pull data about.
    /// * `description`: Description to apply to the body.
    #[link_name = "Simulation.ApplyBodyDescription"]
    pub fn apply_body_description(
        simulation_handle: SimulationHandle,
        body_handle: BodyHandle,
        description: BodyDescription,
    );
    #[link_name = "Simulation.AddStatic"]
    pub fn add_static(
        simulation_handle: SimulationHandle,
        static_description: StaticDescription,
    ) -> StaticHandle;
    #[link_name = "Simulation.RemoveStatic"]
    pub fn remove_static(simulation_handle: SimulationHandle, static_handle: StaticHandle);
    /// Gets a pointer to data associated with a static.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Simulation to pull a static's state from.
    /// * `static_handle`: Static handle to pull data about.
    ///
    /// # Returns
    ///
    /// Pointer to the static's data.
    ///
    /// # Remarks
    ///
    /// This is a direct pointer. The memory location associated with a static can move if other statics are removed from the simulation; do not hold a pointer beyond the point where it may be invalidated.
    #[link_name = "Simulation.GetStatic"]
    pub fn get_static(
        simulation_handle: SimulationHandle,
        static_handle: StaticHandle,
    ) -> *mut Static;
    /// Gets a static's description.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Simulation to pull a static's state from.
    /// * `static_handle`: Static handle to pull data about.
    ///
    /// # Returns
    ///
    /// Description of the static..
    #[link_name = "Simulation.GetStaticDescription"]
    pub fn get_static_description(
        simulation_handle: SimulationHandle,
        static_handle: StaticHandle,
    ) -> StaticDescription;
    /// Applies a description to a static.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Simulation to pull a static's state from.
    /// * `static_handle`: Static handle to pull data about.
    #[link_name = "Simulation.ApplyStaticDescription"]
    pub fn apply_static_description(
        simulation_handle: SimulationHandle,
        static_handle: StaticHandle,
        description: StaticDescription,
    );

    /// Steps the simulation forward a single time.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to step.
    /// * `dt`: Duration of the timestep.
    /// * `thread_dispatcher_handle`: Handle of the thread dispatcher to use, if any. Can be a null reference.
    #[link_name = "Simulation.Timestep"]
    pub fn timestep(
        simulation_handle: SimulationHandle,
        dt: f32,
        callback: *mut (),
        thread_dispatcher_handle: ThreadDispatcherHandle,
    );
    /// Grabs a collidable's bounding boxes in the broad phase.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to pull data from.
    /// * `body_handle`: Body to pull bounding box data about.
    /// * `min`: Minimum bounds of the collidable's bounding box.
    /// * `max`: Maximum bounds of the collidable's bounding box.
    #[link_name = "Simulation.GetBodyBoundingBoxInBroadPhase"]
    pub fn get_body_bounding_box_in_broad_phase(
        simulation_handle: SimulationHandle,
        body_handle: BodyHandle,
        min: *mut crate::types::math::scalar::Vector3,
        max: *mut Vector3,
    );
    /// Grabs a collidable's bounding boxes in the broad phase.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to pull data from.
    /// * `static_handle`: Static to pull bounding box data about.
    /// * `min`: Minimum bounds of the collidable's bounding box.
    /// * `max`: Maximum bounds of the collidable's bounding box.
    #[link_name = "Simulation.GetStaticBoundingBoxInBroadPhase"]
    pub fn get_static_bounding_box_in_broad_phase(
        simulation_handle: SimulationHandle,
        static_handle: StaticHandle,
        min: *mut Vector3,
        max: *mut Vector3,
    );
    /// Gets the mapping from body handles to the body's location in storage.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to pull data from.
    /// * `body_handle_to_index_mapping`: Mapping from a body handle to the body's memory location.
    ///
    /// # Remarks
    ///
    /// The buffer returned by this function can be invalidated if the simulation resizes it.
    #[link_name = "Simulation.GetBodyHandleToLocationMappings"]
    pub fn get_body_handle_to_location_mapping(
        simulation_handle: SimulationHandle,
        body_handle_to_index_mapping: *mut Buffer<BodyMemoryLocation>,
    );
    /// Gets the body sets for a simulation. Slot 0 is the active set. Subsequent sets are sleeping. Not every slot beyond slot 0 is filled.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to pull data from.
    /// * `body_sets`: Mapping from a body handle to the body's memory location.
    ///
    /// # Remarks
    ///
    /// The buffer returned by this function can be invalidated if the simulation resizes it.
    #[link_name = "Simulation.GetBodySets"]
    pub fn get_body_sets(simulation_handle: SimulationHandle, body_sets: *mut Buffer<BodySet>);
    /// Gets the mapping from body handles to the body's location in storage.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to pull data from.
    /// * `static_handle_to_index_mapping`: Mapping from a static handle to the static's memory location.
    ///
    /// # Remarks
    ///
    /// The buffer returned by this function can be invalidated if the simulation resizes it.
    #[link_name = "Simulation.GetStaticHandleToLocationMapping"]
    pub fn get_static_handle_to_location_mapping(
        simulation_handle: SimulationHandle,
        static_handle_to_index_mapping: *mut Buffer<i32>,
    );
    /// Gets the statics set for a simulation.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to pull data from.
    /// * `statics`: The set of all statics within a simulation.
    /// * `count`: Number of statics in the simulation.
    ///
    /// # Remarks
    ///
    /// The buffer returned by this function can be invalidated if the simulation resizes it. The count is a snapshot.
    #[link_name = "Simulation.GetStatics"]
    pub fn get_statics(
        simulation_handle: SimulationHandle,
        statics: *mut Buffer<Static>,
        count: *mut i32,
    );
}
