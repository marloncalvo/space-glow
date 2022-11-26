pub mod sine;

pub struct Signals {
    sigs: Vec<Box<dyn Signal>>,
}

impl Signals {
    pub fn new() -> Self {
        Self {
            sigs: vec![],
        }
    }

    pub fn add_signal(&mut self, sig: impl Signal + 'static) {
        self.sigs.push(Box::new(sig));
    }

}

impl Signal for Signals {
    fn sample_rads(&self, rads: f32) -> f32 {
        &self
            .sigs
            .iter()
            .fold(0.0, |cur_amp, sig| cur_amp + sig.sample_rads(rads))
            / self.sigs.len() as f32
    }

    fn sample_range_rads(&self, rads_left: f32, rads_right: f32, sample_rate: f32) -> Vec<f32> {
        unimplemented!();
    }
}

pub trait Signal {
    /// Returns the amplitude of the signal at `r` radians.
    /// `r` radians refers to the radians travelled by a 1Hz wave.
    /// There are 2π radians in a 1Hz period. Therefore, 0.5 second passed = π(pi) radians
    fn sample_rads(&self, rads: f32) -> f32;


    /// Returns a vector of float32 amplitudes beginning at rads_left
    /// and ending at the first sample after rads_right
    fn sample_range_rads(&self, rads_left: f32, rads_right: f32, sample_rate: f32) -> Vec<f32>;
}