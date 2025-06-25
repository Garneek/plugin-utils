use std::sync::Arc;

use std::time::{Duration, Instant};

mod file_processing;
use file_processing::*;

mod points_processing;
use points_processing::*;

use plotters::chart::ChartBuilder;
use plotters::prelude::BitMapBackend;
use plotters::prelude::IntoDrawingArea;
use plotters::series::LineSeries;
use plotters::style::RED;
use plotters::style::WHITE;

use nih_plug::util;

use crate::{DspCoreProcessor, ParamsBlock, SingleChannelProcessor};

const RESOLUTION: usize = 100;
const SAMPLE_RATE: usize = 44100;
const SAMPLES: usize = SAMPLE_RATE * 10;

/// Type of data for param plot
#[derive(Clone, Copy)]
pub enum PlotType {
    /// Root mean square
    ///
    /// Calculates root mean square of the signal, relative to unmodified sound
    Rms,
    /// Peak of the wave
    ///
    /// Calculates a the peak of whole processed sound, relative to unmodified sound
    Peak,
    /// Mean peak of smaller blocks of data
    ///
    /// This splits the sound into smaller blocks and calculates peak for each one. Returns mean of the peaks
    MeanLoudness,
}

/// Data of the parameter to be plotted
pub struct PlotParamData {
    /// Min value of the parameter
    ///
    /// You can change it from actual min of the param to create a plot of just a subsection of your param range
    pub param_min: f32,
    /// Max value of the parameter
    ///
    /// You can change it from actual max of the param to create a plot of just a subsection of your param range
    pub param_max: f32,
    /// Should the parameter values be treated as dB rather then a number
    pub param_db: bool,
}

fn rescale_value(param_data: &PlotParamData, val: f32) -> f32 {
    param_data.param_min + val * (param_data.param_max - param_data.param_min) / RESOLUTION as f32
}

// fix later, the border is expanding proportional to the coordinate, not the size
fn expand_border(val: f32, min: bool) -> f32 {
    val * if (val.signum() <= 0_f32 && min) || (val.signum() > 0_f32 && !min) {
        1.05_f32
    } else {
        0.95_f32
    }
}

// Gets points data by modulating a parameter and collecting RMS/Peak data
fn get_points<SCP>(
    mut proc: DspCoreProcessor<SCP>,
    change_param: fn(&mut <SCP as SingleChannelProcessor>::ParamsBlock, f32),
    param_data: &PlotParamData,
    audio_path: &str,
    plot_type: PlotType,
) -> Vec<(f32, f32)>
where
    SCP: SingleChannelProcessor,
{
    let mut points = vec![(0_f32, 0_f32); RESOLUTION + 1];

    let read_file_instant = Instant::now();
    let (mut a, mut b) = read_data(audio_path);
    println!("File reading time: {:?}", read_file_instant.elapsed());

    let mut buf = buffer_from_vec(&mut a, &mut b);
    let start_val = match plot_type {
        PlotType::Rms => get_rms(&buf),
        PlotType::Peak => get_peak(&buf),
        PlotType::MeanLoudness => get_mean_loudness(&mut buf),
    };

    let mut param_change_duration = Duration::new(0, 0);
    let mut copy_data_duration = Duration::new(0, 0);
    let mut process_duration = Duration::new(0, 0);

    let (mut a1, mut b1) = (vec![0_f32; SAMPLES], vec![0_f32; SAMPLES]);

    for (i, val) in (0..=RESOLUTION)
        .into_iter()
        .map(|i| (i, rescale_value(param_data, i as f32)))
    {
        let t = Instant::now();
        change_param(
            &mut proc.params_block,
            if param_data.param_db {
                util::db_to_gain(val)
            } else {
                val
            },
        );
        param_change_duration += t.elapsed();

        let t = Instant::now();
        a1.copy_from_slice(&a);
        b1.copy_from_slice(&b);

        let mut buf = buffer_from_vec(&mut a1, &mut b1);
        copy_data_duration += t.elapsed();

        let t = Instant::now();
        proc.process(&mut buf);
        process_duration += t.elapsed();

        points[i] = (
            val,
            match plot_type {
                PlotType::Rms => get_rms(&buf),
                PlotType::Peak => get_peak(&buf),
                PlotType::MeanLoudness => get_mean_loudness(&mut buf),
            } / start_val,
        );
    }

    println!("Param change time: {:?}", param_change_duration);
    println!("Copy data time: {:?}", copy_data_duration);
    println!("Process time: {:?}", process_duration);

    points
}

/// Plot param values against gain they produce
///
/// The function takes in closures to modify params, and in return creates an image, with a plot of the parameter against gain
///
/// `params` should just be your `YourParams::default()`
///
/// `change_param` should be a closure that takes [`crate::ParamsBlock`] and sets the param you want to modulate to  a provided value
///
/// `zero_params` should be a closure that sets all [`crate::ParamsBlock`] data to a "zero" state for your plugin
///
/// `audio_path` should point to an audio file at least 1 minute long with sample rate of 44100 samples/s, currently supported
/// file types: `mp3`
///
/// Output path for files will be:
///
/// `param_name-plot_type-plot.png` for plot
///
/// `param_name-plot_type-data.txt` for data of the points
///
/// To use this function, in your plugin `Cargo.toml` add a feature `test`
///
/// ```
/// test = ["plugin-utils/test"]
/// ```
///
/// Create/go to your `main.rs` file and add a main function
///
/// ```
/// #[cfg(feature = "test")]
/// fn main() {
///     // use in the main function, to only look for these when "test" feature is on
///     use my_plugin::MyParams;
///     use my_plugin::MyParamsBlock;
///     use plugin_utils::dsp_utils::plot;
///     use plugin_utils::dsp_utils::PlotParamData;
///     use plugin_utils::dsp_utils::PlotType;
///     use std::sync::Arc;
///
///     plot::<my_plugin::MySingleChannelProcessor>(
///         // Some version of MyParams, can be default or anything else
///         Arc::new(MyParams::default()),
///         // Set your chosen parameter block to a new vector with given val.
///         // Make sure the block_size is accesible (pub)
///         |params_block, val| params_block.gain = vec![val; params_block.block_size],
///         // Zero the gain, easy since we have one param
///         |params_block| params_block.gain = vec![0; params_block.block_size],
///         &PlotParamData {
///             param_min: -30_f32, // Min value of the plot/param
///             param_max: 30_f32,  // Max value of the plot/param
///             param_db: true,     // Gain should be treated as dB value
///         },
///         // Name of the parameter, this will be used in the file path and plot title
///         "gain",
///         64, // Block size of your processor
///         "audio.mp3", // Path to the, at least, 1 minute long audio clip (in 44100 samples/s)
///         PlotType::Rms, // Rms plot type
///     );
/// }
/// ```
///
/// To run the plotter run `cargo run --release --features=test`
///
/// To build the plugin run `cargo xtask --bundle plugin-name --release`
pub fn plot<SCP>(
    params: Arc<<<SCP as SingleChannelProcessor>::ParamsBlock as ParamsBlock>::Params>,
    change_param: fn(&mut <SCP as SingleChannelProcessor>::ParamsBlock, f32),
    zero_params: fn(&mut <SCP as SingleChannelProcessor>::ParamsBlock),
    param_data: &PlotParamData,
    param_name: &str,
    block_size: usize,
    audio_path: &str,
    plot_type: PlotType,
) where
    SCP: SingleChannelProcessor,
{
    let mut proc: DspCoreProcessor<SCP> = DspCoreProcessor::new(params.clone(), block_size, 2);
    zero_params(&mut proc.params_block);

    let mut points = get_points::<SCP>(proc, change_param, param_data, audio_path, plot_type);

    match plot_type {
        PlotType::Peak | PlotType::MeanLoudness => convert_points_to_db(&mut points, 1_f32),
        PlotType::Rms => convert_points_to_db(&mut points, 2_f32),
    }

    let mut max_y = std::f32::MIN;
    let mut min_y = std::f32::MAX;

    for i in 0..points.len() {
        max_y = max_y.max(points[i].1);
        min_y = min_y.min(points[i].1);
    }

    let plot_path = get_path(param_name, plot_type, true);
    save_points_txt(&points, &get_path(param_name, plot_type, false));

    let root_area = BitMapBackend::new(&plot_path, (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let root_area = root_area
        .titled(
            match plot_type {
                PlotType::Rms => "RMS",
                PlotType::Peak => "Peak",
                PlotType::MeanLoudness => "MeanPeak",
            },
            ("sans-serif", 30),
        )
        .unwrap();

    let mut cc = ChartBuilder::on(&root_area)
        .margin(5)
        .set_all_label_area_size(50)
        .caption(param_name, ("sans-serif", 20))
        .build_cartesian_2d(
            expand_border(param_data.param_min, true)..expand_border(param_data.param_max, false),
            expand_border(min_y, true)..expand_border(max_y, false),
        )
        .unwrap();

    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.5}", v))
        .y_label_formatter(&|v| format!("{:.5}", v))
        .draw()
        .unwrap();

    cc.draw_series(LineSeries::new(points, &RED)).unwrap();
    root_area.present().unwrap();
}
