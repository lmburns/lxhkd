use super::{
    keys::{self, CharacterMap, XKeyCode},
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
// use rayon::prelude::*;

// use xcb::{Cookie, Reply};
// use xcb_util::keysyms::KeySymbols;
// use xkbcommon::xkb::{self, Keycode, Keymap, State};
// use x11::keysym::{XK_Num_Lock, XK_Scroll_Lock};

use x11rb::{
    connection::{Connection, RequestConnection},
    errors::ReplyError,
    properties, protocol,
    protocol::{
        xkb::{self, ConnectionExt as _, GetMapReply, KeyModMap, KeySymMap, MapPart, ID},
        xproto::{self, ConnectionExt, EventMask, Keycode, Keysym, ModMask},
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
    #[error("failed to get virutal modmap maps")]
    AcquireVirtualModmap,
    #[error("failed to lookup keysym {1:?} at index {0}")]
    LookupKeysyms(u8, KeySymMap),
    #[error("failed to lookup keysym {0:?}")]
    LookupKeysymHash(Keysym),
    #[error("failed to build a `CharacterMap` for (Keysym={0:?}, Keycode={1}): {2}")]
    BuildCharacterMap(Keysym, Keycode, anyhow::Error),
}

/// State of the keyboard
pub(crate) struct Keyboard<'a> {
    /// Connection to the X-Server
    conn:        &'a RustConnection,
    /// Root window.
    root:        xproto::Window,
    /// The characters, keysyms, etc making up the `Keyboard`
    charmap:     Vec<CharacterMap>,
    /// The minimum keycode
    min_keycode: u8,
    /// The maximum keycode
    max_keycode: u8,
}

// /// The device's ID
// device_id:  Xid,

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

        // let k = conn.get_keyboard_mapping();
        // let k = conn.get_modifier_mapping();
        // let k = conn.change_keyboard_mapping();

        let mut keyboard = Self {
            conn,
            root,
            charmap: Vec::new(),
            min_keycode: 0,
            max_keycode: 0,
        };

        keyboard.generate_charmap()?;

        Ok(keyboard)
    }

    /// Get the `GetMapReply`
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
            .context("failed to get xkb mapp")?
            .reply()
            .context("failed to get 'GetMapReply' reply")
    }

    /// Generate the `CharacterMap`
    pub(crate) fn generate_charmap(&mut self) -> Result<()> {
        let get_reply = self.get_map_reply()?;

        self.min_keycode = get_reply.min_key_code;
        self.max_keycode = get_reply.max_key_code;

        let map = get_reply.map;

        let keysym_hash = KeysymHash::HASH;

        let key_types = map.types_rtrn.as_ref().ok_or(Error::AcquireKeytypes)?;
        let sym_maps = map.syms_rtrn.as_ref().ok_or(Error::AcquireKeysyms)?;
        let key_modmap = map.modmap_rtrn.as_ref().ok_or(Error::AcquireModmap)?;
        let virtual_mod = map
            .vmodmap_rtrn
            .as_ref()
            .ok_or(Error::AcquireVirtualModmap)?;

        let vmods = map.vmods_rtrn.as_ref().ok_or(Error::AcquireVirtualModmap)?;

        // for v in virtual_mod.iter() {
        //     println!("vmod: {:#?} -- kc: {:#?}", v.vmods, v.keycode);
        // }

        // for (unsigned int i = 0; i < num_mod; i++) {
        //   for (unsigned int j = 0; j < reply->keycodes_per_modifier; j++) {
        //     xcb_keycode_t mkc = mod_keycodes[i * reply->keycodes_per_modifier + j];
        //     if (mkc == XCB_NO_SYMBOL) continue;
        //     if (keycode == mkc) modfield |= (1 << i);

        for (idx, symm) in sym_maps.iter().enumerate() {
            let kc = self.min_keycode + idx as u8;
            let group_cnt = symm.group_info & 0x0f;

            for group in 0..group_cnt {
                let key_type_idx = symm.kt_index[group as usize & 0x03];
                let key_type = key_types
                    .get(key_type_idx as usize)
                    .ok_or_else(|| Error::LookupKeysyms(key_type_idx, symm.clone()))?;

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

                    // 'vmod_search: for vmod in &key_type

                    let hash = KeysymHash::HASH;

                    match hash
                        .get_keysym(keysym)
                        .ok_or(Error::LookupKeysymHash(keysym))
                    {
                        Ok(ks) => {
                            let charmap = CharacterMap::new(
                                ks.to_string(),
                                kc,
                                u16::from(modmask | keys::get_keycode_modifier(key_modmap, kc)),
                                keysym,
                                key_level,
                                group,
                            );

                            // println!("CHAR: {:#?}", charmap);

                            self.charmap.push(charmap);
                        },
                        Err(e) => {
                            log::debug!(
                                "failed to build a `CharacterMap` for (Keysym={:?}, Keycode={}): \
                                 {}",
                                keysym.to_string().green().bold(),
                                kc.to_string().yellow().bold(),
                                e
                            );
                        },
                    }
                }
            }
        }

        // "L1", "L2"... get added multiple times with different `modmask`

        // self.generate_real_mods();

        Ok(())
    }

    // TODO: needs work
    /// Generate the real modifiers (`shift`, `lock`, `control`, `mod1` -
    /// `mod5`) by mapping corresponding `modmasks` from the already built
    /// database from [`generate_charmap`](self::generate_charmap)
    pub(crate) fn generate_real_mods(&mut self) {
        for charmap in self.charmap.clone() {
            match charmap.modmask {
                // Putting the modmap here requires it be created every time which requires another
                // clone

                // m if m == 1 << 0 => {
                //     let mut modmap = charmap;
                //     modmap.utf = String::from("shift");
                //     self.charmap.push(modmap);
                // },
                m if m == 1 << 1 => {
                    let mut modmap = charmap;
                    modmap.utf = String::from("lock");
                    self.charmap.push(modmap);
                },
                m if m == 1 << 2 => {
                    let mut modmap = charmap;
                    modmap.utf = String::from("ctrl");
                    self.charmap.push(modmap);
                },
                m if m == 1 << 3 => {
                    let mut modmap = charmap;
                    modmap.utf = String::from("mod1");
                    self.charmap.push(modmap);
                },
                m if m == 1 << 4 => {
                    let mut modmap = charmap;
                    modmap.utf = String::from("mod2");
                    self.charmap.push(modmap);
                },
                m if m == 1 << 5 => {
                    let mut modmap = charmap;
                    modmap.utf = String::from("mod3");
                    self.charmap.push(modmap);
                },
                m if m == 1 << 6 => {
                    let mut modmap = charmap;
                    modmap.utf = String::from("mod4");
                    self.charmap.push(modmap);
                },
                m if m == 1 << 7 => {
                    let mut modmap = charmap;
                    modmap.utf = String::from("mod5");
                    self.charmap.push(modmap);
                },
                _ => {},
            }
        }
    }

    // /// Return the device's ID
    // pub(crate) fn device_id(&self) -> Xid {
    //     self.device_id
    // }

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

    /// Return the `CharacterMap`
    pub(crate) fn charmap(&self) -> &Vec<CharacterMap> {
        &self.charmap
    }

    /// Debugging function to display the current keysym mappings
    pub(crate) fn dump_charmap(&self) {
        println!("{:#?}", self.charmap);
    }

    /// Grab control of all keyboard input
    pub(crate) fn grab_keyboard(&self) {
        if let Err(e) = self.conn.grab_keyboard(
            true,        // owner events
            self.root(), // window
            x11rb::CURRENT_TIME,
            xproto::GrabMode::ASYNC,
            xproto::GrabMode::ASYNC,
        ) {
            lxhkd_fatal!("failed to grab keyboard: {}", e);
        }
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
                    if *modifier == 0 {
                        key.mask
                    } else {
                        key.mask | *modifier
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
