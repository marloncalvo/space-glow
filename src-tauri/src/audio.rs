// #![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::f32::consts::PI;
use std::{thread, time};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Sample;

use crate::signals::sine::*;
use crate::signals::{Signal, Signals};

const TWOPI: f32 = PI;

fn play() {
    // Init cpal
    let (dev, cfg) = supported_cfgs();
    let sample_rate: f32 = cfg.sample_rate.0 as f32;

    // Create signal with continuous and periodic sine wave components
    let mut sigs = Signals::new();
    sigs.add_signal(Sine::new(0.5, 440.0, 0.0));
    // sines.add_sine(Sine::new(1.0, 554.37, 0.0));
    // sines.add_sine(Sine::new(1.0, 587.33, 0.0));
    // sines.add_sine(Sine::new(1.0, 659.25, 0.0));

    let mut count: f32 = 0.0;
    let stream = dev
        .build_output_stream(
            &cfg,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    count += 1.0;
                    let rads = count / sample_rate * PI;
                    *sample = Sample::from(&sigs.sample_rads(rads));
                    // println!("rads={:?}\t amp={:?}", rads, sig.sample_rads(rads));
                }
            },
            move |err| {
                eprintln!("an error occurred on the output audio stream: {}", err);
            },
        )
        .unwrap();
    stream.play().unwrap();
    thread::sleep(time::Duration::from_millis(4000));
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

fn supported_cfgs() -> (cpal::Device, cpal::StreamConfig) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let mut supported_cfgs = device
        .supported_output_configs()
        .expect("error while querying configs");
    let cfg = supported_cfgs
        .next()
        .expect("no supported configs")
        .with_max_sample_rate()
        .config();
    (device, cfg)
}

// f = 400Hz -> T = 1/f -> T = 1/400. Cycle takes 0.0025 seconds.
// sample_rate is 48KHz. Cycle takes 1/48K. Need 120 samples for one 400Hz cycle.
// So, we need (1/f)/(1/sample_rate)
