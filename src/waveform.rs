use simplemad::{Decoder, Frame, SimplemadError};
use std::fs::File;
use std::time::Duration;

pub struct Waveform {
    pub data: Vec<i32>,
    pub length: Duration,
}

pub enum Error {
    MadError(SimplemadError),
    IOError(std::io::Error),
}

pub fn generate(filename: &str, width: u32) -> Result<Waveform, Error> {
    let file = File::open(filename).map_err(Error::IOError)?;
    let decoder = Decoder::decode(file).map_err(Error::MadError)?;
    // Filter out all invalid (metadata) frames
    let valid_frames = decoder.filter_map(|f| f.ok()).collect::<Vec<Frame>>();

    let sample_rate = valid_frames[0].sample_rate;
    let num_channels = valid_frames[0].samples.len() as f32;
    let duration_per_sample = Duration::new(0, 1000000000 / sample_rate);

    let duration: Duration = valid_frames.iter().map(|f| f.duration).sum();
    let duration_per_slice = duration / width;

    let mut data = Vec::with_capacity(width as usize);
    let mut current_time = Duration::new(0, 0);
    let mut min = 0.0f32;
    let mut max = 0.0f32;
    let mut current_slice_end = duration_per_slice;

    for frame in valid_frames {
        // All frames should have same sample rate
        assert!(frame.sample_rate == sample_rate);
        let num_samples = frame.samples[0].len();
        for sample in 0..num_samples {
            current_time += duration_per_sample;
            if current_time > current_slice_end {
                data.push(min);
                data.push(max);
                min = 0.0;
                max = 0.0;
                current_slice_end += duration_per_slice;
            }
            // sum channels
            let value = frame
                .samples
                .iter()
                .map(|ch| ch[sample].to_f32())
                .sum::<f32>()
                / num_channels;
            min = min.min(value);
            max = max.max(value);
        }
    }

    let scaled = data
        .into_iter()
        .map(|sample| (sample * 127.0) as i32)
        .collect();

    Ok(Waveform {
        data: scaled,
        length: duration,
    })
}
