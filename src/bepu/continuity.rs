
#[repr(u32)]
#[derive(Ord, PartialOrd, PartialEq, Eq, Clone)]
pub enum ContinuousDetectionMode {
    Discrete,
    Passive,
    Continuous
}

#[repr(C)]
#[derive(Clone)]
pub struct ContinuousDetection {
    mode: ContinuousDetectionMode,
    minimum_sweep_timestep: f32,
    sweep_convergence_threshold: f32,
}

impl ContinuousDetection {
    pub fn allow_expansion_beyond_speculative_margin(&self) -> bool {
        return self.mode > ContinuousDetectionMode::Discrete;
    }

    pub fn discrete() -> ContinuousDetection {
        ContinuousDetection {
            mode: ContinuousDetectionMode::Discrete,
            minimum_sweep_timestep: 0.0,
            sweep_convergence_threshold: 0.0,
        }
    }

    pub fn passive() -> ContinuousDetection {
        ContinuousDetection {
            mode: ContinuousDetectionMode::Passive,
            minimum_sweep_timestep: 0.0,
            sweep_convergence_threshold: 0.0,
        }
    }

    pub fn continuous(minimum_sweep_timestep: f32, sweep_convergence_threshold: f32) -> ContinuousDetection {
        ContinuousDetection {
            mode: ContinuousDetectionMode::Continuous,
            minimum_sweep_timestep,
            sweep_convergence_threshold,
        }
    }
}