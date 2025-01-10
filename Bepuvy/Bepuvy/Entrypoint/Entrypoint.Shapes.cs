
using BepuPhysics;
using BepuPhysics.Collidables;
using BepuUtilities.Memory;
using System.Numerics;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace Bepuvy;

public static partial class Entrypoints
{
    /// <summary>
    /// Adds a sphere shape to the simulation.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to add the shape to.</param>
    /// <param name="sphere">Shape to add to the simulation.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.AddSphere")]
    public static TypedIndex AddSphere([TypeName("SimulationHandle")] InstanceHandle simulationHandle, Sphere sphere)
    {
        return simulations[simulationHandle].Shapes.Add(sphere);
    }

    /// <summary>
    /// Adds a capsule shape to the simulation.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to add the shape to.</param>
    /// <param name="capsule">Shape to add to the simulation.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.AddCapsule")]
    public static TypedIndex AddCapsule([TypeName("SimulationHandle")] InstanceHandle simulationHandle, Capsule capsule)
    {
        return simulations[simulationHandle].Shapes.Add(capsule);
    }

    /// <summary>
    /// Adds a box shape to the simulation.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to add the shape to.</param>
    /// <param name="box">Shape to add to the simulation.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.AddBox")]
    public static TypedIndex AddBox([TypeName("SimulationHandle")] InstanceHandle simulationHandle, Box box)
    {
        return simulations[simulationHandle].Shapes.Add(box);
    }

    /// <summary>
    /// Adds a triangle shape to the simulation.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to add the shape to.</param>
    /// <param name="triangle">Shape to add to the simulation.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.AddTriangle")]
    public static TypedIndex AddTriangle([TypeName("SimulationHandle")] InstanceHandle simulationHandle, Triangle triangle)
    {
        return simulations[simulationHandle].Shapes.Add(triangle);
    }

    /// <summary>
    /// Adds a cylinder shape to the simulation.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to add the shape to.</param>
    /// <param name="cylinder">Shape to add to the simulation.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.AddCylinder")]
    public static TypedIndex AddCylinder([TypeName("SimulationHandle")] InstanceHandle simulationHandle, Cylinder cylinder)
    {
        return simulations[simulationHandle].Shapes.Add(cylinder);
    }

    /// <summary>
    /// Adds a convex hull shape to the simulation.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to add the shape to.</param>
    /// <param name="convexHull">Shape to add to the simulation.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.AddConvexHull")]
    public static TypedIndex AddConvexHull([TypeName("SimulationHandle")] InstanceHandle simulationHandle, ConvexHull convexHull)
    {
        return simulations[simulationHandle].Shapes.Add(convexHull);
    }

    /// <summary>
    /// Adds a compound shape to the simulation.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to add the shape to.</param>
    /// <param name="bigCompound">Shape to add to the simulation.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.AddCompound")]
    public static TypedIndex AddCompound([TypeName("SimulationHandle")] InstanceHandle simulationHandle, Compound bigCompound)
    {
        return simulations[simulationHandle].Shapes.Add(bigCompound);
    }

    /// <summary>
    /// Adds a big compound shape to the simulation.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to add the shape to.</param>
    /// <param name="bigCompound">Shape to add to the simulation.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.AddBigCompound")]
    public static TypedIndex AddBigCompound([TypeName("SimulationHandle")] InstanceHandle simulationHandle, BigCompound bigCompound)
    {
        return simulations[simulationHandle].Shapes.Add(bigCompound);
    }

    /// <summary>
    /// Adds a mesh shape to the simulation.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to add the shape to.</param>
    /// <param name="mesh">Shape to add to the simulation.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.AddMesh")]
    public static TypedIndex AddMesh([TypeName("SimulationHandle")] InstanceHandle simulationHandle, Mesh mesh)
    {
        return simulations[simulationHandle].Shapes.Add(mesh);
    }

    /// <summary>
    /// Removes a shape from the simulation. Does not return any shape allocated buffers to buffer pools.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to remove the shape from.</param>
    /// <param name="shape">Shape to remove from the simulation.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.RemoveShape")]
    public static void RemoveShape([TypeName("SimulationHandle")] InstanceHandle simulationHandle, TypedIndex shape)
    {
        simulations[simulationHandle].Shapes.Remove(shape);
    }

    /// <summary>
    /// Removes a shape from the simulation. If the shape has resources that were allocated from a buffer pool, they will be returned to the specified pool.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to remove the shape from.</param>
    /// <param name="bufferPoolHandle">Buffer pool to return shape resources to, if any.</param>
    /// <param name="shape">Shape to remove from the simulation.</param>
    /// <remarks>The same buffer pool must be used for both allocation and deallocation.</remarks>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.RemoveAndDestroyShape")]
    public static void RemoveAndDestroyShape([TypeName("SimulationHandle")] InstanceHandle simulationHandle, [TypeName("BufferPoolHandle")] InstanceHandle bufferPoolHandle, TypedIndex shape)
    {
        simulations[simulationHandle].Shapes.RemoveAndDispose(shape, bufferPools[bufferPoolHandle]);
    }

    /// <summary>
    /// Removes a shape and all references child shapes from the simulation. If the shapes had resources that were allocated from a buffer pool, they will be returned to the specified pool.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to remove the shape from.</param>
    /// <param name="bufferPoolHandle">Buffer pool to return shape resources to, if any.</param>
    /// <param name="shape">Shape to remove from the simulation.</param>
    /// <remarks>The same buffer pool must be used for both allocation and deallocation.</remarks>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.RemoveAndDestroyShapeRecursively")]
    public static void RemoveAndDestroyShapeRecursively([TypeName("SimulationHandle")] InstanceHandle simulationHandle, [TypeName("BufferPoolHandle")] InstanceHandle bufferPoolHandle, TypedIndex shape)
    {
        simulations[simulationHandle].Shapes.RecursivelyRemoveAndDispose(shape, bufferPools[bufferPoolHandle]);
    }

    /// <summary>
    /// Creates a convex hull shape from a point set.
    /// </summary>
    /// <param name="bufferPoolHandle">Buffer pool to allocate resources from for the compound's acceleration structures.</param>
    /// <param name="points">Points in the convex hull.</param>
    /// <param name="centerOfMass">Center of mass computed for the hull and subtracted from all the points in the points used for the final shape.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.CreateConvexHull")]
    public unsafe static ConvexHull CreateConvexHull([TypeName("SimulationHandle")] InstanceHandle bufferPoolHandle, [TypeName("Buffer<Vector3>")] Buffer<Vector3> points, Vector3* centerOfMass)
    {
        ConvexHullHelper.CreateShape(points, bufferPools[bufferPoolHandle], out *centerOfMass, out var hull);
        return hull;
    }

    /// <summary>
    /// Returns buffers allocated for a convex hull shape.
    /// </summary>
    /// <param name="bufferPoolHandle">Buffer pool to return resources to. Must be the same pool that resources were allocated from.</param>
    /// <param name="convexHull">Convex hull to destroy.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.DestroyConvexHull")]
    public unsafe static void DestroyConvexHull([TypeName("SimulationHandle")] InstanceHandle bufferPoolHandle, ConvexHull* convexHull)
    {
        convexHull->Dispose(bufferPools[bufferPoolHandle]);
    }

    /// <summary>
    /// Returns buffers allocated for a compound shape.
    /// </summary>
    /// <param name="bufferPoolHandle">Buffer pool to return resources to. Must be the same pool that resources were allocated from.</param>
    /// <param name="compound">Compound to destroy.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.DestroyCompound")]
    public unsafe static void DestroyCompound([TypeName("SimulationHandle")] InstanceHandle bufferPoolHandle, Compound* compound)
    {
        compound->Dispose(bufferPools[bufferPoolHandle]);
    }

    /// <summary>
    /// Creates a big compound shape from a list of children.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to which the shapes referenced by the compound children belong.</param>
    /// <param name="bufferPoolHandle">Buffer pool to allocate resources from for the compound's acceleration structures.</param>
    /// <param name="children">Children of the compound.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.CreateBigCompound")]
    public static BigCompound CreateBigCompound([TypeName("SimulationHandle")] InstanceHandle simulationHandle, [TypeName("BufferPool")] InstanceHandle bufferPoolHandle, [TypeName("Buffer<CompoundChild>")] Buffer<CompoundChild> children)
    {
        return new BigCompound(children, simulations[simulationHandle].Shapes, bufferPools[bufferPoolHandle]);
    }

    /// <summary>
    /// Returns buffers allocated for a big compound shape.
    /// </summary>
    /// <param name="bufferPoolHandle">Buffer pool to return resources to. Must be the same pool that resources were allocated from.</param>
    /// <param name="bigCompound">Big compound to destroy.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.DestroyBigCompound")]
    public static unsafe void DestroyBigCompound([TypeName("SimulationHandle")] InstanceHandle bufferPoolHandle, BigCompound* bigCompound)
    {
        bigCompound->Dispose(bufferPools[bufferPoolHandle]);
    }

    /// <summary>
    /// Creates a mesh shape from triangles.
    /// </summary>
    /// <param name="bufferPoolHandle">Buffer pool to allocate resources from for the compound's acceleration structures.</param>
    /// <param name="triangles">Triangles composing the mesh.</param>
    /// <param name="scale">Scale of the mesh.</param>
    /// <remarks>This uses a pretty old sweep builder. Large meshes will take a while. There are ways to do this much faster if required; see https://github.com/bepu/bepuphysics2/blob/master/Demos/DemoMeshHelper.cs#L186.</remarks>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Mesh.Create")]
    public static Mesh CreateMesh([TypeName("BufferPoolHandle")] InstanceHandle bufferPoolHandle, [TypeName("Buffer<Triangle>")] Buffer<Triangle> triangles, Vector3 scale)
    {
        return new Mesh(triangles, scale, bufferPools[bufferPoolHandle]);
    }

    /// <summary>
    /// Returns buffers allocated for a mesh shape.
    /// </summary>
    /// <param name="bufferPoolHandle">Buffer pool to return resources to. Must be the same pool that resources were allocated from.</param>
    /// <param name="mesh">Mesh to destroy.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Mesh.Destroy")]
    public unsafe static void DestroyMesh([TypeName("BufferPoolHandle")] InstanceHandle bufferPoolHandle, Mesh* mesh)
    {
        mesh->Dispose(bufferPools[bufferPoolHandle]);
    }

    /// <summary>
    /// Computes the inertia of a sphere.
    /// </summary>
    /// <param name="sphere">Shape to compute the inertia of.</param>
    /// <param name="mass">Mass to use in the inertia calculation.</param>
    /// <returns>Inertia of the shape.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Sphere.ComputeInertia")]
    public static BodyInertia ComputeSphereInertia(Sphere sphere, float mass)
    {
        return sphere.ComputeInertia(mass);
    }

    /// <summary>
    /// Computes the inertia of a capsule.
    /// </summary>
    /// <param name="capsule">Shape to compute the inertia of.</param>
    /// <param name="mass">Mass to use in the inertia calculation.</param>
    /// <returns>Inertia of the shape.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Capsule.ComputeInertia")]
    public static BodyInertia ComputeCapsuleInertia(Capsule capsule, float mass)
    {
        return capsule.ComputeInertia(mass);
    }

    /// <summary>
    /// Computes the inertia of a box.
    /// </summary>
    /// <param name="box">Shape to compute the inertia of.</param>
    /// <param name="mass">Mass to use in the inertia calculation.</param>
    /// <returns>Inertia of the shape.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Box.ComputeInertia")]
    public static BodyInertia ComputeBoxInertia(Box box, float mass)
    {
        return box.ComputeInertia(mass);
    }

    /// <summary>
    /// Computes the inertia of a triangle.
    /// </summary>
    /// <param name="triangle">Shape to compute the inertia of.</param>
    /// <param name="mass">Mass to use in the inertia calculation.</param>
    /// <returns>Inertia of the shape.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Triangle.ComputeInertia")]
    public static BodyInertia ComputeTriangleInertia(Triangle triangle, float mass)
    {
        return triangle.ComputeInertia(mass);
    }

    /// <summary>
    /// Computes the inertia of a cylinder.
    /// </summary>
    /// <param name="cylinder">Shape to compute the inertia of.</param>
    /// <param name="mass">Mass to use in the inertia calculation.</param>
    /// <returns>Inertia of the shape.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Cylinder.ComputeInertia")]
    public static BodyInertia ComputeCylinderInertia(Cylinder cylinder, float mass)
    {
        return cylinder.ComputeInertia(mass);
    }

    /// <summary>
    /// Computes the inertia of a convex hull.
    /// </summary>
    /// <param name="convexHull">Shape to compute the inertia of.</param>
    /// <param name="mass">Mass to use in the inertia calculation.</param>
    /// <returns>Inertia of the shape.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "ConvexHull.ComputeInertia")]
    public static BodyInertia ComputeConvexHullInertia(ConvexHull convexHull, float mass)
    {
        return convexHull.ComputeInertia(mass);
    }

    /// <summary>
    /// Computes the inertia of a convex.
    /// </summary>
    /// <param name="convex">Index of a convex to calculate the inertia for.</param>
    /// <param name="mass">Mass to use in the inertia calculation.</param>
    /// <returns>Inertia of the shape. If the shape index was not a convex, this returns a zeroed inverse inertia tensor.</returns>
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) }, EntryPoint = "Simulation.Shapes.ComputeConvexInertia")]
    public static BodyInertia ComputeConvexInertia([TypeName("SimulationHandle")] InstanceHandle simulationHandle, TypedIndex convex, float mass)
    {
        if (simulations[simulationHandle].Shapes[convex.Type] is IConvexShapeBatch convexBatch)
        {
            return convexBatch.ComputeInertia(convex.Index, mass);
        }
        return new BodyInertia() { InverseMass = 1f / mass };
    }

    /// <summary>
    /// Computes the inertia associated with a set of compound children. Does not recenter the children.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to which the shapes referenced by the compound children belong.</param>
    /// <param name="children">Children of the compound.</param>
    /// <param name="childMasses">Masses of the children composing the compound.</param>
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) }, EntryPoint = "Simulation.Shapes.ComputeCompoundInertia")]
    public static BodyInertia ComputeCompoundInertia([TypeName("SimulationHandle")] InstanceHandle simulationHandle, [TypeName("Buffer<CompoundChild>")] Buffer<CompoundChild> children, [TypeName("Buffer<float>")] Buffer<float> childMasses)
    {
        return CompoundBuilder.ComputeInertia(children, childMasses, simulations[simulationHandle].Shapes);
    }

    /// <summary>
    /// Computes the inertia associated with a set of compound children. Recenters all children onto the computed local center of mass.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to which the shapes referenced by the compound children belong.</param>
    /// <param name="children">Children of the compound.</param>
    /// <param name="childMasses">Masses of the children composing the compound.</param>
    /// <param name="centerOfMass">Computed center of mass that was subtracted from the position of compound children.</param>
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) }, EntryPoint = "Simulation.Shapes.ComputeCompoundInertiaWithRecentering")]
    public static unsafe BodyInertia ComputeCompoundInertiaWithRecentering([TypeName("SimulationHandle")] InstanceHandle simulationHandle, [TypeName("Buffer<CompoundChild>")] Buffer<CompoundChild> children, [TypeName("Buffer<float>")] Buffer<float> childMasses, Vector3* centerOfMass)
    {
        return CompoundBuilder.ComputeInertia(children, childMasses, simulations[simulationHandle].Shapes, out *centerOfMass);
    }

    /// <summary>
    /// Computes the inertia associated with a mesh by treating its triangles as a soup with no volume. Does not recenter the triangles on a computed center of mass.
    /// </summary>
    /// <param name="mesh">Mesh to compute the inertia of.</param>
    /// <param name="mass">Mass of the mesh.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Mesh.ComputeOpenInertia")]
    public static BodyInertia ComputeOpenMeshInertia(Mesh mesh, float mass)
    {
        return mesh.ComputeOpenInertia(mass);
    }

    /// <summary>
    /// Computes the inertia associated with a mesh by treating it as a closed volume. Does not recenter the triangles on a computed center of mass.
    /// </summary>
    /// <param name="mesh">Mesh to compute the inertia of.</param>
    /// <param name="mass">Mass of the mesh.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Mesh.ComputeClosedInertia")]
    public static BodyInertia ComputeClosedMeshInertia(Mesh mesh, float mass)
    {
        return mesh.ComputeClosedInertia(mass);
    }

    /// <summary>
    /// Computes the inertia associated with a mesh by treating its triangles as a soup with no volume. Recenters all children onto the computed local center of mass.
    /// </summary>
    /// <param name="mesh">Mesh to compute the inertia of.</param>
    /// <param name="mass">Mass of the mesh.</param>
    /// <param name="centerOfMass">The center of mass offset relative to the mesh coordinate system.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Mesh.ComputeOpenInertiaWithRecentering")]
    public static unsafe BodyInertia ComputeOpenMeshInertiaWithRecentering(Mesh mesh, float mass, Vector3* centerOfMass)
    {
        return mesh.ComputeOpenInertia(mass, out *centerOfMass);
    }

    /// <summary>
    /// Computes the inertia associated with a mesh by treating it as a closed volume. Recenters all children onto the computed local center of mass.
    /// </summary>
    /// <param name="mesh">Mesh to compute the inertia of.</param>
    /// <param name="mass">Mass of the mesh.</param>
    /// <param name="centerOfMass">The center of mass offset relative to the mesh coordinate system.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Mesh.ComputeClosedInertiaWithRecentering")]
    public static unsafe BodyInertia ComputeClosedMeshInertiaWithRecentering(Mesh mesh, float mass, Vector3* centerOfMass)
    {
        return mesh.ComputeClosedInertia(mass, out *centerOfMass);
    }

    /// <summary>
    /// Gets a pointer to a sphere shape's data stored within the simulation's shapes buffers.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to remove the shape from.</param>
    /// <param name="shape">Shape reference to request from the simulation.</param>
    /// <returns>Pointer to the shape's data in the simulation's shapes buffers.</returns>
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) }, EntryPoint = "Simulation.Shapes.GetSphereData")]
    public static unsafe Sphere* GetSphereShapeData([TypeName("SimulationHandle")] InstanceHandle simulationHandle, TypedIndex shape)
    {
        return (Sphere*)Unsafe.AsPointer(ref simulations[simulationHandle].Shapes.GetShape<Sphere>(shape.Index));
    }

    /// <summary>
    /// Gets a pointer to a capsule shape's data stored within the simulation's shapes buffers.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to remove the shape from.</param>
    /// <param name="shape">Shape reference to request from the simulation.</param>
    /// <returns>Pointer to the shape's data in the simulation's shapes buffers.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.GetCapsuleData")]
    public static unsafe Capsule* GetCapsuleShapeData([TypeName("SimulationHandle")] InstanceHandle simulationHandle, TypedIndex shape)
    {
        return (Capsule*)Unsafe.AsPointer(ref simulations[simulationHandle].Shapes.GetShape<Capsule>(shape.Index));
    }

    /// <summary>
    /// Gets a pointer to a box shape's data stored within the simulation's shapes buffers.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to remove the shape from.</param>
    /// <param name="shape">Shape reference to request from the simulation.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.GetBoxData")]
    public static unsafe Box* GetBoxShapeData([TypeName("SimulationHandle")] InstanceHandle simulationHandle, TypedIndex shape)
    {
        return (Box*)Unsafe.AsPointer(ref simulations[simulationHandle].Shapes.GetShape<Box>(shape.Index));
    }

    /// <summary>
    /// Gets a pointer to a triangle shape's data stored within the simulation's shapes buffers.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to remove the shape from.</param>
    /// <param name="shape">Shape reference to request from the simulation.</param>
    /// <returns>Pointer to the shape's data in the simulation's shapes buffers.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.GetTriangleData")]
    public unsafe static Triangle* GetTriangleShapeData([TypeName("SimulationHandle")] InstanceHandle simulationHandle, TypedIndex shape)
    {
        return (Triangle*)Unsafe.AsPointer(ref simulations[simulationHandle].Shapes.GetShape<Triangle>(shape.Index));
    }

    /// <summary>
    /// Gets a pointer to a cylinder shape's data stored within the simulation's shapes buffers.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to remove the shape from.</param>
    /// <param name="shape">Shape reference to request from the simulation.</param>
    /// <returns>Pointer to the shape's data in the simulation's shapes buffers.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.GetCapsuleCylinderData")]
    public static unsafe Cylinder* GetCylinderShapeData([TypeName("SimulationHandle")] InstanceHandle simulationHandle, TypedIndex shape)
    {
        return (Cylinder*)Unsafe.AsPointer(ref simulations[simulationHandle].Shapes.GetShape<Cylinder>(shape.Index));
    }

    /// <summary>
    /// Gets a pointer to a convex hull shape's data stored within the simulation's shapes buffers.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to remove the shape from.</param>
    /// <param name="shape">Shape reference to request from the simulation.</param>
    /// <returns>Pointer to the shape's data in the simulation's shapes buffers.</returns>
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) }, EntryPoint = "Simulation.Shapes.GetConvexHullData")]
    public unsafe static ConvexHull* GetConvexHullShapeData([TypeName("SimulationHandle")] InstanceHandle simulationHandle, TypedIndex shape)
    {
        return (ConvexHull*)Unsafe.AsPointer(ref simulations[simulationHandle].Shapes.GetShape<ConvexHull>(shape.Index));
    }

    /// <summary>
    /// Gets a pointer to a compound shape's data stored within the simulation's shapes buffers.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to remove the shape from.</param>
    /// <param name="shape">Shape reference to request from the simulation.</param>
    /// <returns>Pointer to the shape's data in the simulation's shapes buffers.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Shapes.GetCompoundData")]
    public static unsafe Compound* GetCompoundShapeData([TypeName("SimulationHandle")] InstanceHandle simulationHandle, TypedIndex shape)
    {
        return (Compound*)Unsafe.AsPointer(ref simulations[simulationHandle].Shapes.GetShape<Compound>(shape.Index));
    }

    /// <summary>
    /// Gets a pointer to a big compound shape's data stored within the simulation's shapes buffers.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to remove the shape from.</param>
    /// <param name="shape">Shape reference to request from the simulation.</param>
    /// <returns>Pointer to the shape's data in the simulation's shapes buffers.</returns>
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) }, EntryPoint = "Simulation.Shapes.GetBigCompoundData")]
    public static unsafe BigCompound* GetBigCompoundShapeData([TypeName("SimulationHandle")] InstanceHandle simulationHandle, TypedIndex shape)
    {
        return (BigCompound*)Unsafe.AsPointer(ref simulations[simulationHandle].Shapes.GetShape<BigCompound>(shape.Index));
    }

    /// <summary>
    /// Gets a pointer to a mesh shape's data stored within the simulation's shapes buffers.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to remove the shape from.</param>
    /// <param name="shape">Shape reference to request from the simulation.</param>
    /// <returns>Pointer to the shape's data in the simulation's shapes buffers.</returns>
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) }, EntryPoint = "Simulation.Shapes.GetMeshData")]
    public static unsafe Mesh* GetMeshShapeData([TypeName("SimulationHandle")] InstanceHandle simulationHandle, TypedIndex shape)
    {
        return (Mesh*)Unsafe.AsPointer(ref simulations[simulationHandle].Shapes.GetShape<Mesh>(shape.Index));
    }
}
