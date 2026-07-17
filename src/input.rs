use std::sync::mpsc::Sender;
use std::collections::HashSet;
use crossterm::event::{Event, KeyEventKind, KeyCode, read};

use crate::synth::SynthCommand;

pub fn listen_keyboard(tx: Sender<SynthCommand>) {
    std::thread::spawn(move || {
        let mut held = HashSet::new();
        
        loop {
            if let Ok(Event::Key(key)) = read() {
                if key.kind == KeyEventKind::Press {
                    if let Some(note) = key_to_note(key.code) {
                        if held.insert(key.code) {
                            tx.send(SynthCommand::PlayNote(note)).unwrap();
                        }
                    }
                }
                if key.kind == KeyEventKind::Release {
                    if let Some(note) = key_to_note(key.code) {
                        held.remove(&key.code);
                        tx.send(SynthCommand::ReleaseNote(note)).unwrap();
                    }
                }
            }
        }
    });
}

fn key_to_note(code: KeyCode) -> Option<u8> {
    match code {
        KeyCode::Char('a') => Some(60),
        KeyCode::Char('w') => Some(61),
        KeyCode::Char('s') => Some(62),
        KeyCode::Char('e') => Some(63),
        KeyCode::Char('d') => Some(64),
        KeyCode::Char('f') => Some(65),
        KeyCode::Char('t') => Some(66),
        KeyCode::Char('g') => Some(67),
        KeyCode::Char('y') => Some(68),
        KeyCode::Char('h') => Some(69),
        KeyCode::Char('u') => Some(70),
        KeyCode::Char('j') => Some(71),
        KeyCode::Char('k') => Some(72),
        _ => None,
    }
}
