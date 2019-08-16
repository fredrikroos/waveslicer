use std::fs::File;
use simplemad::{Decoder, Frame, SimplemadError};
use std::time::Duration;

pub fn process_file(file: File, width: u32) -> Result<Vec<(f32,f32)>, SimplemadError> {
    let decoder = Decoder::decode(file)?;
    // Filter out all invalid (metadata) frames
    let valid_frames = decoder.filter_map(|f| f.ok()).collect::<Vec<Frame>>();

    let duration : Duration = valid_frames.iter().map(|f| f.duration).sum();
    let duration_per_slice = duration / width;

    let mut wave_slices = Vec::with_capacity(width as usize);
    let mut current_time = Duration::new(0,0);
    let mut left = 0.0f32;
    let mut right = 0.0f32;
    let mut current_slice_end = duration_per_slice;

    for frame in valid_frames {
        let num_samples = frame.samples[0].len();
        let duration_per_sample = Duration::new(0, 1000000000 / frame.sample_rate);
        for sample in 0..num_samples {
            current_time += duration_per_sample;
            if current_time > current_slice_end {
                wave_slices.push((left, right));
                left = 0.0;
                right = 0.0;
                current_slice_end += duration_per_slice;
            }
            left = left.max(frame.samples[0][sample].to_f32().abs());
            right = right.max(frame.samples[1][sample].to_f32().abs());
        }
    }
    
    Ok(wave_slices)
}