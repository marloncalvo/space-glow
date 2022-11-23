#![allow(unused_imports)]
#![allow(dead_code)]
// use core::time;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Data, Sample, SampleFormat};
use std::f32::consts::PI;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn play() {
    let (dev, cfg) = supported_cfgs();
    let f1: f32 = 400.0; // frequency 1
                         // let f2: f32 = 300.0;
    let sample_rate: f32 = cfg.sample_rate.0 as f32;
    let q1 = sample_rate / (f1 * PI);
    let mut count: u32 = 0;
    let stream = dev
        .build_output_stream(
            &cfg,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    count += 1;
                    let amp: f32 = (count as f32 / q1).sin();
                    *sample = Sample::from(&amp);
                    // println!("count={:?}, amplitude={:?}", count, amp);
                }
            },
            move |err| {
                eprintln!("an error occurred on the output audio stream: {}", err);
            },
        )
        .unwrap();
    stream.play().unwrap();
    thread::sleep(time::Duration::from_millis(500));
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
