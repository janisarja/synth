use std::f32::consts::TAU;

pub struct Oscillator {
    phase: f32,
    waveform: Waveform,
}

impl Oscillator {
    pub fn new() -> Self {
        Self {
            phase: 0.0,
            waveform: Waveform::Sine,
        }
    }

    pub fn next_sample(&mut self, phase_increment: f32) -> f32 {
        let sample = self.waveform.sample(self.phase);

        self.phase += phase_increment;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sample
    }
}

enum Waveform {
    Sine,
}

impl Waveform {
    fn sample(&self, phase: f32) -> f32 {
        match self {
            Waveform::Sine => (phase * TAU).sin() * 0.2,
        }
    }
}
