//! Keycodes, Modmask, and Mouse Buttons

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
use crate::parse::parser::{Token, TokenizedLine};
use anyhow::{Context, Result};
use colored::Colorize;
use indexmap::IndexMap;
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
    properties,
    protocol::{
        self,
        xkb::{self, ConnectionExt as _, GetMapReply, KeyModMap, MapPart, ID},
        xproto::{
            self,
            Button,
            ButtonIndex,
            ButtonMask,
            ButtonPressEvent,
            ButtonReleaseEvent,
            ConnectionExt,
            EventMask,
            KeyPressEvent,
            KeyReleaseEvent,
            Keycode,
            Keysym,
            ModMask,
            MotionNotifyEvent,
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
    #[error("failed to lookup keysym {0:?}")]
    InvalidButton(Button),
}

// ================== Keycode =====================

// TODO: Allow mouse buttons
// TODO: Add hyper and meh

/// A key press (code) and the held modifiers
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct XKeyCode {
    /// The held mask of the modifiers
    pub(crate) mask: ModifierMask,
    /// The key code that was held
    pub(crate) code: Keycode,
}

impl XKeyCode {
    /// Convert `XKeyCode` to a `CharacterMap`
    pub(crate) fn to_charmap(self, charmaps: &[CharacterMap]) -> Option<CharacterMap> {
        charmaps.iter().find(|c| self.code == c.code).cloned()
    }
}

impl From<CharacterMap> for XKeyCode {
    fn from(charmap: CharacterMap) -> Self {
        Self { mask: ModifierMask::new(charmap.modmask), code: charmap.code }
    }
}

impl From<KeyPressEvent> for XKeyCode {
    fn from(event: KeyPressEvent) -> Self {
        Self { mask: ModifierMask::new(event.state), code: event.detail }
    }
}

impl From<&KeyPressEvent> for XKeyCode {
    fn from(event: &KeyPressEvent) -> Self {
        Self { mask: ModifierMask::new(event.state), code: event.detail }
    }
}

impl fmt::Display for XKeyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "code: {} mask: {}", self.code, self.mask)
    }
}

// ================== Modmask =====================

/// Builtin (real) modifiers available within XOrg. These modifiers can be set
/// to anything, which is why there is not an `Alt`, `Control_L`, `Hyper_R`, etc
/// available.
///
/// `bool`s are needed here to combine multiple modifiers
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Default, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub(crate) struct ModifierMask {
    mask: u16,
}

impl ModifierMask {
    /// Create a new `ModifierMask`
    pub(crate) const fn new(mask: u16) -> Self {
        Self { mask }
    }

    /// Return the inner `mask`
    pub(crate) const fn mask(self) -> u16 {
        self.mask
    }

    /// Combine masks
    pub(crate) fn combine_u16(&mut self, other: u16) {
        self.mask |= other;
    }

    /// Combine masks
    pub(crate) fn combine_modmask(&mut self, other: ModifierMask) {
        self.mask |= other.mask;
    }

    /// Ignore modifiers
    pub(crate) fn ignore(&mut self, mask: u16) {
        self.mask &= !(mask);
    }

    /// Filter `lock` modifiers
    pub(crate) fn filter_ignored(&mut self) {
        self.mask &= !(u16::from(ModMask::LOCK | ModMask::M2));
    }

    /// Return modifiers to ignore when grabbign keys
    pub(crate) fn return_ignored(mask: ModifierMask) -> [ModifierMask; 4] {
        let mut iter = [mask, mask, mask, mask];

        // Modifier with the `Lock` mask
        iter[1].combine_u16(u16::from(ModMask::LOCK));
        // `Num_Lock` is sometimes (most of the time) set to `mod2`
        iter[2].combine_u16(u16::from(ModMask::M2));
        // Some applications register the lock as `Lock` and `mod2` together
        iter[3].combine_u16(u16::from(ModMask::LOCK | ModMask::M2));

        iter
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

impl From<u16> for ModifierMask {
    fn from(mask: u16) -> ModifierMask {
        ModifierMask::new(mask)
    }
}

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
        ModifierMask { mask: self.mask ^ rhs.mask }
    }
}

impl BitAnd for ModifierMask {
    type Output = ModifierMask;

    fn bitand(self, rhs: ModifierMask) -> ModifierMask {
        ModifierMask { mask: self.mask & rhs.mask }
    }
}

impl BitOr for ModifierMask {
    type Output = ModifierMask;

    fn bitor(self, rhs: ModifierMask) -> ModifierMask {
        ModifierMask { mask: self.mask | rhs.mask }
    }
}

impl FromStr for ModifierMask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "shift" => Ok(Self { mask: ModMask::SHIFT.into() }),
            "lock" => Ok(Self { mask: ModMask::LOCK.into() }),
            "control" | "ctrl" => Ok(Self { mask: ModMask::CONTROL.into() }),
            "mod1" => Ok(Self { mask: ModMask::M1.into() }),
            "mod2" => Ok(Self { mask: ModMask::M2.into() }),
            "mod3" => Ok(Self { mask: ModMask::M3.into() }),
            "mod4" => Ok(Self { mask: ModMask::M4.into() }),
            "mod5" => Ok(Self { mask: ModMask::M5.into() }),
            "any" => Ok(Self { mask: ModMask::ANY.into() }),
            _ => Ok(Self { mask: 0 }),
        }
    }
}

impl fmt::Display for ModifierMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.mask)
    }
}

// ================ Button Code ===================

// /// The available buttons on a mouse. This is more easily represented as an
// /// enum, as compared to the `XKeyCode`, which has it's own hash table to
// lookup /// based on codes
// #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
// pub(crate) enum MouseButton {
//     Left,       // 1
//     Middle,     // 2
//     Right,      // 3
//     ScrollUp,   // 4
//     ScrollDown, // 5
// }
//
// impl From<MouseButton> for u8 {
//     fn from(b: MouseButton) -> u8 {
//         match b {
//             MouseButton::Left => 1,
//             MouseButton::Middle => 2,
//             MouseButton::Right => 3,
//             MouseButton::ScrollUp => 4,
//             MouseButton::ScrollDown => 5,
//         }
//     }
// }

// ================ Button Code ===================

/// A wrapper around a `Button`, which is a type for `u8`
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) struct ButtonCode(u8);

impl fmt::Display for ButtonCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u8> for ButtonCode {
    fn from(b: u8) -> ButtonCode {
        ButtonCode(b)
    }
}

impl From<ButtonCode> for u8 {
    fn from(b: ButtonCode) -> u8 {
        b.0
    }
}

impl FromStr for ButtonCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "left" | "mouse1" => Ok(Self(1)),
            "middle" | "mouse2" => Ok(Self(2)),
            "right" | "mouse3" => Ok(Self(3)),
            "scrollup" | "mouse4" => Ok(Self(4)),
            "scrolldown" | "mouse5" => Ok(Self(5)),
            _ => Ok(Self(0)),
        }
    }
}

// ================== Buttons =====================

/// A button press (code) on a mouse and the held modifiers
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) struct XButton {
    /// The held mask of the modifiers (TODO: Does this need to be ButtonMask?)
    mask: ModifierMask,
    /// The code of the button that was pressed
    code: ButtonCode,
}

impl XButton {
    /// Create a new `XButton`
    pub(crate) fn new(mask: ModifierMask, code: ButtonCode) -> Self {
        Self { mask, code }
    }

    /// Return the `ButtonCode`
    pub(crate) fn code(self) -> u8 {
        self.code.into()
    }

    /// Return the `ModifierMask`
    pub(crate) fn modmask(self) -> ModifierMask {
        self.mask
    }
}

impl From<ButtonPressEvent> for XButton {
    fn from(event: ButtonPressEvent) -> Self {
        Self {
            mask: ModifierMask::new(event.state),
            code: ButtonCode(event.detail),
        }
    }
}

impl From<&ButtonPressEvent> for XButton {
    fn from(event: &ButtonPressEvent) -> Self {
        Self {
            mask: ModifierMask::new(event.state),
            code: ButtonCode(event.detail),
        }
    }
}

impl fmt::Display for XButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "code: {} mask: {}", self.code, self.mask)
    }
}

// ================== Key Info ====================

// MouseKeys:  using the numeric pad keys to move the mouse;
// StickyKeys: where modifiers will lock until the next key press
// SlowKeys:   Have to be pressed for certain amout of time
// BounceKeys: If pressed more than once in certain time, only on registers

// ================ CharacterMap ==================

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
#[derive(Debug, Clone, Eq, PartialEq, Default)]
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

    /// Return the `CharacterMap` corresponding to the given `Keysym` code. For
    /// example, in the `KeysymHash` `Hyper_L` is defined as `0xffed`, which is
    /// what is required for this function. If nothing is found, a blank
    /// `CharacterMap` is returned
    pub(crate) fn charmap_from_keysym_code_or_null(charmaps: &[Self], keysym: Keysym) -> Self {
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

    /// Return a blank `CharacterMap` for mouse button events
    pub(crate) fn blank_charmap(name: &str) -> Self {
        Self {
            utf:     String::from(name),
            code:    0,
            modmask: 0,
            symbol:  0,
            level:   0,
            vmod:    0,
            group:   0,
        }
    }

    /// Return the `CharacterMap` corresponding to the `Keysym` code
    pub(crate) fn charmap_from_keysym_code(charmaps: &[Self], keysym: Keysym) -> Option<Self> {
        charmaps.iter().find(|c| c.symbol == keysym).cloned()
    }

    /// Return the `CharacterMap` corresponding to the `Keysym` UTF-8
    /// representation
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

    // /// Return a vector of `CharacterMap`s from a flattened `TokenizedLine`
    // pub(crate) fn charmap_hash_from_flatoke<'a>(charmaps: &'a [Self], line:
    // Vec<&Token>) -> IndexMap<Keycode, Self> {     let mut indexmap =
    // IndexMap::new();
    //
    //     println!("LINE: {:#?}", line);
    //
    //     indexmap
    // }
}

// ================ Helper Funcs ==================

/// Get the `ModMask` of a `Keycode` based on the set modifiers found in
/// [`KeyModMap`](x11rb::protocol::KeyModMap)
pub(super) fn get_modmask_from_keycode(modmap: &[KeyModMap], keycode: Keycode) -> u8 {
    modmap
        .iter()
        .find(|m| m.keycode == keycode)
        .map_or(0, |m| m.mods)
}
