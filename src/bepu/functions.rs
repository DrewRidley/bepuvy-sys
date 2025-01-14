use super::{
    bodies::*, collidable_property::*, collisions::*, handles::*, interop_math::*,
    pose_integration::*, shapes::*, statics::*, utilities::*, *,
};

#[cfg(target_feature = "avx512f")]
pub const WIDEST_LANE: usize = 16;

#[cfg(all(target_feature = "sse2", not(target_feature = "avx512f")))]
pub const WIDEST_LANE: usize = 8;

#[cfg(not(any(target_feature = "avx512f", target_feature = "sse2")))]
pub const WIDEST_LANE: usize = 4; // Fallback for systems without AVX512F or SSE2

extern "C" {
    #[link_name = "Utils.GetSIMDWidth"]
    pub fn get_simd_width() -> SIMDWidth;
    /// Gets the number of threads exposed by the operating system on this platform. Cores with SMT can show as having multiple threads.
    ///
    /// # Returns
    ///
    /// Number of threads exposed by the operating system on this platform.
    #[link_name = "Utils.GetPlatformThreadCount"]
    pub fn get_platform_thread_count() -> i32;
    /// Creates a new buffer pool.
    ///
    /// # Arguments
    ///
    /// * `minimum_block_allocation_size`: Minimum size of individual block allocations. Must be a power of 2.
    /// Pools with single allocations larger than the minimum will use the minimum value necessary to hold one element.
    /// Buffers will be suballocated from blocks.
    /// * `expected_used_slot_count_per_pool`: Number of suballocations to preallocate reference space for.
    /// This does not preallocate actual blocks, just the space to hold references that are waiting in the pool.
    #[link_name = "BufferPool.Create"]
    pub fn create_buffer_pool(
        minimum_block_allocation_size: i32,
        expected_used_slot_count_per_pool: i32,
    ) -> BufferPoolHandle;
    /// Releases all allocations held by the buffer pool. The buffer pool remains in a usable state.
    ///
    /// # Arguments
    ///
    /// * `handle`: Buffer pool to clear.
    #[link_name = "BufferPool.Clear"]
    pub fn clear_buffer_pool(handle: BufferPoolHandle);
    /// Releases all allocations held by the buffer pool and releases the buffer pool reference. The handle is invalidated.
    ///
    /// # Arguments
    ///
    /// * `handle`: Buffer pool to destroy.
    #[link_name = "BufferPool.Destroy"]
    pub fn destroy_buffer_pool(handle: BufferPoolHandle);
    /// Allocates a buffer from the buffer pool of the given size.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to allocate from.
    /// * `size_in_bytes`: Size of the buffer to allocate in bytes.
    ///
    /// # Returns
    ///
    /// Allocated buffer.
    #[link_name = "BufferPool.Allocate"]
    pub fn allocate(buffer_pool_handle: BufferPoolHandle, size_in_bytes: i32) -> ByteBuffer;
    /// Allocates a buffer from the buffer pool with at least the given size.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to allocate from.
    /// * `size_in_bytes`: Size of the buffer to allocate in bytes.
    ///
    /// # Returns
    ///
    /// Allocated buffer.
    #[link_name = "BufferPool.AllocateAtLeast"]
    pub fn allocate_at_least(
        buffer_pool_handle: BufferPoolHandle,
        size_in_bytes: i32,
    ) -> ByteBuffer;
    /// Resizes a buffer from the buffer pool to the given size, reallocating if necessary.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to allocate from.
    /// * `buffer`: Buffer to resize.
    /// * `new_size_in_bytes`: Target size of the buffer to allocate in bytes.
    /// * `copy_count`: Number of bytes to copy from the old buffer into the new buffer.
    #[link_name = "BufferPool.Resize"]
    pub fn resize(
        buffer_pool_handle: BufferPoolHandle,
        buffer: *mut ByteBuffer,
        new_size_in_bytes: i32,
        copy_count: i32,
    );
    /// Resizes a buffer from the buffer pool to at least the given size, reallocating if necessary.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to allocate from.
    /// * `buffer`: Buffer to resize.
    /// * `target_size_in_bytes`: Target size of the buffer to allocate in bytes.
    /// * `copy_count`: Number of bytes to copy from the old buffer into the new buffer.
    #[link_name = "BufferPool.ResizeToAtLeast"]
    pub fn resize_to_at_least(
        buffer_pool_handle: BufferPoolHandle,
        buffer: *mut ByteBuffer,
        target_size_in_bytes: i32,
        copy_count: i32,
    );
    /// Returns a buffer to the buffer pool.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to return the buffer to.
    /// * `buffer`: Buffer to return to the pool.
    #[link_name = "BufferPool.Deallocate"]
    pub fn deallocate(buffer_pool_handle: BufferPoolHandle, buffer: *mut ByteBuffer);
    /// Returns a buffer to the buffer pool by its id.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to return the buffer to.
    /// * `buffer_id`: Id of the buffer to return to the pool.
    #[link_name = "BufferPool.DeallocateId"]
    pub fn deallocate_by_id(buffer_pool_handle: BufferPoolHandle, buffer_id: i32);
    /// Creates a new thread dispatcher.
    ///
    /// # Arguments
    ///
    /// * `thread_count`: Number of threads to use within the thread dispatcher.
    /// * `thread_pool_allocation_block_size`: Minimum size in bytes of blocks allocated in per-thread buffer pools. Allocations requiring more space can result in larger block sizes, but no pools will allocate smaller blocks.
    #[link_name = "ThreadDispatcher.Create"]
    pub fn create_thread_dispatcher(
        thread_count: i32,
        thread_pool_allocation_block_size: i32,
    ) -> ThreadDispatcherHandle;
    /// Releases all resources held by a thread dispatcher and invalidates its handle.
    ///
    /// # Arguments
    ///
    /// * `handle`: Thread dispatcher to destroy.
    #[link_name = "ThreadDispatcher.Destroy"]
    pub fn destroy_thread_dispatcher(handle: ThreadDispatcherHandle);
    /// Releases all resources held by a thread dispatcher and invalidates its handle.
    ///
    /// # Arguments
    ///
    /// * `handle`: Thread dispatcher to check the thread count of.
    #[link_name = "ThreadDispatcher.GetThreadCount"]
    pub fn get_thread_count(handle: ThreadDispatcherHandle) -> i32;

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
        pose_integrator_callbacks: PoseIntegratorCallbacks<WIDEST_LANE>,
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
        min: *mut Vector3,
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
    /// Computes the total number of bytes allocated from native memory in this buffer pool.
    /// Includes allocated memory regardless of whether it currently has outstanding references.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to check the allocation size of.
    ///
    /// # Returns
    ///
    /// Total number of bytes allocated from native memory in this buffer pool.
    #[link_name = "BufferPool.GetAllocatedMemorySizeInPool"]
    pub fn get_allocated_memory_size_in_pool(buffer_pool_handle: BufferPoolHandle) -> u64;
    /// Computes the total number of bytes allocated from native memory in a dispatcher's per-thread pools.
    /// Includes allocated memory regardless of whether it currently has outstanding references.
    ///
    /// # Arguments
    ///
    /// * `thread_dispatcher_handle`: Thread dispatcher to check allocations for.
    ///
    /// # Returns
    ///
    /// Total number of bytes allocated from native memory in this thread dispatcher's per-thread pool.
    #[link_name = "ThreadDispatcher.GetAllocatedMemorySize"]
    pub fn get_allocated_memory_size_in_thread_dispatcher(
        thread_dispatcher_handle: ThreadDispatcherHandle,
    ) -> u64;
    /// Estimates the number of bytes managed by the garbage collector.
    ///
    /// # Returns
    ///
    /// Estimated number of bytes allocated from managed memory.
    #[link_name = "Utils.GetGCMemorySize"]
    pub fn get_gc_allocated_memory_size() -> u64;
    /// Adds a sphere shape to the simulation.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to add the shape to.
    /// * `sphere`: Shape to add to the simulation.
    #[link_name = "Simulation.Shapes.AddSphere"]
    pub fn add_sphere(simulation_handle: SimulationHandle, sphere: Sphere) -> TypedIndex;
    /// Adds a capsule shape to the simulation.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to add the shape to.
    /// * `capsule`: Shape to add to the simulation.
    #[link_name = "Simulation.Shapes.AddCapsule"]
    pub fn add_capsule(simulation_handle: SimulationHandle, capsule: Capsule) -> TypedIndex;
    /// Adds a box shape to the simulation.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to add the shape to.
    /// * `box`: Shape to add to the simulation.
    #[link_name = "Simulation.Shapes.AddBox"]
    pub fn add_box(simulation_handle: SimulationHandle, box_: Box) -> TypedIndex;
    /// Adds a triangle shape to the simulation.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to add the shape to.
    /// * `triangle`: Shape to add to the simulation.
    #[link_name = "Simulation.Shapes.AddTriangle"]
    pub fn add_triangle(simulation_handle: SimulationHandle, triangle: Triangle) -> TypedIndex;
    /// Adds a cylinder shape to the simulation.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to add the shape to.
    /// * `cylinder`: Shape to add to the simulation.
    #[link_name = "Simulation.Shapes.AddCylinder"]
    pub fn add_cylinder(simulation_handle: SimulationHandle, cylinder: Cylinder) -> TypedIndex;
    /// Adds a convex hull shape to the simulation.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to add the shape to.
    /// * `convex_hull`: Shape to add to the simulation.
    #[link_name = "Simulation.Shapes.AddConvexHull"]
    pub fn add_convex_hull(
        simulation_handle: SimulationHandle,
        convex_hull: ConvexHull,
    ) -> TypedIndex;
    /// Adds a compound shape to the simulation.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to add the shape to.
    /// * `big_compound`: Shape to add to the simulation.
    #[link_name = "Simulation.Shapes.AddCompound"]
    pub fn add_compound(simulation_handle: SimulationHandle, big_compound: Compound) -> TypedIndex;
    /// Adds a big compound shape to the simulation.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to add the shape to.
    /// * `big_compound`: Shape to add to the simulation.
    #[link_name = "Simulation.Shapes.AddBigCompound"]
    pub fn add_big_compound(
        simulation_handle: SimulationHandle,
        big_compound: BigCompound,
    ) -> TypedIndex;
    /// Adds a mesh shape to the simulation.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to add the shape to.
    /// * `mesh`: Shape to add to the simulation.
    #[link_name = "Simulation.Shapes.AddMesh"]
    pub fn add_mesh(simulation_handle: SimulationHandle, mesh: Mesh) -> TypedIndex;
    /// Removes a shape from the simulation. Does not return any shape allocated buffers to buffer pools.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to remove the shape from.
    /// * `shape`: Shape to remove from the simulation.
    #[link_name = "Simulation.Shapes.RemoveShape"]
    pub fn remove_shape(simulation_handle: SimulationHandle, shape: TypedIndex);
    /// Removes a shape from the simulation. If the shape has resources that were allocated from a buffer pool, they will be returned to the specified pool.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to remove the shape from.
    /// * `buffer_pool_handle`: Buffer pool to return shape resources to, if any.
    /// * `shape`: Shape to remove from the simulation.
    ///
    /// # Remarks
    ///
    /// The same buffer pool must be used for both allocation and deallocation.
    #[link_name = "Simulation.Shapes.RemoveAndDestroyShape"]
    pub fn remove_and_destroy_shape(
        simulation_handle: SimulationHandle,
        buffer_pool_handle: BufferPoolHandle,
        shape: TypedIndex,
    );
    /// Removes a shape and all references child shapes from the simulation. If the shapes had resources that were allocated from a buffer pool, they will be returned to the specified pool.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to remove the shape from.
    /// * `buffer_pool_handle`: Buffer pool to return shape resources to, if any.
    /// * `shape`: Shape to remove from the simulation.
    ///
    /// # Remarks
    ///
    /// The same buffer pool must be used for both allocation and deallocation.
    #[link_name = "Simulation.Shapes.RemoveAndDestroyShapeRecursively"]
    pub fn remove_and_destroy_shape_recursively(
        simulation_handle: SimulationHandle,
        buffer_pool_handle: BufferPoolHandle,
        shape: TypedIndex,
    );
    /// Creates a convex hull shape from a point set.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to allocate resources from for the compound's acceleration structures.
    /// * `points`: Points in the convex hull.
    /// * `center_of_mass`: Center of mass computed for the hull and subtracted from all the points in the points used for the final shape.
    #[link_name = "Simulation.Shapes.CreateConvexHull"]
    pub fn create_convex_hull(
        buffer_pool_handle: BufferPoolHandle,
        points: Buffer<Vector3>,
        center_of_mass: *mut Vector3,
    ) -> ConvexHull;
    /// Returns buffers allocated for a convex hull shape.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to return resources to. Must be the same pool that resources were allocated from.
    /// * `convex_hull`: Convex hull to destroy.
    #[link_name = "Simulation.Shapes.DestroyConvexHull"]
    pub fn destroy_convex_hull(buffer_pool_handle: BufferPoolHandle, convex_hull: *mut ConvexHull);
    /// Returns buffers allocated for a compound shape.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to return resources to. Must be the same pool that resources were allocated from.
    /// * `compound`: Compound to destroy.
    #[link_name = "Simulation.Shapes.DestroyCompound"]
    pub fn destroy_compound(buffer_pool_handle: BufferPoolHandle, compound: *mut Compound);
    /// Creates a big compound shape from a list of children.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to which the shapes referenced by the compound children belong.
    /// * `buffer_pool_handle`: Buffer pool to allocate resources from for the compound's acceleration structures.
    /// * `children`: Children of the compound.
    #[link_name = "Simulation.Shapes.CreateBigCompound"]
    pub fn create_big_compound(
        simulation_handle: SimulationHandle,
        buffer_pool_handle: BufferPoolHandle,
        children: Buffer<CompoundChild>,
    ) -> BigCompound;
    /// Returns buffers allocated for a big compound shape.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to return resources to. Must be the same pool that resources were allocated from.
    /// * `big_compound`: Big compound to destroy.
    #[link_name = "Simulation.Shapes.DestroyBigCompound"]
    pub fn destroy_big_compound(
        buffer_pool_handle: BufferPoolHandle,
        big_compound: *mut BigCompound,
    );
    /// Creates a mesh shape from triangles.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to allocate resources from for the compound's acceleration structures.
    /// * `triangles`: Triangles composing the mesh.
    /// * `scale`: Scale of the mesh.
    ///
    /// # Remarks
    ///
    /// This uses a pretty old sweep builder. Large meshes will take a while. There are ways to do this much faster if required; see https://github.com/bepu/bepuphysics2/blob/master/Demos/DemoMeshHelper.cs#L186.
    #[link_name = "Mesh.Create"]
    pub fn create_mesh(
        buffer_pool_handle: BufferPoolHandle,
        triangles: Buffer<Triangle>,
        scale: Vector3,
    ) -> Mesh;
    /// Returns buffers allocated for a mesh shape.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to return resources to. Must be the same pool that resources were allocated from.
    /// * `mesh`: Mesh to destroy.
    #[link_name = "Mesh.Destroy"]
    pub fn destroy_mesh(buffer_pool_handle: BufferPoolHandle, mesh: *mut Mesh);
    /// Computes the inertia of a sphere.
    ///
    /// # Arguments
    ///
    /// * `sphere`: Shape to compute the inertia of.
    /// * `mass`: Mass to use in the inertia calculation.
    ///
    /// # Returns
    ///
    /// Inertia of the shape.
    #[link_name = "Sphere.ComputeInertia"]
    pub fn compute_sphere_inertia(sphere: Sphere, mass: f32) -> BodyInertia;
    /// Computes the inertia of a capsule.
    ///
    /// # Arguments
    ///
    /// * `capsule`: Shape to compute the inertia of.
    /// * `mass`: Mass to use in the inertia calculation.
    ///
    /// # Returns
    ///
    /// Inertia of the shape.
    #[link_name = "Capsule.ComputeInertia"]
    pub fn compute_capsule_inertia(capsule: Capsule, mass: f32) -> BodyInertia;
    /// Computes the inertia of a box.
    ///
    /// # Arguments
    ///
    /// * `box`: Shape to compute the inertia of.
    /// * `mass`: Mass to use in the inertia calculation.
    ///
    /// # Returns
    ///
    /// Inertia of the shape.
    #[link_name = "Box.ComputeInertia"]
    pub fn compute_box_inertia(box_: Box, mass: f32) -> BodyInertia;
    /// Computes the inertia of a triangle.
    ///
    /// # Arguments
    ///
    /// * `triangle`: Shape to compute the inertia of.
    /// * `mass`: Mass to use in the inertia calculation.
    ///
    /// # Returns
    ///
    /// Inertia of the shape.
    #[link_name = "Triangle.ComputeInertia"]
    pub fn compute_triangle_inertia(triangle: Triangle, mass: f32) -> BodyInertia;
    /// Computes the inertia of a cylinder.
    ///
    /// # Arguments
    ///
    /// * `cylinder`: Shape to compute the inertia of.
    /// * `mass`: Mass to use in the inertia calculation.
    ///
    /// # Returns
    ///
    /// Inertia of the shape.
    #[link_name = "Cylinder.ComputeInertia"]
    pub fn compute_cylinder_inertia(cylinder: Cylinder, mass: f32) -> BodyInertia;
    /// Computes the inertia of a convex hull.
    ///
    /// # Arguments
    ///
    /// * `convex_hull`: Shape to compute the inertia of.
    /// * `mass`: Mass to use in the inertia calculation.
    ///
    /// # Returns
    ///
    /// Inertia of the shape.
    #[link_name = "ConvexHull.ComputeInertia"]
    pub fn compute_convex_hull_inertia(convex_hull: ConvexHull, mass: f32) -> BodyInertia;
    /// Computes the inertia of a convex.
    ///
    /// # Arguments
    ///
    /// * `convex`: Index of a convex to calculate the inertia for.
    /// * `mass`: Mass to use in the inertia calculation.
    ///
    /// # Returns
    ///
    /// Inertia of the shape. If the shape index was not a convex, this returns a zeroed inverse inertia tensor.
    #[link_name = "Simulation.Shapes.ComputeConvexInertia"]
    pub fn compute_convex_inertia(
        simulation_handle: SimulationHandle,
        convex: TypedIndex,
        mass: f32,
    ) -> BodyInertia;
    /// Computes the inertia associated with a set of compound children. Does not recenter the children.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to which the shapes referenced by the compound children belong.
    /// * `children`: Children of the compound.
    /// * `child_masses`: Masses of the children composing the compound.
    #[link_name = "Simulation.Shapes.ComputeCompoundInertia"]
    pub fn compute_compound_inertia(
        simulation_handle: SimulationHandle,
        children: Buffer<CompoundChild>,
        child_masses: Buffer<f32>,
    ) -> BodyInertia;
    /// Computes the inertia associated with a set of compound children. Recenters all children onto the computed local center of mass.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to which the shapes referenced by the compound children belong.
    /// * `children`: Children of the compound.
    /// * `child_masses`: Masses of the children composing the compound.
    /// * `center_of_mass`: Computed center of mass that was subtracted from the position of compound children.
    #[link_name = "Simulation.Shapes.ComputeCompoundInertiaWithRecentering"]
    pub fn compute_compound_inertia_with_recentering(
        simulation_handle: SimulationHandle,
        children: Buffer<CompoundChild>,
        child_masses: Buffer<f32>,
        center_of_mass: *mut Vector3,
    ) -> BodyInertia;
    /// Computes the inertia associated with a mesh by treating its triangles as a soup with no volume. Does not recenter the triangles on a computed center of mass.
    ///
    /// # Arguments
    ///
    /// * `mesh`: Mesh to compute the inertia of.
    /// * `mass`: Mass of the mesh.
    #[link_name = "Mesh.ComputeOpenInertia"]
    pub fn compute_open_mesh_inertia(mesh: Mesh, mass: f32) -> BodyInertia;
    /// Computes the inertia associated with a mesh by treating it as a closed volume. Does not recenter the triangles on a computed center of mass.
    ///
    /// # Arguments
    ///
    /// * `mesh`: Mesh to compute the inertia of.
    /// * `mass`: Mass of the mesh.
    #[link_name = "Mesh.ComputeClosedInertia"]
    pub fn compute_closed_mesh_inertia(mesh: Mesh, mass: f32) -> BodyInertia;
    /// Computes the inertia associated with a mesh by treating its triangles as a soup with no volume. Recenters all children onto the computed local center of mass.
    ///
    /// # Arguments
    ///
    /// * `mesh`: Mesh to compute the inertia of.
    /// * `mass`: Mass of the mesh.
    #[link_name = "Mesh.ComputeOpenInertiaWithRecentering"]
    pub fn compute_open_mesh_inertia_with_recentering(
        mesh: Mesh,
        mass: f32,
        center_of_mass: *mut Vector3,
    ) -> BodyInertia;
    /// Computes the inertia associated with a mesh by treating it as a closed volume. Recenters all children onto the computed local center of mass.
    ///
    /// # Arguments
    ///
    /// * `mesh`: Mesh to compute the inertia of.
    /// * `mass`: Mass of the mesh.
    #[link_name = "Mesh.ComputeClosedInertiaWithRecentering"]
    pub fn compute_closed_mesh_inertia_with_recentering(
        mesh: Mesh,
        mass: f32,
        center_of_mass: *mut Vector3,
    ) -> BodyInertia;
    /// Gets a pointer to a sphere shape's data stored within the simulation's shapes buffers.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to remove the shape from.
    /// * `shape`: Shape reference to request from the simulation.
    ///
    /// # Returns
    ///
    /// Pointer to the shape's data in the simulation's shapes buffers.
    #[link_name = "Simulation.Shapes.GetSphereData"]
    pub fn get_sphere_shape_data(
        simulation_handle: SimulationHandle,
        shape: TypedIndex,
    ) -> *mut Sphere;
    /// Gets a pointer to a capsule shape's data stored within the simulation's shapes buffers.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to remove the shape from.
    /// * `shape`: Shape reference to request from the simulation.
    ///
    /// # Returns
    ///
    /// Pointer to the shape's data in the simulation's shapes buffers.
    #[link_name = "Simulation.Shapes.GetCapsuleData"]
    pub fn get_capsule_shape_data(
        simulation_handle: SimulationHandle,
        shape: TypedIndex,
    ) -> *mut Capsule;
    /// Gets a pointer to a box shape's data stored within the simulation's shapes buffers.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to remove the shape from.
    /// * `shape`: Shape reference to request from the simulation.
    ///
    /// # Returns
    ///
    /// Pointer to the shape's data in the simulation's shapes buffers.
    #[link_name = "Simulation.Shapes.GetBoxData"]
    pub fn get_box_shape_data(simulation_handle: SimulationHandle, shape: TypedIndex) -> *mut Box;
    /// Gets a pointer to a triangle shape's data stored within the simulation's shapes buffers.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to remove the shape from.
    /// * `shape`: Shape reference to request from the simulation.
    ///
    /// # Returns
    ///
    /// Pointer to the shape's data in the simulation's shapes buffers.
    #[link_name = "Simulation.Shapes.GetTriangleData"]
    pub fn get_triangle_shape_data(
        simulation_handle: SimulationHandle,
        shape: TypedIndex,
    ) -> *mut Triangle;
    /// Gets a pointer to a cylinder shape's data stored within the simulation's shapes buffers.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to remove the shape from.
    /// * `shape`: Shape reference to request from the simulation.
    ///
    /// # Returns
    ///
    /// Pointer to the shape's data in the simulation's shapes buffers.
    #[link_name = "Simulation.Shapes.GetCylinderData"]
    pub fn get_cylinder_shape_data(
        simulation_handle: SimulationHandle,
        shape: TypedIndex,
    ) -> *mut Cylinder;
    /// Gets a pointer to a convex hull shape's data stored within the simulation's shapes buffers.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to remove the shape from.
    /// * `shape`: Shape reference to request from the simulation.
    ///
    /// # Returns
    ///
    /// Pointer to the shape's data in the simulation's shapes buffers.
    #[link_name = "Simulation.Shapes.GetConvexHullData"]
    pub fn get_convex_hull_shape_data(
        simulation_handle: SimulationHandle,
        shape: TypedIndex,
    ) -> *mut ConvexHull;
    /// Gets a pointer to a compound shape's data stored within the simulation's shapes buffers.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to remove the shape from.
    /// * `shape`: Shape reference to request from the simulation.
    ///
    /// # Returns
    ///
    /// Pointer to the shape's data in the simulation's shapes buffers.
    #[link_name = "Simulation.Shapes.GetCompoundData"]
    pub fn get_compound_shape_data(
        simulation_handle: SimulationHandle,
        shape: TypedIndex,
    ) -> *mut Compound;
    /// Gets a pointer to a big compound shape's data stored within the simulation's shapes buffers.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to remove the shape from.
    /// * `shape`: Shape reference to request from the simulation.
    ///
    /// # Returns
    ///
    /// Pointer to the shape's data in the simulation's shapes buffers.
    #[link_name = "Simulation.Shapes.GetBigCompoundData"]
    pub fn get_big_compound_shape_data(
        simulation_handle: SimulationHandle,
        shape: TypedIndex,
    ) -> *mut BigCompound;
    /// Gets a pointer to a mesh shape's data stored within the simulation's shapes buffers.
    ///
    /// # Arguments
    ///
    /// * `simulation_handle`: Handle of the simulation to remove the shape from.
    /// * `shape`: Shape reference to request from the simulation.
    ///
    /// # Returns
    ///
    /// Pointer to the shape's data in the simulation's shapes buffers.
    #[link_name = "Simulation.Shapes.GetMeshData"]
    pub fn get_mesh_shape_data(simulation_handle: SimulationHandle, shape: TypedIndex)
        -> *mut Mesh;
}
