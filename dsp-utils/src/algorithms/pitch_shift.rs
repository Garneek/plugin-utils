use super::HannWindow;
use super::Process;
use super::SignalWindow;
use super::WindowedProcess;

// WIP

pub type PitchShift = WindowedProcess<PitchShiftProcess, HannWindow>;

pub struct PitchShiftProcess {
    temp: Vec<f32>,
    _windowed: Vec<f32>,
    window: HannWindow,
}

impl Process for PitchShiftProcess {
    type Message = ();
    type Data = f32;
    fn new(_block_size: usize) -> Self {
        unimplemented!();
        // Self {
        //     temp: vec![0_f32; block_size * 2],
        //     windowed: vec![0_f32; block_size * 2],
        //     window: HannWindow::new(block_size),
        // }
    }
    fn process(&mut self, block: &mut [f32], data: &Self::Data) -> Self::Message {
        self.temp.copy_from_slice(block);
        let half_len = block.len() / 2;
        let mut src_idx = 0;
        let mut tar_idx = 0;
        while src_idx < half_len {
            let v = tar_idx as f32 * data;
            let beg = (v.floor() as usize) % block.len();
            src_idx = (v.ceil() as usize) % block.len();
            let fract = v.fract();

            self.temp[tar_idx] = block[beg] * (1_f32 - fract) + block[src_idx] * fract;
            tar_idx += 1;
        }

        if src_idx < half_len {}

        // self.unwindowed.copy_from_slice(&block[0..half_len]);

        self.window.apply(block);
        for i in 0..half_len {
            block[i] += block[i + half_len];
        }
        let (a, b) = block.split_at_mut(half_len);
        b.copy_from_slice(&a);
        // a.copy_from_slice(&self.unwindowed);

        for i in 0..block.len() {
            let val = i as f32 * data;
            let bef = (val.floor() as usize) % block.len();
            let aft = (val.ceil() as usize) % block.len();
            let fract = val.fract();

            self.temp[i] = block[bef] * (1_f32 - fract) + block[aft] * fract;
        }

        block.copy_from_slice(&self.temp);
    }
}
