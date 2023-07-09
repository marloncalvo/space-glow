use core::time;
use std::f32::consts::PI;

pub fn play() {
    use cpal::traits::HostTrait;
    use cpal::{Data, Sample, SampleFormat, U24};
    use cpal::*;
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    use cpal::traits::DeviceTrait;
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported configs")
        .with_max_sample_rate();
    println!("{:?}", supported_config);
    use cpal::traits::StreamTrait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    let lock = Arc::new(Mutex::new(0));
    let number_of_samples = 500;
    let config = supported_config.config();
    let freq = 400;
    let freq_f64 = freq as f64;
    let sample_rate_f64 = config.sample_rate.0 as f64;
    let quot = ((1.0 / freq_f64) / (1.0 / sample_rate_f64)) as u32;
    let quot_f32 = quot as f32;
    println!("quot={:?}", quot);
    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    let mut count = lock.lock().unwrap();
                    let count_u32 = *count + 1;
                    *count += 1;
                    let inner = (PI * (count_u32 as f32)) / quot_f32;
                    let amplitude = (inner).sin() * 10.0;
                    *sample = amplitude.to_float_sample();
                    // println!(
                    //     "count={:?}, amplitude={:?}, inner={:?}",
                    //     count, amplitude, inner
                    // );
                }
            },
            move |err| {
                eprintln!("an error occurred on the output audio stream: {}", err);
            },
            None,
        )
        .unwrap();
    stream.play().unwrap();
    use std::{thread, time};
    thread::sleep(time::Duration::from_secs(2));
    stream.pause().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_plays() {
        play();
    }
    fn check_cfg() {}
}

// f = 400Hz -> T = 1/f -> T = 1/400. Cycle takes 0.0025 seconds.
// sample_rate is 48KHz. Cycle takes 1/48K. Need 120 samples for one 400Hz cycle.
// So, we need (1/f)/(1/sample_rate)
