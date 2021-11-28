use super::{
    keys::{CharacterMap, XKeyCode},
    keysym::{KeysymHash, XKeysym},
};
use crate::{
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
    properties, protocol,
    protocol::{
        xkb::{self, ConnectionExt as _, KeyModMap, MapPart, ID, GetMapReply},
        xproto::{self, ConnectionExt, EventMask, ModMask},
        xtest,
    },
    rust_connection::RustConnection,
    wrapper::ConnectionExt as _,
};

// TODO: GetControlsReply for key repeat
// ListComponentsReply = keymaps keycodes

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
}

/// State of the keyboard
pub(crate) struct Keyboard<'a> {
    /// Connection to the X-Server
    conn:        &'a RustConnection,
    /// Root window.
    root:        xproto::Window,
    /// Map from keycodes in the index to keysyms the corresponding keys yield.
    keysym_map:  KeysymHash,
    /// The characters, keysyms, etc making up the `Keyboard`
    charmap:     Vec<CharacterMap>,
    /// The minimum keycode
    min_keycode: u8,
    /// The maximum keycode
    max_keycode: u8,
}
// /// The device's ID
// device_id:  Xid,

// *
// /// The XKB library context used
// context:    xproto::Context,
// /// Current state of the keyboard
// state:      State,
// /// The current keymap.
// keymap:     Keymap,
// /// Allows acces to largest,smallest key and mods per key,
// etc setup:
// xcb::x::Setup,

impl<'a> Keyboard<'a> {
    /// Construct a new instance of `Keyboard`
    pub(crate) fn new(conn: &'a RustConnection, screen_num: usize) -> Result<Self> {
        let screen = conn.setup();
        let root = screen.roots[screen_num].clone().root;
        let (xkb_min, xkb_max) = xkb::X11_XML_VERSION;

        // TODO: do i need both?
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

        // let events = (xcb::xkb::EVENT_TYPE_NEW_KEYBOARD_NOTIFY
        //     | xcb::xkb::EVENT_TYPE_MAP_NOTIFY
        //     | xcb::xkb::EVENT_TYPE_STATE_NOTIFY) as u16;
        //
        // // xcb::xkb::MAP_PART_KEY_BEHAVIORS |
        // let map_parts = (xcb::xkb::MAP_PART_KEY_TYPES
        //     | xcb::xkb::MAP_PART_KEY_SYMS
        //     | xcb::xkb::MAP_PART_MODIFIER_MAP
        //     | xcb::xkb::MAP_PART_EXPLICIT_COMPONENTS
        //     | xcb::xkb::MAP_PART_KEY_ACTIONS
        //     | xcb::xkb::MAP_PART_VIRTUAL_MODS
        //     | xcb::xkb::MAP_PART_VIRTUAL_MOD_MAP) as u16;
        //

        // let k = conn.get_keyboard_mapping();
        // let k = conn.get_modifier_mapping();
        // let k = conn.change_keyboard_mapping();

        let mut keyboard = Self {
            conn,
            root,
            keysym_map: KeysymHash::init(),
            charmap: Vec::new(),
            min_keycode: 0,
            max_keycode: 0
        };

        keyboard.generate_charmap()?;

        Ok(keyboard)
    }

    /// Get the `GetMapReply`
    pub(crate) fn get_map_reply(&self) -> Result<GetMapReply> {
        Ok(self
            .conn
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
            .context("failed to get xkb mapp")?
            .reply()
            .context("failed to get 'GetMapReply' reply")?)
    }

    /// Generate the `CharacterMap`
    pub(crate) fn generate_charmap(&mut self) -> Result<()> {
        let get_reply = self.get_map_reply()?;

        self.min_keycode = get_reply.min_key_code;
        self.max_keycode = get_reply.max_key_code;

        let map = get_reply.map;

        let key_types = map.types_rtrn.as_ref().ok_or(Error::AcquireKeytypes)?;
        let sym_maps = map.syms_rtrn.as_ref().ok_or(Error::AcquireKeysyms)?;
        let key_modmaps = map.modmap_rtrn.as_ref().ok_or(Error::AcquireModmap)?;

        for t in key_types.iter() {
            println!("keytypes: {:#?}", t);
        }

        Ok(())
    }

    // /// Return the connection to the X-Server
    // pub(crate) fn connection(&self) -> impl connection::Connection {
    //     self.conn
    // }

    /// Return the root window
    pub(crate) fn root(&self) -> xproto::Window {
        self.root
    }

    // /// Return the device's ID
    // pub(crate) fn device_id(&self) -> Xid {
    //     self.device_id
    // }

    /// Flush actions to the X-Server
    pub(crate) fn flush(&self) -> bool {
        self.conn.flush().is_ok()
    }

    /// Grab control of all keyboard input
    pub(crate) fn grab_keyboard(&self) -> Result<()> {
        if let Err(e) = self.conn.grab_keyboard(
            true,        // owner events
            self.root(), // window
            x11rb::CURRENT_TIME,
            xproto::GrabMode::ASYNC,
            xproto::GrabMode::ASYNC,
        ) {
            lxhkd_fatal!("failed to grab keyboard: {}", e);
        }

        Ok(())
    }

    /// Ungrab/release the keyboard
    pub(crate) fn ungrab_keyboard(&self) {
        if let Err(e) = self.conn.ungrab_keyboard(x11rb::CURRENT_TIME) {
            lxhkd_fatal!("failed to ungrab keyboard: {}", e);
        }
    }

    /// Grab a specified key
    pub(crate) fn grab_key(&self, keycodes: &[XKeyCode]) {
        for modifier in &[0, u16::from(ModMask::M2)] {
            for key in keycodes {
                if let Err(e) = self.conn.grab_key(
                    false,
                    self.root,
                    if *modifier != 0 {
                        key.mask | *modifier
                    } else {
                        key.mask
                    },
                    key.code,
                    xproto::GrabMode::ASYNC,
                    xproto::GrabMode::ASYNC,
                ) {
                    lxhkd_fatal!(
                        "failed to grab key {:?} at modifier {}: {}",
                        key,
                        modifier,
                        e
                    );
                }
            }
        }
    }

    /// Ungrab any grabbed key
    pub(crate) fn ungrab_key(&self) {
        if let Err(e) = self.conn.ungrab_key(
            xproto::Grab::ANY,    // key
            self.root,            // window
            xproto::ModMask::ANY, // modifier
        ) {
            lxhkd_fatal!("failed to ungrab key: {}", e);
        }
    }

    // /// Generate a keysym map from a mock state of the keyboard. This is used to
    // /// create a mapping between [`XKeysym`](super::keysym::XKeysym) and
    // /// [`Keycode`](xkb::Keycode)
    // fn generate_keysyms(&mut self) {
    //     let mut keycode = self.setup.min_keycode();
    //     while keycode != self.setup.max_keycode() {
    //         let sym = self.state.key_get_syms(keycode);
    //         xkbcommon::xkb
    //
    //         let key = xkbcommon::xcb::xkb::Key(&self.state, keycode);
    //         let sym = key.sym();
    //
    //         log::debug!(
    //             "dummy: keycode {:?} {} keysym {:?} ({:?})",
    //             keycode,
    //             "=>".green(),
    //             sym,
    //             sym.map_or(format!("<{}>", "invalid".red().bold()), |s| s.utf8())
    //         );
    //
    //         self.keysym_map.push(sym.map(XKeysym::new));
    //
    //         keycode = Keycode(keycode.0 + 1);
    //     }
    // }

    /// Debugging function to display the current keysym mappings
    pub(crate) fn dump_keysyms(&self) {
        println!("{:#?}", self.keysym_map);
    }

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

    // fn get_keycode(&self, xcode: xkb::Keycode) -> key::Code {
    //     let xcode = xcode as usize;
    //     if xcode >= self.keycode_table.len() {
    //         eprintln!("keycode 0x{:x} is out of bounds", xcode);
    //         return key::Code::Unknown;
    //     }
    //     self.keycode_table[xcode]
    // }

    //////////////////////////////////////////////////////////////////////

    // fn get_keysym(&self, xsym: xkb::Keysym) -> key::Sym {
    //     if xsym >= 0x20 && xsym < 0x80 {
    //         let mut xsym = xsym;
    //         if xsym >= 0x61 && xsym <= 0x7a {
    //             xsym &= !(key::SYM_LATIN1_SMALL_MASK as u32);
    //         }
    //         unsafe { mem::transmute(xsym) }
    //     } else if xsym >= xkb::KEY_F1 && xsym <= xkb::KEY_F24 {
    //         unsafe { mem::transmute((key::Sym::F1 as u32) + (xsym - xkb::KEY_F1))
    // }     } else if let Some(k) = self.keysym_map.get(&xsym) {
    //         *k
    //     } else {
    //         key::Sym::Unknown
    //     }
    // }

    // /// Return the keysym from the keycode
    // pub(crate) fn keycode_to_keysym(&self, keycode: Keycode) -> Option<XKeysym> {
    //     let idx = (keycode.0 - self.setup.min_keycode().0) as usize;
    //
    //     if idx <= self.setup.max_keycode().0 as usize {
    //         self.keysym_map[index]
    //     } else {
    //         None
    //     }
    // }

    // /// Return the keycode from the keysym
    // pub(crate) fn keycode_from_keysym2(&self, keysym: XKeysym) -> Option<Keycode>
    // {     self.keysym_map
    //         .iter()
    //         .position(|e| e == Some(keysym))
    //         .map(|pos| (self.setup.min_keycode().0 + (pos as u32)) as Keycode)
    // }

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
    // /// TODO: Serialize the modifier masks for the current state of the keyboard
    // pub(crate) fn mod_mask(&mut self) -> xkb::ModMask {
    //     use xkb::state::component;
    //
    //     state::Serialize(&mut self.state).mods(component::MODS_EFFECTIVE)
    // }
    //
    // /// Update the keymap and set the keyboard state
    // pub(crate) fn update_keymap(&mut self) -> Result<()> {
    //     self.device_id = xkb::x11::get_core_keyboard_device_id(&self.conn);
    //     self.keymap = xkb::x11::keymap_new_from_device(
    //         &self.context,
    //         &self.conn,
    //         self.device_id,
    //         xkb::KEYMAP_COMPILE_NO_FLAGS,
    //     );
    //
    //     self.state = xkb::x11::state_new_from_device(&self.keymap, &self.conn,
    // self.device_id);
    //
    //     Ok(())
    // }
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

    // /// Read the next keypress
    // pub(crate) fn next_keypress(&self) -> Result<Option<KeyPressParseAttempt>> {
    //     if let Some(event) = self.conn.poll_for_event() {
    //         let attempt = self.attempt_to_parse_as_keypress(event);
    //         if let Ok(Some(_)) = attempt {
    //             return attempt;
    //         }
    //     }
    //
    //     Ok(self.conn.has_error().map(|_| None)?)
    // }
    //
}
