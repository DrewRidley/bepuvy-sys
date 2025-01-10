using System.Diagnostics;
using System.Numerics;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using AbominationInterop;
using BepuPhysics;
using BepuPhysics.Collidables;
using BepuPhysics.CollisionDetection;
using BepuUtilities.Collections;
using BepuUtilities.Memory;
using Bepuvy.Callbacks;

namespace Bepuvy;

public static partial class Entrypoints
{
    
     //We don't want to do runtime checks in the callbacks, so we jump through some fun hoops to construct a type.
    private static unsafe InstanceHandle CreateSimulationWithScalarIntegrationState<TConservationType, TSubstepUnconstrained, TIntegrateKinematicVelocities, TScalarIntegration>(
      BufferPool pool, NarrowPhaseCallbacks narrowPhaseCallbacks, PoseIntegratorCallbacksInterop poseIntegratorCallbacksInterop, SolveDescription solveDescription, SimulationAllocationSizes initialAllocationSizes, RustCallback capture)
    {
        void* integrateVelocityFunction = poseIntegratorCallbacksInterop.IntegrateVelocity;
        
        if (integrateVelocityFunction == null)
            throw new NullReferenceException("Velocity integration callback is not defined. Was the wrong callback provided for the scalar state/SIMD width?");
        var poseIntegratorCallbacks = new PoseIntegratorCallbacks<TConservationType, TSubstepUnconstrained, TIntegrateKinematicVelocities, TScalarIntegration>
        {
            InitializeFunction = poseIntegratorCallbacksInterop.Initialize,
            PrepareForIntegrationFunction = poseIntegratorCallbacksInterop.PrepareForIntegration,
            IntegrateVelocityFunction = integrateVelocityFunction,
            Callback = capture
        };
        //For now, the native side can't define custom timesteppers. This isn't fundamental, but exposing it would be somewhat annoying, so punted.
        var simulation = Simulation.Create(pool, narrowPhaseCallbacks, poseIntegratorCallbacks, solveDescription, initialAllocationSizes: initialAllocationSizes);
        var handle = simulations.Add(simulation);
        //The usual narrow phase callbacks initialization could not be done because there was no handle available for the native side to use, so call it now.
        ((NarrowPhase<NarrowPhaseCallbacks>)simulation.NarrowPhase).Callbacks.Initialize(handle);
        //Same for pose integrator callbacks.
        ((PoseIntegrator<PoseIntegratorCallbacks<TConservationType, TSubstepUnconstrained, TIntegrateKinematicVelocities, TScalarIntegration>>)simulation.PoseIntegrator).Callbacks.Initialize(handle);
        return handle;
    }
    
    private static InstanceHandle CreateSimulationWithKinematicIntegrationState<TConservationType, TSubstepUnconstrained, TIntegrateKinematicVelocities>(
        BufferPool pool, NarrowPhaseCallbacks narrowPhaseCallbacks, PoseIntegratorCallbacksInterop poseIntegratorCallbacksInterop, SolveDescription solveDescription, SimulationAllocationSizes initialAllocationSizes, RustCallback capture)
    {
        if (poseIntegratorCallbacksInterop.UseScalarCallback != 0)
            return CreateSimulationWithScalarIntegrationState<TConservationType, TSubstepUnconstrained, TIntegrateKinematicVelocities, True>(pool, narrowPhaseCallbacks, poseIntegratorCallbacksInterop, solveDescription, initialAllocationSizes, capture);
        return CreateSimulationWithScalarIntegrationState<TConservationType, TSubstepUnconstrained, TIntegrateKinematicVelocities, False>(pool, narrowPhaseCallbacks, poseIntegratorCallbacksInterop, solveDescription, initialAllocationSizes, capture);
    }
    private static InstanceHandle CreateSimulationWithUnconstrainedSubstepState<TConservationType, TSubstepUnconstrained>(
        BufferPool pool, NarrowPhaseCallbacks narrowPhaseCallbacks, PoseIntegratorCallbacksInterop poseIntegratorCallbacksInterop, SolveDescription solveDescription, SimulationAllocationSizes initialAllocationSizes, RustCallback capture)
    {
        if (poseIntegratorCallbacksInterop.IntegrateVelocityForKinematics != 0)
            return CreateSimulationWithKinematicIntegrationState<TConservationType, TSubstepUnconstrained, True>(pool, narrowPhaseCallbacks, poseIntegratorCallbacksInterop, solveDescription, initialAllocationSizes, capture);
        return CreateSimulationWithKinematicIntegrationState<TConservationType, TSubstepUnconstrained, False>(pool, narrowPhaseCallbacks, poseIntegratorCallbacksInterop, solveDescription, initialAllocationSizes, capture);
    }

    private static InstanceHandle CreateSimulationWithConservationType<TConservationType>(
        BufferPool pool, NarrowPhaseCallbacks narrowPhaseCallbacks, PoseIntegratorCallbacksInterop poseIntegratorCallbacksInterop, SolveDescription solveDescription, SimulationAllocationSizes initialAllocationSizes, RustCallback capture)
    {
        if (poseIntegratorCallbacksInterop.AllowSubstepsForUnconstrainedBodies != 0)
            return CreateSimulationWithUnconstrainedSubstepState<TConservationType, True>(pool, narrowPhaseCallbacks, poseIntegratorCallbacksInterop, solveDescription, initialAllocationSizes, capture);
        return CreateSimulationWithUnconstrainedSubstepState<TConservationType, False>(pool, narrowPhaseCallbacks, poseIntegratorCallbacksInterop, solveDescription, initialAllocationSizes, capture);

    }
    
    private static unsafe InstanceHandle CreateSimulation(BufferPool pool, NarrowPhaseCallbacks narrowPhaseCallbacks, PoseIntegratorCallbacksInterop poseIntegratorCallbacksInterop, SolveDescription solveDescription, SimulationAllocationSizes initialAllocationSizes, RustCallback capture)
    {
        switch (poseIntegratorCallbacksInterop.AngularIntegrationMode)
        {
            case AngularIntegrationMode.ConserveMomentumWithGyroscopicTorque:
                return CreateSimulationWithConservationType<AngularIntegrationModeConserveWithGyroTorque>(pool, narrowPhaseCallbacks, poseIntegratorCallbacksInterop, solveDescription, initialAllocationSizes, capture);
            case AngularIntegrationMode.ConserveMomentum:
                return CreateSimulationWithConservationType<AngularIntegrationModeConserve>(pool, narrowPhaseCallbacks, poseIntegratorCallbacksInterop, solveDescription, initialAllocationSizes, capture);
            default:
                return CreateSimulationWithConservationType<AngularIntegrationModeNonConserving>(pool, narrowPhaseCallbacks, poseIntegratorCallbacksInterop, solveDescription, initialAllocationSizes, capture);
        }
    }

    /// <summary>
    /// Creates a new simulation.
    /// </summary>
    /// <param name="bufferPool">Buffer pool for the simulation's main allocations.</param>
    /// <param name="narrowPhaseCallbacks">Narrow phase callbacks to be invoked by the simulation.</param>
    /// <param name="poseIntegratorCallbacks">Pose integration state and callbacks to be invoked by the simulation.</param>
    /// <param name="callback">A rust closure that captures external state to be used during integration</param>
    /// <param name="solveDescriptionInterop">Defines velocity iteration count and substep counts for the simulation's solver.</param>
    /// <param name="initialAllocationSizes">Initial capacities to allocate within the simulation.</param>
    /// <returns></returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Create")]
    [return: TypeName("SimulationHandle")]
    public static unsafe InstanceHandle CreateSimulation(
        [TypeName("BufferPoolHandle")] InstanceHandle bufferPool,
        [TypeName("NarrowPhaseCallbacks")] NarrowPhaseCallbacksInterop narrowPhaseCallbacks,
        [TypeName("PoseIntegratorCallbacks")] PoseIntegratorCallbacksInterop poseIntegratorCallbacks,
        [TypeName("RustCallback")] void* callback,
        [TypeName("SolveDescription")] SolveDescriptionInterop solveDescriptionInterop, SimulationAllocationSizes initialAllocationSizes)
    {
        var solveDescription = new SolveDescription
        {
            VelocityIterationCount = solveDescriptionInterop.VelocityIterationCount,
            SubstepCount = solveDescriptionInterop.SubstepCount,
            FallbackBatchThreshold = solveDescriptionInterop.FallbackBatchThreshold,
            VelocityIterationScheduler = solveDescriptionInterop.VelocityIterationScheduler != null ? Marshal.GetDelegateForFunctionPointer<SubstepVelocityIterationScheduler>((IntPtr)solveDescriptionInterop.VelocityIterationScheduler) : null
        };
        var narrowPhaseCallbacksImpl = new NarrowPhaseCallbacks
        {
            InitializeFunction = narrowPhaseCallbacks.InitializeFunction,
            DisposeFunction = narrowPhaseCallbacks.DisposeFunction,
            AllowContactGenerationFunction = narrowPhaseCallbacks.AllowContactGenerationFunction,
            AllowContactGenerationBetweenChildrenFunction = narrowPhaseCallbacks.AllowContactGenerationBetweenChildrenFunction,
            ConfigureConvexContactManifoldFunction = narrowPhaseCallbacks.ConfigureConvexContactManifoldFunction,
            ConfigureNonconvexContactManifoldFunction = narrowPhaseCallbacks.ConfigureNonconvexContactManifoldFunction,
            ConfigureChildContactManifoldFunction = narrowPhaseCallbacks.ConfigureChildContactManifoldFunction
        };

        RustCallback rustCallback = new RustCallback() { Callback = callback };

        InstanceHandle handle = CreateSimulation(bufferPools[bufferPool], narrowPhaseCallbacksImpl, poseIntegratorCallbacks, solveDescription, initialAllocationSizes, rustCallback);
        InstanceHandle callbackHandle = callbacks.Add(rustCallback);
        
        Debug.Assert(Equals(callbackHandle, handle), "The callback and simulation arrays are misaligned.");

        return handle;
    }
    
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Destroy")]
    public static  void DestroySimulation([TypeName("SimulationHandle")] InstanceHandle handle)
    { 
        simulations[handle].Dispose();
        simulations.Remove(handle);
        callbacks.Remove(handle);
    }
    
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.AddBody")]
     public static BodyHandle AddBody([TypeName("SimulationHandle")] InstanceHandle simulationHandle, BodyDescription bodyDescription)
     {
         return simulations[simulationHandle].Bodies.Add(bodyDescription);
     }
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.RemoveBody")]
     public static unsafe void RemoveBody([TypeName("SimulationHandle")] InstanceHandle simulationHandle, BodyHandle bodyHandle)
     {
         simulations[simulationHandle].Bodies.Remove(bodyHandle);
     }

     /// <summary>
     /// Gets a pointer to the dynamic state associated with a body. Includes pose, velocity, and inertia.
     /// </summary>
     /// <param name="simulationHandle">Simulation to pull a body's state from.</param>
     /// <param name="bodyHandle">Body handle to pull data about.</param>
     /// <returns>Pointer to the body's dynamic state.</returns>
     /// <remarks>This is a direct pointer. The memory location associated with a body can move other bodies are removed from the simulation; do not hold a pointer beyond the point where it may be invalidated.</remarks>
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.GetBodyDynamics")]
     public static unsafe BodyDynamics* GetBodyDynamics([TypeName("SimulationHandle")] InstanceHandle simulationHandle, BodyHandle bodyHandle)
     {
         return (BodyDynamics*)Unsafe.AsPointer(ref simulations[simulationHandle].Bodies[bodyHandle].Dynamics);
     }

     /// <summary>
     /// Gets a pointer to the collidable associated with a body.
     /// </summary>
     /// <param name="simulationHandle">Simulation to pull a body's state from.</param>
     /// <param name="bodyHandle">Body handle to pull data about.</param>
     /// <returns>Pointer to the body's collidable.</returns>
     /// <remarks>This is a direct pointer. The memory location associated with a body can move if other bodies are removed from the simulation; do not hold a pointer beyond the point where it may be invalidated.</remarks>
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.GetBodyCollidable")]
     public static unsafe Collidable* GetBodyCollidable([TypeName("SimulationHandle")] InstanceHandle simulationHandle, BodyHandle bodyHandle)
     {
         return (Collidable*)Unsafe.AsPointer(ref simulations[simulationHandle].Bodies[bodyHandle].Collidable);
     }

     /// <summary>
     /// Gets a pointer to the activity state associated with a body.
     /// </summary>
     /// <param name="simulationHandle">Simulation to pull a body's state from.</param>
     /// <param name="bodyHandle">Body handle to pull data about.</param>
     /// <returns>Pointer to the body's activity state.</returns>
     /// <remarks>This is a direct pointer. The memory location associated with a body can move if other bodies are removed from the simulation; do not hold a pointer beyond the point where it may be invalidated.</remarks>
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.GetBodyActivity")]
     public static unsafe BodyActivity* GetBodyActivity([TypeName("SimulationHandle")] InstanceHandle simulationHandle, BodyHandle bodyHandle)
     {
         return (BodyActivity*)Unsafe.AsPointer(ref simulations[simulationHandle].Bodies[bodyHandle].Activity);
     }

     /// <summary>
     /// Gets a pointer to the list of constraints associated with a body.
     /// </summary>
     /// <param name="simulationHandle">Simulation to pull a body's state from.</param>
     /// <param name="bodyHandle">Body handle to pull data about.</param>
     /// <returns>Pointer to the body's constraint list.</returns>
     /// <remarks>This is a direct pointer. The memory location associated with a body can move if other bodies are removed from the simulation; do not hold a pointer beyond the point where it may be invalidated.</remarks>
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.GetBodyConstraints")]
     [return: TypeName("QuickList<BodyConstraintReference>*")]
     public static unsafe QuickList<ConstraintReference>* GetBodyConstraints([TypeName("SimulationHandle")] InstanceHandle simulationHandle, BodyHandle bodyHandle)
     {
         return (QuickList<ConstraintReference>*)Unsafe.AsPointer(ref simulations[simulationHandle].Bodies[bodyHandle].Constraints);
     }

     /// <summary>
     /// Gets a description of a body.
     /// </summary>
     /// <param name="simulationHandle">Simulation to pull a body's state from.</param>
     /// <param name="bodyHandle">Body handle to pull data about.</param>
     /// <returns>Description of a body.</returns>
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.GetBodyDescription")]
     public static BodyDescription GetBodyDescription([TypeName("SimulationHandle")] InstanceHandle simulationHandle, BodyHandle bodyHandle)
     {
         return simulations[simulationHandle].Bodies.GetDescription(bodyHandle);
     }

     /// <summary>
     /// Applies a description to a body.
     /// </summary>
     /// <param name="simulationHandle">Simulation to pull a body's state from.</param>
     /// <param name="bodyHandle">Body handle to pull data about.</param>
     /// <param name="description">Description to apply to the body.</param>
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.ApplyBodyDescription")]
     public static  void ApplyBodyDescription([TypeName("SimulationHandle")] InstanceHandle simulationHandle, BodyHandle bodyHandle, BodyDescription description)
     {
         simulations[simulationHandle].Bodies.ApplyDescription(bodyHandle, description);
     }

     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.AddStatic")]
     public static  StaticHandle AddStatic([TypeName("SimulationHandle")] InstanceHandle simulationHandle, StaticDescription staticDescription)
     {
         return simulations[simulationHandle].Statics.Add(staticDescription);
     }
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.RemoveStatic")]
     public static  void RemoveStatic([TypeName("SimulationHandle")] InstanceHandle simulationHandle, StaticHandle staticHandle)
     {
         simulations[simulationHandle].Statics.Remove(staticHandle);
     }
     /// <summary>
     /// Gets a pointer to data associated with a static.
     /// </summary>
     /// <param name="simulationHandle">Simulation to pull a static's state from.</param>
     /// <param name="staticHandle">Static handle to pull data about.</param>
     /// <returns>Pointer to the static's data.</returns>
     /// <remarks>This is a direct pointer. The memory location associated with a static can move if other statics are removed from the simulation; do not hold a pointer beyond the point where it may be invalidated.</remarks>
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.GetStatic")]
     public static unsafe Static* GetStatic([TypeName("SimulationHandle")] InstanceHandle simulationHandle, StaticHandle staticHandle)
     {
         return (Static*)Unsafe.AsPointer(ref simulations[simulationHandle].Statics.GetDirectReference(staticHandle));
     }

     /// <summary>
     /// Gets a static's description.
     /// </summary>
     /// <param name="simulationHandle">Simulation to pull a static's state from.</param>
     /// <param name="staticHandle">Static handle to pull data about.</param>
     /// <returns>Description of the static..</returns>
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.GetStaticDescription")]
     public static StaticDescription GetStaticDescription([TypeName("SimulationHandle")] InstanceHandle simulationHandle, StaticHandle staticHandle)
     {
         return simulations[simulationHandle].Statics.GetDescription(staticHandle);
     }

     /// <summary>
     /// Applies a description to a static.
     /// </summary>
     /// <param name="simulationHandle">Simulation to pull a static's state from.</param>
     /// <param name="staticHandle">Static handle to pull data about.</param>
     /// <param name="description">The description to be applied to the static.</param>
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.ApplyStaticDescription")]
     public static void ApplyStaticDescription([TypeName("SimulationHandle")] InstanceHandle simulationHandle, StaticHandle staticHandle, StaticDescription description)
     {
         simulations[simulationHandle].Statics.ApplyDescription(staticHandle, description);
     }

    /// <summary>
    /// Steps the simulation forward a single time.
    /// </summary>
    /// <param name="simulationHandle">Handle of the simulation to step.</param>
    /// <param name="dt">Duration of the timestep.</param>
    /// <param name="callback"></param>
    /// <param name="threadDispatcherHandle">Handle of the thread dispatcher to use, if any. Can be a null reference.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Timestep")]
     public static unsafe void Timestep([TypeName("SimulationHandle")] InstanceHandle simulationHandle, float dt, void* callback, [TypeName("ThreadDispatcherHandle")] InstanceHandle threadDispatcherHandle = new())
     {
         var threadDispatcher = threadDispatcherHandle.Null ? null : threadDispatchers[threadDispatcherHandle];
        
         //If callback captures rust state, it will exist on the stack.
         //Since this is a stack ptr, it will change each timestep hence it must be updated.
         callbacks[simulationHandle].Callback = callback;
         
         simulations[simulationHandle].Timestep(dt, threadDispatcher);
     }

     /// <summary>
     /// Grabs a collidable's bounding boxes in the broad phase.
     /// </summary>
     /// <param name="simulationHandle">Handle of the simulation to pull data from.</param>
     /// <param name="bodyHandle">Body to pull bounding box data about.</param>
     /// <param name="min">Minimum bounds of the collidable's bounding box.</param>
     /// <param name="max">Maximum bounds of the collidable's bounding box.</param>
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.GetBodyBoundingBoxInBroadPhase")]
     public static unsafe void GetBodyBoundingBoxInBroadPhase([TypeName("SimulationHandle")] InstanceHandle simulationHandle, BodyHandle bodyHandle, Vector3* min, Vector3* max)
     {
         simulations[simulationHandle].Bodies[bodyHandle].GetBoundsReferencesFromBroadPhase(out var minPointer, out var maxPointer);
         *min = *minPointer;
         *max = *maxPointer;
     }

     /// <summary>
     /// Grabs a collidable's bounding boxes in the broad phase.
     /// </summary>
     /// <param name="simulationHandle">Handle of the simulation to pull data from.</param>
     /// <param name="staticHandle">Static to pull bounding box data about.</param>
     /// <param name="min">Minimum bounds of the collidable's bounding box.</param>
     /// <param name="max">Maximum bounds of the collidable's bounding box.</param>
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.GetStaticBoundingBoxInBroadPhase")]
     public static unsafe void GetStaticBoundingBoxInBroadPhase([TypeName("SimulationHandle")] InstanceHandle simulationHandle, StaticHandle staticHandle, Vector3* min, Vector3* max)
     {
         simulations[simulationHandle].Statics[staticHandle].GetBoundsReferencesFromBroadPhase(out var minPointer, out var maxPointer);
         *min = *minPointer;
         *max = *maxPointer;
     }

     /// <summary>
     /// Gets the mapping from body handles to the body's location in storage.
     /// </summary>
     /// <param name="simulationHandle">Handle of the simulation to pull data from.</param>
     /// <param name="bodyHandleToIndexMapping">Mapping from a body handle to the body's memory location.</param>
     /// <remarks>The buffer returned by this function can be invalidated if the simulation resizes it.</remarks>
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.GetBodyHandleToLocationMappings")]
     public static unsafe void GetBodyHandleToLocationMapping([TypeName("SimulationHandle")] InstanceHandle simulationHandle, [TypeName("Buffer<BodyMemoryLocation>*")] Buffer<BodyMemoryLocation>* bodyHandleToIndexMapping)
     {
         *bodyHandleToIndexMapping = simulations[simulationHandle].Bodies.HandleToLocation;
     }

     /// <summary>
     /// Gets the body sets for a simulation. Slot 0 is the active set. Subsequent sets are sleeping. Not every slot beyond slot 0 is filled.
     /// </summary>
     /// <param name="simulationHandle">Handle of the simulation to pull data from.</param>
     /// <param name="bodySets">Mapping from a body handle to the body's memory location.</param>
     /// <remarks>The buffer returned by this function can be invalidated if the simulation resizes it.</remarks>
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.GetBodySets")]
     public static unsafe void GetBodySets([TypeName("SimulationHandle")] InstanceHandle simulationHandle, [TypeName("Buffer<BodySet>*")] Buffer<BodySet>* bodySets)
     {
         *bodySets = simulations[simulationHandle].Bodies.Sets;
     }

     /// <summary>
     /// Gets the mapping from body handles to the body's location in storage.
     /// </summary>
     /// <param name="simulationHandle">Handle of the simulation to pull data from.</param>
     /// <param name="staticHandleToIndexMapping">Mapping from a static handle to the static's memory location.</param>
     /// <remarks>The buffer returned by this function can be invalidated if the simulation resizes it.</remarks>
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.GetStaticHandleToLocationMapping")]
     public static unsafe void GetStaticHandleToLocationMapping([TypeName("SimulationHandle")] InstanceHandle simulationHandle, [TypeName("Buffer<int32_t>*")] Buffer<int>* staticHandleToIndexMapping)
     {
         *staticHandleToIndexMapping = simulations[simulationHandle].Statics.HandleToIndex;
     }

     /// <summary>
     /// Gets the statics set for a simulation.
     /// </summary>
     /// <param name="simulationHandle">Handle of the simulation to pull data from.</param>
     /// <param name="statics">The set of all statics within a simulation.</param>
     /// <param name="count">Number of statics in the simulation.</param>
     /// <remarks>The buffer returned by this function can be invalidated if the simulation resizes it. The count is a snapshot.</remarks>
     [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.GetStatics")]
     public static unsafe void GetStatics([TypeName("SimulationHandle")] InstanceHandle simulationHandle, [TypeName("Buffer<Static>*")] Buffer<Static>* statics, [TypeName("int32_t*")] int* count)
     {
         *statics = simulations[simulationHandle].Statics.StaticsBuffer;
         *count = simulations[simulationHandle].Statics.Count;
     }
}
