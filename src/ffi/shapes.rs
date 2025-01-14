use crate::types::{
    body::BodyInertia,
    handles::{BufferPoolHandle, SimulationHandle, TypedIndex},
    math::scalar::Vector3,
    shapes::*,
    utilities::Buffer,
};

extern "C" {
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
