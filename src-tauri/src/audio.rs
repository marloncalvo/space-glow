use core::time;
use std::f32::consts::PI;

fn play() {
    use cpal::traits::HostTrait;
    use cpal::{Data, Sample, SampleFormat};
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");
    use cpal::traits::{DeviceTrait};
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported configs")
        .with_max_sample_rate();
    println!("{:?}", supported_config);
    use cpal::traits::{StreamTrait};
    use std::sync::{Arc, Mutex};
    use std::sync::atomic::{AtomicUsize, Ordering};
    let lock = Arc::new(Mutex::new(0));
    let numberOfSamples = 500;
    let stream = device.build_output_stream(
        &supported_config.config(), 
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                let mut count = lock.lock().unwrap();
                let count_f32 = (*count + 1) as f32;
                *count += 1;
                let amplitude = (400.0 * PI * count_f32).sin() * 0.2;
                *sample = Sample::from(&0.0);
                println!("count={:?}, amplitude={:?}", count, amplitude);
            }
        }, 
        move |err| {
            eprintln!("an error occurred on the output audio stream: {}", err);
        }).unwrap();
    stream.play().unwrap();
    use std::{thread, time};
    thread::sleep(time::Duration::from_secs(5));
    stream.pause().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_plays() {
        play();
    }
}