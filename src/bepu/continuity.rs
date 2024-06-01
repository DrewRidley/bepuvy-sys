/// Defines how a collidable will handle collision detection in the presence of velocity.
#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ContinuousDetectionMode {
    /// No sweep tests are performed. Default speculative contact generation will occur within the speculative margin.
    ///
    /// # Remarks
    ///
    /// The collidable's bounding box will not be expanded by velocity beyond the speculative margin.
    ///
    /// This is the cheapest mode. If a Discrete mode collidable is moving quickly and the maximum speculative margin is limited,
    /// the fact that its bounding box is not expanded may cause it to miss a collision even with a non-Discrete collidable.
    Discrete = 0,
    /// No sweep tests are performed. Default speculative contact generation will occur within the speculative margin.
    ///
    /// # Remarks
    ///
    /// The collidable's bounding box will be expanded by velocity without being limited by the speculative margin.
    ///
    /// This is useful when a collidable may move quickly and does not itself require continuous detection, but there exist other collidables with continuous modes
    /// that should avoid missing collisions.
    Passive = 1,
    /// Collision detection will start with a sweep test to identify a likely time of impact. Speculative contacts will be generated for the predicted collision.
    ///
    /// # Remarks
    ///
    /// This mode can capture angular motion with very few ghost collisions. It can, however, miss secondary collisions that would have occurred due to the primary impact's velocity change.
    Continuous = 2,
}

/// Defines how a collidable handles collisions with significant velocity.
#[repr(C)]
pub struct ContinuousDetection {
    /// The continuous collision detection mode.
    pub mode: ContinuousDetectionMode,
    /// If using `ContinuousDetectionMode::Continuous`, this defines the minimum progress that the sweep test will make when searching for the first time of impact.
    /// Collisions lasting less than `minimum_sweep_timestep` may be missed by the sweep test. Using larger values can significantly increase the performance of sweep tests.
    pub minimum_sweep_timestep: f32,
    /// If using `ContinuousDetectionMode::Continuous`, sweep tests will terminate if the time of impact region has been refined to be smaller than `sweep_convergence_threshold`.
    /// Values closer to zero will converge more closely to the true time of impact, but for speculative contact generation larger values usually work fine.
    /// Larger values allow the sweep to terminate much earlier and can significantly improve sweep performance.
    pub sweep_convergence_threshold: f32,
}

impl ContinuousDetection {
    /// Gets whether the continuous collision detection configuration will permit bounding box expansion beyond the calculated speculative margin.
    pub fn allow_expansion_beyond_speculative_margin(&self) -> bool {
        self.mode > ContinuousDetectionMode::Discrete
    }

    /// No sweep tests are performed. Default speculative contact generation will occur within the speculative margin.
    ///
    /// # Returns
    ///
    /// Detection settings for the given discrete configuration.
    ///
    /// # Remarks
    ///
    /// The collidable's bounding box will not be expanded by velocity beyond the speculative margin.
    ///
    /// This can be marginally cheaper than Passive modes if using a limited maximum speculative margin. If a Discrete mode collidable is moving quickly and the maximum speculative margin is limited,
    /// the fact that its bounding box is not expanded may cause it to miss a collision even with a non-Discrete collidable.
    ///
    /// Note that Discrete and Passive only differ if maximum speculative margin is restricted.
    pub fn discrete() -> Self {
        Self {
            mode: ContinuousDetectionMode::Discrete,
            minimum_sweep_timestep: 0.0,
            sweep_convergence_threshold: 0.0,
        }
    }

    /// No sweep tests are performed. Default speculative contact generation will occur within the speculative margin.
    ///
    /// # Returns
    ///
    /// Detection settings for the passive configuration.
    ///
    /// # Remarks
    ///
    /// The collidable's bounding box and speculative margin will be expanded by velocity.
    ///
    /// This is useful when a collidable may move quickly and does not itself require continuous detection, but there exist other collidables with continuous modes that should avoid missing collisions.
    pub fn passive() -> Self {
        Self {
            mode: ContinuousDetectionMode::Passive,
            minimum_sweep_timestep: 0.0,
            sweep_convergence_threshold: 0.0,
        }
    }

    /// Collision detection will start with a sweep test to identify a likely time of impact. Speculative contacts will be generated for the predicted collision.
    ///
    /// # Arguments
    ///
    /// * `minimum_sweep_timestep`: Minimum progress that the sweep test will make when searching for the first time of impact.
    /// Collisions lasting less than `minimum_sweep_timestep` may be missed by the sweep test. Using larger values can significantly increase the performance of sweep tests.
    /// * `sweep_convergence_threshold`: Threshold against which the time of impact region is compared for sweep termination.
    /// If the region has been refined to be smaller than `sweep_convergence_threshold`, the sweep will terminate.
    /// Values closer to zero will converge more closely to the true time of impact, but for speculative contact generation larger values usually work fine.
    /// Larger values allow the sweep to terminate much earlier and can significantly improve sweep performance.
    ///
    /// # Returns
    ///
    /// Detection settings for the given continuous configuration.
    ///
    /// # Remarks
    ///
    /// This mode can capture angular motion with very few ghost collisions. It can, however, miss secondary collisions that would have occurred due to the primary impact's velocity change.
    pub fn continuous(minimum_sweep_timestep: f32, sweep_convergence_threshold: f32) -> Self {
        Self {
            mode: ContinuousDetectionMode::Continuous,
            minimum_sweep_timestep,
            sweep_convergence_threshold,
        }
    }
}
