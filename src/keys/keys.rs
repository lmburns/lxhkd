// TODO: Allow mouse buttons

// Keysyms
//  - https://wiki.linuxquestions.org/wiki/List_of_Keysyms_Recognised_by_Xmodmap
// XOrg xkb proto Documentation
//  - https://www.x.org/releases/X11R7.7/doc/kbproto/xkbproto.html
//
// Just about same info
//
// XOrg xkb lib Documentation
//  - https://www.x.org/releases/X11R7.6/doc/libX11/specs/XKB/xkblib.html

use super::keysym::KeysymHash;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt,
    ops::{BitAnd, BitOr, BitXor},
    str::FromStr,
};
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

pub(crate) type KeycodeMask = u16;

/// A key press (code) and the held modifiers
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct XKeyCode {
    /// The held mask of the modifiers
    pub(crate) mask: KeycodeMask,
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

    /// Convert `XKeyCode` to a `CharacterMap`
    pub(crate) fn to_charmap(self, charmaps: &[CharacterMap]) -> Option<CharacterMap> {
        charmaps.iter().find(|c| self.code == c.code).cloned()
    }
}

impl From<CharacterMap> for XKeyCode {
    fn from(charmap: CharacterMap) -> XKeyCode {
        XKeyCode {
            mask: charmap.modmask,
            code: charmap.code,
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

/////////////////////////

/// Builtin (real) modifiers available within XOrg. These modifiers can be set
/// to anything, which is why there is not an `Alt`, `Control_L`, `Hyper_R`, etc
/// available.
///
/// `bool`s are needed here to combine multiple modifiers
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash, Serialize, Deserialize)]
pub(crate) struct ModifierMask {
    mask: u16,
}

impl ModifierMask {
    /// Create a new `ModifierMask`
    pub(crate) fn new(mask: u16) -> Self {
        Self { mask }
    }

    /// Return the inner `mask`
    pub(crate) fn mask(self) -> u16 {
        self.mask
    }

    /// Combine masks
    pub(crate) fn combine(&mut self, other: ModifierMask) {
        self.mask |= other.mask;
    }

    /// Filter `lock` modifiers
    pub(crate) fn filter_igored(&mut self) {
        self.mask &= !(u16::from(ModMask::LOCK | ModMask::M2));
    }

    /// Determine if the `ModifierMask` contains a `ctrl` modifier
    pub(crate) fn has_ctrl(self) -> bool {
        (self.mask & u16::from(ModMask::CONTROL)) != 0
    }

    /// Determine if the `ModifierMask` contains a `shift` modifier
    pub(crate) fn has_shift(self) -> bool {
        (self.mask & u16::from(ModMask::SHIFT)) != 0
    }

    /// Determine if the `ModifierMask` contains a `lock` modifier
    pub(crate) fn has_lock(self) -> bool {
        (self.mask & u16::from(ModMask::LOCK)) != 0
    }

    /// Determine if the `ModifierMask` contains a `mod1` modifier
    pub(crate) fn has_mod1(self) -> bool {
        (self.mask & u16::from(ModMask::M1)) != 0
    }

    /// Determine if the `ModifierMask` contains a `mod2` modifier
    pub(crate) fn has_mod2(self) -> bool {
        (self.mask & u16::from(ModMask::M2)) != 0
    }

    /// Determine if the `ModifierMask` contains a `mod3` modifier
    pub(crate) fn has_mod3(self) -> bool {
        (self.mask & u16::from(ModMask::M3)) != 0
    }

    /// Determine if the `ModifierMask` contains a `mod4` modifier
    pub(crate) fn has_mod4(self) -> bool {
        (self.mask & u16::from(ModMask::M4)) != 0
    }

    /// Determine if the `ModifierMask` contains a `mod5` modifier
    pub(crate) fn has_mod5(self) -> bool {
        (self.mask & u16::from(ModMask::M5)) != 0
    }

    /// Determine if the `ModifierMask` contains a `any` modifier
    pub(crate) fn has_mod_any(self) -> bool {
        (self.mask & u16::from(ModMask::ANY)) != 0
    }

    /// Determine if the `ModifierMask` contains the given modifier
    pub(crate) fn has_mod(self, other: u16) -> bool {
        (self.mask & other) != 0
    }

    /// Clear a modifier
    pub(crate) fn clear_mod(&mut self, other: u16) {
        self.mask &= !other;
    }

    // TODO: has_none, has_any, has_all
}
// AltMask        = (1 << 16),
// MetaMask       = (1 << 17),
// SuperMask      = (1 << 18),
// HyperMask      = (1 << 19),
// ModeSwitchMask = (1 << 20),
// NumLockMask    = (1 << 21),
// ScrollLockMask = (1 << 22),
// NoMask         = (1 << 25),

// Hotkey_Flag_Alt = (1 << 0),
// Hotkey_Flag_LAlt = (1 << 1),
// Hotkey_Flag_RAlt = (1 << 2),
// Hotkey_Flag_Shift = (1 << 3),
// Hotkey_Flag_LShift = (1 << 4),
// Hotkey_Flag_RShift = (1 << 5),
// Hotkey_Flag_Cmd = (1 << 6),
// Hotkey_Flag_LCmd = (1 << 7),
// Hotkey_Flag_RCmd = (1 << 8),
// Hotkey_Flag_Control = (1 << 9),
// Hotkey_Flag_LControl = (1 << 10),
// Hotkey_Flag_RControl = (1 << 11),
// Hotkey_Flag_Fn = (1 << 12),
// Hotkey_Flag_Passthrough = (1 << 13),
// Hotkey_Flag_Activate = (1 << 14),
// Hotkey_Flag_NX = (1 << 15),
// Hotkey_Flag_Hyper = (Hotkey_Flag_Cmd | Hotkey_Flag_Alt | Hotkey_Flag_Shift |
//                      Hotkey_Flag_Control),
// Hotkey_Flag_Meh = (Hotkey_Flag_Control | Hotkey_Flag_Shift | Hotkey_Flag_Alt)

impl From<ModifierMask> for u16 {
    fn from(mask: ModifierMask) -> u16 {
        mask.mask
    }
}

impl PartialEq<ModifierMask> for u16 {
    fn eq(&self, rhs: &ModifierMask) -> bool {
        *self == rhs.mask
    }
}

impl PartialEq<u16> for ModifierMask {
    fn eq(&self, rhs: &u16) -> bool {
        self.mask == *rhs
    }
}

impl BitXor for ModifierMask {
    type Output = ModifierMask;

    fn bitxor(self, rhs: ModifierMask) -> ModifierMask {
        ModifierMask {
            mask: self.mask ^ rhs.mask,
        }
    }
}

impl BitAnd for ModifierMask {
    type Output = ModifierMask;

    fn bitand(self, rhs: ModifierMask) -> ModifierMask {
        ModifierMask {
            mask: self.mask & rhs.mask,
        }
    }
}

impl BitOr for ModifierMask {
    type Output = ModifierMask;

    fn bitor(self, rhs: ModifierMask) -> ModifierMask {
        ModifierMask {
            mask: self.mask | rhs.mask,
        }
    }
}

///////////////////////

// TODO: Add hyper and meh

/// Builtin (real) modifiers available within XOrg. These modifiers can be set
/// to anything, which is why there is not an `Alt`, `Control_L`, `Hyper_R`, etc
/// available.
///
/// `bool`s are needed here to combine multiple modifiers
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Hash)]
pub(crate) struct KeyModifier {
    pub(crate) shift: bool,
    pub(crate) lock:  bool,
    pub(crate) ctrl:  bool,
    pub(crate) mod1:  bool, // alt
    pub(crate) mod2:  bool, // num_lock
    pub(crate) mod3:  bool,
    pub(crate) mod4:  bool, // super
    pub(crate) mod5:  bool, // iso_level3
    pub(crate) any:   bool,
}

// impl KeyModifier {
//     /// Determine if the modifier key was held
//     pub(crate) fn was_held(self, mask: u16) -> bool {
//         mask & u16::from(self) > 0
//     }
//
//     /// Combine masks
//     pub(crate) fn combine(&mut self, other: Self) {
//         self.inner = ModMask(self.inner.0 as KeyCodeMask | other.inner.0 as
// KeyCodeMask);     }
// }
//
// impl From<CharacterMap> for KeyModifier {
//     fn from(charmap: CharacterMap) -> KeyModifier {
//         KeyModifier::from(charmap.modmask)
//     }
// }
//
// impl FromStr for KeyModifier {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(match s.to_ascii_lowercase().trim() {
//             "shift" => Self::Shift,
//             "lock" => Self::Lock,
//             "ctrl" | "control" => Self::Ctrl,
//             "mod1" => Self::Mod1,
//             "mod2" => Self::Mod2,
//             "mod3" => Self::Mod3,
//             "mod4" => Self::Mod4,
//             "mod5" => Self::Mod5,
//             "any" => Self::Any,
//             _ => Self::None,
//         })
//     }
// }

// impl From<KeyModifier> for u16 {
//     fn from(modifier: KeyModifier) -> u16 {
//         let mut mask = 0;
//
//         if modifier.shift
//     }
// }

// impl From<u16> for KeyModifier {
//     // NumLock = Mod2
//     fn from(mask: u16) -> KeyModifier {
//         match mask {
//             s if s == u16::from(ModMask::SHIFT) => KeyModifier::Shift,
//             s if s == u16::from(ModMask::LOCK) => KeyModifier::Lock,
//             s if s == u16::from(ModMask::CONTROL) => KeyModifier::Ctrl,
//             s if s == u16::from(ModMask::M1) => KeyModifier::Mod1,
//             s if s == u16::from(ModMask::M2) => KeyModifier::Mod2,
//             s if s == u16::from(ModMask::M3) => KeyModifier::Mod3,
//             s if s == u16::from(ModMask::M4) => KeyModifier::Mod4,
//             s if s == u16::from(ModMask::M5) => KeyModifier::Mod5,
//             s if s == u16::from(ModMask::ANY) => KeyModifier::Any,
//             _ => KeyModifier::None,
//         }
//     }
// }

// /// The UTF-8 representation of a key. This is used with the configuration
// file and allows for the conversion of something like `Hyper_L` to its
// corresponding `Keycode` `0xffed` #[derive(Debug, PartialEq, Eq, Hash, Clone,
// Copy)] pub(crate) struct Key {
//     /// The modifiers in `String` format
//     pub(crate) modifiers: KeyModifier,
//     /// The `Keysym` in `String` format
//     pub(crate) keysym:
// }

// TODO: Create something for Mods

/// Represents an individual character/keypress. If it is found within an
/// `XModmap` output of `xmodmap -pke`, then it will be in here.
///
/// This means that `Control_L`, `Scroll_Lock`, `Caps_Lock`, etc. (virtual
/// modifiers) will be contained. ~~As well as the real modifiers which contain
/// `mod1` - `mod5`, `control` (with no suffix), etc~~. The real modifiers are
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
    ///  - ONE_LEVEL: does not depend on a any modifiers
    ///  - TWO_LEVEL: depends on `Shift` modifier (`Lock` doesn't affect)
    ///  - THREE_LEVEL: `Shift` + extra modifier (`Lock` doesn't affect)
    ///  - FOUR_LEVEL: `Shift` + extra modifier not found in `THREE_LEVEL`
    pub(crate) level:   u8,
    /// The virtual modifiiers of a key. If it is not a modifier then this field
    /// is 0. This field may not be needed
    pub(crate) vmod:    u16,
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
        vmod: u16,
        group: u8,
    ) -> Self {
        Self {
            utf: keysym_str,
            code: keycode,
            modmask: mask,
            symbol: keysym,
            level,
            vmod,
            group: u16::from(group),
        }
    }

    /// Generate a new `CharacterMap`
    pub(crate) fn new1(
        keysym: Keysym,
        keycode: Keycode,
        mask: u16,
        level: u8,
        vmod: u16,
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
            vmod,
            group: u16::from(group),
        })
    }

    /// Return the `CharacterMap` corresponding to the given `Keysym` code. For
    /// example, in the `KeysymHash` `Hyper_L` is defined as `0xffed`, which is
    /// what is required for this function
    pub(crate) fn charmap_from_keysym_code(charmaps: &[Self], keysym: Keysym) -> Self {
        if let Some(map) = charmaps.iter().find(|c| c.symbol == keysym) {
            map.clone()
        } else {
            Self {
                utf:     String::from("<null>"),
                code:    0,
                modmask: 0,
                symbol:  keysym,
                level:   0,
                vmod:    0,
                group:   0,
            }
        }
    }

    /// Return the `CharacterMap` based on the `Keysym` UTF-8 representation
    pub(crate) fn charmap_from_keysym_utf(charmaps: &[Self], utf: &str) -> Option<Self> {
        charmaps.iter().find(|c| c.utf == utf).cloned()
    }

    /// Return the `CharacterMap` corresponding to the `Keycode` given
    pub(crate) fn charmap_from_keycode(charmaps: &[Self], keycode: Keycode) -> Option<Self> {
        charmaps.iter().find(|c| c.code == keycode).cloned()
    }

    /// Return the `CharacterMap` corresponding to the `XKeyCode` given
    pub(crate) fn charmap_from_xkeycode(charmaps: &[Self], keycode: XKeyCode) -> Option<Self> {
        charmaps.iter().find(|c| c.code == keycode.code).cloned()
    }
}

/// Get the `ModMask` of a `Keycode` based on the set modifiers found in
/// [`KeyModMap`](x11rb::protocol::KeyModMap)
pub(crate) fn get_modmask_from_keycode(modmap: &[KeyModMap], keycode: Keycode) -> u8 {
    modmap
        .iter()
        .find(|m| m.keycode == keycode)
        .map_or(0, |m| m.mods)
}
