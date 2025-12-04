pub trait SignalWindow {
    fn new(block_size: usize) -> Self;
    fn apply(&self, block: &mut [f32]);
}

pub struct HannWindow {
    window: Vec<f32>,
}

impl SignalWindow for HannWindow {
    fn new(block_size: usize) -> Self {
        Self {
            window: (0..block_size * 2)
                .into_iter()
                .map(|i| {
                    (std::f32::consts::PI * i as f32 / ((2 * block_size + 1) as f32))
                        .sin()
                        .powi(2)
                })
                .collect(),
        }
    }

    fn apply(&self, block: &mut [f32]) {
        block
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| *x *= self.window[i]);
    }
}

pub struct SineWindow {
    window: Vec<f32>,
}

impl SignalWindow for SineWindow {
    fn new(block_size: usize) -> Self {
        Self {
            window: (0..block_size * 2)
                .into_iter()
                .map(|i| (std::f32::consts::PI * i as f32 / ((2 * block_size + 1) as f32)).sin())
                .collect(),
        }
    }
    fn apply(&self, block: &mut [f32]) {
        block
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| *x *= self.window[i]);
    }
}
