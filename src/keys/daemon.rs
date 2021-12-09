//! Daemon that runs the application in the background

// TODO: Allow the user to add keybindings while the daemon is running

use super::{
    chord::{Chain, ChainLink, Chord},
    event_handler::Handler,
    keyboard::Keyboard,
    keys::{CharacterMap, ModifierMask},
    xcape_state::{XcapeKeyState, XcapeState},
};
use crate::{
    config::{Action, Config},
    keys::keysym::XKeysym,
    lxhkd_fatal,
    parse::parser::{Line, TokenizedLine},
};
use anyhow::{Context, Result};
use colored::Colorize;
use crossbeam_channel as channel;
use indexmap::IndexMap;
use std::{
    collections::BTreeMap,
    fmt,
    sync::{Arc, Mutex},
};
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
pub(crate) struct Daemon {
    /// The current keyboard setup
    keyboard:      Arc<Keyboard>,
    /// Configuration file of the user
    config:        Config,
    /// The parsed bindings registered in all modes
    bindings:      BTreeMap<Chain, Action>,
    /// The parsed `Xcape` keys
    xcape:         XcapeState,
    /// Current chain being pressed
    active_chain:  Chain,
    /// Tracker of last keypress
    last_keypress: Timestamp,
}
// /// Max allowed time between keypresses
// keypress_timeout: u32,

impl Daemon {
    /// Create a new `Daemon`
    pub(crate) fn new(keyboard: Keyboard, config: Config) -> Self {
        // keypress_timeout: config.global.timeout.unwrap_or(300),
        Self {
            keyboard: Arc::new(keyboard),
            config,
            bindings: BTreeMap::new(),
            xcape: XcapeState::default(),
            active_chain: Chain::default(),
            last_keypress: 0,
        }
    }

    // TODO: If binding contains an unknown, then confirm it is skipped

    /// Parse the configuration bindings
    pub(crate) fn process_bindings(&mut self) -> Result<()> {
        let mut parsed_bindings = BTreeMap::new();

        if let Some(bindings) = &self.config.bindings {
            for (mut idx, l) in bindings.keys().enumerate() {
                idx += 1;

                let line = Line::new_plus(l, idx);
                let mut tokenized = line.tokenize();
                tokenized.parse_tokens()?;

                if let Some(mut chain) = tokenized.convert_to_chain(self.keyboard.charmap(), false)
                {
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

        // println!("BINDINGS = {:#?}", parsed_bindings);
        // std::process::exit(1);

        self.bindings = parsed_bindings;

        Ok(())
    }

    /// Parse the `Xcape` bindings, turning them into an `XcapeState` object
    pub(crate) fn process_xcape(&mut self) -> Result<()> {
        let mut xcape_state = XcapeState::new(self.keyboard.charmap());

        if let Some(xcape) = &self.config.xcape {
            for (mut idx, l) in xcape.keys().enumerate() {
                idx += 1;

                // Keys that are being converted
                let line = Line::new_plus(l, idx);
                let mut tokenized_from = line.tokenize();
                tokenized_from.parse_tokens()?;
                if let Some(chain_from) =
                    tokenized_from.convert_to_chain(self.keyboard.charmap(), true)
                {
                    let action_to = xcape
                        .get_index(idx - 1)
                        .context(
                            "failed to get valid index of item in configuration's `Xcape` section",
                        )?
                        .1;

                    // Keys that are the converted
                    let line = Line::new_plus(action_to, idx);
                    let mut tokenized_to = line.tokenize();
                    tokenized_to.parse_tokens()?;
                    if let Some(chain_to) =
                        tokenized_to.convert_to_chain(self.keyboard.charmap(), true)
                    {
                        xcape_state.insert(
                            XcapeKeyState::from_chains(&chain_from, &chain_to)
                                .context("failed to insert chains into `XcapeKeyState`")?,
                        );
                    }
                }
            }
        }

        self.xcape = xcape_state;

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
                                    ch.charmap().utf(),
                                    ch.charmap().code(),
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
        // println!("BINDINGS: {:#?}", self.bindings);
        // std::process::exit(1);

        for chain in self.bindings.keys() {
            self.keyboard.grab_key(chain.chords());
        }

        if !self.xcape.is_empty() {
            self.keyboard.xcape().run(&mut self.xcape)?;
        }

        loop {
            self.keyboard.flush();

            // let event = self.keyboard.wait_for_event()?;
            while let Some(event) = self.keyboard.poll_for_event() {
                match event {
                    Event::KeyPress(ev) => {
                        log::trace!("handling key press: {:#?}", event);
                        if let Some(chord) = Handler::handle_key_press(&ev, &self.keyboard) {
                            self.process_chords(chord, ev.time, ev.response_type);
                        }
                        self.keyboard.allow_events(xproto::KEY_PRESS_EVENT, false)?;
                    },
                    Event::KeyRelease(ev) => {
                        log::trace!("handling key release: {:#?}", event);
                        if let Some(chord) = Handler::handle_key_release(&ev, &self.keyboard) {
                            self.process_chords(chord, ev.time, ev.response_type);
                        }
                        self.keyboard
                            .allow_events(xproto::KEY_RELEASE_EVENT, false)?;
                    },
                    Event::ButtonPress(_ev) => {
                        log::trace!("handling button press: {:#?}", event);
                    },
                    Event::ButtonRelease(_ev) => {
                        log::trace!("handling button release: {:#?}", event);
                    },
                    // ====
                    // TODO: Match first event for these
                    // ====
                    Event::XkbNewKeyboardNotify(ev) => {
                        log::info!(
                            "`XkbNewKeyboardNotify` event; old_device:{} => new_device:{}",
                            ev.old_device_id,
                            ev.device_id
                        );
                        todo!(); // update_keymap
                    },
                    Event::XkbMapNotify(ev) => {
                        log::info!("`XkbMapNotify` event; changed:{}", ev.changed);
                        todo!(); // update_keymap
                    },
                    Event::XkbStateNotify(ev) => {
                        log::info!(
                            "`XkbStateNotify` event; mods:{}, changed:{}",
                            ev.mods,
                            ev.changed
                        );
                        todo!(); // update_state
                    },
                    //
                    Event::Error(e) => {
                        // TODO: Does this need to exit?
                        self.keyboard.cleanup();
                        lxhkd_fatal!("there was an error with the X-Server: {:?}", e);
                    },
                    _ => {
                        log::trace!("{}:ignoring event: {:#?}", "bind".red().bold(), event);
                    },
                }
            }
        }

        self.keyboard.cleanup();

        Ok(())
    }
}
