//! The intermediate structures in the following steps:
//!     (1) Configuration file gets parsed
//!     (2) Further tokenized into `TokenizedLine`s
//!     (3) Transformed into `Chord`s and `Chain`s
//!     (4) Interface with mappings and bindings

use super::{
    keys::{ButtonCode, CharacterMap, ModifierMask, XButton},
    keysym::XKeysym,
};
use crate::{
    config::Action,
    parse::parser::{Line, Token, TokenizedLine},
};
use anyhow::{Context, Result};
use colored::Colorize;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::BTreeMap, fmt};
use thiserror::Error;
use x11rb::protocol::xproto::Keycode;

// =================== Error ======================

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("not a valid keypress")]
    InvalidKeypress,

    #[error("unable to decode line in configuration file: {0}")]
    DecodeChord(String),
}

// =================== Chord ======================

/// An abstraction of a step in the process of binding keys.
///
/// If this is created from configuration bindings, then everything except
/// `modmask` and `event_type` are filled in. Once this `Chord` gets parsed by
/// the [`Handler`](super::event_handler::Handler), the `modmask` and
/// `event_type` from the reply are filled in
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Chord {
    /// Extra information about the key
    charmap:    CharacterMap,
    /// The keysym of the main key in the chord. This is usually not a modifier
    keysym:     XKeysym,
    /// The button to the chord
    button:     XButton,
    /// Modmask -- only used X-Server events, not configuration
    modmask:    ModifierMask,
    /// Event's response code -- only use X-Server events, not configuration
    event_type: u8,
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "keysym: {}, modmask: {:?}",
            self.keysym, self.charmap.modmask
        )
    }
}

impl PartialOrd for Chord {
    fn partial_cmp(&self, other: &Chord) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Chord {
    fn cmp(&self, other: &Chord) -> Ordering {
        self.keysym
            .cmp(&other.keysym)
            .then(self.charmap.modmask.cmp(&other.charmap.modmask))
    }
}

impl Chord {
    /// Create a new `Chord`
    pub(crate) fn new(
        charmap: &CharacterMap,
        modmask: u16,
        button: ButtonCode,
        event_type: u8,
    ) -> Self {
        let mut mask = ModifierMask::new(modmask);
        mask.filter_ignored();

        Self {
            charmap: charmap.clone(),
            keysym: XKeysym::from(charmap.symbol),
            button: XButton::new(mask, button),
            modmask: mask,
            event_type,
        }
    }

    /// Return the [`Keysym`](super::keysym::XKeysym) of the `Chord`
    pub(crate) fn keysym(&self) -> XKeysym {
        self.keysym
    }

    /// Return the [`Modmask`] as a `u16` of the `CharacterMap`
    pub(crate) fn modmask_inner(&self) -> u16 {
        self.charmap.modmask
    }

    /// Return the the [`ModifierMask`] of the entire `Chord`
    pub(crate) fn modmask(&self) -> ModifierMask {
        self.modmask
    }

    /// Return the [`CharacterMap`] of the `Chord`
    pub(crate) fn charmap(&self) -> &CharacterMap {
        &self.charmap
    }

    /// Return the [`XButton`] of the `Chord`
    pub(crate) fn button(&self) -> XButton {
        self.button
    }

    /// Return the [`event_type`] of the `Chord`
    pub(crate) fn event_type(&self) -> u8 {
        self.event_type
    }

    // /// Make a new `Chord`
    // pub(crate) fn make() -> Self {}
}

// =================== Chain ======================

/// Ordered sequence of `Chord`s that make up a keymapping
#[derive(Debug, PartialEq, Eq, Clone, Default, Ord, PartialOrd)]
pub(crate) struct Chain {
    /// The set of `Chord`s
    chords:     Vec<Chord>,
    /// Total `ModMask` of all `Chord`s
    modmask:    ModifierMask,
    /// Key event is release or press
    is_release: bool,
}

impl Chain {
    /// Create a new `Chain`
    pub(crate) fn new(chords: Vec<Chord>, is_release: bool, modmask: ModifierMask) -> Self {
        Self { chords, modmask, is_release }
    }

    /// Check whether the `Chain` has release `Chord`s
    pub(crate) const fn is_release(&self) -> bool {
        self.is_release
    }

    /// Return the `Chords`
    pub(crate) fn chords(&self) -> &Vec<Chord> {
        &self.chords
    }

    /// Return the the total `ModifierMask` as a `u16`
    pub(crate) fn modmask(&self) -> u16 {
        u16::from(self.modmask)
    }

    /// Clear all active `Chord`s
    pub(crate) fn clear(&mut self) {
        self.chords.clear();
    }

    /// Add `Chord` to the vector of chords
    pub(crate) fn push(&mut self, chord: Chord) {
        self.chords.push(chord);
    }

    /// Return the length of the `Chord`s
    pub(crate) fn len(&self) -> usize {
        self.chords.len()
    }

    /// Determine whether there are any `Chord`s in the `Chain`
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the first `Chord`
    pub(crate) fn first(&self) -> Chord {
        self.chords[0].clone()
    }

    /// Check whether a `Chain` is a prefix of another
    pub(crate) fn is_prefix_of(&self, other: &Chain) -> bool {
        other.chords.starts_with(&self.chords)
    }

    /// Check whether the `Chain`s are matches of each other
    pub(crate) fn matches(&self, other: &Chain) -> ChainLink {
        if self.is_prefix_of(other) {
            if self.chords.len() == other.len() {
                ChainLink::Full
            } else {
                ChainLink::Partial
            }
        } else {
            ChainLink::None
        }
    }

    /// Alternate way to match. May be quicker
    pub(crate) fn match_chain(&self, seen: &Chain) -> ChainLink {
        for (idx, chord) in seen.chords().iter().enumerate() {
            if self.chords[idx] != *chord {
                return ChainLink::None;
            }
        }

        if self.chords.len() == seen.len() {
            ChainLink::Full
        } else {
            ChainLink::Partial
        }
    }
}

/// Return a match level to the `Chain`
/// Instead of using 'yes' or 'no', match on three levels
pub(crate) enum ChainLink {
    /// Doesn't match at all
    None,
    /// Matches somewhat
    Partial,
    /// They are the same
    Full,
}
