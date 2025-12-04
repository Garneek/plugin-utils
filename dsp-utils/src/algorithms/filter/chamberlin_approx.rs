// Algorithm source: https://www.musicdsp.org/en/latest/Filters/23-state-variable.html

use std::f32::consts::PI;

use crate::algorithms::Filter;

use super::FilterType;

/// Filter `musicdsp.org/en/latest/Filters/23-state-variable.html` Supports
/// `LowPass`, `HighPass`, `BandPass` and `Notch` configurations
pub struct ChamberlinApprox {
    fs: f32,
    freq: f32,
    f: f32,
    q: f32,

    low: f32,
    high: f32,
    band: f32,
    notch: f32,

    f_type: FilterType,
}

impl Filter for ChamberlinApprox {
    fn init(&mut self) {
        self.f = 2_f32 * (PI * self.freq / self.fs).sin();
    }

    fn new(sample_rate: f32, f_type: FilterType) -> Self {
        Self {
            fs: sample_rate,
            freq: -1_f32,
            f: 0_f32,
            q: 1_f32,
            low: 0_f32,
            high: 0_f32,
            band: 0_f32,
            notch: 0_f32,
            f_type,
        }
    }

    fn set_type(&mut self, f_type: FilterType) -> bool {
        self.f_type = f_type;

        match self.f_type {
            FilterType::LowPass
            | FilterType::HighPass
            | FilterType::BandPass
            | FilterType::Notch => true,
            _ => false,
        }
    }

    fn process(&mut self, sample: f32, cutoff_frequency: f32, q: f32) -> f32 {
        if cutoff_frequency != self.freq {
            self.freq = cutoff_frequency;
            self.init();
        }

        self.q = q;

        self.low = self.f.mul_add(self.band, self.low);
        self.high = self
            .q
            .mul_add(-self.band, sample.mul_add(self.q, -self.low));
        self.band = self.f.mul_add(self.high, self.band);
        self.notch = self.low + self.high;

        match self.f_type {
            FilterType::LowPass => self.low,
            FilterType::HighPass => self.high,
            FilterType::BandPass => self.band,
            FilterType::Notch => self.notch,
            _ => sample,
        }
    }
}
