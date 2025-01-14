use super::bodies::*;
use super::handles::*;
use super::interop_math::*;
use super::shapes::Vector3Wide;
use std::simd::LaneCount;
use std::simd::Simd;
use std::simd::SupportedLaneCount;

/// Defines how a pose integrator should handle angular velocity integration.
#[repr(C)]
#[derive(PartialEq, Eq)]
pub enum AngularIntegrationMode {
    /// Angular velocity is directly integrated and does not change as the body pose changes. Does not conserve angular momentum.
    Nonconserving = 0,
    /// Approximately conserves angular momentum by updating the angular velocity according to the change in orientation. Does a decent job for gyroscopes, but angular velocities will tend to drift towards a minimal inertia axis.
    ConserveMomentum = 1,
    /// Approximately conserves angular momentum by including an implicit gyroscopic torque. Best option for Dzhanibekov effect simulation, but applies a damping effect that can make gyroscopes less useful.
    ConserveMomentumWithGyroscopicTorque = 2,
}

/// Defines pose integrator state and callbacks.
#[repr(C)]
pub struct PoseIntegratorCallbacks<const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    /// How the pose integrator should handle angular velocity integration.
    pub angular_integration_mode: AngularIntegrationMode,
    /// Whether the integrator should use only one step for unconstrained bodies when using a substepping solver.
    /// If true, unconstrained bodies use a single step of length equal to the dt provided to `Simulation::timestep`.
    /// If false, unconstrained bodies will be integrated with the same number of substeps as the constrained bodies in the solver.
    pub allow_substeps_for_unconstrained_bodies: bool,
    /// Whether the velocity integration callback should be called for kinematic bodies.
    /// If true, `IntegrateVelocity` will be called for bundles including kinematic bodies.
    /// If false, kinematic bodies will just continue using whatever velocity they have set.
    /// Most use cases should set this to false.
    pub integrate_velocity_for_kinematics: bool,
    /// Whether to use a scalar or vectorized integrator callback. If true, `IntegrateVelocityScalar` will be used.
    /// The scalar callback has much higher overhead due to the required data transpositions.
    /// If false, `IntegrateVelocitySIMD` will be called.
    pub use_scalar_callback: bool,
    /// Called after the simulation is created.
    ///
    /// # Arguments
    ///
    /// * `simulation`: Simulation to which these callbacks belong.
    pub initialize: Option<unsafe extern "C" fn(simulation: SimulationHandle)>,
    /// Called before each simulation stage which could execute velocity integration.
    ///
    /// # Arguments
    ///
    /// * `simulation`: Simulation to which these callbacks belong.
    /// * `dt`: Timestep duration that subsequent velocity integrations will be invoked with.
    pub prepare_for_integration:
        Option<unsafe extern "C" fn(simulation: SimulationHandle, dt: f32)>,
    /// Called for every active body during each integration pass when `use_scalar_callback` is true.
    ///
    /// # Arguments
    ///
    /// * `simulation`: Simulation to which these callbacks belong.
    /// * `body_index`: Current index of the body being integrated in the active body set. This is distinct from the `BodyHandle`; the body index can change over time.
    /// * `position`: Current position of the body.
    /// * `orientation`: Current orientation of the body.
    /// * `local_inertia`: Inertia properties of the body in its local space.
    /// * `worker_index`: Index of the thread worker processing this callback.
    /// * `dt`: Timestep duration that subsequent velocity integrations will be invoked with.
    /// * `velocity`: Velocity of the body to be updated by this callback.
    pub integrate_velocity: unsafe extern "C" fn(
        simulation: SimulationHandle,
        body_index: *mut Simd<i32, N>,
        position: *mut Vector3Wide<N>,
        orientation: *mut Quaternion,
        local_inertia: *mut BodyInertia,
        worker_index: i32,
        dt: f32,
        velocity: *mut BodyVelocity,
        data: *mut (),
    ),
}
