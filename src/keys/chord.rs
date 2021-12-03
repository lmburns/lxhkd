//! The intermediate structures in the following steps:
//!     (1) Configuration file gets parsed
//!     (2) Further tokenized into `TokenizedLine`s
//!     (3) Transformed into `Chord`s and `Chain`s
//!     (4) Interface with mappings and bindings

use super::{
    keys::{CharacterMap, ModifierMask},
    keysym::XKeysym,
};
use crate::parse::parser::{Line, Token, TokenizedLine};
use anyhow::{Context, Result};
use colored::Colorize;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt};
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Chord2 {
    /// An ordered hash of the `Keycode` linked to the full key information
    charmaps:   IndexMap<Keycode, CharacterMap>,
    /// The keysym of the main key in the chord. This is usually not a modifier
    keysym:     XKeysym,
    /// The total modmask
    modmask:    ModifierMask,
    /// Whether the binding is a press or release
    is_release: bool,
}

impl Chord2 {
    pub(crate) fn new(
        charmaps: IndexMap<Keycode, CharacterMap>,
        modmask: ModifierMask,
        is_release: bool,
    ) -> Result<Self> {
        let keysym = XKeysym::from(
            charmaps
                .last()
                .context("failed to get last item in index map")?
                .1
                .symbol,
        );

        Ok(Self { charmaps, keysym, modmask, is_release })
    }
}

// impl fmt::Display for Chord {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "keysym: {}, modmask: {:?}", self.keysym, self.modmask)
//     }
// }
//
// impl PartialOrd for Chord {
//     fn partial_cmp(&self, other: &Chord) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
// impl Ord for Chord {
//     fn cmp(&self, other: &Chord) -> Ordering {
//         let modmask = u16::from(self.modmask);
//
//         self.keysym
//             .cmp(&other.keysym)
//             .then(modmask.cmp(&other.modmask.into()))
//     }
// }

// =================== Chord ======================

/// The abstraction of a key on the keyboard plus a one or more modifiers being
/// held down produces an [`XKeysym`](super::keysym::XKeysym)
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Copy)]
pub(crate) struct Chord {
    /// The keysym of the chord. This is usually not a modifier
    keysym:  XKeysym,
    /// Modifier mask of held keys
    modmask: ModifierMask,
}

impl Chord {
    /// Convert a vector of `CharacterMap`s to a single `Chord`. The
    /// `CharacterMap` is created from a
    /// [`TokenizedLine`](crate::parse::parser::TokenizedLine)
    pub(crate) fn from_charmaps(charmaps: &[CharacterMap]) {
        for ch in charmaps {
            println!("code: {}, mask; {}", ch.code, ch.modmask);
        }
    }
}

// impl<'a> TryFrom<TokenizedLine<'a>> for Chord {
//     type Error = Error;
//
//     fn try_from(line: TokenizedLine<'a>) -> Result<Self, Self::Error> {
//         let flattened = line.flatten_it();
//     }
// }

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "keysym: {}, modmask: {:?}", self.keysym, self.modmask)
    }
}

impl PartialOrd for Chord {
    fn partial_cmp(&self, other: &Chord) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Chord {
    fn cmp(&self, other: &Chord) -> Ordering {
        let modmask = u16::from(self.modmask);

        self.keysym
            .cmp(&other.keysym)
            .then(modmask.cmp(&other.modmask.into()))
    }
}

impl Chord {
    /// Create a new instance of a `Chord`
    pub(crate) fn new(keysym: XKeysym, mut modmask: ModifierMask) -> Self {
        modmask.filter_ignored();
        Self { keysym, modmask }
    }

    /// Return the [`Keysym`](super::keysym::XKeysym) of the `Chord`
    pub(crate) fn keysym(self) -> XKeysym {
        self.keysym
    }

    /// Return the [`Modmask`] as a `u16` of the `Chord`
    pub(crate) fn modmask(self) -> u16 {
        self.modmask.mask()
    }
}

// =================== Chain ======================

/// A chain of [`Chord`](self::Chord)'s
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Clone)]
pub(crate) struct Chain {
    /// The chords that make up the chain
    chords: Vec<Chord>,
}

// impl From<TokenizedLine<'a>> for Chain {
//     fn from(tok: TokenizedLine<'a>) -> Chain {
//     }
// }

// impl Chain {
//     /// Create a new instance of `Chain` from a string
//     pub(crate) fn from_string(line: &str) -> Result<Self, Error> {
//         let mut chords = vec![];
//
//         for expr in line.split(' ') {
//             chords.push(Chord::from_string(expr)?);
//         }
//
//         Ok(Chain { chords })
//     }
//
//     /// Determine whether the `Chain` is a prefix of another
//     pub(crate) fn is_prefix_of(&self, other: &Chain) -> bool {
//         other.chords.starts_with(&self.chords)
//     }
//
//     /// Return the inner vector of [`Chord`]'s
//     pub(crate) fn chords(&self) -> &Vec<ChordDesc> {
//         &self.chords
//     }
//
//     /// Remove all chords
//     pub(crate) fn clear(&mut self) {
//         self.chords.clear();
//     }
//
//     /// Add `Chord` to the vector of chords
//     pub(crate) fn push(&mut self, chord: Chord) {
//         self.chords.push(chord);
//     }
//
//     /// Return the length of the `Chord`'s
//     pub(crate) fn len(&self) -> usize {
//         self.chords.len()
//     }
//
//     /// Determine whether there are any `Chords` in the `Chain`
//     pub(crate) fn is_empty(&self) -> bool {
//         self.len() == 0
//     }
// }
