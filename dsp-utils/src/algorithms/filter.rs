mod chamberlin_approx;

pub use chamberlin_approx::ChamberlinApprox;

/// Filter taking single sample inputs
pub trait Filter {
    /// Initialize filter
    fn new(sample_rate: f32, f_type: FilterType) -> Self;
    /// Change filter type, should return `true` if the type is supported, `false` otherwise
    fn set_type(&mut self, f_type: FilterType) -> bool;
    /// Process sample and return the result
    fn process(&mut self, sample: f32, cutoff_frequency: f32, q: f32) -> f32;
    /// Initialize/reinitialize constants
    fn init(&mut self);
}

/// Filter types
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
    Notch,
    LowShelf,
    HighShelf,
}
