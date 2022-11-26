use crate::signals::Signal;
use std::f32::consts::PI;

pub struct Sine {
    amp: f32,
    hz: f32,
    phase: f32, // in radians
}

impl Sine {
    pub fn new(amp: f32, hz: f32, phase: f32) -> Self {
        Self {
            amp: amp,
            hz: hz,
            phase: phase,
        }
    }

    /// Creates a default 440Hz sine wave with unit (1) amplitude and zero phase.
    pub fn default() -> Self {
        Self::new(1.0, 440.0, 0.0)
    }

    /// Returns a Sine wave phased backward 1.5pi radians.
    pub fn from_cos(amp: f32, hz: f32, phase: f32) -> Self {
        Self::new(amp, hz, phase - 1.5 * PI)
    }

    /// Returns a Sine wave phased forward 1.5pi radians.
    pub fn to_cos(amp: f32, hz: f32, phase: f32) -> Self {
        Self::new(amp, hz, phase + 1.5 * PI)
    }
}

impl Signal for Sine {
    fn sample_rads(&self, rads: f32) -> f32 {
        self.amp * (self.hz * (rads + self.phase)).sin()
    }

    fn sample_range_rads(&self, rads_left: f32, rads_right: f32, sample_rate: f32) -> Vec<f32> {
        if rads_left > rads_right {
            panic!("rads_right must be equal to or greater than rads_left")
        };
        let rads_per_samp: f32 = 1.0 / sample_rate;
        let mut rads = rads_left;

        // Start with first sample in order to easily go one sample past rads_right
        let mut samples: Vec<f32> = vec![self.sample_rads(rads_left)];
        while rads <= rads_right {
            rads += rads_per_samp;
            samples.push(self.sample_rads(rads));
        }
        samples
    }
}

/// Simple vector of Sine wave structs.
pub struct Sines {
    sines: Vec<Sine>,
}

impl Sines {
    pub fn new() -> Self {
        Self { sines: vec![] }
    }

    pub fn add_sine(&mut self, sine: Sine) {
        self.sines.push(sine);
    }
}

impl Signal for Sines {
    fn sample_rads(&self, rads: f32) -> f32 {
        &self
            .sines
            .iter()
            .fold(0.0, |cur_amp, sine| cur_amp + sine.sample_rads(rads))
            / self.sines.len() as f32
    }

    fn sample_range_rads(&self, rads_left: f32, rads_right: f32, sample_rate: f32) -> Vec<f32> {
        unimplemented!();
    }
}
