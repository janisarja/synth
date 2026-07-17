use crate::synth::Oscillator;

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

        self.voices.retain(|voice| !voice.released);

        for voice in &mut self.voices {
            sample += voice.next_sample(sample_rate);
        }

        // TODO: Implement a proper mixer.
        sample /= self.voices.len().max(1) as f32;

        sample
    }

    pub fn handle_event(&mut self, event: SynthCommand) {
        let length = self.voices.len();
        println!("{length} voices");
        match event {
            SynthCommand::PlayNote(note) => self.voices.push(Voice::new(note)),
            SynthCommand::ReleaseNote(note) => self.release_note(note),
        }
    }

    fn release_note(&mut self, note: u8) {
        for voice in &mut self.voices {
            if voice.note == note && !voice.released {
                voice.release();
                break;
            }
        }
    }
}

struct Voice {
    note: u8,
    oscillator: Oscillator,
    released: bool,
}

impl Voice {
    pub fn new(note: u8) -> Self {
        Self {
            note: note,
            oscillator: Oscillator::new(),
            released: false,
        }
    }

    fn release(&mut self) {
        self.released = true;
    }

    fn next_sample(&mut self, sample_rate: f32) -> f32 {
        let phase_increment = note_to_freq(self.note) / sample_rate;
        self.oscillator.next_sample(phase_increment)
    }
}

fn note_to_freq(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}
