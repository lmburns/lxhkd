// TODO: Allow mouse buttons

// Keysyms
//  - https://wiki.linuxquestions.org/wiki/List_of_Keysyms_Recognised_by_Xmodmap
// XOrg Documentation
//  - https://www.x.org/releases/X11R7.7/doc/kbproto/xkbproto.html

use super::keysym::KeysymHash;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt, str::FromStr};
use thiserror::Error;

use x11rb::{
    connection,
    connection::{Connection, RequestConnection},
    errors::ReplyError,
    properties, protocol,
    protocol::{
        xkb::{self, ConnectionExt as _, GetMapReply, KeyModMap, MapPart, ID},
        xproto::{
            self, ButtonPressEvent, ButtonReleaseEvent, ConnectionExt, EventMask, KeyPressEvent,
            Keycode, Keysym, ModMask, MotionNotifyEvent,
        },
        xtest,
    },
    rust_connection::RustConnection,
    wrapper::ConnectionExt as _,
};

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("failed to lookup keysym {0:?}")]
    LookupKeysymHash(Keysym),
}

pub(crate) type KeyCodeMask = u16;

/// A key press and the held modifiers
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct XKeyCode {
    /// The held mask of the modifiers
    pub(crate) mask: KeyCodeMask,
    /// The key code that was held
    pub(crate) code: Keycode,
}

impl XKeyCode {
    /// Remove the modifier given (used with Scroll_Lock/Num_Lock)
    pub(crate) fn ignore_modifier(self, mask: ModMask) -> Self {
        Self {
            mask: self.mask & !(u16::from(mask)),
            code: self.code,
        }
    }
}

impl From<KeyPressEvent> for XKeyCode {
    fn from(event: KeyPressEvent) -> Self {
        Self {
            mask: event.state,
            code: event.detail,
        }
    }
}

impl From<&KeyPressEvent> for XKeyCode {
    fn from(event: &KeyPressEvent) -> Self {
        Self {
            mask: event.state,
            code: event.detail,
        }
    }
}

// impl TryFrom<XcbGenericEvent> for XKeyCode {
//     type Error = Error;
//
//     fn try_from(event: XcbGenericEvent) -> Result<Self> {
//         let resp = event.response_type();
//         if resp == xcb::KEY_PRESS as u8 {
//             let key_press: &xcb::x::KeyPressEvent = unsafe {
// xcb::cast_event(&e) };             Ok(key_press.into())
//         } else {
//             Err(Error::InvalidKeypress)
//         }
//     }
// }
//
// impl TryFrom<&xcb::Event> for XKeyCode {
//     type Error = Error;
//
//     fn try_from(event: &xcb::Event) -> Result<Self> {
//         let resp = event.response_type();
//         if resp == xcb::KEY_PRESS as u8 {
//             let key_press: &xcb::x::KeyPressEvent = unsafe {
// xcb::base::cast_event(e) };             Ok(key_press.into())
//         } else {
//             Err(Error::InvalidKeypress)
//         }
//     }
// }

// AltMask        = (1 << 16),
// MetaMask       = (1 << 17),
// SuperMask      = (1 << 18),
// HyperMask      = (1 << 19),
// ModeSwitchMask = (1 << 20),
// NumLockMask    = (1 << 21),
// ScrollLockMask = (1 << 22),
// NoMask         = (1 << 25),

// TODO: Add hyper and meh

/// Builtin (real) modifiers available within XOrg. These modifiers can be set
/// to anything, which is why there is not an `Alt`, `Control_L`, `Hyper_R`, etc
/// available
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Copy, Clone)]
pub(crate) enum KeyModifier {
    None,
    Shift,
    Lock,
    Ctrl,
    Mod1, // alt
    Mod2, // num_lock
    Mod3,
    Mod4, // super
    Mod5, // iso_level3
    Any,
}

impl KeyModifier {
    /// Determine if the modifier key was held
    pub(crate) fn was_held(self, mask: u16) -> bool {
        mask & u16::from(self) > 0
    }
}

impl FromStr for KeyModifier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().trim() {
            "shift" => Self::Shift,
            "lock" => Self::Lock,
            "ctrl" | "control" => Self::Ctrl,
            "mod1" => Self::Mod1,
            "mod2" => Self::Mod2,
            "mod3" => Self::Mod3,
            "mod4" => Self::Mod4,
            "mod5" => Self::Mod5,
            "any" => Self::Any,
            _ => Self::None,
        })
    }
}

impl From<KeyModifier> for u16 {
    // NumLock = Mod2
    fn from(modifier: KeyModifier) -> u16 {
        u16::from(match modifier {
            KeyModifier::Shift => ModMask::SHIFT,  // 0x01 1
            KeyModifier::Lock => ModMask::LOCK,    // 0x02 2
            KeyModifier::Ctrl => ModMask::CONTROL, // 0x04 4
            KeyModifier::Mod1 => ModMask::M1,      // 0x08 8
            KeyModifier::Mod2 => ModMask::M2,      // 0x10 16
            KeyModifier::Mod3 => ModMask::M3,      // 0x20 32
            KeyModifier::Mod4 => ModMask::M4,      // 0x40 64
            KeyModifier::Mod5 => ModMask::M5,      // 0x80 128
            KeyModifier::Any => ModMask::ANY,      // 0x8000
            KeyModifier::None => ModMask::from(0_u16),
        })
    }
}

// TODO: Create something for Mods

/// Represents an individual character/keypress. If it is found within an
/// `XModmap` output of `xmodmap -pke`, then it will be in here.
///
/// This means that `Control_L`, `Scroll_Lock`, `Caps_Lock`, etc. (virtual
/// modifiers) will be contained. As well as the real modifiers which contain
/// `mod1` - `mod5`, `control` (with no suffix), etc. The real modifiers are
/// what is shown when `xmodmap` is used with no arguments.
///
/// There is no way to determine for sure what someone's keyboard layout will
/// be, so hardcoding `mod4` as `super` isn't the correct thing to do. Instead,
/// since `mod4` is represented the same regardless of which key it is mapped
/// to, we will scan the built `CharacterMap` and find the corresponding key
/// that has the same `modmask` and set everything else the same
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct CharacterMap {
    /// The UTF-8 representation of the key. E.g., `Hyper_L`
    pub(crate) utf:     String,
    /// The code of the physical key on the keyboard this key is on
    pub(crate) code:    Keycode,
    /// The modifiers to apply to this key
    pub(crate) modmask: u16,
    /// The symbol that represents this key after applying modifiers
    pub(crate) symbol:  Keysym,
    /// The level the key is in
    ///  - ONE_LEVEL = doesn't depend on modifiers
    ///  - TWO_LEVEL = depends on `Shift` modifier (`Lock` doesn't influence)
    ///  - THREE_LEVEL = depends on `Shift` modifier + another (not `Lock`)
    ///  - FOUR_LEVEL = `Shift` + another not in THREE (not `Lock`)
    pub(crate) level:   u8,
    /// The group that this key is in
    pub(crate) group:   u16,
}

impl CharacterMap {
    /// Generate a new `CharacterMap`
    pub(crate) fn new(
        keysym_str: String,
        keycode: Keycode,
        mask: u16,
        keysym: Keysym,
        level: u8,
        group: u8,
    ) -> Self {
        Self {
            utf: keysym_str,
            code: keycode,
            modmask: mask,
            symbol: keysym,
            level,
            group: u16::from(group),
        }
    }

    /// Generate a new `CharacterMap`
    pub(crate) fn new1(
        keysym: Keysym,
        keycode: Keycode,
        mask: u16,
        level: u8,
        group: u8,
    ) -> Result<Self> {
        let hash = KeysymHash::HASH;

        Ok(Self {
            utf: hash
                .get_keysym(keysym)
                .ok_or(Error::LookupKeysymHash(keysym))?
                .to_string(),
            code: keycode,
            modmask: mask,
            symbol: keysym,
            level,
            group: u16::from(group),
        })
    }

    /// Return the `CharacterMap` corresponding to the given `Keysym`
    pub(crate) fn charcodemap_from_keysym(charmaps: &[Self], keysym: Keysym) -> Self {
        if let Some(map) = charmaps.iter().find(|c| c.symbol == keysym) {
            map.clone()
        } else {
            Self {
                utf:     String::from("<null>"),
                code:    0,
                modmask: 0,
                symbol:  keysym,
                level:   0,
                group:   0,
            }
        }
    }
}

/// Get the `ModMask` of a `Keycode`
pub(crate) fn get_keycode_modifier(modmap: &[KeyModMap], keycode: Keycode) -> u8 {
    if let Some(m) = modmap.iter().find(|m| m.keycode == keycode) {
        m.mods
    } else {
        0
    }
}
