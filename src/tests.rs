use std::ptr;

use crate::bepu::{
    collisions::*,
    functions::*,
    handles::*,
    pose_integration::{AngularIntegrationMode, PoseIntegratorCallbacks},
    SimulationAllocationSizes, SolveDescription,
};

#[test]
fn test_create_and_destroy_world() {
    // Initialize the physics engine
    unsafe {
        Initialize();
    }

    // Create a buffer pool with specified size and alignment
    let buffer_pool = unsafe { CreateBufferPool(131072, 16) };
    assert!(!buffer_pool.is_null(), "Failed to create buffer pool");

    // Create a thread dispatcher; here with 1 thread and stack size of 16384
    let thread_dispatcher = unsafe { CreateThreadDispatcher(1, 16384) };
    assert!(
        !thread_dispatcher.is_null(),
        "Failed to create thread dispatcher"
    );

    // Example narrow phase and pose integrator callback functions
    unsafe extern "C" fn default_allow_contact(
        _simulation: SimulationHandle,
        _worker_index: i32,
        _a: CollidableReference,
        _b: CollidableReference,
        _speculative_margin: *mut f32,
    ) -> bool {
        true
    }

    let narrow_phase_callbacks = NarrowPhaseCallbacks {
        allow_contact_generation_function: Some(default_allow_contact),
        configure_nonconvex_contact_manifold_function: None,
        initialize_function: None,
        dispose_function: None,
        allow_contact_generation_between_children_function: None,
        configure_child_contact_manifold_function: None,
        configure_convex_contact_manifold_function: None,
    };

    let pose_integrator_callbacks = PoseIntegratorCallbacks {
        angular_integration_mode: AngularIntegrationMode::Nonconserving,
        allow_substeps_for_unconstrained_bodies: false,
        integrate_velocity_for_kinematics: false,
        use_scalar_callback: false,
        initialize: None,
        prepare_for_integration: None,
        integrate_velocity_scalar: None,
        integrate_velocity_simd128: None,
        integrate_velocity_simd256: None,
    };

    // Create a simulation with a custom setup description
    let simulation = unsafe {
        CreateSimulation(
            buffer_pool,
            narrow_phase_callbacks,
            pose_integrator_callbacks,
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
        )
    };

    assert!(!simulation.is_null(), "Failed to create simulation");

    // Clean up resources after the test completes
    unsafe {
        Destroy();
    }
}
