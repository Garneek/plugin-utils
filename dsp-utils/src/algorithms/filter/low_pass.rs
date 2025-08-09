use crate::algorithms::Filter;

pub struct LowPassFilter {
    alpha: f32,
    last: f32,
}

impl Filter for LowPassFilter {
    fn new(_block_size: usize, sample_rate: f32) -> Self {
        Self {
            alpha: 2_f32 * std::f32::consts::PI / sample_rate,
            last: 0_f32,
        }
    }

    fn process(&mut self, block: &mut [f32], cutoff_frequency: &[f32]) {
        for i in 0..block.len() {
            block[i] = self.last
                + ((self.alpha * cutoff_frequency[i]) / (self.alpha * cutoff_frequency[i] + 1_f32))
                    * (block[i] - self.last);
            self.last = block[i];
        }
    }
}
