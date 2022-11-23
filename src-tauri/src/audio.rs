// use core::time;
use std::{f32::consts::PI};
use cpal::traits::HostTrait;
use cpal::{Data, Sample, SampleFormat};
use cpal::traits::{DeviceTrait};
use cpal::traits::{StreamTrait};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{thread, time};

type FuncT = dyn Fn(f32) -> f32;

struct Sine<'a> {
    amplitude: &'a FuncT,
    omega: &'a FuncT,
    phi: &'a FuncT,
    alpha: &'a FuncT,
}

// https://www.quora.com/Is-there-a-formula-to-add-two-sine-waves-with-different-amplitudes-different-periods-and-different-phase-shifts

impl Sine<'_> {
    fn add(wave_0: Self, wave_1: Self) -> Self {
        // Usage: Sine::add(wave1, wave2)
        let w_add = |t: f32| -> f32 {(wave_0.omega)(t) + (wave_1.omega)(t)};
        let w_sub = |t: f32| -> f32 {(wave_0.omega)(t) - (wave_1.omega)(t)};
        
        let phi_add = |t: f32| -> f32 {(wave_0.phi)(t) + (wave_1.phi)(t)};
        let phi_sub = |t: f32| -> f32 {(wave_0.phi)(t) - (wave_1.phi)(t)};

        let a_add = |t: f32| -> f32 {(wave_0.amplitude)(t) + (wave_1.amplitude)(t)};

        let a = |t: f32| -> f32 {
            (wave_0.amplitude)(t).powi(2) +
            (wave_1.amplitude)(t).powi(2) +
            (2.0 * (wave_0.amplitude)(t) * (wave_1.amplitude)(t)) * 
            (((w_sub)(t)*t)+(phi_sub)(t)).cos()
        };

        let big_phi = |t: f32| -> f32 {
            (
                (((wave_0.amplitude)(t) - (wave_1.amplitude)(t)) / ((wave_0.amplitude)(t) + (wave_1.amplitude)(t))) *
                (((w_sub)(t)*t/2.0)+((phi_sub)(t)/2.0)).tan()
            ).tanh()
        };

        let omega = |t: f32| -> f32 {(w_add)(t)*t/2.0};
        let phi = |t: f32| -> f32 {(phi_add)(t)/2.0 + (big_phi)(t)};
        let alpha = |t: f32| -> f32 {(wave_0.alpha)(t) + (wave_1.alpha)(t)};

        return Sine{
            amplitude: &a,
            omega: &omega,
            phi: &phi,
            alpha: &alpha
        };
    }

    fn subtract(wave1: Self, wave2: Self) -> Self {
        unimplemented!()
    }
}

fn play() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported configs")
        .with_max_sample_rate();
    println!("{:?}", supported_config);
    let config = supported_config.config();
    let freq: f32 = 400 as f32;
    let sample_rate: f32 = config.sample_rate.0 as f32;
    let quot: f32 = sample_rate / (freq * PI);
    let mut count: u32 = 0;
    println!("quot={:?}", quot);
    let stream = device.build_output_stream(
        &config, 
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                count += 1;
                let amp: f32 = (count as f32 / quot).sin();
                *sample = Sample::from(&amp);
                // println!("count={:?}, inner={:?}, other_count={:?}", count, inner, (count as f32) / quot_f32);
            }
        }, 
        move |err| {
            eprintln!("an error occurred on the output audio stream: {}", err);
        }).unwrap();
    stream.play().unwrap();
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
    fn check_cfg() {
        
    }
}

// f = 400Hz -> T = 1/f -> T = 1/400. Cycle takes 0.0025 seconds.
// sample_rate is 48KHz. Cycle takes 1/48K. Need 120 samples for one 400Hz cycle.
// So, we need (1/f)/(1/sample_rate)