use std::f32::consts::TAU;
use std::time::Duration;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    build_stream();
}

enum Waveform {
    Sine,
}

struct Oscillator {
    frequency: f32,
    phase: f32,
    waveform: Waveform,
}

impl Oscillator {
    fn next_sample(&mut self, sample_rate: f32) -> f32 {
        let sample = match self.waveform {
            Waveform::Sine => sine_wave(self.phase),
        };

        self.phase += self.frequency / sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sample
    }
}

struct Voice {
    oscillator: Oscillator,
}

impl Voice {
    fn next_sample(&mut self, sample_rate: f32) -> f32 {
        self.oscillator.next_sample(sample_rate)
    }
}

fn sine_wave(phase: f32) -> f32 {
    (phase * TAU).sin() * 0.2
}

fn build_stream() {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("No output device.");

    let config = device
        .default_output_config()
        .expect("No default config.");

    let sample_rate = config.sample_rate() as f32;
    let channels = config.channels() as usize;

    let mut voice = Voice {
        oscillator: Oscillator {
            frequency: 440.0,
            phase: 0.0,
            waveform: Waveform::Sine,
        }
    };

    let stream = device
        .build_output_stream(
            config.into(),
            move |data: &mut [f32], _| {
                for frame in data.chunks_mut(channels) {
                    let sample = voice.next_sample(sample_rate);

                    for out in frame {
                        *out = sample;
                    }
                }
            },
            |err| eprintln!("Stream error: {err}"),
            None,
        )
        .unwrap();

    stream.play().unwrap();

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}
