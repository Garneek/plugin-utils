use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use crate::core::file_processing::read_data;
use crate::core::points_processing::buffer_from_vec;
use crate::DspCoreProcessor;
use crate::ParamsBlock;
use crate::SingleChannelProcessor;

const SAMPLES: usize = 44100 * 120;

pub fn measure_time<SCP>(
    params: Arc<<<SCP as SingleChannelProcessor>::ParamsBlock as ParamsBlock>::Params>,
    zero_params: fn(&mut <SCP as SingleChannelProcessor>::ParamsBlock),
    block_size: usize,
    buffer_size: usize,
    audio_path: &str,
) -> (Duration, usize)
where
    SCP: SingleChannelProcessor,
{
    let mut proc: DspCoreProcessor<SCP> = DspCoreProcessor::new(params, block_size, 2);
    zero_params(&mut proc.params_block);

    let (l, r) = read_data(audio_path, 0, SAMPLES);
    assert_eq!(l.len(), SAMPLES);
    assert_eq!(r.len(), SAMPLES);

    let mut processed_samples = 0;
    let mut timer = Duration::new(0, 0);

    let (mut a, mut b) = (vec![0_f32; buffer_size], vec![0_f32; buffer_size]);

    while processed_samples < SAMPLES - buffer_size {
        a.copy_from_slice(&r[processed_samples..processed_samples + buffer_size]);
        b.copy_from_slice(&l[processed_samples..processed_samples + buffer_size]);

        let mut buffer = buffer_from_vec(&mut a, &mut b);

        let before_process = Instant::now();
        proc.process(&mut buffer);
        timer += before_process.elapsed();

        processed_samples += buffer_size;
    }

    (timer, processed_samples)
}

pub fn benchmark<SCP>(
    params: Arc<<<SCP as SingleChannelProcessor>::ParamsBlock as ParamsBlock>::Params>,
    zero_params: fn(&mut <SCP as SingleChannelProcessor>::ParamsBlock),
    block_size: usize,
    buffer_size: usize,
    audio_path: &str,
) where
    SCP: SingleChannelProcessor,
{
    let (gain_time, gain_samples) = measure_time::<crate::core::sample_plugins::GainScp>(
        Arc::new(crate::core::sample_plugins::GainParams::default()),
        |params_block| params_block.gain = vec![0.5_f32; params_block.block_size],
        block_size,
        buffer_size,
        audio_path,
    );

    let (clip_time, clip_samples) = measure_time::<crate::core::sample_plugins::ClipScp>(
        Arc::new(crate::core::sample_plugins::ClipParams::default()),
        |params_block| {
            params_block.gain = vec![0.5_f32; params_block.block_size];
            params_block.drive = vec![2_f32; params_block.block_size]
        },
        block_size,
        buffer_size,
        audio_path,
    );

    let (bench_time, bench_samples) =
        measure_time::<SCP>(params, zero_params, block_size, buffer_size, audio_path);

    assert_eq!(bench_samples, gain_samples);
    assert_eq!(bench_samples, clip_samples);

    println!("Benchmark results:");
    println!("Processed {} samples", bench_samples);
    println!("Simple gain plugin processed those in: {:.3?}", gain_time);
    println!("Simple clip plugin processed those in: {:.3?}", clip_time);
    println!("Benchmarked plugin processed those in: {:.3?}", bench_time);
    println!(
        "\nThe plugin was {:.3} times slower then gain plugin",
        bench_time.as_secs_f64() / gain_time.as_secs_f64()
    );
    println!(
        "The plugin was {:.3} times slower then clip plugin",
        bench_time.as_secs_f64() / clip_time.as_secs_f64()
    );
}
