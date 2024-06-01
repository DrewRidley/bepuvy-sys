/// Represents the constant PI * 2
pub const TWO_PI: f32 = std::f32::consts::PI * 2.0;

/// Settings for a spring.
#[repr(C)]
pub struct SpringSettings {
    /// Target number of undamped oscillations per unit of time, scaled by 2 * PI.
    pub angular_frequency: f32,
    /// Twice the ratio of the spring's actual damping to its critical damping.
    pub twice_damping_ratio: f32,
}

impl SpringSettings {
    /// Gets the target number of undamped oscillations per unit of time.
    pub fn frequency(&self) -> f32 {
        self.angular_frequency / TWO_PI
    }

    /// Sets the target number of undamped oscillations per unit of time.
    pub fn set_frequency(&mut self, value: f32) {
        self.angular_frequency = value * TWO_PI;
    }

    /// Gets the ratio of the spring's actual damping to its critical damping. 0 is undamped, 1 is critically damped, and higher values are overdamped.
    pub fn damping_ratio(&self) -> f32 {
        self.twice_damping_ratio / 2.0
    }

    /// Sets the ratio of the spring's actual damping to its critical damping. 0 is undamped, 1 is critically damped, and higher values are overdamped.
    pub fn set_damping_ratio(&mut self, value: f32) {
        self.twice_damping_ratio = value * 2.0;
    }

    /// Constructs a new spring settings instance.
    ///
    /// # Arguments
    ///
    /// * `frequency`: Target number of undamped oscillations per unit of time.
    /// * `damping_ratio`: Ratio of the spring's actual damping to its critical damping. 0 is undamped, 1 is critically damped, and higher values are overdamped.
    pub fn new(frequency: f32, damping_ratio: f32) -> Self {
        Self {
            angular_frequency: frequency * TWO_PI,
            twice_damping_ratio: damping_ratio * 2.0,
        }
    }
}

impl Default for SpringSettings {
    fn default() -> Self {
        Self {
            angular_frequency: 0.0,
            twice_damping_ratio: 0.0,
        }
    }
}
