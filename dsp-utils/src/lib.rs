//! DSP utility functions for nih_plug
//!
//! Includes [`Buffer`] abstraction using [`DspCoreProcessor`], as well as some general DSP functions and structures.
//!
//! [`DspCoreProcessor`] enables you to write one structure that handles DSP, with constant block size, independent of engine
//! or DAW settings.
//!
//! # Examples
//!
//! Example project using [`DspCoreProcessor`]
//! ```no_run
//! // The main struct of the plugin
//! struct PluginStruct {
//!     params: Arc<PluginParams>,
//!     dsp: Option<DspCoreProcessor<PluginSingleChannelProcessor>>,
//! }
//!
//! impl Default for PluginStruct {
//!     [...]
//! }
//!
//! impl Plugin for PluginStruct{
//!     fn initialize( [...] ) -> bool {
//!         self.dsp = Some(DspCoreProcessor::new(
//!             self.params.clone(),
//!             BLOCK_SIZE, // The size to which input from buffer will be split
//!             CHANNELS, // Number of channels for which the data should be allocated
//!         ));
//!         true
//!     }
//!
//!     fn process( [...] ) -> ProcessStatus {
//!         // After checking if proc exists send the buffer for processing
//!         if let Some(proc) = &mut self.dsp {
//!             proc.process(buffer)
//!         } else {
//!             ProcessStatus::Error("DSP data not initialized")
//!         }
//!     }
//!        
//!     [...]
//! }
//!
//! #[derive(Params)]
//! struct PluginParams {
//!     #[id = "gain"]
//!     pub gain: FloatParam,
//! }
//!
//! impl Default for PluginParams {
//!     [...]
//! }
//!
//! // A structure from which we will gather param data in our SingleChannelProcessor
//! struct PluginParamsBlock {
//!     params: Arc<PluginParams>,
//!     block_size: usize,
//!     pub gain: Vec<f32>,
//! }
//!
//! impl ParamsBlock for PluginParamsBlock{
//!     // Main Params structure type
//!     type Params = PluginParams;
//!     
//!     // Initialize new and allocate space
//!     fn new(params: Arc<Self::Params>, block_size: usize) -> Self {
//!         Self { params, gain: vec![0_f32; block_size], block_size }
//!     }
//!
//!     // Update params block data from params values
//!     fn from_params(&mut self) {
//!         self.params.gain.smoothed.next_block(&mut self.gain, self.block_size);
//!     }
//! }
//!
//! // The main structure where DSP is actually done
//! struct PluginSingleChannelProcessor{}
//!
//! impl SingleChannelProcessor for PluginSingleChannelProcessor{
//!     // The type in which the param data will be provided
//!     type ParamsBlock = PluginParamsBlock;
//!
//!     fn new(_block_size: usize) -> Self {
//!         Self{}
//!     }
//!
//!     // Process a block, that will always be of length BLOCK_SIZE
//!     fn process(
//!             &mut self,
//!             block: &[f32],
//!             output: &mut [f32],
//!             params_block: &Self::ParamsBlock,
//!         ) -> ProcessStatus {
//!         output.copy_from_slice(block); // Copy to output
//!         for i in 0..block.len(){
//!             output[i] *= params_block.gain[i]; // Apply gain
//!         }
//!         ProcessStatus::Normal // Return status
//!     }
//! }
//! ```

use std::sync::Arc;

use nih_plug::buffer::Buffer;
use nih_plug::params::Params;
use nih_plug::prelude::ProcessStatus;

#[cfg(feature = "test")]
mod plotter;

#[cfg(feature = "test")]
pub use plotter::*;

mod misc;
pub use misc::*;

mod mdct;
pub use mdct::MDCT;

/// A trait used to process a single channel.
///
/// Always receives blocks of size defined on new call of [`DspCoreProcessor`]
///
/// # Examples
///
/// ```no_run
/// struct ApplyGain {
///     block_size: usize,
/// }
///
/// impl SingleChannelProcessor for ApplyGain {
///     type ParamsBlock = ImplementsParamsBlock;
///
///     fn new(block_size: usize) -> Self {
///         // Allocate whatever data is needed here
///         Self { block_size }
///     }
///
///     fn process(
///         &mut self,
///         block: &[f32],
///         output: &mut [f32],
///         params_block: &Self::ParamsBlock,
///     ) -> nih_plug::prelude::ProcessStatus {
///         // Copy the input slice to output
///         output.copy_from_slice(block);
///         
///         // For each sample, multiply it by parameter1 for that point in time
///         for i in 0..self.block_size {
///             output[i] *= params_block.parameter1[i]
///         }
///
///         // Return normal ProcessStatus
///         nih_plug::prelude::ProcessStatus::Normal
///     }
/// }
/// ```
pub trait SingleChannelProcessor {
    /// Type that is used as to store blocks of your parameter data.
    ///
    /// Must implement [`ParamsBlock`] trait. It will be updated every processed block. The parameters should be extracted by setting
    /// block parameter fields in the struct implementing [`ParamsBlock`] as pub.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// struct MyParamsBlock{
    ///     // Define parameter as public
    ///     pub parameter1: Vec<f32>,   
    /// }
    ///
    /// impl ParamsBlock for MyParamsBlock{
    ///    [...]
    /// }
    /// ```
    ///
    /// In this case the parameter inside process function could be accessed by ```params_block.parameter1[index]```
    type ParamsBlock: ParamsBlock;

    /// Initialize the processor struct.
    ///
    /// Allocate all data you need here. `block_size` is the length of all blocks in the process function. It is also what
    /// you set [`DspCoreProcessor`] to divide the blocks supplied by DAW to.
    fn new(block_size: usize) -> Self;

    /// Process single block.
    ///
    /// The function should process the `block` slice, and output the processed samples into `output` slice.
    /// In the `params_block` variable, if you implemented [`Self::ParamsBlock`] correctly, there should be all params data you need
    /// to process the block.
    fn process(
        &mut self,
        block: &[f32],
        output: &mut [f32],
        params_block: &Self::ParamsBlock,
    ) -> ProcessStatus;
}

/// Trait for a structure holding blocks of [`Params`] data
///
/// This should be implemented by having a field of type `Vec<f32>` for every param in your [`Self::Params`] type, that you want to
/// access in [`SingleChannelProcessor`]
///
/// # Examples
///
/// ```no_run
/// struct MyParamsBlock {
///     params: Arc<ImplementsParams>,
///     block_size: usize,
///     pub parameter1: Vec<f32>,
/// }
///
/// impl ParamsBlock for MyParamsBlock {
///     type Params = ImplementsParams;
///
///     fn new(params: std::sync::Arc<Self::Params>, block_size: usize) -> Self {
///         Self {
///             params,
///             block_size,
///             parameter1: vec![0_f32; block_size],
///         }
///     }
///
///     fn from_params(&mut self) {
///         // Fill self.parameter1 with param data from params.parameter1
///         self.params
///             .parameter1
///             .smoothed
///             .next_block(&mut self.parameter1, self.block_size);
///     }
/// }
/// ```
pub trait ParamsBlock {
    /// Type that implements [`nih_plug::prelude::Params`] and contains [`nih_plug::prelude::Param`] fields
    type Params: Params;
    /// Initialize the [`ParamsBlock`]
    ///
    /// Allocate the data here. Be sure to save params variable as a field in your ParamsBlock struct
    fn new(params: Arc<Self::Params>, block_size: usize) -> Self;
    /// Update the blocks of params, filling them from [`nih_plug::prelude::Params`]
    fn from_params(&mut self);
}

/// Struct that splits blocks into given `block_size` and processes them with defined SingleChannelProcessor.
///
/// In your plugin struct set your processor field type as `Option<DspCoreProcessor<YourSingleChannelProcessor>>`, initially set
/// it to `None` and initialize it in `initialize` function
///
/// `block_size` given in the [`DspCoreProcessor::new`], will also be the value of how many samples of delay are generated from
/// this part of the dsp system. That is because the number of samples in a [`Buffer`] does not need to be divisible by
/// `block_size`, thus we need to prevent overflow/underflow of samples in each [`SingleChannelProcessor::process`] call.
///
/// # Examples
///
/// ```no_run
/// struct PluginStruct {
///     params: Arc<ParamsStruct>,
///     // DspCoreProcessor field
///     dsp: Option<DspCoreProcessor<YourSingleChannelProcessor>>,
/// }
///
/// impl Default for PluginStruct {
///     fn default() -> Self {
///         Self {
///             params: Arc::new(ParamsStruct::default()),
///             // Initially None
///             dsp: None,
///         }
///     }
/// }
///     
/// impl Plugin for PluginStruct {
///     // Plugin::initialize function
///     fn initialize(
///         &mut self,
///         audio_io_layout: &AudioIOLayout,
///         _buffer_config: &BufferConfig,
///         _context: &mut impl InitContext<Self>,
///     ) -> bool {
///         // In initialize we create the processor
///         self.dsp = Some(DspCoreProcessor::new(
///             // Params
///             self.params.clone(),
///             // block_size, size to which to split the incoming blocks
///             64,
///             // Number of channels
///             match audio_io_layout.main_input_channels {
///                 Some(v) => v.get() as usize,
///                 None => {
///                     return false;
///                 }
///             },
///         ));
///         true
///     }
///
///     // Plugin::process function
///     fn process(
///         &mut self,
///         buffer: &mut Buffer,
///         _aux: &mut AuxiliaryBuffers,
///         _context: &mut impl ProcessContext<Self>,
///     ) -> ProcessStatus {
///         // After a check, process buffer
///         if let Some(processor) = &mut self.dsp {
///             processor.process(buffer)
///         } else {
///             ProcessStatus::Error("DSP data not initialized")
///         }
///     }
///     [...]
/// }
/// ```
pub struct DspCoreProcessor<SCP: SingleChannelProcessor> {
    channel_processor: Vec<SCP>,

    overflow: usize,
    temp: Vec<f32>,
    buffer: Vec<Vec<f32>>,
    params_block: SCP::ParamsBlock,

    block_size: usize,
    channels: usize,
}

impl<SCP: SingleChannelProcessor> DspCoreProcessor<SCP> {
    /// Initialize the struct.
    ///
    /// `block_size` should be the number of samples per block, that you want the SingleChannelProcessor
    /// to get each time [`SingleChannelProcessor::process`] is called
    ///
    /// `channels` can be lower/higher then the actual channels amount. If that happens the lower value is chosen as the
    /// amount of channels. This cannot be zero

    pub fn new(
        params: Arc<<<SCP as SingleChannelProcessor>::ParamsBlock as ParamsBlock>::Params>,
        block_size: usize,
        channels: usize,
    ) -> Self {
        Self {
            channel_processor: (0..channels)
                .into_iter()
                .map(|_| SCP::new(block_size))
                .collect(),
            overflow: block_size,
            temp: vec![0_f32; block_size],
            buffer: Vec::from_iter(std::iter::repeat_n(
                Vec::from_iter(std::iter::repeat_n(0_f32, block_size)),
                channels,
            )),

            params_block: SCP::ParamsBlock::new(params, block_size),

            block_size,
            channels,
        }
    }

    /// Process the `buffer` using supplied processor type.
    ///
    /// The `buffer` will be split into channels, and then into blocks of given length that will be processed by
    /// [`SingleChannelProcessor`]. The struct stores overflow samples and will prevent any half processed blocks
    pub fn process(&mut self, buffer: &mut Buffer) -> ProcessStatus {
        let samples = buffer.samples();
        let channels = buffer.channels().min(self.channels);
        let slice = buffer.as_slice();

        if slice.len() == 0 {
            return ProcessStatus::Error("No channels");
        }

        let mut index = self.block_size - self.overflow;
        if index != 0 {
            for channel in 0..channels {
                self.temp[0..self.overflow]
                    .copy_from_slice(&self.buffer[channel][0..self.overflow]);

                self.temp[self.overflow..self.block_size]
                    .copy_from_slice(&slice[channel][0..index]);
                slice[channel][0..index]
                    .copy_from_slice(&self.buffer[channel][self.overflow..self.block_size]);

                #[cfg(not(feature = "test"))]
                self.params_block.from_params();

                match self.channel_processor[channel].process(
                    &self.temp,
                    &mut self.buffer[channel],
                    &self.params_block,
                ) {
                    ProcessStatus::Error(e) => return ProcessStatus::Error(e),
                    _ => {}
                };
            }
            self.overflow = self.block_size;
        }

        let index_static = index;
        for _ in 0..(samples - index_static) / self.block_size {
            for channel in 0..channels {
                self.temp
                    .copy_from_slice(&slice[channel][index..index + self.block_size]);
                slice[channel][index..index + self.block_size]
                    .copy_from_slice(&self.buffer[channel][0..self.block_size]);

                #[cfg(not(feature = "test"))]
                self.params_block.from_params();

                match self.channel_processor[channel].process(
                    &self.temp,
                    &mut self.buffer[channel],
                    &self.params_block,
                ) {
                    ProcessStatus::Error(e) => return ProcessStatus::Error(e),
                    _ => {}
                };
            }
            index += self.block_size;
        }

        if index != samples {
            self.overflow = samples - index;

            for channel in 0..channels {
                let mut t;
                for i in 0..self.overflow {
                    t = slice[channel][i + index];
                    slice[channel][i + index] = self.buffer[channel][i];
                    self.buffer[channel][i] = t;
                }
            }
        }

        ProcessStatus::Normal
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use nih_plug::{buffer::Buffer, params::Params};

    use crate::{DspCoreProcessor, ParamsBlock, SingleChannelProcessor};

    #[derive(Params)]
    struct ImplementsParams {}
    struct Block {}
    impl ParamsBlock for Block {
        type Params = ImplementsParams;
        fn new(_params: std::sync::Arc<Self::Params>, _block_size: usize) -> Self {
            Self {}
        }
        fn from_params(&mut self) {}
    }
    struct Single {
        block_size: usize,
    }
    impl SingleChannelProcessor for Single {
        type ParamsBlock = Block;
        fn new(block_size: usize) -> Self {
            Self { block_size }
        }
        fn process(
            &mut self,
            block: &[f32],
            output: &mut [f32],
            _params_block: &Self::ParamsBlock,
        ) -> nih_plug::plugin::ProcessStatus {
            assert_eq!(self.block_size, block.len());
            assert_eq!(self.block_size, output.len());

            output.copy_from_slice(block);
            for i in 0..output.len() {
                output[i] += 1_f32;
            }
            nih_plug::plugin::ProcessStatus::Normal
        }
    }

    // Makes sure the overflow prevention system is working as intended, every sample is processed once and in blocks of block_size

    #[test]
    fn test_overflow_prevention_system() {
        let mut real_buffers = vec![vec![0_f32; 128]; 3];
        let mut buffer = Buffer::default();
        unsafe {
            buffer.set_slices(128, |slices| {
                let (one, two) = real_buffers.split_at_mut(1);
                let (two, three) = two.split_at_mut(1);
                *slices = vec![&mut one[0], &mut two[0], &mut three[0]];
            })
        }

        let mut proc: DspCoreProcessor<Single> =
            DspCoreProcessor::new(Arc::new(ImplementsParams {}), 23, 3);

        proc.process(&mut buffer);

        // First pass generates 23 samples of delay, so there should be that many zeros
        let mut expected = vec![0_f32; 23];
        expected.extend_from_slice(&[1_f32; 128 - 23]);

        assert_eq!(expected, buffer.as_slice_immutable()[0]);
        assert_eq!(expected, buffer.as_slice_immutable()[1]);
        assert_eq!(expected, buffer.as_slice_immutable()[2]);

        let mut real_buffers = vec![vec![0_f32; 128]; 3];
        let mut buffer = Buffer::default();
        unsafe {
            buffer.set_slices(128, |slices| {
                let (one, two) = real_buffers.split_at_mut(1);
                let (two, three) = two.split_at_mut(1);
                *slices = vec![&mut one[0], &mut two[0], &mut three[0]];
                // *slices = vec![&mut real_buffers[0]];
            })
        }

        proc.process(&mut buffer);

        let expected = [1_f32; 128];

        assert_eq!(expected, buffer.as_slice_immutable()[0]);
        assert_eq!(expected, buffer.as_slice_immutable()[1]);
        assert_eq!(expected, buffer.as_slice_immutable()[2]);
    }
}
