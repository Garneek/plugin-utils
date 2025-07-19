use super::HannWindow;
use super::Process;
use super::WindowedProcess;

pub type PitchShift = WindowedProcess<PitchShiftProcess, HannWindow>;

pub struct PitchShiftProcess {
    temp: Vec<f32>,
}

impl Process for PitchShiftProcess {
    type Message = ();
    type Data = f32;
    fn new(block_size: usize) -> Self {
        Self {
            temp: vec![0_f32; block_size],
        }
    }
    fn process(&mut self, block: &mut [f32], data: &Self::Data) -> Self::Message {
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
