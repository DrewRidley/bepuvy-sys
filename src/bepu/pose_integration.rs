use super::bodies::*;
use super::handles::*;
use super::interop_math::*;

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
pub struct PoseIntegratorCallbacks {
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
    /// If false, `IntegrateVelocitySIMD128` or `IntegrateVelocitySIMD256` will be called.
    /// Use `GetSIMDWidth` to know which vectorized callback would be invoked.
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
    pub integrate_velocity_scalar: Option<
        unsafe extern "C" fn(
            simulation: SimulationHandle,
            body_index: i32,
            position: Vector3,
            orientation: Quaternion,
            local_inertia: BodyInertia,
            worker_index: i32,
            dt: f32,
            velocity: *mut BodyVelocity,
        ),
    >,
    /// Called for every active body bundle during each integration pass when `use_scalar_callback` is false and SIMD width is 128.
    ///
    /// # Arguments
    ///
    /// * `simulation`: Simulation to which these callbacks belong.
    /// * `body_indices`: Current indices of the body bundle being integrated in the active body set. This is distinct from the `BodyHandle`; the body index can change over time.
    /// * `positions`: Current positions of the body bundle.
    /// * `orientations`: Current orientations of the body bundle.
    /// * `local_inertias`: Inertia properties of the body bundle in their local space.
    /// * `integration_mask`: Mask indicating which bodies in the bundle should be integrated.
    /// * `worker_index`: Index of the thread worker processing this callback.
    /// * `dt`: Timestep duration that subsequent velocity integrations will be invoked with.
    /// * `body_velocities`: Velocity of the body bundle to be updated by this callback.
    pub integrate_velocity_simd128: Option<
        unsafe extern "C" fn(
            simulation: SimulationHandle,
            body_indices: Vector128I,
            positions: *mut Vector3SIMD128,
            orientations: *mut QuaternionSIMD128,
            local_inertias: *mut BodyInertiaSIMD128,
            integration_mask: Vector128I,
            worker_index: i32,
            dt: Vector128F,
            body_velocities: *mut BodyVelocitySIMD128,
        ),
    >,
    /// Called for every active body bundle during each integration pass when `use_scalar_callback` is false and SIMD width is 256.
    ///
    /// # Arguments
    ///
    /// * `simulation`: Simulation to which these callbacks belong.
    /// * `body_indices`: Current indices of the body bundle being integrated in the active body set. This is distinct from the `BodyHandle`; the body index can change over time.
    /// * `positions`: Current positions of the body bundle.
    /// * `orientations`: Current orientations of the body bundle.
    /// * `local_inertias`: Inertia properties of the body bundle in their local space.
    /// * `integration_mask`: Mask indicating which bodies in the bundle should be integrated.
    /// * `worker_index`: Index of the thread worker processing this callback.
    /// * `dt`: Timestep duration that subsequent velocity integrations will be invoked with.
    /// * `body_velocities`: Velocity of the body bundle to be updated by this callback.
    pub integrate_velocity_simd256: Option<
        unsafe extern "C" fn(
            simulation: SimulationHandle,
            body_indices: Vector256I,
            positions: *mut Vector3SIMD256,
            orientations: *mut QuaternionSIMD256,
            local_inertias: *mut BodyInertiaSIMD256,
            integration_mask: Vector256I,
            worker_index: i32,
            dt: Vector256F,
            body_velocities: *mut BodyVelocitySIMD256,
        ),
    >,
}
