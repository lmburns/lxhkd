//! Handle `Events` sent from the X-Server

use super::{chord::Chord, keyboard::Keyboard, keys::CharacterMap};
use anyhow::{Context, Result};
use colored::Colorize;
use x11rb::{
    connection::{Connection, RequestConnection},
    protocol::xproto::{self, ButtonPressEvent, KeyPressEvent, KeyReleaseEvent},
};

// ================== Handler =====================

/// Wrapper struct around handling of X-Events
pub(crate) struct Handler;

impl Handler {
    /// Handles the `KeyPressEvent` sent from the X-Server. A `KeyPressEvent`
    /// and `KeyReleaseEvent` are the same besides their event mask.  A new
    /// `Chord` is created from the matching `CharacterMap` and `mask` from
    /// the event
    pub(crate) fn handle_key_press(event: &KeyPressEvent, keyboard: &Keyboard) -> Option<Chord> {
        let keycode = event.detail;
        let mask = event.state;

        match event.response_type {
            xproto::KEY_PRESS_EVENT => {
                log::debug!("key press handler: kc:{}-mask:{}", keycode, mask);
                // println!("PRESS EVENT; {:#?}", event);

                let charmap = CharacterMap::charmap_from_keycode(keyboard.charmap(), keycode)
                    .with_context(|| {
                        format!("failed to find a `CharacterMap` for keycode={}", keycode)
                    })
                    .ok()?;

                Some(Chord::new(&charmap, mask, 0.into(), event.response_type))
            },
            xproto::KEY_RELEASE_EVENT => {
                // log::debug!("key release handler: kc:{}-mask:{}", keycode, mask);
                // // println!("RELEASE EVENT; {:#?}", event);
                //
                // let charmap = CharacterMap::charmap_from_keycode(keyboard.charmap(), keycode)
                //     .with_context(|| {
                //         format!("failed to find a `CharacterMap` for keycode={}", keycode)
                //     })
                //     .ok()?;
                //
                // Some(Chord::new(&charmap, mask))
                None
            },
            _ => None,
        }
    }

    pub(crate) fn handle_key_release(
        event: &KeyReleaseEvent,
        keyboard: &Keyboard,
    ) -> Option<Chord> {
        let keycode = event.detail;
        let mask = event.state;

        match event.response_type {
            xproto::KEY_PRESS_EVENT => {
                // log::debug!("key press handler: kc:{}-mask:{}", keycode, mask);
                // // println!("PRESS EVENT; {:#?}", event);
                //
                // let charmap = CharacterMap::charmap_from_keycode(keyboard.charmap(), keycode)
                //     .with_context(|| {
                //         format!("failed to find a `CharacterMap` for keycode={}", keycode)
                //     })
                //     .ok()?;
                //
                // Some(Chord::new(&charmap, mask))
                None
            },
            xproto::KEY_RELEASE_EVENT => {
                log::debug!("key release handler: kc:{}-mask:{}", keycode, mask);
                // println!("RELEASE EVENT; {:#?}", event);

                let charmap = CharacterMap::charmap_from_keycode(keyboard.charmap(), keycode)
                    .with_context(|| {
                        format!("failed to find a `CharacterMap` for keycode={}", keycode)
                    })
                    .ok()?;

                Some(Chord::new(&charmap, mask, 0.into(), event.response_type))
            },
            _ => None,
        }
    }
}
