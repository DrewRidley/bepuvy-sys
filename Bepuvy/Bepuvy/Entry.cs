using System.Numerics;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using BepuPhysics;
using BepuPhysics.Collidables;
using BepuPhysics.CollisionDetection;
using BepuPhysics.Constraints;
using BepuUtilities;
using BepuUtilities.Memory;

namespace Bepuvy;

// FFI-compatible function pointers for custom pose integrator callbacks
[StructLayout(LayoutKind.Sequential)]
public unsafe struct CustomPoseIntegratorCallbacksVTable
{
    public delegate* unmanaged<IntPtr, void> Initialize;
    public delegate* unmanaged<IntPtr, float, void> PrepareForIntegration;
    public delegate* unmanaged<IntPtr, IntPtr, IntPtr, IntPtr, IntPtr, IntPtr, int, IntPtr, IntPtr, void> IntegrateVelocity;
}


public unsafe struct InteropPoseIntegratorCallbacks : IPoseIntegratorCallbacks
{
    public IntPtr CustomCallbacksPtr; // Pointer to the Rust struct implementing the callbacks
    public CustomPoseIntegratorCallbacksVTable VTable;

    public InteropPoseIntegratorCallbacks(IntPtr callbacksPtr, CustomPoseIntegratorCallbacksVTable vtable)
    {
        CustomCallbacksPtr = callbacksPtr;
        VTable = vtable;
    }

    public void Initialize(Simulation simulation)
    {
        VTable.Initialize(CustomCallbacksPtr);
    }

    public void PrepareForIntegration(float dt)
    {
        VTable.PrepareForIntegration(CustomCallbacksPtr, dt);
    }

    public unsafe void IntegrateVelocity(Vector<int> bodyIndices, Vector3Wide position, QuaternionWide orientation,
        BodyInertiaWide localInertia, Vector<int> integrationMask, int workerIndex, Vector<float> dt,
        ref BodyVelocityWide velocity)
    {
        // Create temporary arrays to hold the data
        int[] bodyIndicesArray = new int[bodyIndices.Length];
        float[] positionArray = new float[position.Length * 3]; // 3 floats per Vector3
        float[] orientationArray = new float[orientation.Length * 4]; // 4 floats per Quaternion
        float[] localInertiaArray = new float[16]; // Assuming 4x4 matrix
        int[] integrationMaskArray = new int[integrationMask.Length];
        float[] dtArray = new float[dt.Length];
        float[] velocityArray = new float[velocity.Linear.Length * 3]; // 3 floats per Vector3

        // Copy data from the Vector<T> and other types into the arrays
        bodyIndices.CopyTo(bodyIndicesArray);
        // ... (similarly copy data from position, orientation, localInertia, integrationMask, dt, velocity.Linear)

        unsafe
        {
            // Fix the pointers to the arrays
            fixed (int* bodyIndicesPtr = &bodyIndicesArray[0])
            fixed (float* positionPtr = &positionArray[0])
            fixed (float* orientationPtr = &orientationArray[0])
            fixed (float* localInertiaPtr = &localInertiaArray[0])
            fixed (int* integrationMaskPtr = &integrationMaskArray[0])
            fixed (float* dtPtr = &dtArray[0])
            fixed (float* velocityPtr = &velocityArray[0])
            {
                VTable.IntegrateVelocity(
                    CustomCallbacksPtr,
                    (IntPtr)bodyIndicesPtr,
                    (IntPtr)positionPtr,
                    (IntPtr)orientationPtr,
                    (IntPtr)localInertiaPtr,
                    (IntPtr)integrationMaskPtr,
                    workerIndex,
                    (IntPtr)dtPtr,
                    (IntPtr)velocityPtr
                );
            }
        }

        // Copy the potentially modified data back from the arrays (if necessary)
        // ...
    }

    public AngularIntegrationMode AngularIntegrationMode => AngularIntegrationMode.Nonconserving;
    public bool AllowSubstepsForUnconstrainedBodies => false;
    public bool IntegrateVelocityForKinematics => false;
}

public unsafe struct InteropNarrowPhaseCallbacks : INarrowPhaseCallbacks
{
    public SpringSettings ContactSpringiness;
    public float MaximumRecoveryVelocity;
    public float FrictionCoefficient;

    public InteropNarrowPhaseCallbacks(SpringSettings contactSpringiness, float maximumRecoveryVelocity = 2f, float frictionCoefficient = 1f)
    {
        ContactSpringiness = contactSpringiness;
        MaximumRecoveryVelocity = maximumRecoveryVelocity;
        FrictionCoefficient = frictionCoefficient;
    }

    public void Initialize(Simulation simulation)
    {
        //Use a default if the springiness value wasn't initialized... at least until struct field initializers are supported outside of previews.
        if (ContactSpringiness.AngularFrequency == 0 && ContactSpringiness.TwiceDampingRatio == 0)
        {
            ContactSpringiness = new(30, 1);
            MaximumRecoveryVelocity = 2f;
            FrictionCoefficient = 1f;
        }
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public bool AllowContactGeneration(int workerIndex, CollidableReference a, CollidableReference b, ref float speculativeMargin)
    {
        //While the engine won't even try creating pairs between statics at all, it will ask about kinematic-kinematic pairs.
        //Those pairs cannot emit constraints since both involved bodies have infinite inertia. Since most of the demos don't need
        //to collect information about kinematic-kinematic pairs, we'll require that at least one of the bodies needs to be dynamic.
        return a.Mobility == CollidableMobility.Dynamic || b.Mobility == CollidableMobility.Dynamic;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public bool AllowContactGeneration(int workerIndex, CollidablePair pair, int childIndexA, int childIndexB)
    {
        return true;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public unsafe bool ConfigureContactManifold<TManifold>(int workerIndex, CollidablePair pair, ref TManifold manifold, out PairMaterialProperties pairMaterial) where TManifold : unmanaged, IContactManifold<TManifold>
    {
        pairMaterial.FrictionCoefficient = FrictionCoefficient;
        pairMaterial.MaximumRecoveryVelocity = MaximumRecoveryVelocity;
        pairMaterial.SpringSettings = ContactSpringiness;
        return true;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public bool ConfigureContactManifold(int workerIndex, CollidablePair pair, int childIndexA, int childIndexB, ref ConvexContactManifold manifold)
    {
        return true;
    }

    public void Dispose()
    {
    }
}

public class SimulationInstance
{
    private BufferPool _pool;
    public ThreadDispatcher Dispatcher;

    public Simulation Sim;

    public Dictionary<string, TypedIndex> Shapes = new();
    private Dictionary<int, BodyInertia> _bodyInertias = new();

    public SimulationInstance(InteropPoseIntegratorCallbacks poseIntegratorCallbacks)
    {
        _pool = new BufferPool();
        Dispatcher = new ThreadDispatcher(2);

        var springSettings = new SpringSettings(30, 1);
        var narrowCallbacks = new InteropNarrowPhaseCallbacks(springSettings);
        var solveDesc = new SolveDescription(8, 1);
        Sim = BepuPhysics.Simulation.Create(_pool, narrowCallbacks, poseIntegratorCallbacks, solveDesc);

        Sim.Statics.Add(new StaticDescription(new Vector3(0, -0.5f, 0),
            Sim.Shapes.Add(new Box(2500, 1, 2500))));
    }

    // ... (Rest of the SimulationInstance class remains the same)
}


public static class Entry
{
    public static SimulationInstance? simulation;

    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) },
        EntryPoint = nameof(CreateSimulation))]
    public unsafe static void CreateSimulation(CustomPoseIntegratorCallbacksVTable* callbacksVTable)
    {
        var callbacks = new InteropPoseIntegratorCallbacks(IntPtr.Zero, *callbacksVTable);
        simulation = new SimulationInstance(callbacks);
    }

    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) },
        EntryPoint = nameof(Cleanup))]
    public static void Cleanup()
    {
        simulation = null;
    }
}
