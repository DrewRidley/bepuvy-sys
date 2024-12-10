using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using BepuPhysics;

namespace Bepuvy;

/// <summary>
/// An equivalent to SolveDescription, but unmanaged and stack allocated.
/// This type is exposed to Native code so it can pass options which are marshalled to managed types.
/// </summary>
[StructLayout(LayoutKind.Explicit)]
public unsafe struct SolveDescription
{
    /// <summary>
    /// Number of velocity iterations to use in the solver if there is no <see cref="VelocityIterationScheduler"/> or if it returns a non-positive value for a substep.
    /// </summary>
    [FieldOffset(0)]
    public int VelocityIterationCount;
    /// <summary>
    /// Number of substeps to execute each time the solver runs.
    /// </summary>
    [FieldOffset(4)]
    public int SubstepCount;
    /// <summary>
    /// Number of synchronzed constraint batches to use before using a fallback approach.
    /// </summary>
    [FieldOffset(8)]
    public int FallbackBatchThreshold;
    /// <summary>
    /// Callback executed to determine how many velocity iterations should be used for a given substep. If null, or if it returns a non-positive value, the <see cref="VelocityIterationCount"/> will be used instead.
    /// </summary>
    [FieldOffset(16)]
    public delegate* unmanaged<int, int> VelocityIterationScheduler;
}

public static partial class Entrypoint
{
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Simulation.Create")]
    public static unsafe InstanceHandle CreateSimulation(InstanceHandle bufferPool, SolveDescription description)
    {
        var solveDescription = new BepuPhysics.SolveDescription
        {
            VelocityIterationCount = description.VelocityIterationCount,
            SubstepCount = description.SubstepCount,
            FallbackBatchThreshold = description.FallbackBatchThreshold,
            VelocityIterationScheduler = description.VelocityIterationScheduler != null ? Marshal.GetDelegateForFunctionPointer<SubstepVelocityIterationScheduler>((IntPtr)solveDescriptionInterop.VelocityIterationScheduler) : null
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
        return CreateSimulation(bufferPools[bufferPool], narrowPhaseCallbacksImpl, poseIntegratorCallbacks, solveDescription, initialAllocationSizes);
    }
}




/*
 
    /// <summary>
    /// Creates a new simulation.
    /// </summary>
    /// <param name="bufferPool">Buffer pool for the simulation's main allocations.</param>
    /// <param name="narrowPhaseCallbacks">Narrow phase callbacks to be invoked by the simulation.</param>
    /// <param name="poseIntegratorCallbacks">Pose integration state and callbacks to be invoked by the simulation.</param>
    /// <param name="solveDescriptionInterop">Defines velocity iteration count and substep counts for the simulation's solver.</param>
    /// <param name="initialAllocationSizes">Initial capacities to allocate within the simulation.</param>
    /// <returns></returns>
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) }, EntryPoint = FunctionNamePrefix + nameof(CreateSimulation))]
    [return: TypeName(SimulationName)]
    public unsafe static InstanceHandle CreateSimulation(
        [TypeName(BufferPoolName)] InstanceHandle bufferPool,
        [TypeName("NarrowPhaseCallbacks")] NarrowPhaseCallbacksInterop narrowPhaseCallbacks,
        [TypeName("PoseIntegratorCallbacks")] PoseIntegratorCallbacksInterop poseIntegratorCallbacks,
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
        return CreateSimulation(bufferPools[bufferPool], narrowPhaseCallbacksImpl, poseIntegratorCallbacks, solveDescription, initialAllocationSizes);
    }

    /// <summary>
    /// Destroys a simulation and invalidates its handle.
    /// </summary>
    /// <param name="handle">Simulation to destroy.</param>

    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) }, EntryPoint = FunctionNamePrefix + nameof(DestroySimulation))]
    public static unsafe void DestroySimulation([TypeName(SimulationName)] InstanceHandle handle)
    {
        simulations[handle].Dispose();
        simulations
 *
 *private static InstanceHandle CreateSimulation(BufferPool pool, NarrowPhaseCallbacks narrowPhaseCallbacks, PoseIntegratorCallbacksInterop poseIntegratorCallbacksInterop, SolveDescription solveDescription, SimulationAllocationSizes initialAllocationSizes)
   {
       switch (poseIntegratorCallbacksInterop.AngularIntegrationMode)
       {
           case AngularIntegrationMode.ConserveMomentumWithGyroscopicTorque:
               return CreateSimulationWithConservationType<AngularIntegrationModeConserveWithGyroTorque>(pool, narrowPhaseCallbacks, poseIntegratorCallbacksInterop, solveDescription, initialAllocationSizes);
           case AngularIntegrationMode.ConserveMomentum:
               return CreateSimulationWithConservationType<AngularIntegrationModeConserve>(pool, narrowPhaseCallbacks, poseIntegratorCallbacksInterop, solveDescription, initialAllocationSizes);
           default:
               return CreateSimulationWithConservationType<AngularIntegrationModeNonconserving>(pool, narrowPhaseCallbacks, poseIntegratorCallbacksInterop, solveDescription, initialAllocationSizes);
       }
   }
 */

