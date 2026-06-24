mod audio;
mod synth;

use crate::synth::Synth;

fn main() {
    let mut synth = Synth::new();
    synth.note_on(69);
    synth.note_on(73);
    synth.note_on(76);
    audio::build_stream(synth);
}
