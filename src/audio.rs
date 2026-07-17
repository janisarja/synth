use std::time::Duration;
use std::sync::mpsc::Receiver;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::synth::{Synth, SynthCommand};

pub fn build_stream(mut synth: Synth, rx: Receiver<SynthCommand>) {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("No output device.");

    let config = device
        .default_output_config()
        .expect("No default config.");

    let sample_rate = config.sample_rate() as f32;
    let channels = config.channels() as usize;

    let stream = device
        .build_output_stream(
            config.into(),
            move |data: &mut [f32], _| {
                while let Ok(event) = rx.try_recv() {
                    synth.handle_event(event);
                }
                for frame in data.chunks_mut(channels) {
                    let sample = synth.next_sample(sample_rate);

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
