use crate::types::{
    callbacks::{narrow_phase::NarrowPhaseCallbacks, pose::PoseIntegratorCallbacks},
    handles::{BufferPoolHandle, SimulationHandle},
    simulation::{SimulationAllocationSizes, SolveDescription},
};

use super::buffer_pool::BufferPool;

pub mod callbacks;

/// This type holds a handle to a simulation owned by the C# code.
///
/// The lifetime of this struct *is* the lifetime of the simulation,
/// when this struct is dropped the simulation is destroyed.
///
/// In order to operate on the simulation data in C# land unique access is required through a mutable reference.
pub struct Simulation {
    /// The pool used for this simulation.
    ///
    /// Does not represent ownership of the pool,
    /// but ensures that when unique access is used given
    /// it is the correct buffer pool.
    pool: BufferPoolHandle,
    handle: SimulationHandle,
}

impl Drop for Simulation {
    fn drop(&mut self) {
        // SAFETY:
        //
        // The ONLY use of ffi::simulation::create_simulation() is in this constructor, and this type holds the only reference to handles.
        // When this drop is called the handle will no longer be used.
        unsafe {
            crate::ffi::simulation::destroy_simulation(self.handle);
        }
    }
}

impl Simulation {
    pub fn new(
        pool: &BufferPool,
        np_callback: NarrowPhaseCallbacks,
        pose_callback: PoseIntegratorCallbacks,
        solve_desc: SolveDescription,
        alloc_sizes: SimulationAllocationSizes,
    ) -> Self {
        Simulation {
            handle: unsafe {
                crate::ffi::simulation::create_simulation(
                    pool.handle(),
                    np_callback,
                    pose_callback,
                    solve_desc,
                    alloc_sizes,
                )
            },
            pool: pool.handle(),
        }
    }
}
