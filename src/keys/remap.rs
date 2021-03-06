//! The state of the keys that are to be remapped

use super::{
    chord::{Chain, Chord},
    keys::{ButtonCode, CharacterMap},
};
use crate::{
    config::Config,
    parse::parser::{TokenizedLine, MOD_PATTERN},
};
use anyhow::{Context, Result};
use colored::Colorize;
use indexmap::IndexMap;
use itertools::Itertools;
use std::{
    sync::atomic::{AtomicBool, AtomicU8, Ordering},
    time::SystemTime,
};
use x11rb::protocol::xproto::{self, Keycode, Keysym, Timestamp};

// ================ RemapKeyState =================

/// Struct that holds data of a single key.
/// This key is mapped to another and will be replaced when tapped
#[derive(Debug, Default)]
pub(crate) struct RemapKeyState {
    /// Key that is going to change
    from_key:  Chord,
    /// Key(s) that will replace the `from_key`
    to_keys:   Vec<Chord>,
    /// State of the key being pressed
    pressed:   AtomicBool,
    /// Is the key generated by this program?
    ///
    /// This is needed because when a fake input is created using
    /// `xtest_fake_input`, the event is parsed through the `daemonize`
    /// function the same way any other event is. There needs to be a way to
    /// determine whether the input was generated by the user
    /// pushing a button or a command in this program
    generated: AtomicBool,
    // If the key is held/used with another key, it is considered to be a modifier
    is_used:   AtomicBool,

    // TODO: Use mouse
    mouse_button: AtomicU8,
}

// /// Is the key being used?
// used:        AtomicBool,
// /// Time the key was pressed
// down_at:     Option<SystemTime>,

impl RemapKeyState {
    /// Create an `RemapKeyState` from 2 [`Chain`](crate::keys::chord::Chain).
    /// `Chain`s are used since `TokenizedLine` already parses the
    /// configuration into `Chain`s
    pub(crate) fn from_chains(from: &Chain, to: &Chain) -> Option<Self> {
        if from.chords().is_empty() || to.chords().is_empty() {
            return None;
        }

        Some(Self {
            from_key:     from.chords().get(0)?.clone(),
            to_keys:      to.chords().iter().map(Clone::clone).collect::<Vec<_>>(),
            pressed:      AtomicBool::new(false),
            mouse_button: AtomicU8::new(from.chords().get(0)?.button().code()),
            generated:    AtomicBool::new(false),
            is_used:      AtomicBool::new(
                to.chords()
                    .iter()
                    .any(|c| MOD_PATTERN.is_match(c.charmap().utf())),
            ),
        })
    }

    /// Return the keys that are being mapped (`from_key`)
    pub(crate) fn from_key(&self) -> &Chord {
        &self.from_key
    }

    /// Return the keys that are being mapped to (`to_keys`)
    pub(crate) fn to_keys(&self) -> &[Chord] {
        &self.to_keys
    }

    /// Return the mouse button
    pub(crate) fn mouse_button(&self) -> u8 {
        self.mouse_button.load(Ordering::Relaxed)
    }

    /// Return `true` if the binding is a mouse button
    pub(crate) fn is_mouse(&self) -> bool {
        self.mouse_button() > 0
    }

    /// Return `true` if binding is a modifier (i.e., held down with other keys)
    pub(crate) fn is_used(&self) -> bool {
        self.is_used.load(Ordering::Relaxed)
    }

    // /// Convert the key that is to be transformed into a `Chord`
    // pub(crate) fn from_to_chord(&self) -> Chord {
    //     Chord::new(
    //         &self.from_key,
    //         self.from_key.modmask(),
    //         ButtonCode::from(self.mouse_button.load(Ordering::Relaxed)),
    //         self.mouse_button
    //             .load(Ordering::Relaxed)
    //             .gt(&0)
    //             .then(|| xproto::BUTTON_PRESS_EVENT)
    //             .unwrap_or(xproto::KEY_PRESS_EVENT),
    //     )
    // }
}

// ================== RemapState ==================

/// The overall state of remapped keys
#[derive(Debug, Default)]
pub(crate) struct RemapState {
    /// Is the mouse held?
    mouse_held:    AtomicBool,
    /// The keys that are remapped when pressed
    remapped_keys: Vec<RemapKeyState>,
}

impl RemapState {
    /// Create a blank `RemapState`
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Return the `mouse_held` field
    pub(crate) fn mouse_held(&self) -> bool {
        self.mouse_held.load(Ordering::Relaxed)
    }

    /// Return the `remapped_keys` field
    pub(crate) fn remapped_keys(&self) -> &[RemapKeyState] {
        &self.remapped_keys
    }

    /// Insert a `RemapKeyState` into the inner vector
    pub(crate) fn insert(&mut self, remapped: RemapKeyState) {
        self.remapped_keys.push(remapped);
    }

    /// Return whether there are any remapped keys
    pub(crate) fn is_empty(&self) -> bool {
        self.remapped_keys.is_empty()
    }

    /// Set the `mouse_held` field
    pub(crate) fn set_mouse_held(&mut self, value: bool) {
        self.mouse_held.store(value, Ordering::Relaxed);
    }

    /// Reverse the action of the mouse. held => not held; not held => held
    pub(crate) fn reverse_mouse(&mut self) {
        self.mouse_held
            .store(!self.mouse_held.load(Ordering::Relaxed), Ordering::Relaxed);
    }

    /// Change inner state to pressed
    pub(crate) fn mark_pressed(&mut self, key: Keycode) -> Option<bool> {
        self.remapped_keys
            .iter()
            .find(|m| m.from_key().charmap().code() == key)
            .map(|ref mut map| {
                let old = &map.pressed;
                map.pressed.store(true, Ordering::Relaxed);

                log::debug!(
                    "{}: {} => {}; {}: {} => true",
                    "updated key".green().bold(),
                    map.from_key().charmap().utf(),
                    map.to_keys().iter().map(|c| c.charmap().utf()).join(","),
                    "press".red().bold(),
                    old.load(Ordering::Relaxed),
                );

                old.load(Ordering::Relaxed)
            })
    }

    /// Mark keys that have already been marked as `pressed` as no longer being
    /// `pressed`
    pub(crate) fn mark_released(&mut self, key: Keycode) -> Option<bool> {
        self.remapped_keys
            .iter()
            .find(|m| m.from_key().charmap().code() == key)
            .map(|ref mut map| {
                let (old_pressed, old_used) = (&map.pressed, &map.is_used);
                map.pressed.store(false, Ordering::Relaxed);
                map.is_used.store(false, Ordering::Relaxed);

                log::debug!(
                    "{}: {} => {}; {}: ({},{}) => (false,false)",
                    "updated key".green().bold(),
                    map.from_key().charmap().utf(),
                    map.to_keys().iter().map(|c| c.charmap().utf()).join(","),
                    "release".red().bold(),
                    old_pressed.load(Ordering::Relaxed),
                    old_used.load(Ordering::Relaxed),
                );

                // (
                old_pressed.load(Ordering::Relaxed)
                // old_used.load(Ordering::Relaxed),
                // )
            })
    }

    /// Mark all keys that are currently held as a modifier. Or if the key is
    /// pressed along with the mouse, it will be marked as a modifier
    ///
    /// Chances are if a key is held when pressing another key, that key is a
    /// modifier. Or if a key is held whenever the mouse is pressed then
    /// that key is a modifier
    pub(crate) fn set_modifier(&mut self) {
        self.remapped_keys
            .iter()
            .filter(|to| to.pressed.load(Ordering::Relaxed) || self.mouse_held())
            .for_each(|mut to| {
                to.is_used.store(true, Ordering::Relaxed);
            });
    }

    // FIXME: If different mappings contain one of same keys, this will pick both up

    /// Mark the keys that are created by this program as being `generated`
    pub(crate) fn mark_generated(&mut self, key: Keycode) {
        if let Some(ref mut map) = &mut self
            .remapped_keys
            .iter()
            .find(|m| m.to_keys().iter().any(|to| to.charmap().code() == key))
        {
            log::debug!(
                "{}: changing generated to true for {}",
                "remap".red().bold(),
                map.to_keys().iter().map(|c| c.charmap().utf()).join(","),
            );

            map.generated.store(true, Ordering::Relaxed);
        }
    }

    /// Check if the key has been `generated`. If so, change it to not
    /// being `generated`
    pub(crate) fn check_if_generated(&mut self, key: Keycode) -> bool {
        if let Some(ref mut map) = &mut self
            .remapped_keys
            .iter()
            .find(|m| m.to_keys().iter().any(|to| to.charmap().code() == key))
        {
            log::debug!(
                "{}: changing generated to false for {}",
                "remap".red().bold(),
                map.to_keys().iter().map(|c| c.charmap().utf()).join(","),
            );
            map.generated.store(false, Ordering::Relaxed);
            true
        } else {
            false
        }
    }
}
