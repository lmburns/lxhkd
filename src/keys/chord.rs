// use super::{
//     keysym::XKeysym,
//     mask::XModMask,
//     types::{KeyCodeMask, KeyCodeValue, XkbEvent},
// };

use colored::Colorize;
use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("not a valid keypress")]
    InvalidKeypress,

    #[error("unable to decode line in configuration file: {0}")]
    DecodeChord(String),
}


/// The abstraction of a key on the keyboard plus a one or more modifiers being
/// held down produces an [`XKeysym`](super::keysym::XKeysym)
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Chord {
    /// The keysym of the chord
    keysym:  XKeysym,
    /// Modifier mask of non-pressed keys
    modmask: XModMask,
}

impl Ord for Chord {
    fn cmp(&self, other: &Chord) -> Ordering {
        let modmask: u32 = self.modmask.into();

        self.keysym
            .cmp(&other.keysym)
            .then(modmask.cmp(&other.modmask.into()))
    }
}

impl PartialOrd for Chord {
    fn partial_cmp(&self, other: &Chord) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Chord {
    /// Construct a chord from a string found in the configuration file
    pub fn from_string(line: &str) -> Result<Self, Error> {
        let mut modmask = XModMask::from(xkb::ModMask(0));
        let keys = line.split('+').map(|s| s.trim()).collect::<Vec<&str>>();

        for key in keys {
            if let Ok(modifier) = KeyModifier::from_str(key) {
                if modmask.from_modifier(modifier) {
                    log::debug!(
                        "key modifier decoded, continuing the chord: {} (modmask={:b})",
                        key,
                        modmask.inner.0
                    );
                } else {
                    log::error!(
                        "unable to decode key modifier's mask: {} (modmask={:b})",
                        key,
                        modmask.inner.0
                    );
                }
            } else if let Ok(sym) = xkb::Keysym::from_str(key) {
                log::debug!(
                    "keysym decoded, assuming the end of chord: {} ({:?})",
                    key,
                    sym
                );
                // modmask.filter_ignored();
                return Ok(Chord::new(XKeysym::new(sym), modmask));
            } else {
                log::error!(
                    "unable to decode keysym or modifier ({}) from the following line in \
                     configuration file:\n{}",
                    key.red().bold(),
                    line
                );
            }
        }

        Err(Error::DecodeChord(line.to_string()))
    }

    /// Create a new instance of a `Chord`
    pub(crate) fn new(keysym: XKeysym, mut modmask: XModMask) -> Self {
        modmask.filter_ignored();
        Self { keysym, modmask }
    }

    /// Return the [`Keysym`](super::keysym::XKeysym) of the `Chord`
    pub(crate) fn keysym(&self) -> XKeysym {
        self.keysym
    }

    /// Return the [`Modmask`] as a `u16` of the `Chord`
    pub(crate) fn modmask(&self) -> u16 {
        self.modmask.0 as u16
    }
}

/// A chain of [`Chord`](self::Chord)'s
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Chain {
    /// The chords that make up the chain
    chords: Vec<Chord>,
}

impl Chain {
    /// Create a new instance of `Chain` from a string
    pub(crate) fn from_string(line: &str) -> Result<Self, Error> {
        let mut chords = vec![];

        for expr in line.split(' ') {
            chords.push(Chord::from_string(expr)?);
        }

        Ok(Chain { chords })
    }

    /// Determine whether the `Chain` is a prefix of another
    pub(crate) fn is_prefix_of(&self, other: &Chain) -> bool {
        other.chords.starts_with(&self.chords)
    }

    /// Return the inner vector of [`Chord`]'s
    pub(crate) fn chords(&self) -> &Vec<ChordDesc> {
        &self.chords
    }

    /// Remove all chords
    pub(crate) fn clear(&mut self) {
        self.chords.clear();
    }

    /// Add `Chord` to the vector of chords
    pub(crate) fn push(&mut self, chord: Chord) {
        self.chords.push(chord);
    }

    /// Return the length of the `Chord`'s
    pub(crate) fn len(&self) -> usize {
        self.chords.len()
    }

    /// Determine whether there are any `Chords` in the `Chain`
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }
}