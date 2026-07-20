use crate::synth::{Oscillator, ADSR};

pub enum SynthCommand {
    PlayNote(u8),
    ReleaseNote(u8),
}

pub struct Synth {
    voices: Vec<Voice>,
}

impl Synth {
    pub fn new() -> Self {
        Self {
            voices: Vec::new(),
        }
    }

    pub fn next_sample(&mut self, sample_rate: f32) -> f32 {
        let mut sample = 0.0;

        self.voices.retain(|voice| !voice.is_dead());

        for voice in &mut self.voices {
            sample += voice.next_sample(sample_rate);
        }

        // TODO: Implement a proper mixer.
        sample /= self.voices.len().max(1) as f32;

        sample
    }

    pub fn handle_event(&mut self, event: SynthCommand) {
        match event {
            SynthCommand::PlayNote(note) => self.play_note(note),
            SynthCommand::ReleaseNote(note) => self.release_note(note),
        }
    }

    fn play_note(&mut self, note: u8) {
        self.voices.push(Voice::new(note))
    }

    fn release_note(&mut self, note: u8) {
        for voice in &mut self.voices {
            if voice.note == note && !voice.is_dead() {
                voice.release();
                break;
            }
        }
    }
}

struct Voice {
    note: u8,
    oscillator: Oscillator,
    envelope: ADSR,
}

impl Voice {
    fn new(note: u8) -> Self {
        Self {
            note: note,
            oscillator: Oscillator::new(),
            envelope: ADSR::new(0.05, 0.05, 0.75, 0.1),
        }
    }

    fn next_sample(&mut self, sample_rate: f32) -> f32 {
        let phase_increment = note_to_freq(self.note) / sample_rate;
        let mut sample = self.oscillator.next_sample(phase_increment);

        let time_increment = 1.0 / sample_rate;
        let amplitude = self.envelope.next_amplitude(time_increment);
        sample *= amplitude;

        sample
    }

    fn release(&mut self) {
        self.envelope.release();
    }

    fn is_dead(&self) -> bool {
        self.envelope.is_dead()
    }
}

fn note_to_freq(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}
