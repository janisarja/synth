mod audio;
mod synth;

use crate::synth::{Synth, SynthEvent};

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<SynthEvent>();
    let synth = Synth::new();

    tx.send(SynthEvent::NoteOn(69)).unwrap();
    tx.send(SynthEvent::NoteOn(73)).unwrap();
    tx.send(SynthEvent::NoteOn(76)).unwrap();

    audio::build_stream(synth, rx);
}
