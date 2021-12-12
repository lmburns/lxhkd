//! The `Keyboard` struct which is the raw interface with the X-Server

use super::{
    chord::Chord,
    event_handler::Handler,
    keys::{self, CharacterMap, ModifierMask, XButton, XKeyCode},
    keysym::{KeysymHash, XKeysym},
    xcape::Xcape,
};
use crate::{
    config::Config,
    lxhkd_fatal,
    types::{Xid, KEYSYMS_PER_KEYCODE},
};
use anyhow::{anyhow, Context, Result};
use colored::{ColoredString, Colorize};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    collections::HashMap,
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
    errors::ReplyError,
    properties,
    protocol::{
        self,
        record::{self, ConnectionExt as _},
        xkb::{
            self,
            BoolCtrl,
            ConnectionExt as _,
            GetCompatMapReply,
            GetControlsReply,
            GetMapReply,
            GetStateReply,
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
            GetInputFocusReply,
            GetKeyboardMappingReply,
            GetModifierMappingReply,
            Keycode,
            Keysym,
            MapIndex,
            ModMask,
        },
        xtest::{self, ConnectionExt as _},
        Event,
    },
    rust_connection::RustConnection,
    wrapper::ConnectionExt as _,
};

// TODO: GetControlsReply for key repeat
// ListComponentsReply = keymaps keycodes
// GetDeviceKeyMappingReply , xinput = keysyms
// GetDeviceModifierMappingReply , xinput = keymaps
// DeviceLedInfo
// GetDeviceInfoReply
// GetNamesReply

// =================== Error ======================

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("{0}")]
    Unsupported(String),
    #[error("failed to initialize X11: {0}")]
    X11Init(#[from] x11rb::errors::ConnectionError),
    #[error("failed to use xkb extension")]
    ExtensionFailure,
    #[error("failed to get device id")]
    DeviceID,
    #[error("failed to get device's keymap")]
    Keymap,
    #[error("failed to update device's keymap")]
    KeymapUpdate,
    #[error("failed to get device's current state")]
    State,

    #[error("failed to get keysym maps")]
    AcquireKeysyms,
    #[error("failed to get key-type maps")]
    AcquireKeytypes,
    #[error("failed to get modmap maps")]
    AcquireModmap,
    #[error("failed to get virutal modmap maps")]
    AcquireVirtualModmap,
    #[error("failed to lookup keysym {1:?} at index {0}")]
    LookupKeysyms(u8, KeySymMap),
    #[error("failed to lookup keysym {0:?}")]
    LookupKeysymHash(Keysym),
    #[error("failed to build a `CharacterMap` for (Keysym={0}, Keycode={1}): {2}")]
    BuildCharacterMap(Keysym, Keycode, anyhow::Error),
    #[error("failed to find a `CharacterMap` for keycode={0}")]
    LookupKeycode(Keycode),
    #[error("failed to poll to next `CharacterMap`: {0}")]
    PollNextCharacterMap(#[from] anyhow::Error),
}

// ================= Keyboard =====================

/// State of the keyboard
#[derive(Clone)]
pub(crate) struct Keyboard {
    /// Connection to the X-Server
    conn:                Arc<RustConnection>,
    /// Connection to the X-Server to read and control data for `xcape`
    xcape:               Xcape,
    /// Root window.
    root:                xproto::Window,
    /// The characters, keysyms, etc making up the `Keyboard`
    charmap:             Vec<CharacterMap>,
    /// The device's ID
    device_id:           Xid,
    /// The minimum keycode
    min_keycode:         u8,
    /// The maximum keycode
    max_keycode:         u8,
    /// The current modifiers on the keyboard that are mapped to keys
    modmap:              Vec<KeyModMap>,
    /// The number of keysyms per keycode
    keysyms_per_keycode: u8,
    /// The delay in which a key begins repeating
    autorepeat_delay:    u16,
    /// The interval at which a key repeats
    autorepeat_interval: u16,
}

impl Keyboard {
    /// Construct a new instance of `Keyboard`
    pub(crate) fn new(
        conn: RustConnection,
        ctrl_conn: RustConnection,
        data_conn: RustConnection,
        screen_num: usize,
        config: &Config,
    ) -> Result<Self> {
        let screen = conn.setup();
        let root = screen.roots[screen_num].clone().root;

        let use_extension = |conn: &RustConnection, extension_name: &'static str| -> Result<()> {
            if conn.extension_information(extension_name)?.is_none() {
                lxhkd_fatal!(
                    "{} X11 extension is unsupported",
                    extension_name.green().bold()
                );
            }

            Ok(())
        };

        // Check `xkb` extension
        use_extension(&conn, xkb::X11_EXTENSION_NAME)?;
        let (min, max) = xkb::X11_XML_VERSION;
        if let Err(e) = conn.xkb_use_extension(min as u16, max as u16) {
            lxhkd_fatal!(
                "`xkb` version is unsupported. Supported versions: {}-{}: {}",
                min,
                max,
                e
            );
        };

        // Check `xtest` extension
        use_extension(&conn, xtest::X11_EXTENSION_NAME)?;
        // conn.query_extension()

        // Check `record` extension
        use_extension(&conn, record::X11_EXTENSION_NAME)?;
        let (min, max) = record::X11_XML_VERSION;
        if let Err(e) = conn.record_query_version(min as u16, max as u16) {
            lxhkd_fatal!(
                "`record` version is unsupported. Supported versions: {}-{}: {}",
                min,
                max,
                e
            );
        };

        // let k = conn.get_keyboard_mapping();
        // let k = conn.get_modifier_mapping();
        // let k = conn.change_keyboard_mapping();

        let mut keyboard = Self {
            min_keycode: screen.min_keycode,
            max_keycode: screen.max_keycode,
            conn: Arc::new(conn),
            xcape: Xcape::new(ctrl_conn, data_conn, config)?,
            root,
            charmap: Vec::new(),
            device_id: 0,
            modmap: Vec::new(),
            keysyms_per_keycode: 0,
            autorepeat_interval: 0,
            autorepeat_delay: 0,
        };

        keyboard.generate_charmap()?;
        keyboard.set_controls(config)?;

        Ok(keyboard)
    }

    // ModDef
    // KeyType

    /// Return the `Xcape` object
    pub(crate) fn xcape(&self) -> &Xcape {
        &self.xcape
    }

    /// Return the root window
    pub(crate) fn root(&self) -> xproto::Window {
        self.root
    }

    /// Return the connection to the X-Server
    pub(crate) fn connection(&self) -> &RustConnection {
        &self.conn
    }

    /// Flush actions to the X-Server
    pub(crate) fn flush(&self) {
        // self.conn.flush().is_ok()
        if let Err(e) = self.conn.flush() {
            log::warn!("failed to flush actions to X-server");
        }
    }

    /// Synchronize events with the X-Server
    pub(crate) fn sync(&self) {
        if let Err(e) = self.conn.sync() {
            log::warn!("failed to sync events with X-server");
        }
    }

    /// Return the device's ID
    pub(crate) fn device_id(&self) -> Xid {
        self.device_id
    }

    /// Return the `CharacterMap`
    pub(crate) fn charmap(&self) -> &Vec<CharacterMap> {
        &self.charmap
    }

    /// Shorter `poll_for_event` (non-blocking)
    pub(crate) fn poll_for_event(&self) -> Option<Event> {
        self.conn
            .poll_for_event()
            .context("failed to poll for next event")
            .ok()?
    }

    /// Shorter `wait_for_event` (blocking)
    pub(crate) fn wait_for_event(&self) -> Result<Event> {
        self.conn
            .wait_for_event()
            .context("failed to wait for next event")
    }

    /// Get the `GetKeyboardMappingReply`. This only contains the `Keysyms` from
    /// the minimum keycode to the maximum keycode. Much simpler that
    /// [`get_map_reply`](Keyboard::get_map_reply), but doesn't provide as much
    /// information
    pub(crate) fn get_keyboard_mapping_reply(&self) -> Result<GetKeyboardMappingReply> {
        self.conn
            .get_keyboard_mapping(self.min_keycode, self.max_keycode - self.min_keycode)
            .context("failed to get XKB `GetKeyboardMappingReply`")?
            .reply()
            .context("failed to get XKB `GetKeyboardMappingReply` reply")
    }

    /// Get the `GetControlsReply` which has the key-repeat-delay and and the
    /// key-repeat-interval
    pub(crate) fn get_controls_reply(&self) -> Result<GetControlsReply> {
        // repeat_delay: 300,
        // repeat_interval: 20,
        self.conn
            .xkb_get_controls(ID::USE_CORE_KBD.into())
            .context("failed to get XKB `GetControlsReply`")?
            .reply()
            .context("failed to get XKB `GetControlsReply` reply")
    }

    // TODO: Fix this
    // XkbSetControls: https://code.woboq.org/qt5/include/X11/XKBlib.h.html

    /// Set the key repeat-delay and repeat-interval
    pub(crate) fn set_controls(&mut self, config: &Config) -> Result<()> {
        let reply = self.get_controls_reply()?;

        self.autorepeat_delay = reply.repeat_delay;
        self.autorepeat_interval = reply.repeat_interval;

        self.conn
            .xkb_set_controls(
                ID::USE_CORE_KBD.into(),
                0_u8,                       // affect_internal_real_mods
                0_u8,                       // internal_real_mods
                0_u8,                       // affect_ignore_lock_real_mods
                0_u8,                       // ignore_lock_real_mods
                0_u8,                       // affect_internal_virtual_mods
                0_u8,                       // internal_virtual_mods
                0_u8,                       // affect_ignore_lock_virtual_mods
                0_u8,                       // ignore_lock_virtual_mods
                0_u8,                       // mouse_keys_dflt_btn
                0_u8,                       // groups_wrap
                0_u8,                       // access_x_options
                0_u16,                      // affect_enabled_controls
                0_u8,                       // enabled_controls
                xkb::BoolCtrl::REPEAT_KEYS, // change_controls
                config.global.autorepeat_delay.unwrap_or(reply.repeat_delay), // repeat_delay
                // The reply from the server is already divided by a thousand
                // FIX: This is not perfect. If the number isn't evenly divisible by 1000
                (1000_f32
                    / f32::from(
                        config
                            .global
                            .autorepeat_interval
                            .unwrap_or(1000 / reply.repeat_interval),
                    )) as u16, // repeat_interval
                0_u16,    // slow_keys_delay
                0_u16,    // debounce_delay
                0_u16,    // mouse_keys_delay
                0_u16,    // mouse_keys_interval
                0_u16,    // mouse_keys_time_to_max
                0_u16,    // mouse_keys_max_speed
                0_i16,    // mouse_keys_curve
                0_u16,    // access_x_timeout
                0_u16,    // access_x_timeout_mask
                0_u16,    // access_x_timeout_values
                0_u16,    // access_x_timeout_options_mask
                0_u16,    // access_x_timeout_options_values
                &[0; 32], // per_key_repeat
            )
            .context("failed to set XKB controls")?
            .check()
            .context("failed to check XKB controls request")?;

        // Get reply again to confirm the change took effect
        let reply = self.get_controls_reply()?;

        let log_info = |delay: u16, reply: u16, slf: u16, autorepeat: &str| {
            // This message will display a supposed error message when it isn't supposed to
            // if the number does not go into 1000 evenly
            //
            // Config != New = something went wrong
            if delay != reply {
                log::trace!(
                    "X-Server did not set correct {}. {} != {}",
                    autorepeat.green().bold(),
                    reply.to_string().red().bold(),
                    delay.to_string().red().bold()
                );
            // Config != Old = it's been changed
            } else if delay == reply && delay != slf {
                log::info!(
                    "changed {}: {} => {}",
                    autorepeat.green().bold(),
                    slf.to_string().red().bold(),
                    delay.to_string().red().bold()
                );
            }
        };

        if let Some(delay) = config.global.autorepeat_delay {
            log_info(
                delay,
                reply.repeat_delay,
                self.autorepeat_delay,
                "autorepeat_delay",
            );
        }

        if let Some(interval) = config.global.autorepeat_interval {
            log_info(
                interval,
                1000 / reply.repeat_interval,
                1000 / self.autorepeat_interval,
                "autorepeat_interval",
            );
        }

        self.autorepeat_delay = reply.repeat_delay;
        self.autorepeat_interval = reply.repeat_interval;

        Ok(())
    }

    /// Query the server for the current keyboard state
    pub(crate) fn get_state(&self) -> Result<GetStateReply> {
        self.conn
            .xkb_get_state(ID::USE_CORE_KBD.into())
            .context("failed to get `GetStateReply`")?
            .reply()
            .context("failed to get XKB `GetStateReply`")
    }

    /// Get the modifier mappings on the keyboard. This returns the keycodes
    /// that are modifiers that are in use on the user's keyboard
    pub(crate) fn get_modifier_mapping(&self) -> Result<GetModifierMappingReply> {
        // self.modifiers = reply.keycodes
        self.conn
            .get_modifier_mapping()
            .context("failed to get `GetModifierMappingReply`")?
            .reply()
            .context("failed to get XKB `GetModifierMappingReply` reply")
    }

    /// TODO: Above as well. Returns information about modifiers. Could create a
    /// modifier array based on index of vmods (if they're always in the same
    /// spot)
    pub(crate) fn get_compat_map_reply(&self) -> Result<GetCompatMapReply> {
        self.conn
            .xkb_get_compat_map(
                ID::USE_CORE_KBD.into(),
                xkb::CMDetail::SYM_INTERP,
                true,
                1,
                20,
            )
            .context("failed to get `GetCompatMappingReply`")?
            .reply()
            .context("failed to get XKB `GetCompatMappingReply` reply")
    }

    /// Get the `GetMapReply`. Provides the minimum and maximum keycode, as well
    /// as `Keysyms` which are "real" modifiers and "virtual" modifiers.
    pub(crate) fn get_map_reply(&self) -> Result<GetMapReply> {
        // | MapPart::EXPLICIT_COMPONENTS
        // | MapPart::KEY_ACTIONS
        // | MapPart::KEY_BEHAVIORS

        self.conn
            .xkb_get_map(
                ID::USE_CORE_KBD.into(), // device spec
                MapPart::KEY_TYPES
                    | MapPart::KEY_SYMS
                    | MapPart::MODIFIER_MAP
                    | MapPart::VIRTUAL_MODS
                    | MapPart::VIRTUAL_MOD_MAP, // what you want back
                0_u16,                   // partial
                0_u8,                    // first_type
                0_u8,                    // n_types
                0_u8,                    // first_key_sym
                0_u8,                    // n_key_syms
                0_u8,                    // first_key_action
                0_u8,                    // n_key_actions
                0_u8,                    // first_key_behavior
                0_u8,                    // n_key_behaviors
                0_u16,                   // virtual_mods
                0_u8,                    // first_key_explicit
                0_u8,                    // n_key_explicit
                0_u8,                    // first_mod_map_key
                0_u8,                    // n_mod_map_keys
                0_u8,                    // first_v_mod_map_key
                0_u8,                    // n_v_mod_map_keys
            )
            .context("failed to get XKB `GetMapReply`")?
            .reply()
            .context("failed to get 'GetMapReply' reply")
    }

    /// Set the lock modifier state for the specified `Group`
    pub(crate) fn latch_lock_state(&self, group: Group) -> Result<()> {
        self.conn
            .xkb_latch_lock_state(
                ID::USE_CORE_KBD.into(),
                0,     // affect_mod_locks
                0,     // mod_locks
                true,  // lock_group
                group, // group_lock
                0,     // affect_mod_latches
                false, // latch_group
                0,     // group_latch
            )
            .context("failed to get latch lock state")?
            .check()
            .context("failed to check latch lock state")?;

        Ok(())
    }

    /// Return the informatin about the focused `Window`
    pub(crate) fn get_input_focus(&self) -> Result<GetInputFocusReply> {
        self.conn
            .get_input_focus()
            .context("failed to get `GetInputFocusReply`")?
            .reply()
            .context("failed to get `GetInputFocusReply` reply")
    }

    /// Release queued up events from grabbing the keyboard/mouse actively
    pub(crate) fn allow_events(&self, event_type: u8, replay_event: bool) -> Result<()> {
        match event_type {
            xproto::KEY_PRESS_EVENT | xproto::KEY_RELEASE_EVENT =>
                if replay_event {
                    self.conn
                        .allow_events(Allow::REPLAY_POINTER, x11rb::CURRENT_TIME)
                        .context("failed to allow `REPLAY_POINTER` event")?
                        .check()
                        .context("failed to check `REPLAY_POINTER` event")?;
                } else {
                    self.conn
                        .allow_events(Allow::SYNC_POINTER, x11rb::CURRENT_TIME)
                        .context("failed to allow `SYNC_POINTER` event")?
                        .check()
                        .context("failed to check `SYNC_POINTER` event")?;
                },
            xproto::BUTTON_PRESS_EVENT | xproto::BUTTON_RELEASE_EVENT =>
                if replay_event {
                    self.conn
                        .allow_events(Allow::REPLAY_KEYBOARD, x11rb::CURRENT_TIME)
                        .context("failed to allow `REPLAY_KEYBOARD` event")?
                        .check()
                        .context("failed to check `REPLAY_KEYBOARD` event")?;
                } else {
                    self.conn
                        .allow_events(Allow::SYNC_KEYBOARD, x11rb::CURRENT_TIME)
                        .context("failed to allow `SYNC_KEYBOARD` event")?
                        .check()
                        .context("failed to check `SYNC_KEYBOARD` event")?;
                },
            _ => {},
        }

        self.flush();
        Ok(())
    }

    /// Generate the [`CharacterMap`](super::keys::CharacterMap)
    pub(crate) fn generate_charmap(&mut self) -> Result<()> {
        let keysym_hash = KeysymHash::HASH;
        let get_reply = self.get_map_reply()?;
        let map = get_reply.map;

        self.device_id = get_reply.device_id;

        // KeyType {
        //     mods_mask: 1,
        //     mods_mods: 1,
        //     mods_vmods: 0,
        //     num_levels: 2,
        //     has_preserve: false,
        //     map: [
        //         KTMapEntry {
        //             active: true,
        //             mods_mask: 1,
        //             level: 1,
        //             mods_mods: 1,
        //             mods_vmods: 0,
        //         },
        //     ],
        //     preserve: [],
        // },
        let key_types = map.types_rtrn.as_ref().ok_or(Error::AcquireKeytypes)?;
        // KeySymMap {
        //     kt_index: [0, 0, 0, 0],
        //     group_info: 1,
        //     width: 1,
        //     syms: [65307],
        // },
        let sym_maps = map.syms_rtrn.as_ref().ok_or(Error::AcquireKeysyms)?;
        let key_modmap = map.modmap_rtrn.as_ref().ok_or(Error::AcquireModmap)?;
        let vmods = map.vmods_rtrn.as_ref().ok_or(Error::AcquireVirtualModmap)?;
        let virtual_mod = map
            .vmodmap_rtrn
            .as_ref()
            .ok_or(Error::AcquireVirtualModmap)?;

        self.modmap = key_modmap.clone();

        for (idx, symm) in sym_maps.iter().enumerate() {
            let kc = self.min_keycode + idx as u8;
            let vmod = virtual_mod
                .iter()
                .find(|v| v.keycode == kc)
                .map_or(0, |v| v.vmods);

            for group in 0..symm.group_info & 0x0f {
                let key_type_idx = symm.kt_index[group as usize & 0x03];
                let key_type = key_types
                    .get(key_type_idx as usize)
                    .ok_or_else(|| Error::LookupKeysyms(key_type_idx, symm.clone()))?;

                // if key_type.has_preserve {
                //     println!("PRESERVER: {:#?}", key_type.preserve);
                // }

                for level in 0..key_type.num_levels {
                    let keysym = symm.syms[level as usize];
                    let mut modmask = 0;
                    let mut key_level = 1;

                    'mod_search: for map in &key_type.map {
                        if map.active && map.level == level {
                            modmask = map.mods_mask;
                            key_level = level;
                            break 'mod_search;
                        }
                    }

                    match keysym_hash
                        .get_str_from_keysym_code(keysym)
                        .ok_or(Error::LookupKeysymHash(keysym))
                    {
                        Ok(ks) => {
                            let charmap = CharacterMap::new(
                                ks.to_string(),
                                kc,
                                u16::from(modmask | keys::get_modmask_from_keycode(key_modmap, kc)),
                                keysym,
                                key_level,
                                vmod,
                                group,
                                true,
                            );

                            // println!("CHAR: {:#?}", charmap);

                            self.charmap.push(charmap);
                        },
                        Err(_) => {
                            log::info!(
                                "failed to build a `CharacterMap` for (Keysym={}, Keycode={})",
                                keysym.to_string().green().bold(),
                                kc.to_string().yellow().bold(),
                            );
                        },
                    }
                }
            }
        }

        // "L1", "L2"... get added multiple times with different `modmask`
        let reply = self.get_keyboard_mapping_reply()?;
        self.keysyms_per_keycode = reply.keysyms_per_keycode;

        // let k = self
        //     .conn
        //     .xkb_get_names(
        //         ID::USE_CORE_KBD.into(),
        //         NameDetail::SYMBOLS | NameDetail::KEYCODES | NameDetail::KEY_NAMES,
        //     )?
        //     .reply()?;

        // let modmap = self.get_modifier_mapping()?;
        // println!("MODMAP: {:#?}", modmap);
        // std::process::exit(1);

        Ok(())
    }

    /// Return the `ModifierMask` of a key based on its' `Keycode`. This should
    /// return the same result as the `CharacterMap` database
    ///
    /// This is an alternate way to do
    /// [`get_modmask_from_keycode`](super::keys::get_modmask_from_keycode)
    /// This method does not require that there be a built `CharacterMap` vector
    pub(crate) fn modmask_from_keycode(&self, keycode: Keycode) -> Result<ModifierMask> {
        let mut modmask = ModifierMask::new(0);

        let r = self.get_modifier_mapping()?;

        let num_mod = r.keycodes.len() / usize::from(r.keycodes_per_modifier());

        for i in 0..num_mod {
            for j in 0..r.keycodes_per_modifier() {
                if keycode
                    == r.keycodes[i * usize::from(r.keycodes_per_modifier()) + usize::from(j)]
                {
                    modmask.combine_u16(1 << i);
                }
            }
        }

        Ok(modmask)
    }

    // ================ Grab / Ungrab =================

    /// Grab control of all keyboard input
    pub(crate) fn grab_keyboard(&self) -> Result<()> {
        let reply = self
            .conn
            .grab_keyboard(
                false,       // owner events
                self.root(), // window
                x11rb::CURRENT_TIME,
                xproto::GrabMode::ASYNC,
                xproto::GrabMode::ASYNC,
            )
            .context("failed to grab keyboard")?
            .reply()
            .context("failed to get reply after grabbing keyboard")?;

        if reply.status == xproto::GrabStatus::ALREADY_GRABBED {
            log::info!("the keyboard is already grabbed");
        } else if reply.status != xproto::GrabStatus::SUCCESS {
            lxhkd_fatal!("failed to grab keyboard. Replied with unsuccessful status");
        }

        Ok(())
    }

    /// Ungrab/release the keyboard
    pub(crate) fn ungrab_keyboard(&self) {
        if let Err(e) = self.conn.ungrab_keyboard(x11rb::CURRENT_TIME) {
            lxhkd_fatal!("failed to ungrab keyboard: {}", e);
        }
    }

    /// Grab a specified key plus possible modifiers
    pub(crate) fn grab_key(&self, chords: &[Chord]) {
        for chord in chords {
            for mask in ModifierMask::return_ignored(chord.modmask()) {
                log::debug!(
                    "grabbing utf:{}-code:{}-mask:{}",
                    chord.charmap().utf(),
                    chord.charmap().code(),
                    mask.mask()
                );
                if let Err(e) = self.conn.grab_key(
                    false,
                    self.root,
                    mask.mask(),
                    chord.charmap().code(),
                    xproto::GrabMode::ASYNC,
                    xproto::GrabMode::ASYNC,
                ) {
                    lxhkd_fatal!(
                        "failed to grab key {:?} with a mask {}: {}",
                        chord.charmap().code(),
                        chord.modmask(),
                        e
                    );
                }
            }
        }
    }

    /// Ungrab a set of `XKeyCode`'s
    pub(crate) fn ungrab_key(&self, chords: &[Chord]) {
        for chord in chords {
            if let Err(e) = self.conn.ungrab_key(
                chord.charmap().code(), // key
                self.root,              // window
                chord.modmask(),        // modifier
            ) {
                lxhkd_fatal!("failed to ungrab key: {}", e);
            }
        }
    }

    /// Ungrab any grabbed key
    pub(crate) fn ungrab_any_key(&self) {
        if let Err(e) = self.conn.ungrab_key(
            xproto::Grab::ANY,    // key
            self.root,            // window
            xproto::ModMask::ANY, // modifier
        ) {
            lxhkd_fatal!("failed to ungrab any key: {}", e);
        }
    }

    /// Grab the `Button`s passed to this function
    pub(crate) fn grab_button(&self, buttons: &[&XButton]) -> Result<()> {
        // self.conn.ungrab_button(ButtonIndex::ANY, self.root, ModMask::ANY)?;

        // EventMask::POINTER_MOTION
        let event_mask = u32::from(
            EventMask::BUTTON_PRESS | EventMask::BUTTON_RELEASE | EventMask::BUTTON_MOTION,
        );
        for button in buttons {
            for mask in ModifierMask::return_ignored(button.modmask()) {
                self.conn.grab_button(
                    false,                   // owner_events
                    self.root,               // grab_window
                    event_mask as u16,       // event_mask
                    xproto::GrabMode::ASYNC, // pointer_mode
                    xproto::GrabMode::ASYNC, // keyboard_mode
                    x11rb::NONE,             // confine_to
                    x11rb::NONE,             // cursor
                    button.code().into(),    // button
                    mask.mask(),             // modifiers
                )?;
            }
        }

        // self.flush();
        Ok(())
    }

    /// Ungrab the given `Button`s
    pub(crate) fn ungrab_button(&self, buttons: &[XButton]) {
        for button in buttons {
            if let Err(e) =
                self.conn
                    .ungrab_button(button.code().into(), self.root, button.modmask())
            {
                lxhkd_fatal!("failed to ungrab button: {}", button);
            }
        }
    }

    /// Ungrab any grabbed button
    pub(crate) fn ungrab_any_button(&self) {
        if let Err(e) =
            self.conn
                .ungrab_button(xproto::ButtonIndex::ANY, self.root, xproto::ModMask::ANY)
        {
            lxhkd_fatal!("failed to ungrab any button: {}", e);
        }

        self.flush();
    }

    // ================= Fake Events ==================

    /// Create a modifier to send as a fake key. Note that `KeyPressEvent` and
    /// `KeyReleaseEvent` are the same besides their `response_type`
    pub(crate) fn make_modifier(
        &self,
        modmask: u16,
        pressed: bool,
        window: xproto::Window,
    ) -> Result<()> {
        let reply = self.get_modifier_mapping()?;
        let keycodes_per_modifier = reply.keycodes_per_modifier();
        let modmap = reply.keycodes;

        println!("MODMASK == {:#?}", modmask);

        for mod_idx in u16::from(MapIndex::SHIFT)..=u16::from(MapIndex::M5) {
            if modmask & (1 << mod_idx) != 0 {
                'inner: for modkey in 0..keycodes_per_modifier {
                    let keycode = modmap
                        [(mod_idx * u16::from(keycodes_per_modifier) + u16::from(modkey)) as usize];

                    if keycode != 0 {
                        println!("KEYCODE == {:#?}", keycode);
                        self.conn
                            .xtest_fake_input(
                                pressed
                                    .then(|| xproto::KEY_PRESS_EVENT)
                                    .unwrap_or(xproto::KEY_RELEASE_EVENT),
                                keycode,
                                x11rb::CURRENT_TIME,
                                window,
                                1,
                                1,
                                0,
                            )?
                            .check()?;

                        // self.make_key_event(
                        //     keycode,
                        //     pressed
                        //         .then(|| xproto::KEY_PRESS_EVENT)
                        //         .unwrap_or(xproto::KEY_RELEASE_EVENT),
                        //     &event,
                        // );

                        // self.make_generic_key_event(
                        //     pressed
                        //         .then(|| xproto::KEY_PRESS_EVENT)
                        //         .unwrap_or(xproto::KEY_RELEASE_EVENT),
                        //     keycode,
                        //     window,
                        // );

                        self.sync();
                        break 'inner;
                    }
                }
            }
        }

        Ok(())
    }

    /// Send a key press or release event
    pub(crate) fn make_key(
        &self,
        chord: &Chord,
        modstate: u16,
        pressed: bool,
        window: xproto::Window,
    ) -> Result<()> {
        let mut modmask = ModifierMask::from(modstate) | chord.modmask();
        let mut xtest = false;

        if window == 0 || window == self.get_input_focus()?.focus {
            xtest = true;
        }

        if xtest {
            // let state = self.get_state()?;
            // let curr_group = state.group;
            //
            // if u16::from(curr_group) != chord.charmap().group() {
            //     self.latch_lock_state((chord.charmap().group() as u8).try_into().context(
            //         "failed to convert `CharacterMap` group to `Group` for
            // `latch_lock_state`",     )?)?;
            // }

            if modmask.mask() != 0 {
                // Send modifier if the key contains a modifier
                println!("MAKING MODIFIER");
                self.make_modifier(modmask.mask(), pressed, window)
                    .context("failed to send key modifier event")?;
            }

            // Send the regular key, without a mask
            println!("== MAKING EVENT ==");
            self.conn
                .xtest_fake_input(
                    pressed
                        .then(|| xproto::KEY_PRESS_EVENT)
                        .unwrap_or(xproto::KEY_RELEASE_EVENT),
                    chord.charmap().code(),
                    x11rb::CURRENT_TIME,
                    window,
                    1,
                    1,
                    0,
                )?
                .check()?;

            // self.make_key_event(
            //     chord.charmap().code(),
            //     pressed
            //     .then(|| xproto::KEY_PRESS_EVENT)
            //     .unwrap_or(xproto::KEY_RELEASE_EVENT),
            //     &event,
            //     );

            // self.make_generic_key_event(
            //     pressed
            //         .then(|| xproto::KEY_PRESS_EVENT)
            //         .unwrap_or(xproto::KEY_RELEASE_EVENT),
            //     chord.charmap().code(),
            //     window,
            // );

            // if u16::from(curr_group) != chord.charmap().group() {
            //     self.latch_lock_state(curr_group)?;
            // }

            // self.sync();
        } else {
            modmask.combine_u16(chord.charmap().modmask() << 13);

            let event = xproto::KeyPressEvent {
                response_type: pressed
                    .then(|| xproto::KEY_PRESS_EVENT)
                    .unwrap_or(xproto::KEY_RELEASE_EVENT),
                detail:        chord.charmap().code(),
                sequence:      0,
                time:          x11rb::CURRENT_TIME,
                root:          window,
                event:         x11rb::NONE,
                child:         x11rb::NONE,
                root_x:        1,
                root_y:        1,
                event_x:       1,
                event_y:       1,
                state:         modmask.mask(),
                same_screen:   true,
            };

            self.conn
                .send_event(
                    true, // propagate
                    window,
                    pressed
                        .then(|| EventMask::KEY_PRESS)
                        .unwrap_or(EventMask::KEY_RELEASE),
                    event,
                )
                .context("failed to send key event")?
                .check()
                .context("failed to check result after sending key event")?;

            self.flush();
        }

        Ok(())
    }

    /// Send a sequence of keys, including modifiers
    pub(crate) fn make_keysequence(
        &self,
        mut chords: Vec<Chord>,
        pressed: bool,
        window: xproto::Window,
    ) -> Result<()> {
        let mut modstate = 0;
        let mut change_keymap = false;
        let mut scratch_code = 0;

        // println!("== MAKING == {:#?}", chords);

        let kb_mapping = self.get_keyboard_mapping_reply()?;

        for i in self.min_keycode..self.max_keycode {
            let mut avail = true;
            'inner: for j in 0..kb_mapping.keysyms_per_keycode {
                let sym_idx = (i - self.min_keycode) * kb_mapping.keysyms_per_keycode + j;

                if kb_mapping.keysyms[sym_idx as usize] == 0 {
                    break 'inner;
                }

                avail = false;
            }

            if avail {
                scratch_code = i;
                break;
            }
        }

        println!("SCRATCH: {}", scratch_code);

        for chord in &mut chords {
            if !chord.charmap().is_bound() {
                log::debug!("changing keyboard mapping");
                self.conn
                    .change_keyboard_mapping(1, scratch_code, 1, &[chord.charmap().symbol()])
                    .context("failed to change keyboard mapping")?
                    .check()
                    .context("failed to check result after changing keyboard mapping")?;
                self.sync();

                chord.update_keycode(scratch_code);
                change_keymap = true;
            }

            self.make_key(chord, modstate, pressed, window)
                .with_context(|| format!("failed to send key: {}", chord.charmap().utf()))?;

            if !chord.charmap().is_bound() {
                self.sync();
            }
        }

        if change_keymap {
            log::debug!("changing keyboard mapping back");
            self.conn.change_keyboard_mapping(1, scratch_code, 1, &[0]);
            self.sync();
        }

        self.flush();

        Ok(())
    }

    // ================ GenericEvent ==================

    /// Make a generic event with no window
    pub(crate) fn make_generic_event_no_window(
        &self,
        event_type: u8,
        sim_keycode: u8,
    ) -> Result<()> {
        log::debug!(
            "{} for {}",
            "generated fake key press".green().bold(),
            sim_keycode
        );
        self.conn
            .xtest_fake_input(
                event_type,          // event type
                sim_keycode,         // detail
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
    pub(crate) fn make_generic_key_event(
        &self,
        event_type: u8,
        sim_keycode: u8,
        window: xproto::Window,
    ) -> Result<()> {
        log::debug!(
            "{} for {}",
            "generated fake key press".green().bold(),
            sim_keycode
        );
        self.conn
            .xtest_fake_input(
                event_type,          // event type
                sim_keycode,         // detail
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

    /// Create a [`KeyPressEvent`](x11rb::protocol::xproto::KeyPressEvent) or
    /// [`KeyReleaseEvent`](x11rb::protocol::xproto::KeyReleaseEvent)
    pub(crate) fn make_keypress_event(
        &self,
        sim_keycode: u8,
        mask: u16,
        event: &xproto::KeyPressEvent,
    ) -> Result<()> {
        log::debug!(
            "{} for {}",
            "generated fake key press".green().bold(),
            sim_keycode
        );

        let event = xproto::KeyPressEvent {
            response_type: xproto::KEY_PRESS_EVENT,
            detail:        sim_keycode,
            sequence:      0,
            time:          x11rb::CURRENT_TIME,
            root:          event.root,
            event:         x11rb::NONE,
            child:         x11rb::NONE,
            root_x:        event.root_x,
            root_y:        event.root_y,
            event_x:       1,
            event_y:       1,
            state:         mask,
            same_screen:   true,
        };

        self.conn
            .send_event(
                true, // propagate
                event.root,
                EventMask::KEY_PRESS,
                event,
            )
            .context("failed to send key event")?
            .check()
            .context("failed to check result after sending key event")?;

        Ok(())
    }

    pub(crate) fn make_keyrelease_event(
        &self,
        sim_keycode: u8,
        mask: u16,
        event: &xproto::KeyReleaseEvent,
    ) -> Result<()> {
        log::debug!(
            "{} for {}",
            "generated fake key press".green().bold(),
            sim_keycode
        );

        let event = xproto::KeyReleaseEvent {
            response_type: xproto::KEY_RELEASE_EVENT,
            detail:        sim_keycode,
            sequence:      0,
            time:          x11rb::CURRENT_TIME,
            root:          event.root,
            event:         x11rb::NONE,
            child:         x11rb::NONE,
            root_x:        event.root_x,
            root_y:        event.root_y,
            event_x:       1,
            event_y:       1,
            state:         mask,
            same_screen:   true,
        };

        self.conn
            .send_event(
                true, // propagate
                event.root,
                EventMask::KEY_RELEASE,
                event,
            )
            .context("failed to send key event")?
            .check()
            .context("failed to check result after sending key event")?;

        Ok(())
    }

    /////////////////////////////////////////////////////

    /// Create a [`KeyPressEvent`](x11rb::protocol::xproto::KeyPressEvent)
    /// This does not allow for sending a modifier (i.e., changed state)
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
        self.conn
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
        self.conn
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
        self.conn
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
        self.conn
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

    /// Create a [`KeyPressEvent`](x11rb::protocol::xproto::KeyPressEvent).
    /// Doesn't need to be given an event
    #[allow(dead_code)]
    pub(crate) fn make_key_press_no_event(&self, sim_keycode: u8) -> Result<()> {
        self.conn
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

    /// Create a [`KeyReleaseEvent`](x11rb::protocol::xproto::KeyReleaseEvent).
    /// Doesn't need to be given an event
    #[allow(dead_code)]
    pub(crate) fn make_key_release_no_event(&self, sim_keycode: u8) -> Result<()> {
        self.conn
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

    /// Create a [`ButtonPressEvent`](x11rb::protocol::xproto::
    /// ButtonPressEvent). Doesn't need to be given an event
    #[allow(dead_code)]
    pub(crate) fn make_button_press_no_event(&self, button: u8) -> Result<()> {
        self.conn
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
    /// ButtonReleaseEvent). Doesn't need to be given an event
    #[allow(dead_code)]
    pub(crate) fn make_button_release_no_event(
        &self,
        button: u8,
        duration_ms: Option<u32>,
    ) -> Result<()> {
        self.conn
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

    // =================== Other ======================

    /// Create a full click of the mouse (`ButtonPress` + `ButtonRelease`)
    #[allow(dead_code)]
    pub(crate) fn make_click(&self, button: u8, duration_ms: u32) -> Result<()> {
        self.make_button_press_no_event(button)?;
        self.make_button_release_no_event(button, Some(duration_ms))?;

        Ok(())
    }

    /// Ungrab everything this program grabbed. Used for when the user stops the
    /// program or the program gracefully exits
    pub(crate) fn cleanup(&self) {
        self.ungrab_keyboard();
        self.ungrab_any_key();
        self.ungrab_any_button();

        self.flush();
    }

    /// List the available `Keysym`s a user can use in their mappings. The
    /// output is in a neat format, and the information is similar to what
    /// `xmodmap -pke` would show
    ///
    /// This function does have a direct command-line interface
    pub(crate) fn list_keysyms(&self) -> Result<()> {
        use cli_table::{
            format::{Border, Justify, Separator},
            print_stdout,
            Cell,
            CellStruct,
            ColorChoice,
            Style,
            Table,
        };
        let mut table = vec![];

        for charmap in &self.charmap {
            table.push(vec![
                charmap.utf().purple().bold().cell().justify(Justify::Left),
                charmap
                    .code()
                    .to_string()
                    .green()
                    .cell()
                    .justify(Justify::Left),
                charmap
                    .symbol()
                    .to_string()
                    .yellow()
                    .cell()
                    .justify(Justify::Left),
                charmap
                    .modmask()
                    .to_string()
                    .red()
                    .bold()
                    .cell()
                    .justify(Justify::Left),
            ]);
        }

        let build_title = |s: &str| -> CellStruct { s.blue().bold().cell().justify(Justify::Left) };

        print_stdout(
            table
                .table()
                .title(vec![
                    build_title("UTF Keysym"),
                    build_title("Keycode"),
                    build_title("Keysym Code"),
                    build_title("Modmask"),
                ])
                .border(Border::builder().build())
                .separator(Separator::builder().build()),
        )
        .context("failure to print table to `stdout`")?;

        Ok(())
    }

    /// === Debugging Function ===
    ///
    /// List the active modifiers
    #[allow(dead_code)]
    pub(crate) fn get_active_mods(&self) {
        self.charmap
            .iter()
            .filter(|c| c.vmod() != 0)
            .for_each(|c| println!("{}: mask: {}", c.utf().green().bold(), c.modmask()));
    }

    /// === Debugging Function ===
    ///
    /// Display the current keysym mappings
    pub(crate) fn dump_charmap(&self) {
        println!("{:#?}", self.charmap);
    }

    /// === Debugging Function ===
    ///
    /// Display the `Lock` modifier masks. Comparing the masks from the
    /// `CharacterMap` database and the method in which the modfield is
    /// extracted from the `Keycode`
    #[allow(dead_code)]
    pub(crate) fn get_lock_fields(&self) -> Result<()> {
        let num_char = CharacterMap::charmap_from_keysym_utf(&self.charmap, "Num_Lock")
            .context("couldn't find `Num_Lock` in `CharacterMap`")?;
        let num_mask = num_char.modmask();
        let num_from_code = self.modmask_from_keycode(num_char.code())?;

        let scroll_char = CharacterMap::charmap_from_keysym_utf(&self.charmap, "Scroll_Lock")
            .context("couldn't find `Scroll_Lock` in `CharacterMap`")?;
        let scroll_mask = scroll_char.modmask();
        let scroll_from_code = self.modmask_from_keycode(scroll_char.code())?;

        let caps_char = CharacterMap::charmap_from_keysym_utf(&self.charmap, "Caps_Lock")
            .context("couldn't find `Caps_Lock` in `CharacterMap`")?;
        let caps_mask = caps_char.modmask();
        let caps_from_code = self.modmask_from_keycode(caps_char.code())?;

        let bold = |s: &str| -> ColoredString { s.green().bold() };
        println!(
            "{}: charmap: {} from code: {}\n{}: charmap: {} from code: {}\n{}: charmap: {} from \
             code: {}",
            bold("Num_Lock"),
            num_mask,
            num_from_code,
            bold("Scroll_Lock"),
            scroll_mask,
            scroll_from_code,
            bold("Caps_Lock"),
            caps_mask,
            caps_from_code,
        );

        Ok(())
    }

    // pub(crate) fn set_cursor(
    //     &self,
    //     window: xproto::Window,
    //     cursor_name: &str,
    // ) -> Result<()> {
    //     self.conn.change_window_attributes(
    //         window,
    //         &ChangeWindowAttributesAux::new()
    //             .cursor(self.cursor_handle.load_cursor(self.conn, cursor_name)?),
    //     )?;
    //
    //     Ok(())
    // }

    ///////////////////////

    // /// Return the modifier field based on a keysym
    // pub(crate) fn modfield_from_keysym(&self, keysym: XKeysym) -> u16 {
    //     let mut modfield = 0_u16;
    //
    //     // TODO: Look into more but this works
    //     if let Some(keycodes) = self.keycode_from_keysym2(keysym) {
    //         // while keycodes != xcb::NO_SYMBOL as u8 {
    //         modfield |= self.modfield_from_keycode(keycodes);
    //         // keycodes += 1;
    //         // }
    //     }
    //
    //     modfield
    // }

    //////////////////////////////////////////////////////////////////////

    // /// Return the keycode from the keysym. Examples of what it is parsing
    // /// can be found from the output of `xmodmap -pki`
    // pub(crate) fn keycode_from_keysym(&self, keysym: xcb::Keysym) ->
    // Option<xcb::Keycode> {     let mut result = None;
    //     // let mut num = 0_u8;
    //
    //     // Match the last item specifying keycode within the database if it is
    // mentioned     // more than once
    //     for kc in self.setup.min_keycode()..=self.setup.max_keycode() {
    //         for col in 0..KEYSYMS_PER_KEYCODE {
    //             if let Ok(reply) = xcb::get_keyboard_mapping(conn, kc,
    // 1).get_reply() {                 let keysyms = reply.keysyms();
    //
    //                 #[allow(clippy::cast_possible_wrap, clippy::cast_lossless)]
    //                 let ks = KeySymbols::new(conn).get_keysym(kc, col as i32);
    //
    //                 if ks == keysym {
    //                     // num += 1;
    //                     // if num == 1 {
    //                     result = Some(kc);
    //                     // } else {
    //                     //     let mut split = result.map(|f| {
    //                     //         f.to_string()
    //                     //             .split("")
    //                     //             .map(ToString::to_string)
    //                     //             .collect::<Vec<_>>()
    //                     //     })?;
    //                     //     // .chars()
    //                     //     // .map(|d| d.to_digit(10).unwrap() as u8)
    //                     //
    //                     //     split[(num - 1) as usize] = kc.to_string();
    //                     //     split[num as usize] = xcb::NO_SYMBOL.to_string();
    //                     //
    //                     //     result = Some(split.join("").parse::<u8>().ok()?);
    //                     // }
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //
    //     result
    // }
    //
    // /// Print the lock-keys keysym codes
    // pub(crate) fn get_lock_fields(&self) {
    //     // TODO: possibly use custom struct XK_..
    //     let scroll_lock = self.modfield_from_keysym(self.conn, XK_Scroll_Lock);
    //     let num_lock = self.modfield_from_keysym(self.conn, XK_Num_Lock);
    //     let caps_lock = xcb::MOD_MASK_LOCK;
    //     println!(
    //         "{}:\nNum_Lock: {}\nCaps_Lock: {}\n Scroll_Lock: {}",
    //         "Lock Fields".yellow().bold(),
    //         num_lock,
    //         caps_lock,
    //         scroll_lock
    //     );
    // }
    //
    //
    // /// Update the device's current state
    // pub(crate) fn update_state(&mut self, event: &xcb::xkb::StateNotifyEvent) {
    //     self.state.update_mask(
    //         event.base_mods().bits(),
    //         event.latched_mods().bits(),
    //         event.locked_mods().bits(),
    //         event.base_group() as xkb::LayoutIndex,
    //         event.latched_group() as xkb::LayoutIndex,
    //         event.locked_group() as xkb::LayoutIndex,
    //     );
    // }
}

impl fmt::Debug for Keyboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "--Keyboard--\nroot: {:?}\ndevice_id: {}\nmin_keycode: {}\nmax_keycode: \
             {}\nautorepeat_delay: {}\nautorepeat_interval: {}",
            self.root,
            self.device_id,
            self.min_keycode,
            self.max_keycode,
            self.autorepeat_delay,
            self.autorepeat_interval,
        )
    }
}
