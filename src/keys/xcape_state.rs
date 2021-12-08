//! The state of the `Xcape` part of this crate.
//!
//! This file contains the struct that holds information about the keys to be
//! simulated when tapped  and their corresponding keycode values

use super::{chord::Chain, keys::CharacterMap};
use crate::{
    config::Config,
    parse::parser::{TokenizedLine, MOD_PATTERN},
};
use anyhow::{Context, Result};
use colored::Colorize;
use indexmap::IndexMap;
use itertools::Itertools;
use regex::{Captures, Regex};
use std::sync::atomic::{AtomicBool, Ordering};
use x11rb::protocol::xproto::{Keycode, Keysym, Timestamp};

// ================ GenericEvent ==================

/// Struct that holds data of a single key.
/// This key is mapped to another and will be replaced when tapped
#[derive(Debug, Default)]
pub(crate) struct XcapeKeyState {
    /// Key that are going to change
    from_keys:      CharacterMap,
    /// Key that will replace the `from_keys`
    to_keys:        Vec<CharacterMap>,
    /// Is the key currently pressed?
    pressed:        AtomicBool,
    /// Is the key generated by this program?
    auto_generated: AtomicBool,

    // ??
    is_modifier: AtomicBool,

    // TODO: Use mouse
    mouse_button: AtomicBool,
}

// use_keycode: bool,
// fake_keys:   Vec<u8>,
// used:        bool,
// down_at:     Timestamp,

impl XcapeKeyState {
    /// Create an `XcapeKeyState` from 2 `Chain`s
    pub(crate) fn from_chains(from: &Chain, to: &Chain) -> Option<Self> {
        if from.chords().is_empty() || to.chords().is_empty() {
            return None;
        }

        Some(Self {
            from_keys:      from.chords()[0].charmap().clone(),
            to_keys:        to
                .chords()
                .iter()
                .map(|c| c.charmap().clone())
                .collect::<Vec<_>>(),
            pressed:        AtomicBool::new(false),
            auto_generated: AtomicBool::new(false),
            mouse_button:   AtomicBool::new(false),
            is_modifier:    AtomicBool::new(
                to.chords()
                    .iter()
                    .any(|c| MOD_PATTERN.is_match(c.charmap().utf())),
            ),
        })
    }

    /// Return the keys that are being mapping (`from_keys`)
    pub(crate) fn from_keys(&self) -> &CharacterMap {
        &self.from_keys
    }

    /// Return the keys that are the new mapped (`to_keys`)
    pub(crate) fn to_keys(&self) -> &[CharacterMap] {
        &self.to_keys
    }

    /// Return `true` if binding is mouse button
    pub(crate) fn mouse_button(&self) -> bool {
        self.mouse_button.load(Ordering::Relaxed)
    }

    /// Return `true` if binding is a modifier
    pub(crate) fn is_modifier(&self) -> bool {
        self.is_modifier.load(Ordering::Relaxed)
    }
}

// ================== XcapeState ==================

/// The overall state of `Xcape`
#[derive(Debug, Default)]
pub(crate) struct XcapeState {
    /// Is the mouse held?
    mouse_held:    AtomicBool,
    /// The keys that are remapped when pressed
    remapped_keys: Vec<XcapeKeyState>,
    /// The characters, keysyms, etc making up the `Keyboard`
    charmap:       Vec<CharacterMap>,
}

impl XcapeState {
    /// Create a blank `XcapeState` except for the `CharacterMap` database
    pub(crate) fn new(charmap: &[CharacterMap]) -> Self {
        Self { charmap: charmap.to_vec(), ..XcapeState::default() }
    }

    /// Return the `mouse_held` field
    pub(crate) fn mouse_held(&self) -> bool {
        self.mouse_held.load(Ordering::Relaxed)
    }

    /// Return the `remapped_keys` field
    pub(crate) fn remapped_keys(&self) -> &[XcapeKeyState] {
        &self.remapped_keys
    }

    /// Return the `charmap` field
    pub(crate) fn charmap(&self) -> &[CharacterMap] {
        &self.charmap
    }

    /// Insert an `XcapeKeyState` into the inner vector
    pub(crate) fn insert(&mut self, remapped: XcapeKeyState) {
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
        let pressed = true;

        self.remapped_keys
            .iter()
            .find(|m| m.from_keys().code() == key)
            .map(|ref mut map| {
                let old = &map.pressed;
                map.pressed.store(pressed, Ordering::Relaxed);

                log::debug!(
                    "{}: {} => {}; {}: {} => {}",
                    "updated key".green().bold(),
                    map.from_keys().utf(),
                    map.to_keys().iter().map(CharacterMap::utf).join(","),
                    "press".red().bold(),
                    old.load(Ordering::Relaxed),
                    pressed
                );

                old.load(Ordering::Relaxed)
            })
    }

    /// Mark keys that have already been marked as `pressed` as released
    pub(crate) fn mark_released(&mut self, key: Keycode) -> Option<(bool, bool)> {
        self.remapped_keys
            .iter()
            .find(|m| m.from_keys().code() == key)
            .map(|ref mut map| {
                let old = (&map.pressed, &map.is_modifier);
                log::debug!(
                    "{}: {} => {}; {}: ({},{}) => (false,false)",
                    "updated key".green().bold(),
                    map.from_keys().utf(),
                    map.to_keys().iter().map(CharacterMap::utf).join(","),
                    "release".red().bold(),
                    old.0.load(Ordering::Relaxed),
                    old.1.load(Ordering::Relaxed),
                );
                map.pressed.store(false, Ordering::Relaxed);
                map.is_modifier.store(false, Ordering::Relaxed);

                (old.0.load(Ordering::Relaxed), old.1.load(Ordering::Relaxed))
            })
    }

    /// Mark all keys that are currently pressed or if the mouse is currently
    /// pressed as being used as a modifier
    pub(crate) fn use_all_mods(&mut self) {
        // for key in &mut self.remapped_keys {
        //     if key.pressed.load(Ordering::Relaxed) ||
        // self.mouse_held.load(Ordering::Relaxed) {         key.is_modifier.
        // store(true, Ordering::Relaxed);     }
        // }

        self.remapped_keys
            .iter()
            .filter(|to| to.pressed.load(Ordering::Relaxed) || self.mouse_held())
            .for_each(|mut to| {
                to.is_modifier.store(true, Ordering::Relaxed);
            });
    }

    // FIXME: If different mappings contain one of same keys, this will pick both up

    /// Mark the keys as being `auto_generated`
    pub(crate) fn mark_auto_generated(&mut self, key: Keycode) {
        if let Some(ref mut map) = &mut self
            .remapped_keys
            .iter()
            .find(|m| m.to_keys().iter().any(|to| to.code() == key))
        {
            log::debug!(
                "{}: changing auto_generated to true for {}",
                "xcape".red().bold(),
                map.to_keys().iter().map(CharacterMap::utf).join(","),
            );

            map.auto_generated.store(true, Ordering::Relaxed);
        }
    }

    /// Check if the key has been `auto_generated`. If so, change it to not
    /// being `auto_generated`
    pub(crate) fn check_if_auto_generated(&mut self, key: Keycode) -> bool {
        if let Some(ref mut map) = &mut self.remapped_keys.iter().find(|m| {
            m.to_keys().iter().any(|to| to.code() == key)
                && m.auto_generated.load(Ordering::Relaxed)
        }) {
            log::debug!(
                "{}: changing auto_generated to false for {}",
                "xcape".red().bold(),
                map.to_keys().iter().map(CharacterMap::utf).join(","),
            );
            map.auto_generated.store(false, Ordering::Relaxed);
            true
        } else {
            false
        }
    }
}
