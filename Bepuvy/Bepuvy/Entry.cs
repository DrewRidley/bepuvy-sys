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



public struct InteropPoseIntegratorCallbacks : IPoseIntegratorCallbacks
{
    /// <summary>
    /// Gravity to apply to dynamic bodies in the simulation.
    /// </summary>
    public Vector3 Gravity;
    /// <summary>
    /// Fraction of dynamic body linear velocity to remove per unit of time. Values range from 0 to 1. 0 is fully undamped, while values very close to 1 will remove most velocity.
    /// </summary>
    public float LinearDamping;
    /// <summary>
    /// Fraction of dynamic body angular velocity to remove per unit of time. Values range from 0 to 1. 0 is fully undamped, while values very close to 1 will remove most velocity.
    /// </summary>
    public float AngularDamping;

    
    public InteropPoseIntegratorCallbacks(Vector3 gravity, float linearDamping = .03f, float angularDamping = .03f) : this()
    {
        Gravity = gravity;
        LinearDamping = linearDamping;
        AngularDamping = angularDamping;
    }
    
    

    
    public void Initialize(Simulation simulation)
    {
        
    }
    
    Vector3Wide gravityWideDt;
    Vector<float> linearDampingDt;
    Vector<float> angularDampingDt;


    public void PrepareForIntegration(float dt)
    {
        linearDampingDt = new Vector<float>(MathF.Pow(MathHelper.Clamp(1 - LinearDamping, 0, 1), dt));
        angularDampingDt = new Vector<float>(MathF.Pow(MathHelper.Clamp(1 - AngularDamping, 0, 1), dt));
        gravityWideDt = Vector3Wide.Broadcast(Gravity * dt);
    }

    public void IntegrateVelocity(Vector<int> bodyIndices, Vector3Wide position, QuaternionWide orientation,
        BodyInertiaWide localInertia, Vector<int> integrationMask, int workerIndex, Vector<float> dt, ref BodyVelocityWide velocity)
    {
        velocity.Linear = (velocity.Linear + gravityWideDt) * linearDampingDt;
        velocity.Angular *= angularDampingDt;
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


public class PyramidSimulation
{
    private BufferPool pool;
    public ThreadDispatcher dispatcher;

    public Simulation sim;

    public TypedIndex boxIndex;
    public BodyInertia boxInertia;
    public Box boxShape;
    
    public PyramidSimulation()
    {
        pool = new BufferPool();
        dispatcher = new ThreadDispatcher(2);

        var springsettings = new SpringSettings(30, 1);
        var narrowcallbacks = new InteropNarrowPhaseCallbacks(springsettings);
        var posecallbacks = new InteropPoseIntegratorCallbacks(new Vector3(0, -10, 0));
        var solvedesc = new SolveDescription(8, 1);
        sim = BepuPhysics.Simulation.Create(pool, narrowcallbacks, posecallbacks, solvedesc);
        
        
        boxShape = new Box(1, 1, 1);
        boxInertia = boxShape.ComputeInertia(1);
        boxIndex = sim.Shapes.Add(boxShape);

        
        sim.Statics.Add(new StaticDescription(new Vector3(0, -0.5f, 0), sim.Shapes.Add(new Box(2500, 1, 2500))));
    }
}


public static class Entry
{
    public static PyramidSimulation pyramid;
    
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) },
        EntryPoint = nameof(SetupPyramidDemo))]
    public static void SetupPyramidDemo()
    {
        pyramid = new PyramidSimulation();
    }

    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) },
        EntryPoint = nameof(SpawnCube))]
    public static int SpawnCube(Vector3 pose)
    {
        BodyHandle handle = pyramid.sim.Bodies.Add(BodyDescription.CreateDynamic(pose,
            pyramid.boxInertia, pyramid.boxIndex, -0.01f));

        return handle.Value;
    }

    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) },
        EntryPoint = nameof(Timestep))]
    public static void Timestep()
    {
        pyramid.sim.Timestep(1f / 60f, pyramid.dispatcher);
    }
    
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) },
        EntryPoint = nameof(GetBodyPose))]
    public static RigidPose GetBodyPose(int handle)
    {
        var bodyHandle = new BodyHandle(handle);
        return pyramid.sim.Bodies.GetDescription(bodyHandle).Pose;
    }
}