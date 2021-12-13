//! Daemon that runs the application in the background

// TODO: Allow the user to add keybindings while the daemon is running

use super::{
    chord::{Chain, ChainLink, Chord},
    event_handler::Handler,
    keyboard::Keyboard,
    keys::{CharacterMap, ModifierMask},
    remap::{RemapKeyState, RemapState},
    xcape_state::{XcapeKeyState, XcapeState},
};
use crate::{
    config::{Action, Config, SHELL},
    keys::keysym::XKeysym,
    lxhkd_fatal,
    parse::parser::{Line, TokenizedLine},
};
use anyhow::{Context, Result};
use colored::Colorize;
use crossbeam_channel as channel;
use indexmap::IndexMap;
use itertools::Itertools;
use std::{
    collections::BTreeMap,
    convert::TryFrom,
    fmt,
    sync::{Arc, Mutex},
};
use x11rb::{
    connection::Connection,
    cookie::RecordEnableContextCookie,
    protocol::{
        record::{self, ConnectionExt as _, EnableContextReply},
        xproto::{self, Timestamp},
        Event,
    },
    x11_utils::TryParse,
};

// TODO: Add layers/modes

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
    /// The parsed remaps in the configuration file
    remaps:        RemapState,
    /// The parsed `Xcape` keys
    xcape:         XcapeState,
    /// Current chain being pressed
    active_chain:  Chain,
    /// Tracker of last keypress
    last_keypress: Timestamp,
    /// List of keys generated by this program
    generated:     Vec<CharacterMap>,
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
            remaps: RemapState::new(),
            xcape: XcapeState::default(),
            active_chain: Chain::default(),
            last_keypress: 0,
            generated: Vec::new(),
        }
    }

    // TODO: These functions are so similar, condense them
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
        self.bindings = parsed_bindings;

        Ok(())
    }

    /// Parse the configuration `remaps`
    pub(crate) fn process_remaps(&mut self) -> Result<()> {
        // TODO: Parse release
        // TODO: Parse mouse button
        let mut parsed_remaps = RemapState::new();

        if let Some(remaps) = &self.config.remaps {
            for (mut idx, l) in remaps.keys().enumerate() {
                idx += 1;

                let line = Line::new_plus(l, idx);
                let mut tokenized = line.tokenize();
                tokenized.parse_tokens()?;

                if let Some(mut chain_from) =
                    tokenized.convert_to_chain(self.keyboard.charmap(), false)
                {
                    let action_to = remaps
                        .get_index(idx - 1)
                        .context(
                            "failed to get valid index of item in configuration's `Remaps` section",
                        )?
                        .1;

                    log::trace!("{}:action: {}", "remaps".red().bold(), action_to);

                    let line = Line::new_plus(action_to, idx);
                    let mut tokenized_to = line.tokenize();
                    tokenized_to.parse_tokens()?;
                    if let Some(chain_to) =
                        tokenized_to.convert_to_chain(self.keyboard.charmap(), true)
                    {
                        parsed_remaps.insert(
                            RemapKeyState::from_chains(&chain_from, &chain_to)
                                .context("failed to insert chains into `XcapeKeyState`")?,
                        );
                    }
                }
            }
        }

        // println!("REMAPS: {:#?}", parsed_remaps);
        // std::process::exit(1);

        self.remaps = parsed_remaps;

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

    /// Combination of the above three functions to parse and process the
    /// configuration file's bindings into the `Daemon` struct
    pub(crate) fn process_configuration(&mut self) -> Result<()> {
        /// The `bindings` section of the configuration file
        /// Remaps keys to shell commands
        self.process_bindings()?;

        /// The `remaps` section of the configuration file
        /// Remaps keys to other keys
        self.process_remaps()?;

        /// The `xcape` section of the configuration file
        /// Remaps keys to other keys when tapped
        self.process_xcape()?;

        Ok(())
    }

    /// Parse the `Chords` generated from actions happening while the `Daemon`
    /// is running
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn process_chords(
        &mut self,
        chord: Chord,
        time: Timestamp,
        response_type: u8,
        window: xproto::Window,
    ) -> Result<()> {
        if self.last_keypress + self.config.global.timeout.unwrap_or(300) < time {
            self.active_chain.clear();
        }

        // println!("ACTIVE CHORD: {:#?}", chord);
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
                // amount of time, and not when the key is actually released
                ChainLink::Full => {
                    // match (chain.is_release(), response_type) {
                    // (true, xproto::KEY_RELEASE_EVENT) | (false, xproto::KEY_PRESS_EVENT) => {
                    log::info!("matched binding: {:?}", action);
                    log::info!(
                        "matched utf-code {:#?}",
                        self.active_chain
                            .chords()
                            .iter()
                            .map(|ch| format!("({}-{})", ch.charmap().utf(), ch.charmap().code(),))
                            .collect::<Vec<_>>()
                            .join(", ")
                    );

                    action.run(&self.config.global.shell);

                    should_clear = true;
                    break;
                },
                /*     _ => {},
                 * }
                 * }, */
            }
        }

        if should_clear {
            self.active_chain.clear();
        }

        self.last_keypress = time;

        Ok(())
    }

    pub(crate) fn daemonize(&mut self) -> Result<()> {
        const RECORD_FROM_SERVER: u8 = 0;
        const START_OF_DATA: u8 = 4;

        self.keyboard
            .gen_record_ctx()
            .context("failed to generate record context")?;

        for reply in self
            .keyboard
            .clone()
            .data_connection()
            .record_enable_context(self.keyboard.id())
            .context("failed to get `record_enable_context`")?
        {
            let reply = reply.context("failed to get `record_enable_context` reply")?;

            if reply.client_swapped {
                log::warn!("byte swapped clients are unsupported");
            } else if reply.category == RECORD_FROM_SERVER {
                let mut remaining = &reply.data[..];
                while !remaining.is_empty() {
                    remaining = self.intercept(&reply.data)?;
                }
            } else if reply.category == START_OF_DATA {
                log::info!("{} is {}", "daemon".red().bold(), "STARTING".green().bold());
            } else {
                log::warn!("`daemon` reply category is unknown: {:#?}", reply);
            }
        }

        Ok(())
    }

    // This intercept function's name was taken directly from `xcape` itself.
    // The outline of this function was also taken from the `x11rb` examples folder,
    // and combined with `xcape-rs`
    //
    /// Intercept a single packet of data, returning the remaining
    pub(crate) fn intercept<'a>(&mut self, data: &'a [u8]) -> Result<&'a [u8]> {
        match data[0] {
            xproto::KEY_PRESS_EVENT => {
                let (event, remaining) = xproto::KeyPressEvent::try_parse(data)
                    .context("failed to parse `KeyPressEvent`")?;
                log::trace!("handling key press: {:#?}", event);
                let key = event.detail;

                // If the key was an `xtest_fake_input`, skip
                if self.remaps.check_if_generated(key) {
                    log::info!("ignore generated: {}", key);
                    return Ok(remaining);
                }

                if self.remaps.mark_pressed(key).is_none() {
                    self.remaps.set_modifier();
                }

                if let Some(chord) = Handler::handle_key_press(&event, &self.keyboard) {
                    self.process_chords(chord, event.time, event.response_type, event.root)?;
                }

                Ok(remaining)
            },
            xproto::KEY_RELEASE_EVENT => {
                let (event, remaining) = xproto::KeyReleaseEvent::try_parse(data)
                    .context("failed to parse `KeyReleaseEvent`")?;
                log::trace!("handling key release: {:#?}", event);
                let key = event.detail;

                // If the key was an `xtest_fake_input`, skip
                if self.remaps.check_if_generated(key) {
                    log::info!("ignore generated: {}", key);
                    Ok(remaining)
                } else {
                    if let Some(new) = self
                        .remaps
                        .remapped_keys()
                        .iter()
                        .find(|c| c.from_key().charmap().code() == key)
                    {
                        if !new.is_used() {
                            log::debug!(
                                "{}:{} => {}:{} -- {}",
                                new.from_key().charmap().utf().purple().bold(),
                                new.from_key().charmap().code(),
                                new.to_keys()
                                    .iter()
                                    .map(|c| c.charmap().utf())
                                    .join(",")
                                    .purple()
                                    .bold(),
                                new.to_keys().iter().map(|c| c.charmap().code()).join(","),
                                "generated fake event".green().bold()
                            );

                            for (code, modmask) in new
                                .to_keys()
                                .iter()
                                .map(|k| (k.charmap().code(), k.modmask()))
                                .collect::<Vec<_>>()
                            {
                                println!("CODE: {}, MASK: {}", code, modmask);

                                // self.keyboard.make_modifier(modmask.mask(), true, ev.root)?;

                                // self.keyboard
                                //     .make_keypress_event(code, modmask, &ev)
                                //     .context("xcape: failed to make key press event")?;

                                self.keyboard
                                    .make_key_press_event(code, &event)
                                    .context("xcape: failed to make key press event")?;

                                self.remaps.mark_generated(code);

                                ///
                                // self.keyboard.make_modifier(modmask.mask(), false,
                                // ev.root)?; self.keyboard
                                //     .make_keyrelease_event(code, modmask, &ev)
                                //     .context("xcape: failed to make key press event")?;
                                self.keyboard
                                    .make_key_release_event(code, &event)
                                    .context("xcape: failed to make key release event")?;

                                self.remaps.mark_generated(code);

                                self.keyboard.flush();
                            }
                        }
                    }
                    let _ = self.remaps.mark_released(key);

                    if let Some(chord) = Handler::handle_key_release(&event, &self.keyboard) {
                        self.process_chords(chord, event.time, event.response_type, event.root)?;
                    }

                    Ok(remaining)
                }
            },
            xproto::BUTTON_PRESS_EVENT => {
                let (event, remaining) = xproto::ButtonPressEvent::try_parse(data)
                    .context("failed to parse `ButtonPressEvent`")?;
                log::trace!("handling button press: {:#?}", event);
                log::debug!(
                    "{}::{}(code:{},mask:{})",
                    "daemon".red().bold(),
                    "ButtonPressEvent".purple().bold(),
                    event.detail,
                    event.state
                );
                self.remaps.set_modifier();
                self.remaps.set_mouse_held(true);

                Ok(remaining)
            },
            xproto::BUTTON_RELEASE_EVENT => {
                let (event, remaining) = xproto::ButtonReleaseEvent::try_parse(data)
                    .context("failed to parse `ButtonReleaseEvent`")?;
                log::trace!("handling button release: {:#?}", event);
                log::debug!(
                    "{}::{}(code:{},mask:{})",
                    "daemon".red().bold(),
                    "ButtonReleaseEvent".purple().bold(),
                    event.detail,
                    event.state
                );

                self.remaps.set_mouse_held(false);

                Ok(remaining)
            },
            0 => {
                // This is a reply, we compute its length as follows
                let (length, _) = u32::try_parse(&data[4..])?;
                let length = usize::try_from(length).unwrap() * 4 + 32;
                log::error!(
                    "{}::UnparsedReply({:?})",
                    "daemon".red().bold(),
                    &data[..length]
                );
                Ok(&data[length..])
            },
            _ => {
                // Error or event always has length 32
                log::error!(
                    "{}::unparsed error/event: {:?}",
                    "daemon".red().bold(),
                    &data[..32]
                );
                Ok(&data[32..])
            },
        }
    }

    /// Start the loop that gets daemonized. Monitor X11 key presses that are
    /// prefixed by keys found within the configuration file
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn daemonize1(&mut self) -> Result<()> {
        // println!("BINDINGS: {:#?}", self.bindings);
        // std::process::exit(1);

        for chain in self.bindings.keys() {
            self.keyboard.grab_key(chain.chords());
        }

        for remap in self.remaps.remapped_keys() {
            self.keyboard.grab_key(&[remap.from_key().clone()]);
        }

        // if !self.xcape.is_empty() {
        //     self.keyboard.xcape().run(&mut self.xcape)?;
        // }

        // self.keyboard.make_keysequence(vec![chord.clone()], true,ev.root)?;
        // self.keyboard.make_keysequence(vec![chord.clone()], false,ev.root)?;

        // self.keyboard.make_generic_event_no_window(xproto::KEY_PRESS_EVENT, 12);
        // self.keyboard.make_generic_event_no_window(xproto::KEY_RELEASE_EVENT, 12);

        loop {
            self.keyboard.flush();

            // let event = self.keyboard.wait_for_event()?;
            while let Some(event) = self.keyboard.poll_for_event() {
                match event {
                    Event::KeyPress(ev) => {
                        log::trace!("handling key press: {:#?}", event);
                        let key = ev.detail;

                        // If the key was an `xtest_fake_input`, skip
                        if self.remaps.check_if_generated(key) {
                            log::info!("ignore generated: {}", key);
                            continue;
                        }

                        if self.remaps.mark_pressed(key).is_none() {
                            self.remaps.set_modifier();
                        }

                        if let Some(chord) = Handler::handle_key_press(&ev, &self.keyboard) {
                            self.process_chords(chord.clone(), ev.time, ev.response_type, ev.root)?;
                        }

                        // Necessary?
                        self.keyboard.allow_events(xproto::KEY_PRESS_EVENT, false)?;
                    },
                    Event::KeyRelease(ev) => {
                        log::trace!("handling key release: {:#?}", event);
                        let key = ev.detail;

                        // If the key was an `xtest_fake_input`, skip
                        if self.remaps.check_if_generated(key) {
                            log::info!("ignore generated: {}", key);
                            continue;
                        } else if let Some(new) = self
                            .remaps
                            .remapped_keys()
                            .iter()
                            .find(|c| c.from_key().charmap().code() == key)
                        {
                            if !new.is_used() {
                                log::debug!(
                                    "{}:{} => {}:{} -- {}",
                                    new.from_key().charmap().utf().purple().bold(),
                                    new.from_key().charmap().code(),
                                    new.to_keys()
                                        .iter()
                                        .map(|c| c.charmap().utf())
                                        .join(",")
                                        .purple()
                                        .bold(),
                                    new.to_keys().iter().map(|c| c.charmap().code()).join(","),
                                    "generated fake event".green().bold()
                                );

                                for (code, modmask) in new
                                    .to_keys()
                                    .iter()
                                    .map(|k| (k.charmap().code(), k.modmask()))
                                    .collect::<Vec<_>>()
                                {
                                    println!("CODE: {}, MASK: {}", code, modmask);

                                    // self.keyboard.make_modifier(modmask.mask(), true, ev.root)?;

                                    // self.keyboard
                                    //     .make_keypress_event(code, modmask, &ev)
                                    //     .context("xcape: failed to make key press event")?;

                                    self.keyboard
                                        .make_key_press_event(code, &ev)
                                        .context("xcape: failed to make key press event")?;

                                    self.remaps.mark_generated(code);

                                    ///
                                    // self.keyboard.make_modifier(modmask.mask(), false,
                                    // ev.root)?; self.keyboard
                                    //     .make_keyrelease_event(code, modmask, &ev)
                                    //     .context("xcape: failed to make key press event")?;
                                    self.keyboard
                                        .make_key_release_event(code, &ev)
                                        .context("xcape: failed to make key release event")?;

                                    self.remaps.mark_generated(code);

                                    self.keyboard.flush();
                                }
                            }
                        }
                        let _ = self.remaps.mark_released(key);

                        if let Some(chord) = Handler::handle_key_release(&ev, &self.keyboard) {
                            self.process_chords(chord, ev.time, ev.response_type, ev.root)?;
                        }

                        // Necessary?
                        self.keyboard
                            .allow_events(xproto::KEY_RELEASE_EVENT, false)?;
                    },
                    Event::ButtonPress(ev) => {
                        log::trace!("handling button press: {:#?}", event);
                        log::debug!(
                            "{}::{}(code:{},mask:{})",
                            "daemon".red().bold(),
                            "ButtonPressEvent".purple().bold(),
                            ev.detail,
                            ev.state
                        );
                        self.remaps.set_modifier();
                        self.remaps.set_mouse_held(true);

                        // Necessary?
                        self.keyboard
                            .allow_events(xproto::BUTTON_PRESS_EVENT, false)?;
                    },
                    Event::ButtonRelease(ev) => {
                        log::trace!("handling button release: {:#?}", event);
                        log::debug!(
                            "{}::{}(code:{},mask:{})",
                            "daemon".red().bold(),
                            "ButtonReleaseEvent".purple().bold(),
                            ev.detail,
                            ev.state
                        );

                        self.remaps.set_mouse_held(false);

                        // Necessary?
                        self.keyboard
                            .allow_events(xproto::BUTTON_RELEASE_EVENT, false)?;
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
                        // self.keyboard.cleanup();
                        log::info!("there was an error with the X-Server: {:?}", e);
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
