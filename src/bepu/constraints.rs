
//Bepu Pi?
const BPI: f32 = 3.14159265359;


#[derive(Default)]
#[repr(C)]
pub struct SpringSettings {
    angular_freq: f32,
    twice_damping_ratio: f32
}

impl SpringSettings {
    pub fn get_freq(&self) -> f32 {
        self.angular_freq / (2.0 * BPI)
    }

    pub fn set_freq(&mut self, freq: f32) {
        self.angular_freq = freq * (2.0 * BPI)
    }

    pub fn get_damping_ratio(&self) -> f32 {
        self.twice_damping_ratio / 2.0
    }

    pub fn set_damping_ratio(&mut self, ratio: f32) {
        self.twice_damping_ratio = ratio * 2.0
    }

    pub fn new(freq: f32, damping_ratio: f32) -> Self {
        Self {
            angular_freq: freq * (2.0 * BPI),
            twice_damping_ratio: damping_ratio * 2.0
        }
    }
}