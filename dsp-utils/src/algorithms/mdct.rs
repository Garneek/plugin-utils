mod dct;
pub use dct::DCT;

/// Mdct processor
///
/// Buffers some data to allow for smooth encoding and decoding. Forward mdct takes in a slice of length `block_size` and outputs
/// its data into a slice of length `block_size * 2`. Inverse mdct does the opposite, and decodes the blocks.
///
/// Can be easily used in [`crate::SingleChannelProcessor`], by setting it as its field.
///
/// # Examples
///
/// ```
/// let mut mdct = MDCT::new(4);
///
/// let block = vec![0_f32, 1_f32, -1_f32, 0_f32];
/// let mut output = vec![0_f32; 8];
///
/// mdct.mdct(&block, &mut output);
///
/// let mut imdct_block = vec![0_f32; 4];
/// mdct.imdct(&mut output, &mut imdct_block);
///
/// // We need two passes to fill in the buffers, that initialize with zeros. Because of that this element produces block_size
/// // amount of samples of delay
/// mdct.mdct(&block, &mut output);
///
/// let mut imdct_block = vec![0_f32; 4];
/// mdct.imdct(&mut output, &mut imdct_block);
/// println!("{:?}", imdct_block);
/// ```
pub struct MDCT {
    dct: dct::DCT,
    block_size: usize,

    dct_buffer: Vec<f32>,
    idct_buffer: Vec<f32>,
    window: Vec<f32>,
    temp_buffer: Vec<f32>,
}

impl MDCT {
    /// Initialize the processor with given `block_size`
    ///
    /// Panics if `block_size` is not a power of 2.
    /// This function allocates memory, and should be used only in [`nih_plug::prelude::Plugin::initialize`] call
    pub fn new(block_size: usize) -> Self {
        assert!(block_size.is_power_of_two());
        Self {
            dct: dct::DCT::new(block_size * 2),
            block_size,

            dct_buffer: vec![0_f32; block_size * 2],
            idct_buffer: vec![0_f32; block_size],
            window: (0..block_size * 2)
                .into_iter()
                .map(|i| {
                    (std::f32::consts::PI * i as f32 / ((2 * block_size + 1) as f32))
                        .sin()
                        .powi(2)
                })
                .collect(),
            temp_buffer: vec![0_f32; block_size * 2],
        }
    }

    /// Processes `block` into `output` slice with forward mdct
    ///
    /// `block` needs to be of length `block_size`
    ///
    /// `output_block` needs to be of length `block_size * 2`
    ///
    /// This will be asserted if built with `benchmark` feature
    pub fn mdct(&mut self, block: &[f32], output_block: &mut [f32]) {
        #[cfg(feature = "benchmark")]
        {
            assert_eq!(block.len(), self.block_size);
            assert_eq!(output_block.len(), self.block_size * 2);
        }

        self.dct_buffer[self.block_size..self.block_size * 2]
            .copy_from_slice(&block[0..self.block_size]);

        output_block.copy_from_slice(&self.dct_buffer);

        self.dct.dct(output_block, self.temp_buffer.as_mut_slice());

        let (first, second) = self.dct_buffer.split_at_mut(self.block_size);
        first.copy_from_slice(second);
    }

    /// Processes `block` into `output` slice with inverse mdct
    ///
    /// `dct_block` needs to be of length `block_size * 2`
    ///
    /// `output_block` needs to be of length `block_size`
    ///
    /// This will be asserted if built with `benchmark` feature
    pub fn imdct(&mut self, dct_block: &mut [f32], output_block: &mut [f32]) {
        #[cfg(feature = "benchmark")]
        {
            assert_eq!(output_block.len(), self.block_size);
            assert_eq!(dct_block.len(), self.block_size * 2);
        }

        self.dct.idct(dct_block, self.temp_buffer.as_mut_slice());

        for i in 0..self.block_size {
            output_block[i] = dct_block[i].mul_add(self.window[i], self.idct_buffer[i]);
        }

        for i in self.block_size..self.block_size * 2 {
            self.idct_buffer[i - self.block_size] = dct_block[i] * self.window[i];
        }
    }
}
