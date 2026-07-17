mod audio;
mod input;
mod synth;

use crate::synth::{Synth, SynthEvent};

fn main() {
    // Channel for input. Sender is passed to input and receiver to audio 
    // processing which calls synth to handle events in audio callback.
    let (tx, rx) = std::sync::mpsc::channel::<SynthEvent>();

    let synth = Synth::new();

    input::listen_keyboard(tx);
    audio::build_stream(synth, rx);
}
