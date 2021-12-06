//! The intermediate structures in the following steps:
//!     (1) Configuration file gets parsed
//!     (2) Further tokenized into `TokenizedLine`s
//!     (3) Transformed into `Chord`s and `Chain`s
//!     (4) Interface with mappings and bindings

use super::{
    keys::{CharacterMap, ModifierMask},
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

/// An abstraction of a single key on the keyboard. Mainly a wrapper for
/// `CharacterMap`
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Chord {
    /// Extra information about the key
    charmap: CharacterMap,
    /// The keysym of the main key in the chord. This is usually not a modifier
    keysym:  XKeysym,
    /// The modmask. This is only used for events sent from the X-Server, not
    /// configuration bindings
    modmask: ModifierMask,
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
    pub(crate) fn new(charmap: &CharacterMap, modmask: u16) -> Self {
        let mut mask = ModifierMask::new(modmask);
        mask.filter_ignored();

        Self {
            charmap: charmap.clone(),
            keysym:  XKeysym::from(charmap.symbol),
            modmask: mask,
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

    // TODO: Use this here or remove Map modifiers to their correct UTF-8
    // representations
    pub(crate) fn map_modifiers<'a>(charmaps: &'a [CharacterMap], tomatch: &'a str) -> &'a str {
        // Will match the `modmask` level of `modN` keys with the `CharacterMap`
        // database, returning the actual modifier for that `mod` key. For example,
        // `match_modmask(3, "Alt_L")` will match `mod1` and if it fails, `Alt_L` will
        // be used instead
        let match_modmask = |mask: u16, or: &'a str| -> &'a str {
            charmaps
                .iter()
                .find(|m| m.modmask == (1 << mask))
                .map_or(or, |a| &a.utf)
        };

        match tomatch.trim() {
            "super" | "lsuper" => "Super_L",
            "rsuper" => "Super_R",
            "hyper" | "lhyper" => "Hyper_L",
            "rhyper" => "Hyper_R",
            "alt" | "lalt" => "Alt_L",
            "ralt" => "Alt_R",
            "shift" | "lshift" => "Shift_L",
            "rshift" => "Shift_R",
            "ctrl" | "lctrl" => "Control_L",
            "rctrl" => "Control_R",
            "mod1" => match_modmask(3, "Alt_L"),
            "mod2" => match_modmask(4, "Num_Lock"),
            // This one is probably not set on most people's keyboards
            "mod3" => match_modmask(5, "Hyper_L"),
            "mod4" => match_modmask(6, "Super_L"),
            "mod5" => match_modmask(7, "ISO_Level3_Lock"),

            other => other,
        }
    }
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

    /// Alternate way to match
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
