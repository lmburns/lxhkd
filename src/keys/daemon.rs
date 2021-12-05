//! Daemon that runs the application in the background

// TODO: Allow the user to add keybindings while the daemon is running

use super::{
    chord::{Chain, ChainLink, Chord},
    keyboard::Keyboard,
};
use crate::{
    config::{Action, Config},
    parse::parser::Line,
};
use anyhow::{Context, Result};
use indexmap::IndexMap;
use std::{collections::BTreeMap, fmt};
use x11rb::protocol::xproto::Timestamp;

// =================== Daemon =====================

/// Global daemon state object.
#[derive(Debug)]
pub(crate) struct Daemon<'a> {
    /// The current keyboard setup
    keyboard:      &'a Keyboard<'a>,
    /// Configuration file of the user
    config:        &'a Config,
    /// The parsed bindings registered in all modes.
    bindings:      BTreeMap<Chain, Action>,
    /// Current chain being pressed
    current_chain: Chain,
    /// Tracker of last keypress
    last_keypress: Timestamp,
}
// /// Max allowed time between keypresses
// keypress_timeout: u32,

impl<'a> Daemon<'a> {
    /// Create a new `Daemon`
    pub(crate) fn new(keyboard: &'a Keyboard<'a>, config: &'a Config) -> Self {
        // keypress_timeout: config.global.timeout.unwrap_or(300),
        Self {
            keyboard,
            config,
            bindings: BTreeMap::new(),
            current_chain: Chain::default(),
            last_keypress: 0,
        }
    }

    /// Parse the configuration bindings
    pub(crate) fn process_bindings(&mut self) -> Result<()> {
        let mut parsed_bindings = BTreeMap::new();

        if let Some(bindings) = &self.config.bindings {
            for (mut idx, l) in bindings.keys().enumerate() {
                idx += 1;

                let line = Line::new_plus(l, idx);
                let mut tokenized = line.tokenize();
                tokenized.parse_tokens()?;

                if let Some(chain) = tokenized.convert_to_chain(&self.keyboard.charmap) {
                    let action = bindings
                        .get_index(idx - 1)
                        .context(
                            "failed to get valid index of item in configuration's `Bindings` \
                             section",
                        )?
                        .1;
                    let cmd = Action::Shell(action.to_string());
                    parsed_bindings.insert(chain, cmd);
                }
            }
        }

        self.bindings = parsed_bindings;

        Ok(())
    }

    /// Parse the `Chords` generated from actions happening while the `Daemon`
    /// is running
    pub(crate) fn process_chords(&mut self, chord: Chord, time: Timestamp) -> Result<()> {
        if self.last_keypress + self.config.global.timeout.unwrap_or(300) < time {
            self.current_chain.clear();
        }

        self.current_chain.push(chord);

        let seen = vec![self
            .keyboard
            .get_next_key(&self.bindings.keys().map(Chain::first).collect::<Vec<_>>())?];

        Ok(())
    }

    // pub(crate) fn get_match(&self, seen: &[Chord]) {
    //     for bind in &self.bindings {
    //         match bind.0.match_chain(&seen) {
    //             ChainLink::None => {},
    //             ChainLink::Partial => partial
    //         }
    //     }
    // }
}

/// The state of the `Daemon` regarding matching keypresses
pub(crate) enum DaemonState {
    /// No keys have been pressed that match any existing keybindings
    None,
    /// Some keys have been pressed that are a prefix to a `Chain`
    Prefix,
    /// All keys in a `Chain` have been pressed
    Full(Chain, Action),
}
