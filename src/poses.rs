use std::{os::raw::c_void, default};

use crate::{handles::SimulationHandle, bodies::BodyVelocity};

/// Defines how a pose integrator should handle angular velocity integration.
#[derive(Default)]
#[repr(C)]
pub enum AngularIntegrationMode {
    /// Angular velocity is directly integrated and does not change as the body pose changes. Does not conserve angular momentum.
    #[default]
    NonConserving = 0,
    /// Approximately conserves angular momentum by updating the angular velocity according to the change in orientation. Does a decent job for gyroscopes, but angular velocities will tend to drift towards a minimal inertia axis.
    ConserveMomentum = 1,
	/// Approximately conserves angular momentum by including an implicit gyroscopic torque. Best option for Dzhanibekov effect simulation, but applies a damping effect that can make gyroscopes less useful.
    ConserveMomentumWithGyroscopicTorque = 2,
}

#[derive(Default)]
#[repr(C)]
pub struct PoseIntegratorCallbacks 
{   
    pub angular_integration_mode: AngularIntegrationMode,
    pub allow_substeps_unconstrained: bool,
    
    //Scalar or vectorized callbacks....
    pub integrate_vel_kine: bool,
    pub scalar_callback: bool,

    pub initialize: Option<fn(simulation: SimulationHandle, dt: f32)>,
    pub prepare_for_integration: Option<fn(simulation: SimulationHandle, dt: f32)>,

    //TODO: use Vec, Quat and fix the last argument.
    pub integrate_vel_scalar: Option< extern "C" fn(simulation: SimulationHandle, body_index: i32, position: [f32; 3], orientation: [f32; 4], local_inertia: [f32; 3], worker_index: i32, dt: f32, velocity: BodyVelocity)>,

    //		void (*IntegrateVelocitySIMD128)(SimulationHandle simulation, Vector128I bodyIndices, Vector3SIMD128* positions, QuaternionSIMD128* orientations, BodyInertiaSIMD128* localInertias, Vector128I integrationMask, int32_t workerIndex, Vector128F dt, BodyVelocitySIMD128* bodyVelocities);
    pub integrate_vel_s128: Option<fn(simulation: SimulationHandle, body_index: i32, position: [f32; 3], orientation: [f32; 4], local_inertia: [f32; 3], worker_index: i32, dt: f32, velocity: BodyVelocity)>,

    pub integrate_vel_s256: Option<fn(simulation: SimulationHandle, body_index: i32, position: [f32; 3], orientation: [f32; 4], local_inertia: [f32; 3], worker_index: i32, dt: f32, velocity: BodyVelocity)>,
}