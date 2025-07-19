mod low_pass;

pub use low_pass::LowPassFilter;

pub trait Filter {
    fn new(block_size: usize, sample_rate: f32) -> Self;
    fn process(&mut self, block: &mut [f32], cutoff_frequency: &[f32]);
}
