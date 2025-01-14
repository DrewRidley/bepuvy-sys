using System.Diagnostics;
using System.Numerics;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Runtime.Intrinsics;
using BepuPhysics;
using BepuPhysics.Collidables;
using BepuPhysics.Constraints;
using BepuUtilities;

namespace Bepuvy.Callbacks;

[StructLayout(LayoutKind.Explicit)]
public unsafe struct PoseIntegratorCallbacksInterop
{
    [FieldOffset(0)]
    public AngularIntegrationMode AngularIntegrationMode;
    [FieldOffset(4)]
    public byte AllowSubstepsForUnconstrainedBodies;
    [FieldOffset(5)]
    public byte IntegrateVelocityForKinematics;
    [FieldOffset(6)]
    public byte UseScalarCallback;

    [FieldOffset(8)]
    public delegate* unmanaged<InstanceHandle, void> Initialize;
    [FieldOffset(16)]
    public delegate* unmanaged<InstanceHandle, float, void> PrepareForIntegration;
    
    [FieldOffset(24)]
    public delegate* unmanaged<InstanceHandle, Vector<int>, Vector3Wide, QuaternionWide, BodyInertiaWide, Vector<int>, int, Vector<float>, BodyVelocityWide*, void*, void> IntegrateVelocity;
}

struct AngularIntegrationModeNonConserving { }
struct AngularIntegrationModeConserve { }
struct AngularIntegrationModeConserveWithGyroTorque { }
struct True { }
struct False { }


//These value typed generic parameters will result in all the branching conditional logic in PoseIntegratorCallbacks getting elided.
public unsafe struct PoseIntegratorCallbacks<TAngularConservationMode, TUnconstrainedSubstepping, TKinematicIntegration, TScalar> : IPoseIntegratorCallbacks
{
    public RustCallback Callback;
    
    public AngularIntegrationMode AngularIntegrationMode
    {
        get
        {
            if (typeof(TAngularConservationMode) == typeof(AngularIntegrationModeConserveWithGyroTorque))
                return AngularIntegrationMode.ConserveMomentumWithGyroscopicTorque;
            if (typeof(TAngularConservationMode) == typeof(AngularIntegrationModeConserve))
                return AngularIntegrationMode.ConserveMomentum;
            return AngularIntegrationMode.Nonconserving;
        }
    }

    public bool AllowSubstepsForUnconstrainedBodies => typeof(TUnconstrainedSubstepping) == typeof(True);

    public bool IntegrateVelocityForKinematics => typeof(TKinematicIntegration) == typeof(True);

    public delegate* unmanaged<InstanceHandle, void> InitializeFunction;
    public delegate* unmanaged<InstanceHandle, float, void> PrepareForIntegrationFunction;
    public void* IntegrateVelocityFunction;
    
    
    public InstanceHandle Simulation;

    public void Initialize(Simulation simulation)
    {
        //No handle exists yet so we can't expose this to the native side.
    }

    public void Initialize(InstanceHandle simulation)
    {
        Simulation = simulation;
        if (InitializeFunction != null)
            InitializeFunction(simulation);
    }

    public void IntegrateVelocity(Vector<int> bodyIndices, Vector3Wide position, QuaternionWide orientation, BodyInertiaWide localInertia, Vector<int> integrationMask, int workerIndex, Vector<float> dt, ref BodyVelocityWide velocity)
    {
        
        var integrateVelocity = (delegate* unmanaged<InstanceHandle, Vector<int>*, Vector3Wide*, QuaternionWide*, BodyInertiaWide*, Vector<int>*, int, Vector<float>*, BodyVelocityWide*, void*, void>) IntegrateVelocityFunction;
        integrateVelocity(Simulation, &bodyIndices, &position, &orientation, &localInertia, &integrationMask, workerIndex, &dt, (BodyVelocityWide*)Unsafe.AsPointer(ref velocity), Callback.Callback);
    }

    public void PrepareForIntegration(float dt)
    {
        //Really SHOULD be a prepare function provided, but it's not technically required like the velocity integration one is.
        if (PrepareForIntegrationFunction != null)
            PrepareForIntegrationFunction(Simulation, dt);
    }
}


public class RustCallback: IDisposable
{
    public unsafe void* Callback;

    public void Dispose()
    {
        // TODO release managed resources here
    }
}
