//! Daemon that runs the application in the background

// TODO: Allow the user to add keybindings while the daemon is running

use super::{
    chord::{Chain, ChainLink, Chord},
    event_handler::Handler,
    keyboard::Keyboard,
    keys::CharacterMap,
};
use crate::{
    config::{Action, Config},
    lxhkd_fatal,
    parse::parser::Line,
};
use anyhow::{Context, Result};
use colored::Colorize;
use indexmap::IndexMap;
use std::{collections::BTreeMap, fmt};
use x11rb::{
    connection::Connection,
    protocol::{
        xproto::{self, Timestamp},
        Event,
    },
};

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
    active_chain:  Chain,
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
            active_chain: Chain::default(),
            last_keypress: 0,
        }
    }

    // TODO: If binding contains an unknown, the skip it

    /// Parse the configuration bindings
    pub(crate) fn process_bindings(&mut self) -> Result<()> {
        let mut parsed_bindings = BTreeMap::new();

        if let Some(bindings) = &self.config.bindings {
            for (mut idx, l) in bindings.keys().enumerate() {
                idx += 1;

                let line = Line::new_plus(l, idx);
                let mut tokenized = line.tokenize();
                tokenized.parse_tokens()?;

                if let Some(chain) = tokenized.convert_to_chain(self.keyboard.charmap()) {
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
    pub(crate) fn process_chords(&mut self, chord: Chord, time: Timestamp, response_type: u8) {
        if self.last_keypress + self.config.global.timeout.unwrap_or(300) < time {
            self.active_chain.clear();
        }

        self.active_chain.push(chord);
        let mut should_clear = true;

        for (chain, action) in &self.bindings {
            match self.active_chain.matches(chain) {
                ChainLink::None => {},
                ChainLink::Partial => {
                    log::info!("partially matched binding: {:#?}", action);
                    log::info!("partially matched chain: {:#?}", self.active_chain.chords());

                    should_clear = false;
                },
                // FIX: Differentiate between key press and release
                // For some reason, when a key is held, it registers a release event after a certain
                // amout of time
                ChainLink::Full => match (chain.is_release(), response_type) {
                    (true, xproto::KEY_RELEASE_EVENT) | (false, xproto::KEY_PRESS_EVENT) => {
                        log::info!("matched binding: {:?}", action);
                        log::info!(
                            "matched utf-code {:#?}",
                            self.active_chain
                                .chords()
                                .iter()
                                .map(|ch| format!(
                                    "({}-{})",
                                    ch.charmap().utf.clone(),
                                    ch.charmap().code,
                                ))
                                .collect::<Vec<_>>()
                                .join(", ")
                        );

                        action.run(&self.config.global.shell);

                        should_clear = true;
                        break;
                    },
                    _ => {},
                },
            }
        }

        if should_clear {
            self.active_chain.clear();
        }

        self.last_keypress = time;
    }

    /// Start the loop that gets daemonized. Monitor X11 key presses that are
    /// prefixed by keys found within the configuration file
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn daemonize(&mut self) -> Result<()> {
        // let mut idx = 0;

        // println!("BINDINGS: {:#?}", self.bindings);
        // std::process::exit(2);
        for chain in self.bindings.keys() {
            self.keyboard.grab_key(chain.chords());
        }

        loop {
            self.keyboard.flush();

            // if idx == 8 {
            //     self.keyboard.cleanup()?;
            //     break;
            // }
            // idx += 1;

            let event = self.keyboard.wait_for_event()?;

            match event {
                Event::KeyPress(ev) => {
                    log::trace!("handling key press: {:#?}", event);
                    if let Some(chord) = Handler::handle_key_press(&ev, self.keyboard) {
                        self.process_chords(chord, ev.time, ev.response_type);
                    }
                },
                Event::KeyRelease(ev) => {
                    log::trace!("handling key release: {:#?}", event);
                    if let Some(chord) = Handler::handle_key_release(&ev, self.keyboard) {
                        self.process_chords(chord, ev.time, ev.response_type);
                    }
                },
                Event::ButtonPress(_ev) => {
                    log::trace!("handling button press: {:#?}", event);
                },
                Event::ButtonRelease(_ev) => {
                    log::trace!("handling button release: {:#?}", event);
                },
                Event::Error(e) => {
                    // TODO: Does this need to exit?
                    self.keyboard.cleanup();
                    lxhkd_fatal!("there was an error with the X-Server: {:?}", e);
                },
                _ => {
                    log::trace!("ignoring event: {:#?}", event);
                },
            }
        }

        Ok(())
    }
}

/// The state of the `Daemon` regarding matching keypresses
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum DaemonState<'a> {
    /// No keys have been pressed that match any existing keybindings
    None,
    /// Some keys have been pressed that are a prefix to a `Chain`
    Prefix,
    /// All keys in a `Chain` have been pressed
    Full(&'a Chain, &'a Action),
}
