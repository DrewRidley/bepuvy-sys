
pub mod handles;
pub mod collisions;
pub mod pose;
pub mod constraints;
pub mod bodies;
pub mod tree;
pub mod math;
pub mod continuity;
pub mod statics;
pub mod shapes;

use handles::*;
use collisions::*;
use pose::*;
use constraints::*;
use bodies::*;
use math::*;

use self::{statics::StaticDescription, shapes::BepuBox};

#[repr(C)]
pub struct SimAllocSize {
    bodies: i32,
    statics: i32,
    islands: i32,
    shapes_per_type: i32,
    constraints: i32,
    constraints_per_type: i32,
    constraints_per_body_estimate: i32,
}

#[repr(C)]
pub struct SolveDesc {
    pub vel_iter_count: i32,
    pub subtep_count: i32,
    pub fallback_batch_thresh: i32,
    pub vel_iter_scheduler: Option<extern "C" fn(i32) -> i32>,
}

impl Default for SimAllocSize {
    fn default() -> Self {
        Self {
            bodies: 4906,
            statics: 4906,
            islands: 16,
            shapes_per_type: 128,
            constraints: 16384,
            constraints_per_type: 256,
            constraints_per_body_estimate: 8,
        }
    }
}


extern "C" {
    pub fn Initialize();
    pub fn GetPlatformThreadCount() -> i32;

    pub fn CreateBufferPool(minBlockAllocSize: i32, expectedUsedSlots: i32) -> BufferPoolHandle;

    pub fn CreateThreadDispatcher(threadCount: i32, threadPoolAllocationBlocks: i32) -> ThreadDispatcherHandle;

    pub fn AddStatic(handle: SimulationHandle, desc: StaticDescription)-> TypedIndex;

    pub fn AddBox(handle: SimulationHandle, boxy: BepuBox) -> TypedIndex;

    pub fn AddBody(handle: SimulationHandle, desc: BodyDescription) -> BodyHandle;

    pub fn AddBody_Debug(handle: SimulationHandle);

    pub fn Destroy();

    pub fn Timestep(handle: SimulationHandle, dt: f32, dispatcher: ThreadDispatcherHandle);

    pub fn GetBodyDynamics(handle: SimulationHandle, body: BodyHandle) -> &'static BodyDynamics;

    pub fn CreateSimulation(
        pool: BufferPoolHandle, 
        narrow_callbacks: NarrowPhaseCallbacks,
        pose_callbacks: PoseIntegratorCallbacks,
        solve_desc: SolveDesc,
        sim_alloc_size: SimAllocSize
    ) -> SimulationHandle;
}

