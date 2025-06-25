use creek::ReadDiskStream;
use creek::SymphoniaDecoder;

use std::fs::File;
use std::io::Write;

use super::PlotType;

use super::SAMPLES;
use super::SAMPLE_RATE;

// Reads data to two vectors for left and right channels. Takes data 30s into the file length defined in SAMPLES
pub(crate) fn read_data(path: &str) -> (Vec<f32>, Vec<f32>) {
    let mut read_disk_stream =
        ReadDiskStream::<SymphoniaDecoder>::new(path, SAMPLE_RATE * 30, Default::default())
            .expect("File not found");
    let _ = read_disk_stream.cache(0, 0);

    read_disk_stream
        .seek(SAMPLE_RATE * 30, Default::default())
        .expect("Could not read stream");

    let mut a = Vec::new();
    let mut b = Vec::new();

    while a.len() < SAMPLES {
        read_disk_stream
            .block_until_ready()
            .expect("Could not block until ready, stream dropped");

        let read_data = read_disk_stream
            .read(SAMPLES - a.len())
            .expect("Could not read the samples");

        a.extend_from_slice(read_data.read_channel(0));
        b.extend_from_slice(read_data.read_channel(1));
    }

    (a, b)
}

// Gets a path for the plot/data depending on type
// plot is wheter the path should be to plot or data true for plot
pub(crate) fn get_path(target_path: &str, plot_type: PlotType, plot: bool) -> String {
    target_path.to_string()
        + "-"
        + match plot_type {
            PlotType::Rms => "rms",
            PlotType::Peak => "peak",
            PlotType::MeanLoudness => "mean-peak",
        }
        + if plot { "-plot.png" } else { "-data.txt" }
}

// Writes points data to file, separated by tabs and newlines
pub(crate) fn save_points_txt(points: &Vec<(f32, f32)>, path: &str) {
    let mut str = String::new();
    for i in 0..points.len() {
        str +=
            &("".to_string() + &points[i].0.to_string() + "\t" + &points[i].1.to_string() + "\n");
    }
    let mut file = File::create(path).expect("Could not create file");
    file.write_all(&str.into_bytes())
        .expect("Could not write to file");
}
