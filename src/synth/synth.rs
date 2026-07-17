use crate::synth::Oscillator;

pub enum SynthEvent {
    NoteOn(u8),
    NoteOff(u8),
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

        for voice in &mut self.voices {
            sample += voice.next_sample(sample_rate);
        }

        // TODO: Implement a proper mixer.
        sample /= self.voices.len().max(1) as f32;

        sample
    }

    pub fn handle_event(&mut self, event: SynthEvent) {
        match event {
            SynthEvent::NoteOn(note) => self.voices.push(Voice::new(note)),
            SynthEvent::NoteOff(note) => println!("{note} off"),
        }
    }
}

struct Voice {
    note: u8,
    frequency: f32,
    oscillator: Oscillator,
}

impl Voice {
    pub fn new(note: u8) -> Self {
        Self {
            note: note,
            frequency: note_to_freq(note),
            oscillator: Oscillator::new()
        }
    }

    fn next_sample(&mut self, sample_rate: f32) -> f32 {
        let phase_increment = self.frequency / sample_rate;
        self.oscillator.next_sample(phase_increment)
    }
}

fn note_to_freq(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}
