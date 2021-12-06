//! The `Keyboard` struct which is the raw interface with the X-Server

use super::{
    chord::Chord,
    event_handler::Handler,
    keys::{self, CharacterMap, ModifierMask, XButton, XKeyCode},
    keysym::{KeysymHash, XKeysym},
};
use crate::{
    config::Config,
    lxhkd_fatal,
    types::{Xid, KEYSYMS_PER_KEYCODE},
};
use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap, fmt, str::FromStr};
use thiserror::Error;

// use xcb::{Cookie, Reply};
// use xcb_util::keysyms::KeySymbols;
// use xkbcommon::xkb::{self, Keycode, Keymap, State};
// use x11::keysym::{XK_Num_Lock, XK_Scroll_Lock};

use x11rb::{
    connection::{Connection, RequestConnection},
    errors::ReplyError,
    properties,
    protocol::{
        self,
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
            ChangeKeyboardControlAux,
            ConnectionExt,
            EventMask,
            GetKeyboardMappingReply,
            GetModifierMappingReply,
            Keycode,
            Keysym,
            ModMask,
        },
        xtest,
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
pub(crate) struct Keyboard<'a> {
    /// Connection to the X-Server
    conn:                &'a RustConnection,
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

impl<'a> Keyboard<'a> {
    /// Construct a new instance of `Keyboard`
    pub(crate) fn new(
        conn: &'a RustConnection,
        screen_num: usize,
        config: &Config,
    ) -> Result<Self> {
        let screen = conn.setup();
        let root = screen.roots[screen_num].clone().root;

        // TODO: query XF86 ext

        let (xkb_min, xkb_max) = xkb::X11_XML_VERSION;

        if let Err(e) = conn.xkb_use_extension(xkb_min as u16, xkb_max as u16) {
            lxhkd_fatal!(
                "xkb version is unsupported. Supported versions: {}-{}: {}",
                xkb_min,
                xkb_max,
                e
            );
        };
        if conn
            .extension_information(xkb::X11_EXTENSION_NAME)?
            .is_none()
        {
            lxhkd_fatal!(
                "xkb X11 extension {} is unsupported",
                xkb::X11_EXTENSION_NAME.green().bold()
            );
        }
        // Used to send test events
        if conn
            .extension_information(xtest::X11_EXTENSION_NAME)?
            .is_none()
        {
            lxhkd_fatal!(
                "xtest X11 extension {} is unsupported",
                xtest::X11_EXTENSION_NAME.green().bold()
            );
        }

        // let k = conn.get_keyboard_mapping();
        // let k = conn.get_modifier_mapping();
        // let k = conn.change_keyboard_mapping();

        let mut keyboard = Self {
            conn,
            root,
            charmap: Vec::new(),
            device_id: 0,
            min_keycode: screen.min_keycode,
            max_keycode: screen.max_keycode,
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
            // Config != New = something went wrong
            if delay != reply {
                log::info!(
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

    // TODO: To help further confirm that keys are modifiers, use the returned array
    // to compare the user set mappings to
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

        let reply = self.get_keyboard_mapping_reply()?;
        self.keysyms_per_keycode = reply.keysyms_per_keycode;

        // "L1", "L2"... get added multiple times with different `modmask`

        // TODO: Use lock mods
        // let r = self.conn.xkb_get_state(ID::USE_CORE_KBD.into())?.reply()?;

        Ok(())
    }

    /// Return the connection to the X-Server
    pub(crate) fn connection(&self) -> &RustConnection {
        self.conn
    }

    /// Return the root window
    pub(crate) fn root(&self) -> xproto::Window {
        self.root
    }

    /// Flush actions to the X-Server
    pub(crate) fn flush(&self) -> bool {
        self.conn.flush().is_ok()
    }

    /// Return the device's ID
    pub(crate) fn device_id(&self) -> Xid {
        self.device_id
    }

    /// Return the `CharacterMap`
    pub(crate) fn charmap(&self) -> &Vec<CharacterMap> {
        &self.charmap
    }

    /// Debugging function to display the current keysym mappings
    pub(crate) fn dump_charmap(&self) {
        println!("{:#?}", self.charmap);
    }

    /// Shorter `wait_for_event`
    pub(crate) fn wait_for_event(&self) -> Result<Event> {
        self.conn
            .wait_for_event()
            .context("failed to wait for next event")
    }

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
                    chord.charmap().utf,
                    chord.charmap().code,
                    mask.mask()
                );
                if let Err(e) = self.conn.grab_key(
                    false,
                    self.root,
                    mask.mask(),
                    chord.charmap().code,
                    xproto::GrabMode::ASYNC,
                    xproto::GrabMode::ASYNC,
                ) {
                    lxhkd_fatal!(
                        "failed to grab key {:?} with a mask {}: {}",
                        chord.charmap().code,
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
                chord.charmap().code, // key
                self.root,            // window
                chord.modmask(),      // modifier
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

    /// Ungrab everything this program grabbed. Used for when the user stops the
    /// program
    pub(crate) fn cleanup(&self) {
        self.ungrab_keyboard();
        self.ungrab_any_key();
        self.ungrab_any_button();

        self.flush();
    }

    /// This has a user interface, where one can list the available `Keysym`s in
    /// a neat format
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
                charmap.utf.purple().bold().cell().justify(Justify::Left),
                charmap
                    .code
                    .to_string()
                    .green()
                    .cell()
                    .justify(Justify::Left),
                charmap
                    .symbol
                    .to_string()
                    .yellow()
                    .cell()
                    .justify(Justify::Left),
                charmap
                    .modmask
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

    // pub(crate) fn latch_lock_state(&self) {
    //     let lockg = self
    //         .charmap
    //         .iter()
    //         .find(|c| c.utf == "Scroll_Lock")
    //         .unwrap();
    //
    //     let l = self
    //         .conn
    //         .xkb_latch_lock_state(
    //             ID::USE_CORE_KBD.into(),
    //             0,
    //             0,
    //             true,
    //             Group::from(lockg.group as u8),
    //             0,
    //             false,
    //             0,
    //         )
    //         .context("failed to get latch lock state")?
    //         .check()
    //         .context("failed to check latch lock state")?;
    // }

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

    // /// Return the modifier-field code from a specified
    // /// [`Keycode`](xcb::Keycode)
    // pub(crate) fn modfield_from_keycode(&self, keycode: Keycode) -> u16 {
    //     let mut modfield = 0_u16;
    //     let mods = self.keymap.mods();
    //
    //     if let Ok(reply) = xcb::x::get_modifier_mapping(self.conn).get_reply() {
    //         if reply.keycodes_per_modifier() > 0 {
    //             let keycodes = reply.keycodes();
    //             let num_mods =
    //                 (keycodes.iter().len() / reply.keycodes_per_modifier() as
    // usize) as u8;
    //
    //             for i in 0..num_mods {
    //                 for j in 0..reply.keycodes_per_modifier() {
    //                     let mkc = keycodes[i * reply.keycodes_per_modifier() +
    // j];                     if mkc == xcb::x::NO_SYMBOL as u8 {
    //                         continue;
    //                     }
    //                     if keycode == mkc {
    //                         modfield |= 1 << i;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     modfield
    // }
    //
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

impl<'a> fmt::Debug for Keyboard<'a> {
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
