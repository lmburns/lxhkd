//! Handle `Events` sent from the X-Server

use super::{chord::Chord, keyboard::Keyboard, keys::CharacterMap};
use anyhow::{Context, Result};
use colored::Colorize;
use x11rb::{
    connection::{Connection, RequestConnection},
    protocol::xproto::KeyPressEvent,
};

// ================== Handler =====================

pub(crate) struct Handler;

impl Handler {
    /// Handles the `KeyPressEvent` sent from the X-Server. A new `Chord` is
    /// created from the matching `CharacterMap` and `mask` from the event
    pub(crate) fn handle_keypress(event: &KeyPressEvent, keyboard: &Keyboard) -> Option<Chord> {
        let keycode = event.detail;
        let mask = event.state;
        // let time = event.time;

        println!("HIT HANDLER: kc; {} mask; {}", keycode, mask);
        let charmap = CharacterMap::charmap_from_keycode(keyboard.charmap(), keycode)
            .with_context(|| format!("failed to find a `CharacterMap` for keycode={}", keycode))
            .ok()?;

        // let ch = Chord::new(&charmap, mask);
        // println!("X-SEVER CHORD: {:#?}", ch);

        Some(Chord::new(&charmap, mask))
    }
}
