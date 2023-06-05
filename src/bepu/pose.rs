use glam::{Vec3, Quat, Vec3A, IVec3};

use super::{handles::SimulationHandle, bodies::BodyVelocity};
use super::bodies::BodyInertia;

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

#[repr(C)]
pub struct Vector128F {
    V0: f32,
    V1: f32,
    V2: f32,
    V3: f32,
}



#[derive(Default)]
#[repr(C)]
pub struct PoseIntegratorCallbacks 
{   
    pub integration_mode: AngularIntegrationMode,


    pub allow_substeps_unconstrained: bool,
    
    //Scalar or vectorized callbacks....
    pub integrate_vel_kine: bool,

    //Whether the callback is scalar or vectorized...
    pub scalar_callback: bool,

    pub initialize: Option<extern "C" fn(simulation: SimulationHandle, dt: f32)>,
    pub prepare_for_integration: Option<extern "C" fn(simulation: SimulationHandle, dt: f32)>,

    //TODO: use Vec, Quat and fix the last argument.
    pub integrate_vel_scalar: Option< extern "C" 
    fn(simulation: SimulationHandle, body_index: i32, position: Vec3, 
        orientation: Quat, local_inertia: BodyInertia, 
        worker_index: i32, dt: f32, velocity: *mut BodyVelocity)>,


    //void (*IntegrateVelocitySIMD128)(SimulationHandle simulation, Vector128I bodyIndices, Vector3SIMD128* positions, QuaternionSIMD128* orientations, BodyInertiaSIMD128* localInertias, Vector128I integrationMask, int32_t workerIndex, Vector128F dt, BodyVelocitySIMD128* bodyVelocities);

}