use nih_plug::formatters;
use nih_plug::params::FloatParam;
use nih_plug::params::Params;
use nih_plug::prelude::FloatRange;
use nih_plug::prelude::SmoothingStyle;
use nih_plug::util;

use crate::ParamsBlock;
use crate::SingleChannelProcessor;

use std::sync::Arc;

#[derive(Params)]
pub(crate) struct GainParams {
    #[id = "gain"]
    pub(crate) gain: FloatParam,
}

impl Default for GainParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

pub(crate) struct GainParamsBlock {
    pub(crate) gain: Vec<f32>,
    pub(crate) block_size: usize,
}

impl ParamsBlock for GainParamsBlock {
    type Params = GainParams;
    fn new(_params: Arc<Self::Params>, block_size: usize) -> Self {
        Self {
            gain: vec![0_f32; block_size],
            block_size,
        }
    }

    fn from_params(&mut self) {}
}

pub(crate) struct GainScp {}
impl SingleChannelProcessor for GainScp {
    type ParamsBlock = GainParamsBlock;

    fn new(_block_size: usize, _sample_rate: f32, _params: Arc<GainParams>) -> Self {
        Self {}
    }

    fn process(
        &mut self,
        block: &[f32],
        output: &mut [f32],
        params_block: &Self::ParamsBlock,
    ) -> nih_plug::prelude::ProcessStatus {
        output.copy_from_slice(block);

        for i in 0..output.len() {
            output[i] *= params_block.gain[i];
        }

        nih_plug::prelude::ProcessStatus::Normal
    }
}

#[derive(Params)]
pub(crate) struct ClipParams {
    #[id = "drive"]
    pub(crate) drive: FloatParam,
    #[id = "gain"]
    pub(crate) gain: FloatParam,
}

impl Default for ClipParams {
    fn default() -> Self {
        Self {
            drive: FloatParam::new(
                "Drive",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

pub(crate) struct ClipParamsBlock {
    pub(crate) drive: Vec<f32>,
    pub(crate) gain: Vec<f32>,
    pub(crate) block_size: usize,
}

impl ParamsBlock for ClipParamsBlock {
    type Params = ClipParams;
    fn new(_params: Arc<Self::Params>, block_size: usize) -> Self {
        Self {
            drive: vec![0_f32; block_size],
            gain: vec![0_f32; block_size],
            block_size,
        }
    }

    fn from_params(&mut self) {}
}

pub(crate) struct ClipScp {}

impl SingleChannelProcessor for ClipScp {
    type ParamsBlock = ClipParamsBlock;

    fn new(_block_size: usize, _sample_rate: f32, _params: Arc<ClipParams>) -> Self {
        Self {}
    }

    fn process(
        &mut self,
        block: &[f32],
        output: &mut [f32],
        params_block: &Self::ParamsBlock,
    ) -> nih_plug::prelude::ProcessStatus {
        for i in 0..output.len() {
            output[i] =
                (block[i] * params_block.drive[i]).clamp(-1_f32, 1_f32) * params_block.gain[i];
        }

        nih_plug::prelude::ProcessStatus::Normal
    }
}
