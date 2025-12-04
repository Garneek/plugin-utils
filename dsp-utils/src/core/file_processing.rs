use creek::ReadDiskStream;
use creek::SymphoniaDecoder;

use std::fs::File;
use std::io::Write;

// Reads data to two vectors for left and right channels. Takes data 30s into the file length defined in SAMPLES
pub(crate) fn read_data(path: &str, start: usize, length: usize) -> (Vec<f32>, Vec<f32>) {
    let mut read_disk_stream =
        ReadDiskStream::<SymphoniaDecoder>::new(path, start, Default::default())
            .expect("File not found");
    let _ = read_disk_stream.cache(0, 0);

    read_disk_stream
        .seek(start, Default::default())
        .expect("Could not read stream");

    let mut a = Vec::new();
    let mut b = Vec::new();

    while a.len() < length {
        read_disk_stream
            .block_until_ready()
            .expect("Could not block until ready, stream dropped");

        let read_data = read_disk_stream
            .read(length - a.len())
            .expect("Could not read the samples");

        a.extend_from_slice(read_data.read_channel(0));
        b.extend_from_slice(read_data.read_channel(1));
    }

    (a, b)
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
