//! The main struct that holds the X-connection(s) that `Xcape` communicates
//! with. This is placed inside of the [`Keyboard`](super::keyboard::Keyboard)

use super::{
    chord::Chord,
    event_handler::Handler,
    keys::{self, CharacterMap, ModifierMask, XButton, XKeyCode},
    keysym::{KeysymHash, XKeysym},
    xcape_state::{XcapeKeyState, XcapeState},
};
use crate::{
    config::Config,
    lxhkd_fatal,
    types::{Xid, KEYSYMS_PER_KEYCODE},
};
use anyhow::{anyhow, Context, Result};
use colored::{ColoredString, Colorize};
use crossbeam_channel::Sender;
use crossbeam_utils::thread as cthread;
use itertools::Itertools;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    collections::HashMap,
    convert::TryFrom,
    env,
    fmt,
    str::FromStr,
    sync::Arc,
    thread,
    time::Duration,
};
use thiserror::Error;

use x11rb::{
    connection::{Connection, RequestConnection},
    cookie::RecordEnableContextCookie,
    errors::ReplyError,
    properties,
    protocol::{
        self,
        record::{self, ConnectionExt as _, EnableContextReply},
        xkb::{
            self,
            BoolCtrl,
            ConnectionExt as _,
            GetCompatMapReply,
            GetControlsReply,
            GetMapReply,
            Group,
            KeyModMap,
            KeySymMap,
            MapPart,
            NameDetail,
            ID,
        },
        xproto::{
            self,
            Allow,
            ChangeKeyboardControlAux,
            ConnectionExt,
            EventMask,
            GetKeyboardMappingReply,
            GetModifierMappingReply,
            Keycode,
            Keysym,
            ModMask,
        },
        xtest::{self, ConnectionExt as _},
        Event,
    },
    rust_connection::RustConnection,
    wrapper::ConnectionExt as _,
    x11_utils::TryParse,
};

// =================== Xcape ======================

// TODO: These conns may need to be Arc<>

#[derive(Clone)]
pub(crate) struct Xcape {
    /// Control connection to the X-Server
    ctrl_conn: Arc<RustConnection>,
    /// Read-data connection to the X-Server
    data_conn: Arc<RustConnection>,
    /// The generated ID to be used for `record`
    id:        u32,
    /// The amount of time before `xcape` registers a new key press
    timeout:   Option<u64>,
}

impl Xcape {
    const RECORD_FROM_SERVER: u8 = 0;
    const START_OF_DATA: u8 = 4;

    /// Construct a new instance of `Xcape`
    pub(crate) fn new(
        ctrl_conn: RustConnection,
        data_conn: RustConnection,
        config: &Config,
    ) -> Result<Self> {
        let id = ctrl_conn
            .generate_id()
            .context("failed to generate an ID for `record`")?;

        Ok(Self {
            ctrl_conn: Arc::new(ctrl_conn),
            data_conn: Arc::new(data_conn),
            id,
            timeout: config.global.xcape_timeout,
        })
    }

    /// Generate the [`record`] configuration
    /// ([`Range`](x11rb::protocol::record::Range))
    pub(crate) fn gen_record_range() -> record::Range {
        let empty = record::Range8 { first: 0, last: 0 };
        let empty_ext =
            record::ExtRange { major: empty, minor: record::Range16 { first: 0, last: 0 } };

        record::Range {
            core_requests:    empty,
            core_replies:     empty,
            ext_requests:     empty_ext,
            ext_replies:      empty_ext,
            delivered_events: empty,
            device_events:    record::Range8 {
                // Want notification of core X11 events from key press (2) to motion notify (6)
                // KeyPress = 2, KeyRelease = 3, ButtonPress = 4, ButtonRelease = 5
                first: xproto::KEY_PRESS_EVENT,
                last:  xproto::MOTION_NOTIFY_EVENT,
            },
            errors:           empty,
            client_started:   false,
            client_died:      false,
        }
    }

    /// Run the `Xcape` bindings
    /// Example is taken from `psychon/x11rb` examples for `record` extension
    pub(crate) fn run(&self, state: &mut XcapeState) -> Result<()> {
        const RECORD_FROM_SERVER: u8 = 0;
        const START_OF_DATA: u8 = 4;

        self.gen_record_ctx()
            .context("failed to generate record context")?;

        for reply in self
            .data_conn
            .record_enable_context(self.id)
            .context("failed to get `record_enable_context`")?
        {
            let reply = reply.context("failed to get `record_enable_context` reply")?;

            if reply.client_swapped {
                log::warn!("byte swapped clients are unsupported");
            } else if reply.category == RECORD_FROM_SERVER {
                let mut remaining = &reply.data[..];
                while !remaining.is_empty() {
                    remaining = self.intercept(&reply.data, state)?;
                }
            } else if reply.category == START_OF_DATA {
                log::info!("{} is {}", "xcape".red().bold(), "STARTING".green().bold());
            } else {
                log::warn!("`xcape` reply category is unknown: {:#?}", reply);
            }
        }

        Ok(())
    }

    // pub(crate) fn setup_record_ctx(&self) ->
    // Result<RecordEnableContextCookie<'_>> {     self.gen_record_ctx()
    //         .context("failed to generate record context")?;
    //
    //     self.data_conn
    //         .record_enable_context(self.id)
    //         .context("failed to get `record_enable_context`")?
    // }

    pub(crate) fn xcape_poll_for_event(&self, state: &mut XcapeState) -> Result<()> {
        self.gen_record_ctx()
            .context("failed to generate record context")?;

        for reply in self
            .data_conn
            .record_enable_context(self.id)
            .context("failed to get `record_enable_context`")?
        {
            let reply = reply.context("failed to get `record_enable_context` reply")?;

            if reply.category == Self::RECORD_FROM_SERVER {
                while let Ok(remaining) = self.intercept(&reply.data, state) {
                    if remaining.is_empty() {
                        log::warn!("no more events from `xcape`");
                        break;
                    }
                }
            }

            // if reply.client_swapped {
            //     log::warn!("byte swapped clients are unsupported");
            // } else if reply.category == Self::RECORD_FROM_SERVER {
            //     let mut remaining = &reply.data[..];
            //     // while !remaining.is_empty() {
            //     //     remaining = self.intercept(&reply.data, state)?;
            //     // }
            // } else if reply.category == Self::START_OF_DATA {
            //     log::info!("{} is {}", "xcape".red().bold(),
            // "STARTING".green().bold()); } else {
            //     log::warn!("`xcape` reply category is unknown: {:#?}",
            // reply); }
        }
        Ok(())
    }

    // This intercept function's name was taken directly from `xcape` itself.
    // The outline of this function was also taken from the `x11rb` examples folder,
    // and combined with `xcape-rs`
    //
    /// Intercept a single packet of data, returning the remaining
    #[allow(clippy::unused_self)]
    pub(crate) fn intercept<'a>(
        &self,
        data: &'a [u8],
        state: &'a mut XcapeState,
    ) -> Result<&'a [u8]> {
        match data[0] {
            xproto::KEY_PRESS_EVENT => {
                let (event, remaining) = xproto::KeyPressEvent::try_parse(data)
                    .context("failed to parse `KeyPressEvent`")?;
                let key = event.detail;

                log::debug!(
                    "{}::{}(code:{},mask:{})",
                    "xcape".red().bold(),
                    "KeyPressEvent".purple().bold(),
                    event.detail,
                    event.state
                );

                if state.check_if_auto_generated(key) {
                    log::info!("ignore autogenerated: {}", key);
                    return Ok(remaining);
                }

                if state.mark_pressed(key).is_none() {
                    state.use_all_mods();
                }

                Ok(remaining)
            },
            xproto::KEY_RELEASE_EVENT => {
                let (event, remaining) = xproto::KeyReleaseEvent::try_parse(data)
                    .context("failed to parse `KeyReleaseEvent`")?;
                let key = event.detail;

                log::debug!(
                    "{}::{}(code:{},mask:{})",
                    "xcape".red().bold(),
                    "KeyReleaseEvent".purple().bold(),
                    event.detail,
                    event.state
                );

                if state.check_if_auto_generated(key) {
                    log::info!("ignore autogenerated: {}", key);
                    Ok(remaining)
                } else {
                    if let Some(new) = state
                        .remapped_keys()
                        .iter()
                        .find(|c| c.from_keys().code() == key)
                    {
                        if !new.is_modifier() {
                            log::debug!(
                                "{}:{} => {}:{} -- {}",
                                new.from_keys().utf().purple().bold(),
                                new.from_keys().code(),
                                new.to_keys()
                                    .iter()
                                    .map(CharacterMap::utf)
                                    .join(",")
                                    .purple()
                                    .bold(),
                                new.to_keys().iter().map(CharacterMap::code).join(","),
                                "generated fake event".green().bold()
                            );

                            for k in new
                                .to_keys()
                                .iter()
                                .map(CharacterMap::code)
                                .collect::<Vec<_>>()
                            {
                                self.make_key_press_event(k, &event)
                                    .context("xcape: failed to make key press event")?;
                                state.mark_auto_generated(k);

                                self.make_key_release_event(k, &event)
                                    .context("xcape: failed to make key release event")?;
                                state.mark_auto_generated(k);

                                self.ctrl_conn
                                    .flush()
                                    .context("xcape: failed to flush events to X-Server")?;
                            }
                        }
                    }
                    let _ = state.mark_released(key);
                    Ok(remaining)
                }
            },
            xproto::BUTTON_PRESS_EVENT => {
                let (event, remaining) = xproto::ButtonPressEvent::try_parse(data)
                    .context("failed to parse `ButtonPressEvent`")?;

                log::debug!(
                    "{}::{}(code:{},mask:{})",
                    "xcape".red().bold(),
                    "ButtonPressEvent".purple().bold(),
                    event.detail,
                    event.state
                );
                state.use_all_mods();
                state.set_mouse_held(true);

                Ok(remaining)
            },
            xproto::BUTTON_RELEASE_EVENT => {
                let (event, remaining) = xproto::ButtonReleaseEvent::try_parse(data)
                    .context("failed to parse `ButtonReleaseEvent`")?;

                log::debug!(
                    "{}::{}(code:{},mask:{})",
                    "xcape".red().bold(),
                    "ButtonReleaseEvent".purple().bold(),
                    event.detail,
                    event.state
                );
                state.set_mouse_held(false);

                Ok(remaining)
            },
            0 => {
                // This is a reply, we compute its length as follows
                let (length, _) = u32::try_parse(&data[4..])?;
                let length = usize::try_from(length).unwrap() * 4 + 32;
                log::error!(
                    "{}::UnparsedReply({:?})",
                    "xcape".red().bold(),
                    &data[..length]
                );
                Ok(&data[length..])
            },
            _ => {
                // Error or event always has length 32
                log::error!(
                    "{}::unparsed error/event: {:?}",
                    "xcape".red().bold(),
                    &data[..32]
                );
                Ok(&data[32..])
            },
        }
    }

    /// Generate the [`record`](x11rb::protocol::record) context
    pub(crate) fn gen_record_ctx(&self) -> Result<()> {
        let range = Self::gen_record_range();

        self.ctrl_conn
            .record_create_context(self.id, 0, &[record::CS::ALL_CLIENTS.into()], &[range])
            .context("failed to create record context")?
            .check()
            .context("failed to check result of creating record context")?;

        let spawn_action = |timeout: u64| {
            cthread::scope(|scope| {
                scope.spawn(|_| {
                    thread::sleep(Duration::from_secs(timeout));
                    self.ctrl_conn
                        .record_disable_context(self.id)
                        .expect("failed to disable record context");
                    self.ctrl_conn.sync().expect("failed to sync X-Server");
                });
            });
        };

        // Apply a timeout, if the user requested
        // Environment variable overrides configuration
        match env::var("LXHKD_XCAPE_TIMEOUT")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
        {
            None => match self.timeout {
                None => {},
                Some(timeout) => spawn_action(timeout),
            },
            Some(timeout) => spawn_action(timeout),
        }

        Ok(())
    }

    // pub(crate) fn disable_record_context(&self) {}

    // ================ GenericEvent ==================

    /// Make a generic event with no window
    pub(crate) fn make_generic_event2(&self, event_type: u8, keycode: u8) -> Result<()> {
        self.ctrl_conn
            .xtest_fake_input(
                event_type,          // event type
                keycode,             // detail
                x11rb::CURRENT_TIME, // time
                x11rb::NONE,
                0,
                0,
                0,
            )
            .context("failed to send `xtest_fake_input`")?
            .check()
            .context("failed to check `xtest_fake_input`")?;
        Ok(())
    }

    /// Wrapper function to create `fake` events. An event does not need to be
    /// sent to this function, instead a `Window` is
    pub(crate) fn make_generic_event(
        &self,
        event_type: u8,
        keycode: u8,
        window: xproto::Window,
    ) -> Result<()> {
        self.ctrl_conn
            .xtest_fake_input(
                event_type,          // event type
                keycode,             // detail
                x11rb::CURRENT_TIME, // time
                window,
                1,
                1,
                0,
            )
            .context("failed to send `xtest_fake_input`")?
            .check()
            .context("failed to check `xtest_fake_input`")?;
        Ok(())
    }

    // ================= With Event ===================

    // TODO: Delete one set of these

    /// Create a [`KeyPressEvent`](x11rb::protocol::xproto::KeyPressEvent)
    pub(crate) fn make_key_press_event(
        &self,
        sim_keycode: u8,
        event: &xproto::KeyPressEvent,
    ) -> Result<()> {
        log::debug!(
            "{} for {}",
            "generated fake key press".green().bold(),
            sim_keycode
        );
        self.ctrl_conn
            .xtest_fake_input(
                xproto::KEY_PRESS_EVENT, // event type
                sim_keycode,             // detail -- simulated keycode
                x11rb::CURRENT_TIME,     // time
                event.root,
                event.root_x,
                event.root_y,
                0,
            )
            .context("failed to send `xtest_fake_input` (KeyPress)")?
            .check()
            .context("failed to check `xtest_fake_input`  (KeyPress)")?;
        Ok(())
    }

    /// Create a [`KeyReleaseEvent`](x11rb::protocol::xproto::KeyReleaseEvent)
    pub(crate) fn make_key_release_event(
        &self,
        sim_keycode: u8,
        event: &xproto::KeyReleaseEvent,
    ) -> Result<()> {
        log::debug!(
            "{} for {}",
            "generated fake key release".green().bold(),
            sim_keycode
        );
        self.ctrl_conn
            .xtest_fake_input(
                xproto::KEY_RELEASE_EVENT, // event type
                sim_keycode,               // detail -- simulated keycode
                x11rb::CURRENT_TIME,       // time
                event.root,
                event.root_x,
                event.root_y,
                0,
            )
            .context("failed to send `xtest_fake_input` (KeyRelease)")?
            .check()
            .context("failed to check `xtest_fake_input` (KeyRelease)")?;
        Ok(())
    }

    /// Create a [`ButtonPressEvent`](x11rb::protocol::xproto::ButtonPressEvent)
    pub(crate) fn make_button_press_event(
        &self,
        button: u8,
        event: &xproto::ButtonPressEvent,
    ) -> Result<()> {
        self.ctrl_conn
            .xtest_fake_input(
                xproto::BUTTON_PRESS_EVENT, // event type
                button,                     // detail -- simulated button code
                x11rb::CURRENT_TIME,        // time
                event.root,
                event.root_x,
                event.root_y,
                0,
            )
            .context("failed to send `xtest_fake_input`  (ButtonPress)")?
            .check()
            .context("failed to check `xtest_fake_input` (ButtonPress)")?;
        Ok(())
    }

    /// Create a [`ButtonReleaseEvent`](x11rb::protocol::xproto::
    /// ButtonReleaseEvent)
    pub(crate) fn make_button_release_event(
        &self,
        button: u8,
        event: &xproto::ButtonReleaseEvent,
    ) -> Result<()> {
        self.ctrl_conn
            .xtest_fake_input(
                xproto::BUTTON_RELEASE_EVENT, // event type
                button,                       // detail -- simulated button code
                x11rb::CURRENT_TIME,          // time
                event.root,
                event.root_x,
                event.root_y,
                0,
            )
            .context("failed to send `xtest_fake_input`  (ButtonPress)")?
            .check()
            .context("failed to check `xtest_fake_input` (ButtonPress)")?;
        Ok(())
    }

    // =============== Without Event ==================

    /// Create a [`KeyPressEvent`](x11rb::protocol::xproto::KeyPressEvent)
    pub(crate) fn make_key_press_event1(&self, sim_keycode: u8) -> Result<()> {
        self.ctrl_conn
            .xtest_fake_input(
                xproto::KEY_PRESS_EVENT, // event type
                sim_keycode,             // detail -- simulated keycode
                x11rb::CURRENT_TIME,     // time
                x11rb::NONE,             // root
                0,                       // root_x
                0,                       // root_y
                0,                       // device_id
            )
            .context("failed to send `xtest_fake_input` (KeyPress)")?
            .check()
            .context("failed to check `xtest_fake_input`  (KeyPress)")?;
        Ok(())
    }

    /// Create a [`KeyReleaseEvent`](x11rb::protocol::xproto::KeyReleaseEvent)
    pub(crate) fn make_key_release_event1(&self, sim_keycode: u8) -> Result<()> {
        self.ctrl_conn
            .xtest_fake_input(
                xproto::KEY_RELEASE_EVENT, // event type
                sim_keycode,               // detail -- simulated keycode
                x11rb::CURRENT_TIME,       // time
                x11rb::NONE,               // root
                0,                         // root_x
                0,                         // root_y
                0,                         // device_id
            )
            .context("failed to send `xtest_fake_input` (KeyRelease)")?
            .check()
            .context("failed to check `xtest_fake_input` (KeyRelease)")?;
        Ok(())
    }

    /// Create a [`ButtonPressEvent`](x11rb::protocol::xproto::ButtonPressEvent)
    pub(crate) fn make_button_press_event1(&self, button: u8) -> Result<()> {
        self.ctrl_conn
            .xtest_fake_input(
                xproto::BUTTON_PRESS_EVENT, // event type
                button,                     // detail -- simulated button code
                x11rb::CURRENT_TIME,        // time
                x11rb::NONE,                // root
                0,                          // root_x
                0,                          // root_y
                0,                          // device_id
            )
            .context("failed to send `xtest_fake_input`  (ButtonPress)")?
            .check()
            .context("failed to check `xtest_fake_input` (ButtonPress)")?;
        Ok(())
    }

    /// Create a [`ButtonReleaseEvent`](x11rb::protocol::xproto::
    /// ButtonReleaseEvent)
    pub(crate) fn make_button_release_event1(
        &self,
        button: u8,
        duration_ms: Option<u32>,
    ) -> Result<()> {
        self.ctrl_conn
            .xtest_fake_input(
                xproto::BUTTON_RELEASE_EVENT,       // event type
                button,                             // detail -- simulated button code
                x11rb::CURRENT_TIME,                // time
                duration_ms.unwrap_or(x11rb::NONE), // root
                0,                                  // root_x
                0,                                  // root_y
                0,                                  // device_id
            )
            .context("failed to send `xtest_fake_input`  (ButtonPress)")?
            .check()
            .context("failed to check `xtest_fake_input` (ButtonPress)")?;
        Ok(())
    }

    // =================== Extra ======================

    /// Create a full click of the mouse (`ButtonPress` + `ButtonRelease`)
    pub(crate) fn make_click(&self, button: u8, duration_ms: u32) -> Result<()> {
        self.make_button_press_event1(button)?;
        self.make_button_release_event1(button, Some(duration_ms))?;

        Ok(())
    }
}

impl fmt::Debug for Xcape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "--Xcape--, timeout: {:?}", self.timeout)
    }
}
