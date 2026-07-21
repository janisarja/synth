mod oscillator;
mod adsr;

use oscillator::Oscillator;
use adsr::ADSR;

pub struct Voice {
    pub note: u8,
    oscillator: Oscillator,
    envelope: ADSR,
}

impl Voice {
    pub fn new(note: u8) -> Self {
        Self {
            note: note,
            oscillator: Oscillator::new(),
            envelope: ADSR::new(0.05, 0.05, 0.75, 0.1),
        }
    }

    pub fn next_sample(&mut self, sample_rate: f32) -> f32 {
        let phase_increment = note_to_freq(self.note) / sample_rate;
        let mut sample = self.oscillator.next_sample(phase_increment);

        let time_increment = 1.0 / sample_rate;
        let amplitude = self.envelope.next_amplitude(time_increment);
        sample *= amplitude;

        sample
    }

    pub fn release(&mut self) {
        self.envelope.release();
    }

    pub fn is_released(&self) -> bool {
        self.envelope.is_released()
    }

    pub fn is_dead(&self) -> bool {
        self.envelope.is_dead()
    }
}

fn note_to_freq(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}
