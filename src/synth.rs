mod voice;

use voice::Voice;

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
            if voice.note == note && !voice.is_released() {
                voice.release();
                break;
            }
        }
    }
}
