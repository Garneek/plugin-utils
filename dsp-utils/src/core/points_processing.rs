use nih_plug::buffer::Buffer;
use nih_plug::util;

// Get RMS value from buffer
pub(crate) fn get_rms(buf: &Buffer) -> f32 {
    let slice = buf.as_slice_immutable();
    let mut sum = 0_f32;
    for channel in slice {
        for i in 0..buf.samples() {
            sum += channel[i].abs();
        }
    }
    (sum / (buf.samples() as f32 * buf.channels() as f32)).sqrt()
}

// Get peak from buffer
pub(crate) fn get_peak(buf: &Buffer) -> f32 {
    let slice = buf.as_slice_immutable();
    let mut max = 0_f32;

    for channel in slice {
        for i in 0..buf.samples() {
            max = max.max(channel[i].abs())
        }
    }
    max
}

// Get mean loudness from buffer
pub(crate) fn get_mean_loudness(buf: &mut Buffer) -> f32 {
    let mut i = 0_f32;
    let mut max_sum = 0_f32;
    for block in buf.iter_blocks(2048) {
        let mut max_val = std::f32::MIN;
        for el in block.1.get(0).unwrap() {
            max_val = max_val.max(el.abs());
        }
        for el in block.1.get(1).unwrap() {
            max_val = max_val.max(el.abs());
        }
        max_sum += max_val;
        i += 1_f32;
    }

    max_sum / i
}

// Convert vectors to a buffer
pub(crate) fn buffer_from_vec<'a>(l: &'a mut Vec<f32>, r: &'a mut Vec<f32>) -> Buffer<'a> {
    assert_eq!(l.len(), r.len());
    let mut buf = Buffer::default();

    unsafe {
        buf.set_slices(l.len(), |slices| {
            *slices = vec![l, r];
        })
    }
    buf
}

// Converts all y coordinates of the points into dB rather then gain
pub(crate) fn convert_points_to_db(points: &mut Vec<(f32, f32)>, multiplier: f32) {
    for i in 0..points.len() {
        points[i] = (points[i].0, util::gain_to_db(points[i].1) * multiplier);
    }
}
