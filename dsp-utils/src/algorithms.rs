pub mod filter;
mod mdct;
mod window;

pub use mdct::DCT;
pub use mdct::MDCT;

pub use window::HannWindow;
pub use window::SignalWindow;

pub use filter::Filter;

pub mod pitch_shift;

pub trait Process {
    type Message;
    type Data;
    fn new(block_size: usize) -> Self;
    fn process(&mut self, block: &mut [f32], data: &Self::Data) -> Self::Message;
}

pub struct WindowedProcess<P: Process, W: SignalWindow> {
    process: P,
    window: W,
    block_size: usize,

    input_buffer: Vec<f32>,
    output_buffer: Vec<f32>,
    temp_buffer_1: Vec<f32>,

    pre_window: bool,
    post_window: bool,
}

impl<P: Process, W: SignalWindow> WindowedProcess<P, W> {
    pub fn new(block_size: usize, pre_window: bool, post_window: bool) -> Self {
        Self {
            process: P::new(block_size),
            window: W::new(block_size),
            block_size,
            input_buffer: vec![0_f32; block_size * 2],
            output_buffer: vec![0_f32; block_size],
            temp_buffer_1: vec![0_f32; block_size * 2],
            pre_window,
            post_window,
        }
    }

    pub fn process(&mut self, block: &mut [f32], data: &P::Data) -> P::Message {
        self.input_buffer[self.block_size..self.block_size * 2].copy_from_slice(block);
        self.temp_buffer_1.copy_from_slice(&self.input_buffer);

        self.input_buffer[0..self.block_size].copy_from_slice(block);

        if self.pre_window {
            self.window.apply(&mut self.temp_buffer_1);
        }

        let msg = self.process.process(&mut self.temp_buffer_1, data);

        if self.post_window {
            self.window.apply(&mut self.temp_buffer_1);
        }

        let (first, second) = self.temp_buffer_1.split_at_mut(self.block_size);

        block.copy_from_slice(first);
        for i in 0..self.block_size {
            block[i] += self.output_buffer[i];
        }
        self.output_buffer.copy_from_slice(second);
        msg
    }
}
