// TODO: Allow mouse buttons

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt, str::FromStr};

use x11rb::{
    connection::{Connection, RequestConnection},
    protocol::{
        xkb::GetMapReply,
        xproto::{
            ButtonPressEvent, ButtonReleaseEvent, KeyPressEvent, Keycode, Keysym, ModMask,
            MotionNotifyEvent,
        },
    },
    rust_connection::RustConnection,
    wrapper::ConnectionExt as _,
};

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
    pub(crate) fn ignore_modifier(&self, mask: ModMask) -> Self {
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

// TODO: Add hyper and meh

/// Builtin modifiers available
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Copy, Clone)]
pub(crate) enum KeyModifier {
    None,
    Shift,
    Lock,
    Ctrl,
    // Alt,
    Mod1, // alt
    Mod2, // num_lock
    Mod3,
    // Super,
    Mod4, // super
    Mod5, // iso_level3
    Any,
}

impl KeyModifier {
    /// Determine if the modifier key was held
    pub(crate) fn was_held(&self, mask: u16) -> bool {
        mask & u16::from(*self) > 0
    }
}

impl FromStr for KeyModifier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().trim() {
            "shift" => Self::Shift,
            "lock" => Self::Lock,
            "ctrl" => Self::Ctrl,
            "mod1" | "alt" => Self::Mod1,
            "mod2" => Self::Mod2,
            "mod3" => Self::Mod3,
            "mod4" | "super" | "meta" => Self::Mod4,
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
            KeyModifier::Shift => ModMask::SHIFT,
            KeyModifier::Lock => ModMask::LOCK,
            KeyModifier::Ctrl => ModMask::CONTROL,
            KeyModifier::Mod1 => ModMask::M1,
            KeyModifier::Mod2 => ModMask::M2,
            KeyModifier::Mod3 => ModMask::M3,
            KeyModifier::Mod4 => ModMask::M4,
            KeyModifier::Mod5 => ModMask::M5,
            KeyModifier::Any => ModMask::ANY,
            KeyModifier::None => ModMask::from(0_u16),
        })
    }
}

use x11rb::{
    connection,
    errors::ReplyError,
    properties, protocol,
    protocol::{
        xkb::{self, ConnectionExt as _, KeyModMap, MapPart, ID},
        xproto::{self, ConnectionExt, EventMask},
        xtest,
    },
};

/// Represents and individual character/keypress
#[derive(Debug, Clone)]
pub(crate) struct CharacterMap {
    /// The UTF-8 representation of the key (if possible)
    key:           char,
    /// The code of the physical key on the keyboard this key is on
    code:          Keycode,
    /// The modifiers to apply to this key
    modmask:       u16,
    /// The symbol that represents this key after applying modifiers
    symbol:        Keysym,
    /// The group that this key is in
    group:         u16,
    /// If the key needs to be bound because there is no physical key
    needs_binding: bool,
}
