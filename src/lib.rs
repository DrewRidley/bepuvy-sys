#![feature(portable_simd)]

use std::{simd::Simd, sync::Arc};

use bepu::{
    collisions::{
        CollidableMobility, CollidablePair, CollidableReference, ConvexContactManifold,
        NarrowPhaseCallbacks, NonconvexContactManifold, PairMaterialProperties,
    },
    constraints::SpringSettings,
    functions::{create_buffer_pool, create_simulation},
    handles::{InstanceHandle, SimulationHandle},
    pose_integration::{AngularIntegrationMode, PoseIntegratorCallbacks},
    shapes::Vector3Wide,
    SimulationAllocationSizes, SolveDescription, WIDEST_LANE,
};
mod bepu;

fn create_default_narrow_phase_callbacks() -> NarrowPhaseCallbacks {
    unsafe extern "C" fn allow_contact_generation(
        _simulation: SimulationHandle,
        _worker_index: i32,
        a: CollidableReference,
        b: CollidableReference,
        _speculative_margin: *mut f32,
    ) -> bool {
        a.mobility() == CollidableMobility::Dynamic || b.mobility() == CollidableMobility::Dynamic
    }

    unsafe extern "C" fn allow_contact_generation_between_children(
        _simulation: SimulationHandle,
        _worker_index: i32,
        _pair: CollidablePair,
        _child_index_a: i32,
        _child_index_b: i32,
    ) -> bool {
        true
    }

    unsafe extern "C" fn configure_contact_manifold(
        _simulation: SimulationHandle,
        _worker_index: i32,
        _pair: CollidablePair,
        _manifold: *mut ConvexContactManifold,
        material: *mut PairMaterialProperties,
    ) -> bool {
        (*material).friction_coefficient = 1.0;
        (*material).maximum_recovery_velocity = 2.0;
        (*material).contact_spring_settings = SpringSettings::new(30.0, 1.0);
        true
    }

    unsafe extern "C" fn configure_nonconvex_contact_manifold(
        _simulation: SimulationHandle,
        _worker_index: i32,
        _pair: CollidablePair,
        _manifold: *mut NonconvexContactManifold,
        material: *mut PairMaterialProperties,
    ) -> bool {
        (*material).friction_coefficient = 1.0;
        (*material).maximum_recovery_velocity = 2.0;
        (*material).contact_spring_settings = SpringSettings::new(30.0, 1.0);
        true
    }

    unsafe extern "C" fn configure_child_contact_manifold(
        _simulation: SimulationHandle,
        _worker_index: i32,
        _pair: CollidablePair,
        _child_index_a: i32,
        _child_index_b: i32,
        _manifold: *mut ConvexContactManifold,
    ) -> bool {
        true
    }

    NarrowPhaseCallbacks {
        initialize_function: None,
        dispose_function: None,
        allow_contact_generation_function: Some(allow_contact_generation),
        allow_contact_generation_between_children_function: Some(
            allow_contact_generation_between_children,
        ),
        configure_convex_contact_manifold_function: Some(configure_contact_manifold),
        configure_nonconvex_contact_manifold_function: Some(configure_nonconvex_contact_manifold),
        configure_child_contact_manifold_function: Some(configure_child_contact_manifold),
    }
}

fn create_default_pose_integrator_callbacks() -> PoseIntegratorCallbacks<WIDEST_LANE> {
    PoseIntegratorCallbacks {
        angular_integration_mode: AngularIntegrationMode::Nonconserving,
        allow_substeps_for_unconstrained_bodies: false,
        integrate_velocity_for_kinematics: false,
        use_scalar_callback: true,
        initialize: None,
        prepare_for_integration: None,
        integrate_velocity: None,
    }
}

/*
public void IntegrateVelocity(Vector<int> bodyIndices, Vector3Wide position, QuaternionWide orientation, BodyInertiaWide localInertia, Vector<int> integrationMask, int workerIndex, Vector<float> dt, ref BodyVelocityWide velocity)
{
    var integrateVelocity = (delegate* unmanaged<InstanceHandle, Vector<int>*, Vector3Wide*, QuaternionWide*, BodyInertiaWide*, Vector<int>*, int, Vector<float>*, BodyVelocityWide*, void*, void>) IntegrateVelocityFunction;
    integrateVelocity(Simulation, &bodyIndices, &position, &orientation, &localInertia, &integrationMask, workerIndex, &dt, (BodyVelocityWide*)Unsafe.AsPointer(ref velocity), Callback.Callback);
}
*/

/*
public struct QuaternionWide
{
    public Vector<float> X;
    public Vector<float> Y;
    public Vector<float> Z;
    public Vector<float> W;
}

public struct QuaternionWide<const N: usize> {
    x: Simd<f32, N>
}
*/

unsafe extern "C" fn integrate_velocity(
    sim: SimulationHandle,
    body_idx: *mut Simd<i32, WIDEST_LANE>,
    position: *mut Vector3Wide<WIDEST_LANE>,
    orientation: *mut QuaternionWide<WIDEST_LANE>,
    localInertia: *mut BodyInertiaWide<WIDEST_LANE>,
    integrationMask: *mut Simd<i32, WIDEST_LANE>,
    workerIdx: i32,
    dt: *mut Simd<f32, WIDEST_LANE>,
    velocities: *mut BodyVelocityWide<WIDEST_LANE>,
) {
}

unsafe fn example() {
    let buffer_pool = create_buffer_pool(131072, 16);

    let sim = create_simulation(
        buffer_pool,
        create_default_narrow_phase_callbacks(),
        create_default_pose_integrator_callbacks(),
        SolveDescription {
            velocity_iteration_count: 8,
            substep_count: 1,
            fallback_batch_threshold: 128,
            velocity_iteration_scheduler: None,
        },
        SimulationAllocationSizes {
            bodies: 4096,
            statics: 4096,
            islands: 4096,
            shapes_per_type: 128,
            constraints: 4096,
            constraints_per_type_batch: 128,
            constraint_count_per_body_estimate: 8,
        },
    );

    //let buffer_pool = unsafe { CreateBufferPool(131072, 16) };
}
