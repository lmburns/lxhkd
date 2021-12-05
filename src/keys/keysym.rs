//! [`Keysym`](x11rb::protocol::xproto::Keysym) wrapper
//! KeysymHash with all keycodes and keysyms in a `BiMap`

use super::keys::CharacterMap;
use anyhow::{Context, Result};
use bimap::BiMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt};
use thiserror::Error;
use x11rb::protocol::xproto::Keysym;
use xkbcommon::xkb;

// ================== Keysym ======================

/// A [`Keysym`](x11rb::protocol::xproto::Keysym) wrapper
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Serialize, Deserialize)]
pub(crate) struct XKeysym(pub(crate) Keysym);

impl XKeysym {
    /// Create a new instance of `XKeysym` from a [`Keysym`](xkb::Keysym)
    pub(crate) fn new(inner: Keysym) -> Self {
        Self(inner)
    }
}

impl From<CharacterMap> for XKeysym {
    fn from(charmap: CharacterMap) -> XKeysym {
        XKeysym(charmap.symbol)
    }
}

impl From<Keysym> for XKeysym {
    fn from(inner: Keysym) -> XKeysym {
        XKeysym(inner)
    }
}

impl Ord for XKeysym {
    fn cmp(&self, other: &XKeysym) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for XKeysym {
    fn partial_cmp(&self, other: &XKeysym) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for XKeysym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ================== Unicode =====================

/// A pair holding the unicode code-point, as well as the `Keysym` of the key
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Unicode {
    keysym:  Keysym,
    unicode: char,
}

impl Unicode {
    /// Create a new instance of a `Unicode` code-pair
    pub(crate) fn new(keysym: Keysym, unicode: char) -> Self {
        Self { keysym, unicode }
    }
}

// ============== KeysymHash Errors ================

/// Errors used for `KeysymHash`
#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("{0} was not found in the database")]
    InvalidKey(String),

    #[error("failed to convert {0} to UTF-8")]
    Utf8Conversion(String),
}

//================== KeysymHash ===================

// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

// Taken from the `x11` crate and modified for serialization

/// Hash of available keymaps
pub(crate) struct KeysymHash(Lazy<BiMap<String, Unicode>>);

impl fmt::Debug for KeysymHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}

impl KeysymHash {
    #[allow(clippy::declare_interior_mutable_const)]
    pub(crate) const HASH: Self = Self(Lazy::new(|| {
        let mut hash = BiMap::new();

        hash.insert(
            String::from("BackSpace"),
            Unicode::new(xkb::KEY_BackSpace, '\u{8}'),
        ); // 0xFF08
        hash.insert(String::from("Tab"), Unicode::new(xkb::KEY_Tab, '\u{9}')); // 0xFF09
        hash.insert(
            String::from("Linefeed"),
            Unicode::new(xkb::KEY_Linefeed, '\u{a}'),
        ); // 0xFF0A
        hash.insert(String::from("Clear"), Unicode::new(xkb::KEY_Clear, '\u{b}')); // 0xFF0B
        hash.insert(
            String::from("Return"),
            Unicode::new(xkb::KEY_Return, '\u{d}'),
        ); // 0xFF0D
        hash.insert(
            String::from("Pause"),
            Unicode::new(xkb::KEY_Pause, '\u{13}'),
        ); // 0xFF13
        hash.insert(
            String::from("Scroll_Lock"),
            Unicode::new(xkb::KEY_Scroll_Lock, '\u{14}'),
        ); // 0xFF14
        hash.insert(
            String::from("Sys_Req"),
            Unicode::new(xkb::KEY_Sys_Req, '\u{15}'),
        ); // 0xFF15
        hash.insert(
            String::from("Escape"),
            Unicode::new(xkb::KEY_Escape, '\u{1b}'),
        ); // 0xFF1B
        hash.insert(
            String::from("Delete"),
            Unicode::new(xkb::KEY_Delete, '\u{0}'),
        ); // 0xFFFF
        hash.insert(
            String::from("Multi_key"),
            Unicode::new(xkb::KEY_Multi_key, '\u{0}'),
        ); // 0xFF20
        hash.insert(String::from("Kanji"), Unicode::new(xkb::KEY_Kanji, '\u{0}')); // 0xFF21
        hash.insert(
            String::from("Muhenkan"),
            Unicode::new(xkb::KEY_Muhenkan, '\u{0}'),
        ); // 0xFF22
        hash.insert(
            String::from("Henkan_Mode"),
            Unicode::new(xkb::KEY_Henkan_Mode, '\u{0}'),
        ); // 0xFF23
        hash.insert(
            String::from("Henkan"),
            Unicode::new(xkb::KEY_Henkan, '\u{0}'),
        ); // 0xFF23
        hash.insert(
            String::from("Romaji"),
            Unicode::new(xkb::KEY_Romaji, '\u{0}'),
        ); // 0xFF24
        hash.insert(
            String::from("Hiragana"),
            Unicode::new(xkb::KEY_Hiragana, '\u{0}'),
        ); // 0xFF25
        hash.insert(
            String::from("Katakana"),
            Unicode::new(xkb::KEY_Katakana, '\u{0}'),
        ); // 0xFF26
        hash.insert(
            String::from("Hiragana_Katakana"),
            Unicode::new(xkb::KEY_Hiragana_Katakana, '\u{0}'),
        ); // 0xFF27
        hash.insert(
            String::from("Zenkaku"),
            Unicode::new(xkb::KEY_Zenkaku, '\u{0}'),
        ); // 0xFF28
        hash.insert(
            String::from("Hankaku"),
            Unicode::new(xkb::KEY_Hankaku, '\u{0}'),
        ); // 0xFF29
        hash.insert(
            String::from("Zenkaku_Hankaku"),
            Unicode::new(xkb::KEY_Zenkaku_Hankaku, '\u{0}'),
        ); // 0xFF2A
        hash.insert(
            String::from("Touroku"),
            Unicode::new(xkb::KEY_Touroku, '\u{0}'),
        ); // 0xFF2B
        hash.insert(
            String::from("Massyo"),
            Unicode::new(xkb::KEY_Massyo, '\u{0}'),
        ); // 0xFF2C
        hash.insert(
            String::from("Kana_Lock"),
            Unicode::new(xkb::KEY_Kana_Lock, '\u{0}'),
        ); // 0xFF2D
        hash.insert(
            String::from("Kana_Shift"),
            Unicode::new(xkb::KEY_Kana_Shift, '\u{0}'),
        ); // 0xFF2E
        hash.insert(
            String::from("Eisu_Shift"),
            Unicode::new(xkb::KEY_Eisu_Shift, '\u{0}'),
        ); // 0xFF2F
        hash.insert(
            String::from("Eisu_toggle"),
            Unicode::new(xkb::KEY_Eisu_toggle, '\u{0}'),
        ); // 0xFF30
        hash.insert(String::from("Home"), Unicode::new(xkb::KEY_Home, '\u{0}')); // 0xFF50
        hash.insert(String::from("Left"), Unicode::new(xkb::KEY_Left, '\u{0}')); // 0xFF51
        hash.insert(String::from("Up"), Unicode::new(xkb::KEY_Up, '\u{0}')); // 0xFF52
        hash.insert(String::from("Right"), Unicode::new(xkb::KEY_Right, '\u{0}')); // 0xFF53
        hash.insert(String::from("Down"), Unicode::new(xkb::KEY_Down, '\u{0}')); // 0xFF54
        hash.insert(String::from("Prior"), Unicode::new(xkb::KEY_Prior, '\u{0}')); // 0xFF55
        hash.insert(
            String::from("Page_Up"),
            Unicode::new(xkb::KEY_Page_Up, '\u{0}'),
        ); // 0xFF55
        hash.insert(String::from("Next"), Unicode::new(xkb::KEY_Next, '\u{0}')); // 0xFF56
        hash.insert(
            String::from("Page_Down"),
            Unicode::new(xkb::KEY_Page_Down, '\u{0}'),
        ); // 0xFF56
        hash.insert(String::from("End"), Unicode::new(xkb::KEY_End, '\u{0}')); // 0xFF57
        hash.insert(String::from("Begin"), Unicode::new(xkb::KEY_Begin, '\u{0}')); // 0xFF58
        hash.insert(
            String::from("Select"),
            Unicode::new(xkb::KEY_Select, '\u{0}'),
        ); // 0xFF60
        hash.insert(String::from("Print"), Unicode::new(xkb::KEY_Print, '\u{0}')); // 0xFF61
        hash.insert(
            String::from("Execute"),
            Unicode::new(xkb::KEY_Execute, '\u{0}'),
        ); // 0xFF62
        hash.insert(
            String::from("Insert"),
            Unicode::new(xkb::KEY_Insert, '\u{0}'),
        ); // 0xFF63
        hash.insert(String::from("Undo"), Unicode::new(xkb::KEY_Undo, '\u{0}')); // 0xFF65
        hash.insert(String::from("Redo"), Unicode::new(xkb::KEY_Redo, '\u{0}')); // 0xFF66
        hash.insert(String::from("Menu"), Unicode::new(xkb::KEY_Menu, '\u{0}')); // 0xFF67
        hash.insert(String::from("Find"), Unicode::new(xkb::KEY_Find, '\u{0}')); // 0xFF68
        hash.insert(
            String::from("Cancel"),
            Unicode::new(xkb::KEY_Cancel, '\u{0}'),
        ); // 0xFF69
        hash.insert(String::from("Help"), Unicode::new(xkb::KEY_Help, '\u{0}')); // 0xFF6A
        hash.insert(String::from("Break"), Unicode::new(xkb::KEY_Break, '\u{0}')); // 0xFF6B
        hash.insert(
            String::from("Mode_switch"),
            Unicode::new(xkb::KEY_Mode_switch, '\u{0}'),
        ); // 0xFF7E
        hash.insert(
            String::from("script_switch"),
            Unicode::new(xkb::KEY_script_switch, '\u{0}'),
        ); // 0xFF7E
        hash.insert(
            String::from("Num_Lock"),
            Unicode::new(xkb::KEY_Num_Lock, '\u{0}'),
        ); // 0xFF7F
        hash.insert(
            String::from("KP_Space"),
            Unicode::new(xkb::KEY_KP_Space, '\u{20}'),
        ); // 0xFF80
        hash.insert(
            String::from("KP_Tab"),
            Unicode::new(xkb::KEY_KP_Tab, '\u{9}'),
        ); // 0xFF89
        hash.insert(
            String::from("KP_Enter"),
            Unicode::new(xkb::KEY_KP_Enter, '\u{d}'),
        ); // 0xFF8D
        hash.insert(String::from("KP_F1"), Unicode::new(xkb::KEY_KP_F1, '\u{0}')); // 0xFF91
        hash.insert(String::from("KP_F2"), Unicode::new(xkb::KEY_KP_F2, '\u{0}')); // 0xFF92
        hash.insert(String::from("KP_F3"), Unicode::new(xkb::KEY_KP_F3, '\u{0}')); // 0xFF93
        hash.insert(String::from("KP_F4"), Unicode::new(xkb::KEY_KP_F4, '\u{0}')); // 0xFF94
        hash.insert(
            String::from("KP_Home"),
            Unicode::new(xkb::KEY_KP_Home, '\u{0}'),
        ); // 0xFF95
        hash.insert(
            String::from("KP_Left"),
            Unicode::new(xkb::KEY_KP_Left, '\u{0}'),
        ); // 0xFF96
        hash.insert(String::from("KP_Up"), Unicode::new(xkb::KEY_KP_Up, '\u{0}')); // 0xFF97
        hash.insert(
            String::from("KP_Right"),
            Unicode::new(xkb::KEY_KP_Right, '\u{0}'),
        ); // 0xFF98
        hash.insert(
            String::from("KP_Down"),
            Unicode::new(xkb::KEY_KP_Down, '\u{0}'),
        ); // 0xFF99
        hash.insert(
            String::from("KP_Prior"),
            Unicode::new(xkb::KEY_KP_Prior, '\u{0}'),
        ); // 0xFF9A
        hash.insert(
            String::from("KP_Page_Up"),
            Unicode::new(xkb::KEY_KP_Page_Up, '\u{0}'),
        ); // 0xFF9A
        hash.insert(
            String::from("KP_Next"),
            Unicode::new(xkb::KEY_KP_Next, '\u{0}'),
        ); // 0xFF9B
        hash.insert(
            String::from("KP_Page_Down"),
            Unicode::new(xkb::KEY_KP_Page_Down, '\u{0}'),
        ); // 0xFF9B
        hash.insert(
            String::from("KP_End"),
            Unicode::new(xkb::KEY_KP_End, '\u{0}'),
        ); // 0xFF9C
        hash.insert(
            String::from("KP_Begin"),
            Unicode::new(xkb::KEY_KP_Begin, '\u{0}'),
        ); // 0xFF9D
        hash.insert(
            String::from("KP_Insert"),
            Unicode::new(xkb::KEY_KP_Insert, '\u{0}'),
        ); // 0xFF9E
        hash.insert(
            String::from("KP_Delete"),
            Unicode::new(xkb::KEY_KP_Delete, '\u{0}'),
        ); // 0xFF9F
        hash.insert(
            String::from("KP_Equal"),
            Unicode::new(xkb::KEY_KP_Equal, '\u{3d}'),
        ); // 0xFFBD
        hash.insert(
            String::from("KP_Multiply"),
            Unicode::new(xkb::KEY_KP_Multiply, '\u{2a}'),
        ); // 0xFFAA
        hash.insert(
            String::from("KP_Add"),
            Unicode::new(xkb::KEY_KP_Add, '\u{2b}'),
        ); // 0xFFAB
        hash.insert(
            String::from("KP_Separator"),
            Unicode::new(xkb::KEY_KP_Separator, '\u{2c}'),
        ); // 0xFFAC
        hash.insert(
            String::from("KP_Subtract"),
            Unicode::new(xkb::KEY_KP_Subtract, '\u{2d}'),
        ); // 0xFFAD
        hash.insert(
            String::from("KP_Decimal"),
            Unicode::new(xkb::KEY_KP_Decimal, '\u{2e}'),
        ); // 0xFFAE
        hash.insert(
            String::from("KP_Divide"),
            Unicode::new(xkb::KEY_KP_Divide, '\u{2f}'),
        ); // 0xFFAF
        hash.insert(String::from("KP_0"), Unicode::new(xkb::KEY_KP_0, '\u{30}')); // 0xFFB0
        hash.insert(String::from("KP_1"), Unicode::new(xkb::KEY_KP_1, '\u{31}')); // 0xFFB1
        hash.insert(String::from("KP_2"), Unicode::new(xkb::KEY_KP_2, '\u{32}')); // 0xFFB2
        hash.insert(String::from("KP_3"), Unicode::new(xkb::KEY_KP_3, '\u{33}')); // 0xFFB3
        hash.insert(String::from("KP_4"), Unicode::new(xkb::KEY_KP_4, '\u{34}')); // 0xFFB4
        hash.insert(String::from("KP_5"), Unicode::new(xkb::KEY_KP_5, '\u{35}')); // 0xFFB5
        hash.insert(String::from("KP_6"), Unicode::new(xkb::KEY_KP_6, '\u{36}')); // 0xFFB6
        hash.insert(String::from("KP_7"), Unicode::new(xkb::KEY_KP_7, '\u{37}')); // 0xFFB7
        hash.insert(String::from("KP_8"), Unicode::new(xkb::KEY_KP_8, '\u{38}')); // 0xFFB8
        hash.insert(String::from("KP_9"), Unicode::new(xkb::KEY_KP_9, '\u{39}')); // 0xFFB9
        hash.insert(String::from("F1"), Unicode::new(xkb::KEY_F1, '\u{0}')); // 0xFFBE
        hash.insert(String::from("F2"), Unicode::new(xkb::KEY_F2, '\u{0}')); // 0xFFBF
        hash.insert(String::from("F3"), Unicode::new(xkb::KEY_F3, '\u{0}')); // 0xFFC0
        hash.insert(String::from("F4"), Unicode::new(xkb::KEY_F4, '\u{0}')); // 0xFFC1
        hash.insert(String::from("F5"), Unicode::new(xkb::KEY_F5, '\u{0}')); // 0xFFC2
        hash.insert(String::from("F6"), Unicode::new(xkb::KEY_F6, '\u{0}')); // 0xFFC3
        hash.insert(String::from("F7"), Unicode::new(xkb::KEY_F7, '\u{0}')); // 0xFFC4
        hash.insert(String::from("F8"), Unicode::new(xkb::KEY_F8, '\u{0}')); // 0xFFC5
        hash.insert(String::from("F9"), Unicode::new(xkb::KEY_F9, '\u{0}')); // 0xFFC6
        hash.insert(String::from("F10"), Unicode::new(xkb::KEY_F10, '\u{0}')); // 0xFFC7
        hash.insert(String::from("F11"), Unicode::new(xkb::KEY_F11, '\u{0}')); // 0xFFC8
        hash.insert(String::from("L1"), Unicode::new(xkb::KEY_L1, '\u{0}')); // 0xFFC8
        hash.insert(String::from("F12"), Unicode::new(xkb::KEY_F12, '\u{0}')); // 0xFFC9
        hash.insert(String::from("L2"), Unicode::new(xkb::KEY_L2, '\u{0}')); // 0xFFC9
        hash.insert(String::from("F13"), Unicode::new(xkb::KEY_F13, '\u{0}')); // 0xFFCA
        hash.insert(String::from("L3"), Unicode::new(xkb::KEY_L3, '\u{0}')); // 0xFFCA
        hash.insert(String::from("F14"), Unicode::new(xkb::KEY_F14, '\u{0}')); // 0xFFCB
        hash.insert(String::from("L4"), Unicode::new(xkb::KEY_L4, '\u{0}')); // 0xFFCB
        hash.insert(String::from("F15"), Unicode::new(xkb::KEY_F15, '\u{0}')); // 0xFFCC
        hash.insert(String::from("L5"), Unicode::new(xkb::KEY_L5, '\u{0}')); // 0xFFCC
        hash.insert(String::from("F16"), Unicode::new(xkb::KEY_F16, '\u{0}')); // 0xFFCD
        hash.insert(String::from("L6"), Unicode::new(xkb::KEY_L6, '\u{0}')); // 0xFFCD
        hash.insert(String::from("F17"), Unicode::new(xkb::KEY_F17, '\u{0}')); // 0xFFCE
        hash.insert(String::from("L7"), Unicode::new(xkb::KEY_L7, '\u{0}')); // 0xFFCE
        hash.insert(String::from("F18"), Unicode::new(xkb::KEY_F18, '\u{0}')); // 0xFFCF
        hash.insert(String::from("L8"), Unicode::new(xkb::KEY_L8, '\u{0}')); // 0xFFCF
        hash.insert(String::from("F19"), Unicode::new(xkb::KEY_F19, '\u{0}')); // 0xFFD0
        hash.insert(String::from("L9"), Unicode::new(xkb::KEY_L9, '\u{0}')); // 0xFFD0
        hash.insert(String::from("F20"), Unicode::new(xkb::KEY_F20, '\u{0}')); // 0xFFD1
        hash.insert(String::from("L10"), Unicode::new(xkb::KEY_L10, '\u{0}')); // 0xFFD1
        hash.insert(String::from("F21"), Unicode::new(xkb::KEY_F21, '\u{0}')); // 0xFFD2
        hash.insert(String::from("R1"), Unicode::new(xkb::KEY_R1, '\u{0}')); // 0xFFD2
        hash.insert(String::from("F22"), Unicode::new(xkb::KEY_F22, '\u{0}')); // 0xFFD3
        hash.insert(String::from("R2"), Unicode::new(xkb::KEY_R2, '\u{0}')); // 0xFFD3
        hash.insert(String::from("F23"), Unicode::new(xkb::KEY_F23, '\u{0}')); // 0xFFD4
        hash.insert(String::from("R3"), Unicode::new(xkb::KEY_R3, '\u{0}')); // 0xFFD4
        hash.insert(String::from("F24"), Unicode::new(xkb::KEY_F24, '\u{0}')); // 0xFFD5
        hash.insert(String::from("R4"), Unicode::new(xkb::KEY_R4, '\u{0}')); // 0xFFD5
        hash.insert(String::from("F25"), Unicode::new(xkb::KEY_F25, '\u{0}')); // 0xFFD6
        hash.insert(String::from("R5"), Unicode::new(xkb::KEY_R5, '\u{0}')); // 0xFFD6
        hash.insert(String::from("F26"), Unicode::new(xkb::KEY_F26, '\u{0}')); // 0xFFD7
        hash.insert(String::from("R6"), Unicode::new(xkb::KEY_R6, '\u{0}')); // 0xFFD7
        hash.insert(String::from("F27"), Unicode::new(xkb::KEY_F27, '\u{0}')); // 0xFFD8
        hash.insert(String::from("R7"), Unicode::new(xkb::KEY_R7, '\u{0}')); // 0xFFD8
        hash.insert(String::from("F28"), Unicode::new(xkb::KEY_F28, '\u{0}')); // 0xFFD9
        hash.insert(String::from("R8"), Unicode::new(xkb::KEY_R8, '\u{0}')); // 0xFFD9
        hash.insert(String::from("F29"), Unicode::new(xkb::KEY_F29, '\u{0}')); // 0xFFDA
        hash.insert(String::from("R9"), Unicode::new(xkb::KEY_R9, '\u{0}')); // 0xFFDA
        hash.insert(String::from("F30"), Unicode::new(xkb::KEY_F30, '\u{0}')); // 0xFFDB
        hash.insert(String::from("R10"), Unicode::new(xkb::KEY_R10, '\u{0}')); // 0xFFDB
        hash.insert(String::from("F31"), Unicode::new(xkb::KEY_F31, '\u{0}')); // 0xFFDC
        hash.insert(String::from("R11"), Unicode::new(xkb::KEY_R11, '\u{0}')); // 0xFFDC
        hash.insert(String::from("F32"), Unicode::new(xkb::KEY_F32, '\u{0}')); // 0xFFDD
        hash.insert(String::from("R12"), Unicode::new(xkb::KEY_R12, '\u{0}')); // 0xFFDD
        hash.insert(String::from("F33"), Unicode::new(xkb::KEY_F33, '\u{0}')); // 0xFFDE
        hash.insert(String::from("R13"), Unicode::new(xkb::KEY_R13, '\u{0}')); // 0xFFDE
        hash.insert(String::from("F34"), Unicode::new(xkb::KEY_F34, '\u{0}')); // 0xFFDF
        hash.insert(String::from("R14"), Unicode::new(xkb::KEY_R14, '\u{0}')); // 0xFFDF
        hash.insert(String::from("F35"), Unicode::new(xkb::KEY_F35, '\u{0}')); // 0xFFE0
        hash.insert(String::from("R15"), Unicode::new(xkb::KEY_R15, '\u{0}')); // 0xFFE0
        hash.insert(
            String::from("Shift_L"),
            Unicode::new(xkb::KEY_Shift_L, '\u{0}'),
        ); // 0xFFE1
        hash.insert(
            String::from("Shift_R"),
            Unicode::new(xkb::KEY_Shift_R, '\u{0}'),
        ); // 0xFFE2
        hash.insert(
            String::from("Control_L"),
            Unicode::new(xkb::KEY_Control_L, '\u{0}'),
        ); // 0xFFE3
        hash.insert(
            String::from("Control_R"),
            Unicode::new(xkb::KEY_Control_R, '\u{0}'),
        ); // 0xFFE4
        hash.insert(
            String::from("Caps_Lock"),
            Unicode::new(xkb::KEY_Caps_Lock, '\u{0}'),
        ); // 0xFFE5
        hash.insert(
            String::from("Shift_Lock"),
            Unicode::new(xkb::KEY_Shift_Lock, '\u{0}'),
        ); // 0xFFE6
        hash.insert(
            String::from("Meta_L"),
            Unicode::new(xkb::KEY_Meta_L, '\u{0}'),
        ); // 0xFFE7
        hash.insert(
            String::from("Meta_R"),
            Unicode::new(xkb::KEY_Meta_R, '\u{0}'),
        ); // 0xFFE8
        hash.insert(String::from("Alt_L"), Unicode::new(xkb::KEY_Alt_L, '\u{0}')); // 0xFFE9
        hash.insert(String::from("Alt_R"), Unicode::new(xkb::KEY_Alt_R, '\u{0}')); // 0xFFEA
        hash.insert(
            String::from("Super_L"),
            Unicode::new(xkb::KEY_Super_L, '\u{0}'),
        ); // 0xFFEB
        hash.insert(
            String::from("Super_R"),
            Unicode::new(xkb::KEY_Super_R, '\u{0}'),
        ); // 0xFFEC
        hash.insert(
            String::from("Hyper_L"),
            Unicode::new(xkb::KEY_Hyper_L, '\u{0}'),
        ); // 0xFFED
        hash.insert(
            String::from("Hyper_R"),
            Unicode::new(xkb::KEY_Hyper_R, '\u{0}'),
        ); // 0xFFEE
        hash.insert(
            String::from("space"),
            Unicode::new(xkb::KEY_space, '\u{20}'),
        ); // 0x020
        hash.insert(
            String::from("exclam"),
            Unicode::new(xkb::KEY_exclam, '\u{21}'),
        ); // 0x021
        hash.insert(
            String::from("quotedbl"),
            Unicode::new(xkb::KEY_quotedbl, '\u{22}'),
        ); // 0x022
        hash.insert(
            String::from("numbersign"),
            Unicode::new(xkb::KEY_numbersign, '\u{23}'),
        ); // 0x023
        hash.insert(
            String::from("dollar"),
            Unicode::new(xkb::KEY_dollar, '\u{24}'),
        ); // 0x024
        hash.insert(
            String::from("percent"),
            Unicode::new(xkb::KEY_percent, '\u{25}'),
        ); // 0x025
        hash.insert(
            String::from("ampersand"),
            Unicode::new(xkb::KEY_ampersand, '\u{26}'),
        ); // 0x026
        hash.insert(
            String::from("apostrophe"),
            Unicode::new(xkb::KEY_apostrophe, '\u{27}'),
        ); // 0x027
        hash.insert(
            String::from("quoteright"),
            Unicode::new(xkb::KEY_quoteright, '\u{27}'),
        ); // 0x027
        hash.insert(
            String::from("parenleft"),
            Unicode::new(xkb::KEY_parenleft, '\u{28}'),
        ); // 0x028
        hash.insert(
            String::from("parenright"),
            Unicode::new(xkb::KEY_parenright, '\u{29}'),
        ); // 0x029
        hash.insert(
            String::from("asterisk"),
            Unicode::new(xkb::KEY_asterisk, '\u{2a}'),
        ); // 0x02a
        hash.insert(String::from("plus"), Unicode::new(xkb::KEY_plus, '\u{2b}')); // 0x02b
        hash.insert(
            String::from("comma"),
            Unicode::new(xkb::KEY_comma, '\u{2c}'),
        ); // 0x02c
        hash.insert(
            String::from("minus"),
            Unicode::new(xkb::KEY_minus, '\u{2d}'),
        ); // 0x02d
        hash.insert(
            String::from("period"),
            Unicode::new(xkb::KEY_period, '\u{2e}'),
        ); // 0x02e
        hash.insert(
            String::from("slash"),
            Unicode::new(xkb::KEY_slash, '\u{2f}'),
        ); // 0x02f
        hash.insert(String::from("0"), Unicode::new(xkb::KEY_0, '\u{30}')); // 0x030
        hash.insert(String::from("1"), Unicode::new(xkb::KEY_1, '\u{31}')); // 0x031
        hash.insert(String::from("2"), Unicode::new(xkb::KEY_2, '\u{32}')); // 0x032
        hash.insert(String::from("3"), Unicode::new(xkb::KEY_3, '\u{33}')); // 0x033
        hash.insert(String::from("4"), Unicode::new(xkb::KEY_4, '\u{34}')); // 0x034
        hash.insert(String::from("5"), Unicode::new(xkb::KEY_5, '\u{35}')); // 0x035
        hash.insert(String::from("6"), Unicode::new(xkb::KEY_6, '\u{36}')); // 0x036
        hash.insert(String::from("7"), Unicode::new(xkb::KEY_7, '\u{37}')); // 0x037
        hash.insert(String::from("8"), Unicode::new(xkb::KEY_8, '\u{38}')); // 0x038
        hash.insert(String::from("9"), Unicode::new(xkb::KEY_9, '\u{39}')); // 0x039
        hash.insert(
            String::from("colon"),
            Unicode::new(xkb::KEY_colon, '\u{3a}'),
        ); // 0x03a
        hash.insert(
            String::from("semicolon"),
            Unicode::new(xkb::KEY_semicolon, '\u{3b}'),
        ); // 0x03b
        hash.insert(String::from("less"), Unicode::new(xkb::KEY_less, '\u{3c}')); // 0x03c
        hash.insert(
            String::from("equal"),
            Unicode::new(xkb::KEY_equal, '\u{3d}'),
        ); // 0x03d
        hash.insert(
            String::from("greater"),
            Unicode::new(xkb::KEY_greater, '\u{3e}'),
        ); // 0x03e
        hash.insert(
            String::from("question"),
            Unicode::new(xkb::KEY_question, '\u{3f}'),
        ); // 0x03f
        hash.insert(String::from("at"), Unicode::new(xkb::KEY_at, '\u{40}')); // 0x040
        hash.insert(String::from("A"), Unicode::new(xkb::KEY_A, '\u{41}')); // 0x041
        hash.insert(String::from("B"), Unicode::new(xkb::KEY_B, '\u{42}')); // 0x042
        hash.insert(String::from("C"), Unicode::new(xkb::KEY_C, '\u{43}')); // 0x043
        hash.insert(String::from("D"), Unicode::new(xkb::KEY_D, '\u{44}')); // 0x044
        hash.insert(String::from("E"), Unicode::new(xkb::KEY_E, '\u{45}')); // 0x045
        hash.insert(String::from("F"), Unicode::new(xkb::KEY_F, '\u{46}')); // 0x046
        hash.insert(String::from("G"), Unicode::new(xkb::KEY_G, '\u{47}')); // 0x047
        hash.insert(String::from("H"), Unicode::new(xkb::KEY_H, '\u{48}')); // 0x048
        hash.insert(String::from("I"), Unicode::new(xkb::KEY_I, '\u{49}')); // 0x049
        hash.insert(String::from("J"), Unicode::new(xkb::KEY_J, '\u{4a}')); // 0x04a
        hash.insert(String::from("K"), Unicode::new(xkb::KEY_K, '\u{4b}')); // 0x04b
        hash.insert(String::from("L"), Unicode::new(xkb::KEY_L, '\u{4c}')); // 0x04c
        hash.insert(String::from("M"), Unicode::new(xkb::KEY_M, '\u{4d}')); // 0x04d
        hash.insert(String::from("N"), Unicode::new(xkb::KEY_N, '\u{4e}')); // 0x04e
        hash.insert(String::from("O"), Unicode::new(xkb::KEY_O, '\u{4f}')); // 0x04f
        hash.insert(String::from("P"), Unicode::new(xkb::KEY_P, '\u{50}')); // 0x050
        hash.insert(String::from("Q"), Unicode::new(xkb::KEY_Q, '\u{51}')); // 0x051
        hash.insert(String::from("R"), Unicode::new(xkb::KEY_R, '\u{52}')); // 0x052
        hash.insert(String::from("S"), Unicode::new(xkb::KEY_S, '\u{53}')); // 0x053
        hash.insert(String::from("T"), Unicode::new(xkb::KEY_T, '\u{54}')); // 0x054
        hash.insert(String::from("U"), Unicode::new(xkb::KEY_U, '\u{55}')); // 0x055
        hash.insert(String::from("V"), Unicode::new(xkb::KEY_V, '\u{56}')); // 0x056
        hash.insert(String::from("W"), Unicode::new(xkb::KEY_W, '\u{57}')); // 0x057
        hash.insert(String::from("X"), Unicode::new(xkb::KEY_X, '\u{58}')); // 0x058
        hash.insert(String::from("Y"), Unicode::new(xkb::KEY_Y, '\u{59}')); // 0x059
        hash.insert(String::from("Z"), Unicode::new(xkb::KEY_Z, '\u{5a}')); // 0x05a
        hash.insert(
            String::from("bracketleft"),
            Unicode::new(xkb::KEY_bracketleft, '\u{5b}'),
        ); // 0x05b
        hash.insert(
            String::from("backslash"),
            Unicode::new(xkb::KEY_backslash, '\u{5c}'),
        ); // 0x05c
        hash.insert(
            String::from("bracketright"),
            Unicode::new(xkb::KEY_bracketright, '\u{5d}'),
        ); // 0x05d
        hash.insert(
            String::from("asciicircum"),
            Unicode::new(xkb::KEY_asciicircum, '\u{5e}'),
        ); // 0x05e
        hash.insert(
            String::from("underscore"),
            Unicode::new(xkb::KEY_underscore, '\u{5f}'),
        ); // 0x05f
        hash.insert(
            String::from("grave"),
            Unicode::new(xkb::KEY_grave, '\u{60}'),
        ); // 0x060
        hash.insert(
            String::from("quoteleft"),
            Unicode::new(xkb::KEY_quoteleft, '\u{60}'),
        ); // 0x060
        hash.insert(String::from("a"), Unicode::new(xkb::KEY_a, '\u{61}')); // 0x061
        hash.insert(String::from("b"), Unicode::new(xkb::KEY_b, '\u{62}')); // 0x062
        hash.insert(String::from("c"), Unicode::new(xkb::KEY_c, '\u{63}')); // 0x063
        hash.insert(String::from("d"), Unicode::new(xkb::KEY_d, '\u{64}')); // 0x064
        hash.insert(String::from("e"), Unicode::new(xkb::KEY_e, '\u{65}')); // 0x065
        hash.insert(String::from("f"), Unicode::new(xkb::KEY_f, '\u{66}')); // 0x066
        hash.insert(String::from("g"), Unicode::new(xkb::KEY_g, '\u{67}')); // 0x067
        hash.insert(String::from("h"), Unicode::new(xkb::KEY_h, '\u{68}')); // 0x068
        hash.insert(String::from("i"), Unicode::new(xkb::KEY_i, '\u{69}')); // 0x069
        hash.insert(String::from("j"), Unicode::new(xkb::KEY_j, '\u{6a}')); // 0x06a
        hash.insert(String::from("k"), Unicode::new(xkb::KEY_k, '\u{6b}')); // 0x06b
        hash.insert(String::from("l"), Unicode::new(xkb::KEY_l, '\u{6c}')); // 0x06c
        hash.insert(String::from("m"), Unicode::new(xkb::KEY_m, '\u{6d}')); // 0x06d
        hash.insert(String::from("n"), Unicode::new(xkb::KEY_n, '\u{6e}')); // 0x06e
        hash.insert(String::from("o"), Unicode::new(xkb::KEY_o, '\u{6f}')); // 0x06f
        hash.insert(String::from("p"), Unicode::new(xkb::KEY_p, '\u{70}')); // 0x070
        hash.insert(String::from("q"), Unicode::new(xkb::KEY_q, '\u{71}')); // 0x071
        hash.insert(String::from("r"), Unicode::new(xkb::KEY_r, '\u{72}')); // 0x072
        hash.insert(String::from("s"), Unicode::new(xkb::KEY_s, '\u{73}')); // 0x073
        hash.insert(String::from("t"), Unicode::new(xkb::KEY_t, '\u{74}')); // 0x074
        hash.insert(String::from("u"), Unicode::new(xkb::KEY_u, '\u{75}')); // 0x075
        hash.insert(String::from("v"), Unicode::new(xkb::KEY_v, '\u{76}')); // 0x076
        hash.insert(String::from("w"), Unicode::new(xkb::KEY_w, '\u{77}')); // 0x077
        hash.insert(String::from("x"), Unicode::new(xkb::KEY_x, '\u{78}')); // 0x078
        hash.insert(String::from("y"), Unicode::new(xkb::KEY_y, '\u{79}')); // 0x079
        hash.insert(String::from("z"), Unicode::new(xkb::KEY_z, '\u{7a}')); // 0x07a
        hash.insert(
            String::from("braceleft"),
            Unicode::new(xkb::KEY_braceleft, '\u{7b}'),
        ); // 0x07b
        hash.insert(String::from("bar"), Unicode::new(xkb::KEY_bar, '\u{7c}')); // 0x07c
        hash.insert(
            String::from("braceright"),
            Unicode::new(xkb::KEY_braceright, '\u{7d}'),
        ); // 0x07d
        hash.insert(
            String::from("asciitilde"),
            Unicode::new(xkb::KEY_asciitilde, '\u{7e}'),
        ); // 0x07e
        hash.insert(
            String::from("nobreakspace"),
            Unicode::new(xkb::KEY_nobreakspace, '\u{a0}'),
        ); // 0x0a0
        hash.insert(
            String::from("exclamdown"),
            Unicode::new(xkb::KEY_exclamdown, '\u{a1}'),
        ); // 0x0a1
        hash.insert(String::from("cent"), Unicode::new(xkb::KEY_cent, '\u{a2}')); // 0x0a2
        hash.insert(
            String::from("sterling"),
            Unicode::new(xkb::KEY_sterling, '\u{a3}'),
        ); // 0x0a3
        hash.insert(
            String::from("currency"),
            Unicode::new(xkb::KEY_currency, '\u{a4}'),
        ); // 0x0a4
        hash.insert(String::from("yen"), Unicode::new(xkb::KEY_yen, '\u{a5}')); // 0x0a5
        hash.insert(
            String::from("brokenbar"),
            Unicode::new(xkb::KEY_brokenbar, '\u{a6}'),
        ); // 0x0a6
        hash.insert(
            String::from("section"),
            Unicode::new(xkb::KEY_section, '\u{a7}'),
        ); // 0x0a7
        hash.insert(
            String::from("diaeresis"),
            Unicode::new(xkb::KEY_diaeresis, '\u{a8}'),
        ); // 0x0a8
        hash.insert(
            String::from("copyright"),
            Unicode::new(xkb::KEY_copyright, '\u{a9}'),
        ); // 0x0a9
        hash.insert(
            String::from("ordfeminine"),
            Unicode::new(xkb::KEY_ordfeminine, '\u{aa}'),
        ); // 0x0aa
        hash.insert(
            String::from("guillemotleft"),
            Unicode::new(xkb::KEY_guillemotleft, '\u{ab}'),
        ); // 0x0ab
        hash.insert(
            String::from("notsign"),
            Unicode::new(xkb::KEY_notsign, '\u{ac}'),
        ); // 0x0ac
        hash.insert(
            String::from("hyphen"),
            Unicode::new(xkb::KEY_hyphen, '\u{ad}'),
        ); // 0x0ad
        hash.insert(
            String::from("registered"),
            Unicode::new(xkb::KEY_registered, '\u{ae}'),
        ); // 0x0ae
        hash.insert(
            String::from("macron"),
            Unicode::new(xkb::KEY_macron, '\u{af}'),
        ); // 0x0af
        hash.insert(
            String::from("degree"),
            Unicode::new(xkb::KEY_degree, '\u{b0}'),
        ); // 0x0b0
        hash.insert(
            String::from("plusminus"),
            Unicode::new(xkb::KEY_plusminus, '\u{b1}'),
        ); // 0x0b1
        hash.insert(
            String::from("twosuperior"),
            Unicode::new(xkb::KEY_twosuperior, '\u{b2}'),
        ); // 0x0b2
        hash.insert(
            String::from("threesuperior"),
            Unicode::new(xkb::KEY_threesuperior, '\u{b3}'),
        ); // 0x0b3
        hash.insert(
            String::from("acute"),
            Unicode::new(xkb::KEY_acute, '\u{b4}'),
        ); // 0x0b4
        hash.insert(String::from("mu"), Unicode::new(xkb::KEY_mu, '\u{b5}')); // 0x0b5
        hash.insert(
            String::from("paragraph"),
            Unicode::new(xkb::KEY_paragraph, '\u{b6}'),
        ); // 0x0b6
        hash.insert(
            String::from("periodcentered"),
            Unicode::new(xkb::KEY_periodcentered, '\u{b7}'),
        ); // 0x0b7
        hash.insert(
            String::from("cedilla"),
            Unicode::new(xkb::KEY_cedilla, '\u{b8}'),
        ); // 0x0b8
        hash.insert(
            String::from("onesuperior"),
            Unicode::new(xkb::KEY_onesuperior, '\u{b9}'),
        ); // 0x0b9
        hash.insert(
            String::from("masculine"),
            Unicode::new(xkb::KEY_masculine, '\u{ba}'),
        ); // 0x0ba
        hash.insert(
            String::from("guillemotright"),
            Unicode::new(xkb::KEY_guillemotright, '\u{bb}'),
        ); // 0x0bb
        hash.insert(
            String::from("onequarter"),
            Unicode::new(xkb::KEY_onequarter, '\u{bc}'),
        ); // 0x0bc
        hash.insert(
            String::from("onehalf"),
            Unicode::new(xkb::KEY_onehalf, '\u{bd}'),
        ); // 0x0bd
        hash.insert(
            String::from("threequarters"),
            Unicode::new(xkb::KEY_threequarters, '\u{be}'),
        ); // 0x0be
        hash.insert(
            String::from("questiondown"),
            Unicode::new(xkb::KEY_questiondown, '\u{bf}'),
        ); // 0x0bf
        hash.insert(
            String::from("Agrave"),
            Unicode::new(xkb::KEY_Agrave, '\u{c0}'),
        ); // 0x0c0
        hash.insert(
            String::from("Aacute"),
            Unicode::new(xkb::KEY_Aacute, '\u{c1}'),
        ); // 0x0c1
        hash.insert(
            String::from("Acircumflex"),
            Unicode::new(xkb::KEY_Acircumflex, '\u{c2}'),
        ); // 0x0c2
        hash.insert(
            String::from("Atilde"),
            Unicode::new(xkb::KEY_Atilde, '\u{c3}'),
        ); // 0x0c3
        hash.insert(
            String::from("Adiaeresis"),
            Unicode::new(xkb::KEY_Adiaeresis, '\u{c4}'),
        ); // 0x0c4
        hash.insert(
            String::from("Aring"),
            Unicode::new(xkb::KEY_Aring, '\u{c5}'),
        ); // 0x0c5
        hash.insert(String::from("AE"), Unicode::new(xkb::KEY_AE, '\u{c6}')); // 0x0c6
        hash.insert(
            String::from("Ccedilla"),
            Unicode::new(xkb::KEY_Ccedilla, '\u{c7}'),
        ); // 0x0c7
        hash.insert(
            String::from("Egrave"),
            Unicode::new(xkb::KEY_Egrave, '\u{c8}'),
        ); // 0x0c8
        hash.insert(
            String::from("Eacute"),
            Unicode::new(xkb::KEY_Eacute, '\u{c9}'),
        ); // 0x0c9
        hash.insert(
            String::from("Ecircumflex"),
            Unicode::new(xkb::KEY_Ecircumflex, '\u{ca}'),
        ); // 0x0ca
        hash.insert(
            String::from("Ediaeresis"),
            Unicode::new(xkb::KEY_Ediaeresis, '\u{cb}'),
        ); // 0x0cb
        hash.insert(
            String::from("Igrave"),
            Unicode::new(xkb::KEY_Igrave, '\u{cc}'),
        ); // 0x0cc
        hash.insert(
            String::from("Iacute"),
            Unicode::new(xkb::KEY_Iacute, '\u{cd}'),
        ); // 0x0cd
        hash.insert(
            String::from("Icircumflex"),
            Unicode::new(xkb::KEY_Icircumflex, '\u{ce}'),
        ); // 0x0ce
        hash.insert(
            String::from("Idiaeresis"),
            Unicode::new(xkb::KEY_Idiaeresis, '\u{cf}'),
        ); // 0x0cf
        hash.insert(String::from("ETH"), Unicode::new(xkb::KEY_ETH, '\u{d0}')); // 0x0d0
        hash.insert(String::from("Eth"), Unicode::new(xkb::KEY_Eth, '\u{d0}')); // 0x0d0
        hash.insert(
            String::from("Ntilde"),
            Unicode::new(xkb::KEY_Ntilde, '\u{d1}'),
        ); // 0x0d1
        hash.insert(
            String::from("Ograve"),
            Unicode::new(xkb::KEY_Ograve, '\u{d2}'),
        ); // 0x0d2
        hash.insert(
            String::from("Oacute"),
            Unicode::new(xkb::KEY_Oacute, '\u{d3}'),
        ); // 0x0d3
        hash.insert(
            String::from("Ocircumflex"),
            Unicode::new(xkb::KEY_Ocircumflex, '\u{d4}'),
        ); // 0x0d4
        hash.insert(
            String::from("Otilde"),
            Unicode::new(xkb::KEY_Otilde, '\u{d5}'),
        ); // 0x0d5
        hash.insert(
            String::from("Odiaeresis"),
            Unicode::new(xkb::KEY_Odiaeresis, '\u{d6}'),
        ); // 0x0d6
        hash.insert(
            String::from("multiply"),
            Unicode::new(xkb::KEY_multiply, '\u{d7}'),
        ); // 0x0d7
        hash.insert(
            String::from("Ooblique"),
            Unicode::new(xkb::KEY_Ooblique, '\u{d8}'),
        ); // 0x0d8
        hash.insert(
            String::from("Ugrave"),
            Unicode::new(xkb::KEY_Ugrave, '\u{d9}'),
        ); // 0x0d9
        hash.insert(
            String::from("Uacute"),
            Unicode::new(xkb::KEY_Uacute, '\u{da}'),
        ); // 0x0da
        hash.insert(
            String::from("Ucircumflex"),
            Unicode::new(xkb::KEY_Ucircumflex, '\u{db}'),
        ); // 0x0db
        hash.insert(
            String::from("Udiaeresis"),
            Unicode::new(xkb::KEY_Udiaeresis, '\u{dc}'),
        ); // 0x0dc
        hash.insert(
            String::from("Yacute"),
            Unicode::new(xkb::KEY_Yacute, '\u{dd}'),
        ); // 0x0dd
        hash.insert(
            String::from("THORN"),
            Unicode::new(xkb::KEY_THORN, '\u{de}'),
        ); // 0x0de
        hash.insert(
            String::from("Thorn"),
            Unicode::new(xkb::KEY_Thorn, '\u{de}'),
        ); // 0x0de
        hash.insert(
            String::from("ssharp"),
            Unicode::new(xkb::KEY_ssharp, '\u{df}'),
        ); // 0x0df
        hash.insert(
            String::from("agrave"),
            Unicode::new(xkb::KEY_agrave, '\u{e0}'),
        ); // 0x0e0
        hash.insert(
            String::from("aacute"),
            Unicode::new(xkb::KEY_aacute, '\u{e1}'),
        ); // 0x0e1
        hash.insert(
            String::from("acircumflex"),
            Unicode::new(xkb::KEY_acircumflex, '\u{e2}'),
        ); // 0x0e2
        hash.insert(
            String::from("atilde"),
            Unicode::new(xkb::KEY_atilde, '\u{e3}'),
        ); // 0x0e3
        hash.insert(
            String::from("adiaeresis"),
            Unicode::new(xkb::KEY_adiaeresis, '\u{e4}'),
        ); // 0x0e4
        hash.insert(
            String::from("aring"),
            Unicode::new(xkb::KEY_aring, '\u{e5}'),
        ); // 0x0e5
        hash.insert(String::from("ae"), Unicode::new(xkb::KEY_ae, '\u{e6}')); // 0x0e6
        hash.insert(
            String::from("ccedilla"),
            Unicode::new(xkb::KEY_ccedilla, '\u{e7}'),
        ); // 0x0e7
        hash.insert(
            String::from("egrave"),
            Unicode::new(xkb::KEY_egrave, '\u{e8}'),
        ); // 0x0e8
        hash.insert(
            String::from("eacute"),
            Unicode::new(xkb::KEY_eacute, '\u{e9}'),
        ); // 0x0e9
        hash.insert(
            String::from("ecircumflex"),
            Unicode::new(xkb::KEY_ecircumflex, '\u{ea}'),
        ); // 0x0ea
        hash.insert(
            String::from("ediaeresis"),
            Unicode::new(xkb::KEY_ediaeresis, '\u{eb}'),
        ); // 0x0eb
        hash.insert(
            String::from("igrave"),
            Unicode::new(xkb::KEY_igrave, '\u{ec}'),
        ); // 0x0ec
        hash.insert(
            String::from("iacute"),
            Unicode::new(xkb::KEY_iacute, '\u{ed}'),
        ); // 0x0ed
        hash.insert(
            String::from("icircumflex"),
            Unicode::new(xkb::KEY_icircumflex, '\u{ee}'),
        ); // 0x0ee
        hash.insert(
            String::from("idiaeresis"),
            Unicode::new(xkb::KEY_idiaeresis, '\u{ef}'),
        ); // 0x0ef
        hash.insert(String::from("eth"), Unicode::new(xkb::KEY_eth, '\u{f0}')); // 0x0f0
        hash.insert(
            String::from("ntilde"),
            Unicode::new(xkb::KEY_ntilde, '\u{f1}'),
        ); // 0x0f1
        hash.insert(
            String::from("ograve"),
            Unicode::new(xkb::KEY_ograve, '\u{f2}'),
        ); // 0x0f2
        hash.insert(
            String::from("oacute"),
            Unicode::new(xkb::KEY_oacute, '\u{f3}'),
        ); // 0x0f3
        hash.insert(
            String::from("ocircumflex"),
            Unicode::new(xkb::KEY_ocircumflex, '\u{f4}'),
        ); // 0x0f4
        hash.insert(
            String::from("otilde"),
            Unicode::new(xkb::KEY_otilde, '\u{f5}'),
        ); // 0x0f5
        hash.insert(
            String::from("odiaeresis"),
            Unicode::new(xkb::KEY_odiaeresis, '\u{f6}'),
        ); // 0x0f6
        hash.insert(
            String::from("division"),
            Unicode::new(xkb::KEY_division, '\u{f7}'),
        ); // 0x0f7
        hash.insert(
            String::from("oslash"),
            Unicode::new(xkb::KEY_oslash, '\u{f8}'),
        ); // 0x0f8
        hash.insert(
            String::from("ugrave"),
            Unicode::new(xkb::KEY_ugrave, '\u{f9}'),
        ); // 0x0f9
        hash.insert(
            String::from("uacute"),
            Unicode::new(xkb::KEY_uacute, '\u{fa}'),
        ); // 0x0fa
        hash.insert(
            String::from("ucircumflex"),
            Unicode::new(xkb::KEY_ucircumflex, '\u{fb}'),
        ); // 0x0fb
        hash.insert(
            String::from("udiaeresis"),
            Unicode::new(xkb::KEY_udiaeresis, '\u{fc}'),
        ); // 0x0fc
        hash.insert(
            String::from("yacute"),
            Unicode::new(xkb::KEY_yacute, '\u{fd}'),
        ); // 0x0fd
        hash.insert(
            String::from("thorn"),
            Unicode::new(xkb::KEY_thorn, '\u{fe}'),
        ); // 0x0fe
        hash.insert(
            String::from("ydiaeresis"),
            Unicode::new(xkb::KEY_ydiaeresis, '\u{ff}'),
        ); // 0x0ff
        hash.insert(
            String::from("Aogonek"),
            Unicode::new(xkb::KEY_Aogonek, '\u{104}'),
        ); // 0x1a1
        hash.insert(
            String::from("breve"),
            Unicode::new(xkb::KEY_breve, '\u{2d8}'),
        ); // 0x1a2
        hash.insert(
            String::from("Lstroke"),
            Unicode::new(xkb::KEY_Lstroke, '\u{141}'),
        ); // 0x1a3
        hash.insert(
            String::from("Lcaron"),
            Unicode::new(xkb::KEY_Lcaron, '\u{13d}'),
        ); // 0x1a5
        hash.insert(
            String::from("Sacute"),
            Unicode::new(xkb::KEY_Sacute, '\u{15a}'),
        ); // 0x1a6
        hash.insert(
            String::from("Scaron"),
            Unicode::new(xkb::KEY_Scaron, '\u{160}'),
        ); // 0x1a9
        hash.insert(
            String::from("Scedilla"),
            Unicode::new(xkb::KEY_Scedilla, '\u{15e}'),
        ); // 0x1aa
        hash.insert(
            String::from("Tcaron"),
            Unicode::new(xkb::KEY_Tcaron, '\u{164}'),
        ); // 0x1ab
        hash.insert(
            String::from("Zacute"),
            Unicode::new(xkb::KEY_Zacute, '\u{179}'),
        ); // 0x1ac
        hash.insert(
            String::from("Zcaron"),
            Unicode::new(xkb::KEY_Zcaron, '\u{17d}'),
        ); // 0x1ae
        hash.insert(
            String::from("Zabovedot"),
            Unicode::new(xkb::KEY_Zabovedot, '\u{17b}'),
        ); // 0x1af
        hash.insert(
            String::from("aogonek"),
            Unicode::new(xkb::KEY_aogonek, '\u{105}'),
        ); // 0x1b1
        hash.insert(
            String::from("ogonek"),
            Unicode::new(xkb::KEY_ogonek, '\u{2db}'),
        ); // 0x1b2
        hash.insert(
            String::from("lstroke"),
            Unicode::new(xkb::KEY_lstroke, '\u{142}'),
        ); // 0x1b3
        hash.insert(
            String::from("lcaron"),
            Unicode::new(xkb::KEY_lcaron, '\u{13e}'),
        ); // 0x1b5
        hash.insert(
            String::from("sacute"),
            Unicode::new(xkb::KEY_sacute, '\u{15b}'),
        ); // 0x1b6
        hash.insert(
            String::from("caron"),
            Unicode::new(xkb::KEY_caron, '\u{2c7}'),
        ); // 0x1b7
        hash.insert(
            String::from("scaron"),
            Unicode::new(xkb::KEY_scaron, '\u{161}'),
        ); // 0x1b9
        hash.insert(
            String::from("scedilla"),
            Unicode::new(xkb::KEY_scedilla, '\u{15f}'),
        ); // 0x1ba
        hash.insert(
            String::from("tcaron"),
            Unicode::new(xkb::KEY_tcaron, '\u{165}'),
        ); // 0x1bb
        hash.insert(
            String::from("zacute"),
            Unicode::new(xkb::KEY_zacute, '\u{17a}'),
        ); // 0x1bc
        hash.insert(
            String::from("doubleacute"),
            Unicode::new(xkb::KEY_doubleacute, '\u{2dd}'),
        ); // 0x1bd
        hash.insert(
            String::from("zcaron"),
            Unicode::new(xkb::KEY_zcaron, '\u{17e}'),
        ); // 0x1be
        hash.insert(
            String::from("zabovedot"),
            Unicode::new(xkb::KEY_zabovedot, '\u{17c}'),
        ); // 0x1bf
        hash.insert(
            String::from("Racute"),
            Unicode::new(xkb::KEY_Racute, '\u{154}'),
        ); // 0x1c0
        hash.insert(
            String::from("Abreve"),
            Unicode::new(xkb::KEY_Abreve, '\u{102}'),
        ); // 0x1c3
        hash.insert(
            String::from("Lacute"),
            Unicode::new(xkb::KEY_Lacute, '\u{139}'),
        ); // 0x1c5
        hash.insert(
            String::from("Cacute"),
            Unicode::new(xkb::KEY_Cacute, '\u{106}'),
        ); // 0x1c6
        hash.insert(
            String::from("Ccaron"),
            Unicode::new(xkb::KEY_Ccaron, '\u{10c}'),
        ); // 0x1c8
        hash.insert(
            String::from("Eogonek"),
            Unicode::new(xkb::KEY_Eogonek, '\u{118}'),
        ); // 0x1ca
        hash.insert(
            String::from("Ecaron"),
            Unicode::new(xkb::KEY_Ecaron, '\u{11a}'),
        ); // 0x1cc
        hash.insert(
            String::from("Dcaron"),
            Unicode::new(xkb::KEY_Dcaron, '\u{10e}'),
        ); // 0x1cf
        hash.insert(
            String::from("Dstroke"),
            Unicode::new(xkb::KEY_Dstroke, '\u{110}'),
        ); // 0x1d0
        hash.insert(
            String::from("Nacute"),
            Unicode::new(xkb::KEY_Nacute, '\u{143}'),
        ); // 0x1d1
        hash.insert(
            String::from("Ncaron"),
            Unicode::new(xkb::KEY_Ncaron, '\u{147}'),
        ); // 0x1d2
        hash.insert(
            String::from("Odoubleacute"),
            Unicode::new(xkb::KEY_Odoubleacute, '\u{150}'),
        ); // 0x1d5
        hash.insert(
            String::from("Rcaron"),
            Unicode::new(xkb::KEY_Rcaron, '\u{158}'),
        ); // 0x1d8
        hash.insert(
            String::from("Uring"),
            Unicode::new(xkb::KEY_Uring, '\u{16e}'),
        ); // 0x1d9
        hash.insert(
            String::from("Udoubleacute"),
            Unicode::new(xkb::KEY_Udoubleacute, '\u{170}'),
        ); // 0x1db
        hash.insert(
            String::from("Tcedilla"),
            Unicode::new(xkb::KEY_Tcedilla, '\u{162}'),
        ); // 0x1de
        hash.insert(
            String::from("racute"),
            Unicode::new(xkb::KEY_racute, '\u{155}'),
        ); // 0x1e0
        hash.insert(
            String::from("abreve"),
            Unicode::new(xkb::KEY_abreve, '\u{103}'),
        ); // 0x1e3
        hash.insert(
            String::from("lacute"),
            Unicode::new(xkb::KEY_lacute, '\u{13a}'),
        ); // 0x1e5
        hash.insert(
            String::from("cacute"),
            Unicode::new(xkb::KEY_cacute, '\u{107}'),
        ); // 0x1e6
        hash.insert(
            String::from("ccaron"),
            Unicode::new(xkb::KEY_ccaron, '\u{10d}'),
        ); // 0x1e8
        hash.insert(
            String::from("eogonek"),
            Unicode::new(xkb::KEY_eogonek, '\u{119}'),
        ); // 0x1ea
        hash.insert(
            String::from("ecaron"),
            Unicode::new(xkb::KEY_ecaron, '\u{11b}'),
        ); // 0x1ec
        hash.insert(
            String::from("dcaron"),
            Unicode::new(xkb::KEY_dcaron, '\u{10f}'),
        ); // 0x1ef
        hash.insert(
            String::from("dstroke"),
            Unicode::new(xkb::KEY_dstroke, '\u{111}'),
        ); // 0x1f0
        hash.insert(
            String::from("nacute"),
            Unicode::new(xkb::KEY_nacute, '\u{144}'),
        ); // 0x1f1
        hash.insert(
            String::from("ncaron"),
            Unicode::new(xkb::KEY_ncaron, '\u{148}'),
        ); // 0x1f2
        hash.insert(
            String::from("odoubleacute"),
            Unicode::new(xkb::KEY_odoubleacute, '\u{151}'),
        ); // 0x1f5
        hash.insert(
            String::from("udoubleacute"),
            Unicode::new(xkb::KEY_udoubleacute, '\u{171}'),
        ); // 0x1fb
        hash.insert(
            String::from("rcaron"),
            Unicode::new(xkb::KEY_rcaron, '\u{159}'),
        ); // 0x1f8
        hash.insert(
            String::from("uring"),
            Unicode::new(xkb::KEY_uring, '\u{16f}'),
        ); // 0x1f9
        hash.insert(
            String::from("tcedilla"),
            Unicode::new(xkb::KEY_tcedilla, '\u{163}'),
        ); // 0x1fe
        hash.insert(
            String::from("abovedot"),
            Unicode::new(xkb::KEY_abovedot, '\u{2d9}'),
        ); // 0x1ff
        hash.insert(
            String::from("Hstroke"),
            Unicode::new(xkb::KEY_Hstroke, '\u{126}'),
        ); // 0x2a1
        hash.insert(
            String::from("Hcircumflex"),
            Unicode::new(xkb::KEY_Hcircumflex, '\u{124}'),
        ); // 0x2a6
        hash.insert(
            String::from("Iabovedot"),
            Unicode::new(xkb::KEY_Iabovedot, '\u{130}'),
        ); // 0x2a9
        hash.insert(
            String::from("Gbreve"),
            Unicode::new(xkb::KEY_Gbreve, '\u{11e}'),
        ); // 0x2ab
        hash.insert(
            String::from("Jcircumflex"),
            Unicode::new(xkb::KEY_Jcircumflex, '\u{134}'),
        ); // 0x2ac
        hash.insert(
            String::from("hstroke"),
            Unicode::new(xkb::KEY_hstroke, '\u{127}'),
        ); // 0x2b1
        hash.insert(
            String::from("hcircumflex"),
            Unicode::new(xkb::KEY_hcircumflex, '\u{125}'),
        ); // 0x2b6
        hash.insert(
            String::from("idotless"),
            Unicode::new(xkb::KEY_idotless, '\u{131}'),
        ); // 0x2b9
        hash.insert(
            String::from("gbreve"),
            Unicode::new(xkb::KEY_gbreve, '\u{11f}'),
        ); // 0x2bb
        hash.insert(
            String::from("jcircumflex"),
            Unicode::new(xkb::KEY_jcircumflex, '\u{135}'),
        ); // 0x2bc
        hash.insert(
            String::from("Cabovedot"),
            Unicode::new(xkb::KEY_Cabovedot, '\u{10a}'),
        ); // 0x2c5
        hash.insert(
            String::from("Ccircumflex"),
            Unicode::new(xkb::KEY_Ccircumflex, '\u{108}'),
        ); // 0x2c6
        hash.insert(
            String::from("Gabovedot"),
            Unicode::new(xkb::KEY_Gabovedot, '\u{120}'),
        ); // 0x2d5
        hash.insert(
            String::from("Gcircumflex"),
            Unicode::new(xkb::KEY_Gcircumflex, '\u{11c}'),
        ); // 0x2d8
        hash.insert(
            String::from("Ubreve"),
            Unicode::new(xkb::KEY_Ubreve, '\u{16c}'),
        ); // 0x2dd
        hash.insert(
            String::from("Scircumflex"),
            Unicode::new(xkb::KEY_Scircumflex, '\u{15c}'),
        ); // 0x2de
        hash.insert(
            String::from("cabovedot"),
            Unicode::new(xkb::KEY_cabovedot, '\u{10b}'),
        ); // 0x2e5
        hash.insert(
            String::from("ccircumflex"),
            Unicode::new(xkb::KEY_ccircumflex, '\u{109}'),
        ); // 0x2e6
        hash.insert(
            String::from("gabovedot"),
            Unicode::new(xkb::KEY_gabovedot, '\u{121}'),
        ); // 0x2f5
        hash.insert(
            String::from("gcircumflex"),
            Unicode::new(xkb::KEY_gcircumflex, '\u{11d}'),
        ); // 0x2f8
        hash.insert(
            String::from("ubreve"),
            Unicode::new(xkb::KEY_ubreve, '\u{16d}'),
        ); // 0x2fd
        hash.insert(
            String::from("scircumflex"),
            Unicode::new(xkb::KEY_scircumflex, '\u{15d}'),
        ); // 0x2fe
        hash.insert(String::from("kra"), Unicode::new(xkb::KEY_kra, '\u{138}')); // 0x3a2
        hash.insert(String::from("kappa"), Unicode::new(xkb::KEY_kappa, '\u{0}')); // 0x3a2
        hash.insert(
            String::from("Rcedilla"),
            Unicode::new(xkb::KEY_Rcedilla, '\u{156}'),
        ); // 0x3a3
        hash.insert(
            String::from("Itilde"),
            Unicode::new(xkb::KEY_Itilde, '\u{128}'),
        ); // 0x3a5
        hash.insert(
            String::from("Lcedilla"),
            Unicode::new(xkb::KEY_Lcedilla, '\u{13b}'),
        ); // 0x3a6
        hash.insert(
            String::from("Emacron"),
            Unicode::new(xkb::KEY_Emacron, '\u{112}'),
        ); // 0x3aa
        hash.insert(
            String::from("Gcedilla"),
            Unicode::new(xkb::KEY_Gcedilla, '\u{122}'),
        ); // 0x3ab
        hash.insert(
            String::from("Tslash"),
            Unicode::new(xkb::KEY_Tslash, '\u{166}'),
        ); // 0x3ac
        hash.insert(
            String::from("rcedilla"),
            Unicode::new(xkb::KEY_rcedilla, '\u{157}'),
        ); // 0x3b3
        hash.insert(
            String::from("itilde"),
            Unicode::new(xkb::KEY_itilde, '\u{129}'),
        ); // 0x3b5
        hash.insert(
            String::from("lcedilla"),
            Unicode::new(xkb::KEY_lcedilla, '\u{13c}'),
        ); // 0x3b6
        hash.insert(
            String::from("emacron"),
            Unicode::new(xkb::KEY_emacron, '\u{113}'),
        ); // 0x3ba
        hash.insert(
            String::from("gcedilla"),
            Unicode::new(xkb::KEY_gcedilla, '\u{123}'),
        ); // 0x3bb
        hash.insert(
            String::from("tslash"),
            Unicode::new(xkb::KEY_tslash, '\u{167}'),
        ); // 0x3bc
        hash.insert(String::from("ENG"), Unicode::new(xkb::KEY_ENG, '\u{14a}')); // 0x3bd
        hash.insert(String::from("eng"), Unicode::new(xkb::KEY_eng, '\u{14b}')); // 0x3bf
        hash.insert(
            String::from("Amacron"),
            Unicode::new(xkb::KEY_Amacron, '\u{100}'),
        ); // 0x3c0
        hash.insert(
            String::from("Iogonek"),
            Unicode::new(xkb::KEY_Iogonek, '\u{12e}'),
        ); // 0x3c7
        hash.insert(
            String::from("Eabovedot"),
            Unicode::new(xkb::KEY_Eabovedot, '\u{116}'),
        ); // 0x3cc
        hash.insert(
            String::from("Imacron"),
            Unicode::new(xkb::KEY_Imacron, '\u{12a}'),
        ); // 0x3cf
        hash.insert(
            String::from("Ncedilla"),
            Unicode::new(xkb::KEY_Ncedilla, '\u{145}'),
        ); // 0x3d1
        hash.insert(
            String::from("Omacron"),
            Unicode::new(xkb::KEY_Omacron, '\u{14c}'),
        ); // 0x3d2
        hash.insert(
            String::from("Kcedilla"),
            Unicode::new(xkb::KEY_Kcedilla, '\u{136}'),
        ); // 0x3d3
        hash.insert(
            String::from("Uogonek"),
            Unicode::new(xkb::KEY_Uogonek, '\u{172}'),
        ); // 0x3d9
        hash.insert(
            String::from("Utilde"),
            Unicode::new(xkb::KEY_Utilde, '\u{168}'),
        ); // 0x3dd
        hash.insert(
            String::from("Umacron"),
            Unicode::new(xkb::KEY_Umacron, '\u{16a}'),
        ); // 0x3de
        hash.insert(
            String::from("amacron"),
            Unicode::new(xkb::KEY_amacron, '\u{101}'),
        ); // 0x3e0
        hash.insert(
            String::from("iogonek"),
            Unicode::new(xkb::KEY_iogonek, '\u{12f}'),
        ); // 0x3e7
        hash.insert(
            String::from("eabovedot"),
            Unicode::new(xkb::KEY_eabovedot, '\u{117}'),
        ); // 0x3ec
        hash.insert(
            String::from("imacron"),
            Unicode::new(xkb::KEY_imacron, '\u{12b}'),
        ); // 0x3ef
        hash.insert(
            String::from("ncedilla"),
            Unicode::new(xkb::KEY_ncedilla, '\u{146}'),
        ); // 0x3f1
        hash.insert(
            String::from("omacron"),
            Unicode::new(xkb::KEY_omacron, '\u{14d}'),
        ); // 0x3f2
        hash.insert(
            String::from("kcedilla"),
            Unicode::new(xkb::KEY_kcedilla, '\u{137}'),
        ); // 0x3f3
        hash.insert(
            String::from("uogonek"),
            Unicode::new(xkb::KEY_uogonek, '\u{173}'),
        ); // 0x3f9
        hash.insert(
            String::from("utilde"),
            Unicode::new(xkb::KEY_utilde, '\u{169}'),
        ); // 0x3fd
        hash.insert(
            String::from("umacron"),
            Unicode::new(xkb::KEY_umacron, '\u{16b}'),
        ); // 0x3fe
        hash.insert(
            String::from("overline"),
            Unicode::new(xkb::KEY_overline, '\u{203e}'),
        ); // 0x47e
        hash.insert(
            String::from("kana_fullstop"),
            Unicode::new(xkb::KEY_kana_fullstop, '\u{3002}'),
        ); // 0x4a1
        hash.insert(
            String::from("kana_openingbracket"),
            Unicode::new(xkb::KEY_kana_openingbracket, '\u{300c}'),
        ); // 0x4a2
        hash.insert(
            String::from("kana_closingbracket"),
            Unicode::new(xkb::KEY_kana_closingbracket, '\u{300d}'),
        ); // 0x4a3
        hash.insert(
            String::from("kana_comma"),
            Unicode::new(xkb::KEY_kana_comma, '\u{3001}'),
        ); // 0x4a4
        hash.insert(
            String::from("kana_conjunctive"),
            Unicode::new(xkb::KEY_kana_conjunctive, '\u{30fb}'),
        ); // 0x4a5
        hash.insert(
            String::from("kana_middledot"),
            Unicode::new(xkb::KEY_kana_middledot, '\u{0}'),
        ); // 0x4a5
        hash.insert(
            String::from("kana_WO"),
            Unicode::new(xkb::KEY_kana_WO, '\u{30f2}'),
        ); // 0x4a6
        hash.insert(
            String::from("kana_a"),
            Unicode::new(xkb::KEY_kana_a, '\u{30a1}'),
        ); // 0x4a7
        hash.insert(
            String::from("kana_i"),
            Unicode::new(xkb::KEY_kana_i, '\u{30a3}'),
        ); // 0x4a8
        hash.insert(
            String::from("kana_u"),
            Unicode::new(xkb::KEY_kana_u, '\u{30a5}'),
        ); // 0x4a9
        hash.insert(
            String::from("kana_e"),
            Unicode::new(xkb::KEY_kana_e, '\u{30a7}'),
        ); // 0x4aa
        hash.insert(
            String::from("kana_o"),
            Unicode::new(xkb::KEY_kana_o, '\u{30a9}'),
        ); // 0x4ab
        hash.insert(
            String::from("kana_ya"),
            Unicode::new(xkb::KEY_kana_ya, '\u{30e3}'),
        ); // 0x4ac
        hash.insert(
            String::from("kana_yu"),
            Unicode::new(xkb::KEY_kana_yu, '\u{30e5}'),
        ); // 0x4ad
        hash.insert(
            String::from("kana_yo"),
            Unicode::new(xkb::KEY_kana_yo, '\u{30e7}'),
        ); // 0x4ae
        hash.insert(
            String::from("kana_tsu"),
            Unicode::new(xkb::KEY_kana_tsu, '\u{30c3}'),
        ); // 0x4af
        hash.insert(
            String::from("kana_tu"),
            Unicode::new(xkb::KEY_kana_tu, '\u{0}'),
        ); // 0x4af
        hash.insert(
            String::from("prolongedsound"),
            Unicode::new(xkb::KEY_prolongedsound, '\u{30fc}'),
        ); // 0x4b0
        hash.insert(
            String::from("kana_A"),
            Unicode::new(xkb::KEY_kana_A, '\u{30a2}'),
        ); // 0x4b1
        hash.insert(
            String::from("kana_I"),
            Unicode::new(xkb::KEY_kana_I, '\u{30a4}'),
        ); // 0x4b2
        hash.insert(
            String::from("kana_U"),
            Unicode::new(xkb::KEY_kana_U, '\u{30a6}'),
        ); // 0x4b3
        hash.insert(
            String::from("kana_E"),
            Unicode::new(xkb::KEY_kana_E, '\u{30a8}'),
        ); // 0x4b4
        hash.insert(
            String::from("kana_O"),
            Unicode::new(xkb::KEY_kana_O, '\u{30aa}'),
        ); // 0x4b5
        hash.insert(
            String::from("kana_KA"),
            Unicode::new(xkb::KEY_kana_KA, '\u{30ab}'),
        ); // 0x4b6
        hash.insert(
            String::from("kana_KI"),
            Unicode::new(xkb::KEY_kana_KI, '\u{30ad}'),
        ); // 0x4b7
        hash.insert(
            String::from("kana_KU"),
            Unicode::new(xkb::KEY_kana_KU, '\u{30af}'),
        ); // 0x4b8
        hash.insert(
            String::from("kana_KE"),
            Unicode::new(xkb::KEY_kana_KE, '\u{30b1}'),
        ); // 0x4b9
        hash.insert(
            String::from("kana_KO"),
            Unicode::new(xkb::KEY_kana_KO, '\u{30b3}'),
        ); // 0x4ba
        hash.insert(
            String::from("kana_SA"),
            Unicode::new(xkb::KEY_kana_SA, '\u{30b5}'),
        ); // 0x4bb
        hash.insert(
            String::from("kana_SHI"),
            Unicode::new(xkb::KEY_kana_SHI, '\u{30b7}'),
        ); // 0x4bc
        hash.insert(
            String::from("kana_SU"),
            Unicode::new(xkb::KEY_kana_SU, '\u{30b9}'),
        ); // 0x4bd
        hash.insert(
            String::from("kana_SE"),
            Unicode::new(xkb::KEY_kana_SE, '\u{30bb}'),
        ); // 0x4be
        hash.insert(
            String::from("kana_SO"),
            Unicode::new(xkb::KEY_kana_SO, '\u{30bd}'),
        ); // 0x4bf
        hash.insert(
            String::from("kana_TA"),
            Unicode::new(xkb::KEY_kana_TA, '\u{30bf}'),
        ); // 0x4c0
        hash.insert(
            String::from("kana_CHI"),
            Unicode::new(xkb::KEY_kana_CHI, '\u{30c1}'),
        ); // 0x4c1
        hash.insert(
            String::from("kana_TI"),
            Unicode::new(xkb::KEY_kana_TI, '\u{0}'),
        ); // 0x4c1
        hash.insert(
            String::from("kana_TSU"),
            Unicode::new(xkb::KEY_kana_TSU, '\u{30c4}'),
        ); // 0x4c2
        hash.insert(
            String::from("kana_TU"),
            Unicode::new(xkb::KEY_kana_TU, '\u{0}'),
        ); // 0x4c2
        hash.insert(
            String::from("kana_TE"),
            Unicode::new(xkb::KEY_kana_TE, '\u{30c6}'),
        ); // 0x4c3
        hash.insert(
            String::from("kana_TO"),
            Unicode::new(xkb::KEY_kana_TO, '\u{30c8}'),
        ); // 0x4c4
        hash.insert(
            String::from("kana_NA"),
            Unicode::new(xkb::KEY_kana_NA, '\u{30ca}'),
        ); // 0x4c5
        hash.insert(
            String::from("kana_NI"),
            Unicode::new(xkb::KEY_kana_NI, '\u{30cb}'),
        ); // 0x4c6
        hash.insert(
            String::from("kana_NU"),
            Unicode::new(xkb::KEY_kana_NU, '\u{30cc}'),
        ); // 0x4c7
        hash.insert(
            String::from("kana_NE"),
            Unicode::new(xkb::KEY_kana_NE, '\u{30cd}'),
        ); // 0x4c8
        hash.insert(
            String::from("kana_NO"),
            Unicode::new(xkb::KEY_kana_NO, '\u{30ce}'),
        ); // 0x4c9
        hash.insert(
            String::from("kana_HA"),
            Unicode::new(xkb::KEY_kana_HA, '\u{30cf}'),
        ); // 0x4ca
        hash.insert(
            String::from("kana_HI"),
            Unicode::new(xkb::KEY_kana_HI, '\u{30d2}'),
        ); // 0x4cb
        hash.insert(
            String::from("kana_FU"),
            Unicode::new(xkb::KEY_kana_FU, '\u{30d5}'),
        ); // 0x4cc
        hash.insert(
            String::from("kana_HU"),
            Unicode::new(xkb::KEY_kana_HU, '\u{0}'),
        ); // 0x4cc
        hash.insert(
            String::from("kana_HE"),
            Unicode::new(xkb::KEY_kana_HE, '\u{30d8}'),
        ); // 0x4cd
        hash.insert(
            String::from("kana_HO"),
            Unicode::new(xkb::KEY_kana_HO, '\u{30db}'),
        ); // 0x4ce
        hash.insert(
            String::from("kana_MA"),
            Unicode::new(xkb::KEY_kana_MA, '\u{30de}'),
        ); // 0x4cf
        hash.insert(
            String::from("kana_MI"),
            Unicode::new(xkb::KEY_kana_MI, '\u{30df}'),
        ); // 0x4d0
        hash.insert(
            String::from("kana_MU"),
            Unicode::new(xkb::KEY_kana_MU, '\u{30e0}'),
        ); // 0x4d1
        hash.insert(
            String::from("kana_ME"),
            Unicode::new(xkb::KEY_kana_ME, '\u{30e1}'),
        ); // 0x4d2
        hash.insert(
            String::from("kana_MO"),
            Unicode::new(xkb::KEY_kana_MO, '\u{30e2}'),
        ); // 0x4d3
        hash.insert(
            String::from("kana_YA"),
            Unicode::new(xkb::KEY_kana_YA, '\u{30e4}'),
        ); // 0x4d4
        hash.insert(
            String::from("kana_YU"),
            Unicode::new(xkb::KEY_kana_YU, '\u{30e6}'),
        ); // 0x4d5
        hash.insert(
            String::from("kana_YO"),
            Unicode::new(xkb::KEY_kana_YO, '\u{30e8}'),
        ); // 0x4d6
        hash.insert(
            String::from("kana_RA"),
            Unicode::new(xkb::KEY_kana_RA, '\u{30e9}'),
        ); // 0x4d7
        hash.insert(
            String::from("kana_RI"),
            Unicode::new(xkb::KEY_kana_RI, '\u{30ea}'),
        ); // 0x4d8
        hash.insert(
            String::from("kana_RU"),
            Unicode::new(xkb::KEY_kana_RU, '\u{30eb}'),
        ); // 0x4d9
        hash.insert(
            String::from("kana_RE"),
            Unicode::new(xkb::KEY_kana_RE, '\u{30ec}'),
        ); // 0x4da
        hash.insert(
            String::from("kana_RO"),
            Unicode::new(xkb::KEY_kana_RO, '\u{30ed}'),
        ); // 0x4db
        hash.insert(
            String::from("kana_WA"),
            Unicode::new(xkb::KEY_kana_WA, '\u{30ef}'),
        ); // 0x4dc
        hash.insert(
            String::from("kana_N"),
            Unicode::new(xkb::KEY_kana_N, '\u{30f3}'),
        ); // 0x4dd
        hash.insert(
            String::from("voicedsound"),
            Unicode::new(xkb::KEY_voicedsound, '\u{309b}'),
        ); // 0x4de
        hash.insert(
            String::from("semivoicedsound"),
            Unicode::new(xkb::KEY_semivoicedsound, '\u{309c}'),
        ); // 0x4df
        hash.insert(
            String::from("kana_switch"),
            Unicode::new(xkb::KEY_kana_switch, '\u{0}'),
        ); // 0xFF7E
        hash.insert(
            String::from("Arabic_comma"),
            Unicode::new(xkb::KEY_Arabic_comma, '\u{60c}'),
        ); // 0x5ac
        hash.insert(
            String::from("Arabic_semicolon"),
            Unicode::new(xkb::KEY_Arabic_semicolon, '\u{61b}'),
        ); // 0x5bb
        hash.insert(
            String::from("Arabic_question_mark"),
            Unicode::new(xkb::KEY_Arabic_question_mark, '\u{61f}'),
        ); // 0x5bf
        hash.insert(
            String::from("Arabic_hamza"),
            Unicode::new(xkb::KEY_Arabic_hamza, '\u{621}'),
        ); // 0x5c1
        hash.insert(
            String::from("Arabic_maddaonalef"),
            Unicode::new(xkb::KEY_Arabic_maddaonalef, '\u{622}'),
        ); // 0x5c2
        hash.insert(
            String::from("Arabic_hamzaonalef"),
            Unicode::new(xkb::KEY_Arabic_hamzaonalef, '\u{623}'),
        ); // 0x5c3
        hash.insert(
            String::from("Arabic_hamzaonwaw"),
            Unicode::new(xkb::KEY_Arabic_hamzaonwaw, '\u{624}'),
        ); // 0x5c4
        hash.insert(
            String::from("Arabic_hamzaunderalef"),
            Unicode::new(xkb::KEY_Arabic_hamzaunderalef, '\u{625}'),
        ); // 0x5c5
        hash.insert(
            String::from("Arabic_hamzaonyeh"),
            Unicode::new(xkb::KEY_Arabic_hamzaonyeh, '\u{626}'),
        ); // 0x5c6
        hash.insert(
            String::from("Arabic_alef"),
            Unicode::new(xkb::KEY_Arabic_alef, '\u{627}'),
        ); // 0x5c7
        hash.insert(
            String::from("Arabic_beh"),
            Unicode::new(xkb::KEY_Arabic_beh, '\u{628}'),
        ); // 0x5c8
        hash.insert(
            String::from("Arabic_tehmarbuta"),
            Unicode::new(xkb::KEY_Arabic_tehmarbuta, '\u{629}'),
        ); // 0x5c9
        hash.insert(
            String::from("Arabic_teh"),
            Unicode::new(xkb::KEY_Arabic_teh, '\u{62a}'),
        ); // 0x5ca
        hash.insert(
            String::from("Arabic_theh"),
            Unicode::new(xkb::KEY_Arabic_theh, '\u{62b}'),
        ); // 0x5cb
        hash.insert(
            String::from("Arabic_jeem"),
            Unicode::new(xkb::KEY_Arabic_jeem, '\u{62c}'),
        ); // 0x5cc
        hash.insert(
            String::from("Arabic_hah"),
            Unicode::new(xkb::KEY_Arabic_hah, '\u{62d}'),
        ); // 0x5cd
        hash.insert(
            String::from("Arabic_khah"),
            Unicode::new(xkb::KEY_Arabic_khah, '\u{62e}'),
        ); // 0x5ce
        hash.insert(
            String::from("Arabic_dal"),
            Unicode::new(xkb::KEY_Arabic_dal, '\u{62f}'),
        ); // 0x5cf
        hash.insert(
            String::from("Arabic_thal"),
            Unicode::new(xkb::KEY_Arabic_thal, '\u{630}'),
        ); // 0x5d0
        hash.insert(
            String::from("Arabic_ra"),
            Unicode::new(xkb::KEY_Arabic_ra, '\u{631}'),
        ); // 0x5d1
        hash.insert(
            String::from("Arabic_zain"),
            Unicode::new(xkb::KEY_Arabic_zain, '\u{632}'),
        ); // 0x5d2
        hash.insert(
            String::from("Arabic_seen"),
            Unicode::new(xkb::KEY_Arabic_seen, '\u{633}'),
        ); // 0x5d3
        hash.insert(
            String::from("Arabic_sheen"),
            Unicode::new(xkb::KEY_Arabic_sheen, '\u{634}'),
        ); // 0x5d4
        hash.insert(
            String::from("Arabic_sad"),
            Unicode::new(xkb::KEY_Arabic_sad, '\u{635}'),
        ); // 0x5d5
        hash.insert(
            String::from("Arabic_dad"),
            Unicode::new(xkb::KEY_Arabic_dad, '\u{636}'),
        ); // 0x5d6
        hash.insert(
            String::from("Arabic_tah"),
            Unicode::new(xkb::KEY_Arabic_tah, '\u{637}'),
        ); // 0x5d7
        hash.insert(
            String::from("Arabic_zah"),
            Unicode::new(xkb::KEY_Arabic_zah, '\u{638}'),
        ); // 0x5d8
        hash.insert(
            String::from("Arabic_ain"),
            Unicode::new(xkb::KEY_Arabic_ain, '\u{639}'),
        ); // 0x5d9
        hash.insert(
            String::from("Arabic_ghain"),
            Unicode::new(xkb::KEY_Arabic_ghain, '\u{63a}'),
        ); // 0x5da
        hash.insert(
            String::from("Arabic_tatweel"),
            Unicode::new(xkb::KEY_Arabic_tatweel, '\u{640}'),
        ); // 0x5e0
        hash.insert(
            String::from("Arabic_feh"),
            Unicode::new(xkb::KEY_Arabic_feh, '\u{641}'),
        ); // 0x5e1
        hash.insert(
            String::from("Arabic_qaf"),
            Unicode::new(xkb::KEY_Arabic_qaf, '\u{642}'),
        ); // 0x5e2
        hash.insert(
            String::from("Arabic_kaf"),
            Unicode::new(xkb::KEY_Arabic_kaf, '\u{643}'),
        ); // 0x5e3
        hash.insert(
            String::from("Arabic_lam"),
            Unicode::new(xkb::KEY_Arabic_lam, '\u{644}'),
        ); // 0x5e4
        hash.insert(
            String::from("Arabic_meem"),
            Unicode::new(xkb::KEY_Arabic_meem, '\u{645}'),
        ); // 0x5e5
        hash.insert(
            String::from("Arabic_noon"),
            Unicode::new(xkb::KEY_Arabic_noon, '\u{646}'),
        ); // 0x5e6
        hash.insert(
            String::from("Arabic_ha"),
            Unicode::new(xkb::KEY_Arabic_ha, '\u{647}'),
        ); // 0x5e7
        hash.insert(
            String::from("Arabic_heh"),
            Unicode::new(xkb::KEY_Arabic_heh, '\u{0}'),
        ); // 0x5e7
        hash.insert(
            String::from("Arabic_waw"),
            Unicode::new(xkb::KEY_Arabic_waw, '\u{648}'),
        ); // 0x5e8
        hash.insert(
            String::from("Arabic_alefmaksura"),
            Unicode::new(xkb::KEY_Arabic_alefmaksura, '\u{649}'),
        ); // 0x5e9
        hash.insert(
            String::from("Arabic_yeh"),
            Unicode::new(xkb::KEY_Arabic_yeh, '\u{64a}'),
        ); // 0x5ea
        hash.insert(
            String::from("Arabic_fathatan"),
            Unicode::new(xkb::KEY_Arabic_fathatan, '\u{64b}'),
        ); // 0x5eb
        hash.insert(
            String::from("Arabic_dammatan"),
            Unicode::new(xkb::KEY_Arabic_dammatan, '\u{64c}'),
        ); // 0x5ec
        hash.insert(
            String::from("Arabic_kasratan"),
            Unicode::new(xkb::KEY_Arabic_kasratan, '\u{64d}'),
        ); // 0x5ed
        hash.insert(
            String::from("Arabic_fatha"),
            Unicode::new(xkb::KEY_Arabic_fatha, '\u{64e}'),
        ); // 0x5ee
        hash.insert(
            String::from("Arabic_damma"),
            Unicode::new(xkb::KEY_Arabic_damma, '\u{64f}'),
        ); // 0x5ef
        hash.insert(
            String::from("Arabic_kasra"),
            Unicode::new(xkb::KEY_Arabic_kasra, '\u{650}'),
        ); // 0x5f0
        hash.insert(
            String::from("Arabic_shadda"),
            Unicode::new(xkb::KEY_Arabic_shadda, '\u{651}'),
        ); // 0x5f1
        hash.insert(
            String::from("Arabic_sukun"),
            Unicode::new(xkb::KEY_Arabic_sukun, '\u{652}'),
        ); // 0x5f2
        hash.insert(
            String::from("Arabic_switch"),
            Unicode::new(xkb::KEY_Arabic_switch, '\u{0}'),
        ); // 0xFF7E
        hash.insert(
            String::from("Serbian_dje"),
            Unicode::new(xkb::KEY_Serbian_dje, '\u{452}'),
        ); // 0x6a1
        hash.insert(
            String::from("Macedonia_gje"),
            Unicode::new(xkb::KEY_Macedonia_gje, '\u{453}'),
        ); // 0x6a2
        hash.insert(
            String::from("Cyrillic_io"),
            Unicode::new(xkb::KEY_Cyrillic_io, '\u{451}'),
        ); // 0x6a3
        hash.insert(
            String::from("Ukrainian_ie"),
            Unicode::new(xkb::KEY_Ukrainian_ie, '\u{454}'),
        ); // 0x6a4
        hash.insert(
            String::from("Ukranian_je"),
            Unicode::new(xkb::KEY_Ukranian_je, '\u{0}'),
        ); // 0x6a4
        hash.insert(
            String::from("Macedonia_dse"),
            Unicode::new(xkb::KEY_Macedonia_dse, '\u{455}'),
        ); // 0x6a5
        hash.insert(
            String::from("Ukrainian_i"),
            Unicode::new(xkb::KEY_Ukrainian_i, '\u{456}'),
        ); // 0x6a6
        hash.insert(
            String::from("Ukranian_i"),
            Unicode::new(xkb::KEY_Ukranian_i, '\u{0}'),
        ); // 0x6a6
        hash.insert(
            String::from("Ukrainian_yi"),
            Unicode::new(xkb::KEY_Ukrainian_yi, '\u{457}'),
        ); // 0x6a7
        hash.insert(
            String::from("Ukranian_yi"),
            Unicode::new(xkb::KEY_Ukranian_yi, '\u{0}'),
        ); // 0x6a7
        hash.insert(
            String::from("Cyrillic_je"),
            Unicode::new(xkb::KEY_Cyrillic_je, '\u{458}'),
        ); // 0x6a8
        hash.insert(
            String::from("Serbian_je"),
            Unicode::new(xkb::KEY_Serbian_je, '\u{0}'),
        ); // 0x6a8
        hash.insert(
            String::from("Cyrillic_lje"),
            Unicode::new(xkb::KEY_Cyrillic_lje, '\u{459}'),
        ); // 0x6a9
        hash.insert(
            String::from("Serbian_lje"),
            Unicode::new(xkb::KEY_Serbian_lje, '\u{0}'),
        ); // 0x6a9
        hash.insert(
            String::from("Cyrillic_nje"),
            Unicode::new(xkb::KEY_Cyrillic_nje, '\u{45a}'),
        ); // 0x6aa
        hash.insert(
            String::from("Serbian_nje"),
            Unicode::new(xkb::KEY_Serbian_nje, '\u{0}'),
        ); // 0x6aa
        hash.insert(
            String::from("Serbian_tshe"),
            Unicode::new(xkb::KEY_Serbian_tshe, '\u{45b}'),
        ); // 0x6ab
        hash.insert(
            String::from("Macedonia_kje"),
            Unicode::new(xkb::KEY_Macedonia_kje, '\u{45c}'),
        ); // 0x6ac
        hash.insert(
            String::from("Byelorussian_shortu"),
            Unicode::new(xkb::KEY_Byelorussian_shortu, '\u{45e}'),
        ); // 0x6ae
        hash.insert(
            String::from("Cyrillic_dzhe"),
            Unicode::new(xkb::KEY_Cyrillic_dzhe, '\u{45f}'),
        ); // 0x6af
        hash.insert(
            String::from("Serbian_dze"),
            Unicode::new(xkb::KEY_Serbian_dze, '\u{0}'),
        ); // 0x6af
        hash.insert(
            String::from("numerosign"),
            Unicode::new(xkb::KEY_numerosign, '\u{2116}'),
        ); // 0x6b0
        hash.insert(
            String::from("Serbian_DJE"),
            Unicode::new(xkb::KEY_Serbian_DJE, '\u{402}'),
        ); // 0x6b1
        hash.insert(
            String::from("Macedonia_GJE"),
            Unicode::new(xkb::KEY_Macedonia_GJE, '\u{403}'),
        ); // 0x6b2
        hash.insert(
            String::from("Cyrillic_IO"),
            Unicode::new(xkb::KEY_Cyrillic_IO, '\u{401}'),
        ); // 0x6b3
        hash.insert(
            String::from("Ukrainian_IE"),
            Unicode::new(xkb::KEY_Ukrainian_IE, '\u{404}'),
        ); // 0x6b4
        hash.insert(
            String::from("Ukranian_JE"),
            Unicode::new(xkb::KEY_Ukranian_JE, '\u{0}'),
        ); // 0x6b4
        hash.insert(
            String::from("Macedonia_DSE"),
            Unicode::new(xkb::KEY_Macedonia_DSE, '\u{405}'),
        ); // 0x6b5
        hash.insert(
            String::from("Ukrainian_I"),
            Unicode::new(xkb::KEY_Ukrainian_I, '\u{406}'),
        ); // 0x6b6
        hash.insert(
            String::from("Ukranian_I"),
            Unicode::new(xkb::KEY_Ukranian_I, '\u{0}'),
        ); // 0x6b6
        hash.insert(
            String::from("Ukrainian_YI"),
            Unicode::new(xkb::KEY_Ukrainian_YI, '\u{407}'),
        ); // 0x6b7
        hash.insert(
            String::from("Ukranian_YI"),
            Unicode::new(xkb::KEY_Ukranian_YI, '\u{0}'),
        ); // 0x6b7
        hash.insert(
            String::from("Cyrillic_JE"),
            Unicode::new(xkb::KEY_Cyrillic_JE, '\u{408}'),
        ); // 0x6b8
        hash.insert(
            String::from("Serbian_JE"),
            Unicode::new(xkb::KEY_Serbian_JE, '\u{0}'),
        ); // 0x6b8
        hash.insert(
            String::from("Cyrillic_LJE"),
            Unicode::new(xkb::KEY_Cyrillic_LJE, '\u{409}'),
        ); // 0x6b9
        hash.insert(
            String::from("Serbian_LJE"),
            Unicode::new(xkb::KEY_Serbian_LJE, '\u{0}'),
        ); // 0x6b9
        hash.insert(
            String::from("Cyrillic_NJE"),
            Unicode::new(xkb::KEY_Cyrillic_NJE, '\u{40a}'),
        ); // 0x6ba
        hash.insert(
            String::from("Serbian_NJE"),
            Unicode::new(xkb::KEY_Serbian_NJE, '\u{0}'),
        ); // 0x6ba
        hash.insert(
            String::from("Serbian_TSHE"),
            Unicode::new(xkb::KEY_Serbian_TSHE, '\u{40b}'),
        ); // 0x6bb
        hash.insert(
            String::from("Macedonia_KJE"),
            Unicode::new(xkb::KEY_Macedonia_KJE, '\u{40c}'),
        ); // 0x6bc
        hash.insert(
            String::from("Byelorussian_SHORTU"),
            Unicode::new(xkb::KEY_Byelorussian_SHORTU, '\u{40e}'),
        ); // 0x6be
        hash.insert(
            String::from("Cyrillic_DZHE"),
            Unicode::new(xkb::KEY_Cyrillic_DZHE, '\u{40f}'),
        ); // 0x6bf
        hash.insert(
            String::from("Serbian_DZE"),
            Unicode::new(xkb::KEY_Serbian_DZE, '\u{0}'),
        ); // 0x6bf
        hash.insert(
            String::from("Cyrillic_yu"),
            Unicode::new(xkb::KEY_Cyrillic_yu, '\u{44e}'),
        ); // 0x6c0
        hash.insert(
            String::from("Cyrillic_a"),
            Unicode::new(xkb::KEY_Cyrillic_a, '\u{430}'),
        ); // 0x6c1
        hash.insert(
            String::from("Cyrillic_be"),
            Unicode::new(xkb::KEY_Cyrillic_be, '\u{431}'),
        ); // 0x6c2
        hash.insert(
            String::from("Cyrillic_tse"),
            Unicode::new(xkb::KEY_Cyrillic_tse, '\u{446}'),
        ); // 0x6c3
        hash.insert(
            String::from("Cyrillic_de"),
            Unicode::new(xkb::KEY_Cyrillic_de, '\u{434}'),
        ); // 0x6c4
        hash.insert(
            String::from("Cyrillic_ie"),
            Unicode::new(xkb::KEY_Cyrillic_ie, '\u{435}'),
        ); // 0x6c5
        hash.insert(
            String::from("Cyrillic_ef"),
            Unicode::new(xkb::KEY_Cyrillic_ef, '\u{444}'),
        ); // 0x6c6
        hash.insert(
            String::from("Cyrillic_ghe"),
            Unicode::new(xkb::KEY_Cyrillic_ghe, '\u{433}'),
        ); // 0x6c7
        hash.insert(
            String::from("Cyrillic_ha"),
            Unicode::new(xkb::KEY_Cyrillic_ha, '\u{445}'),
        ); // 0x6c8
        hash.insert(
            String::from("Cyrillic_i"),
            Unicode::new(xkb::KEY_Cyrillic_i, '\u{438}'),
        ); // 0x6c9
        hash.insert(
            String::from("Cyrillic_shorti"),
            Unicode::new(xkb::KEY_Cyrillic_shorti, '\u{439}'),
        ); // 0x6ca
        hash.insert(
            String::from("Cyrillic_ka"),
            Unicode::new(xkb::KEY_Cyrillic_ka, '\u{43a}'),
        ); // 0x6cb
        hash.insert(
            String::from("Cyrillic_el"),
            Unicode::new(xkb::KEY_Cyrillic_el, '\u{43b}'),
        ); // 0x6cc
        hash.insert(
            String::from("Cyrillic_em"),
            Unicode::new(xkb::KEY_Cyrillic_em, '\u{43c}'),
        ); // 0x6cd
        hash.insert(
            String::from("Cyrillic_en"),
            Unicode::new(xkb::KEY_Cyrillic_en, '\u{43d}'),
        ); // 0x6ce
        hash.insert(
            String::from("Cyrillic_o"),
            Unicode::new(xkb::KEY_Cyrillic_o, '\u{43e}'),
        ); // 0x6cf
        hash.insert(
            String::from("Cyrillic_pe"),
            Unicode::new(xkb::KEY_Cyrillic_pe, '\u{43f}'),
        ); // 0x6d0
        hash.insert(
            String::from("Cyrillic_ya"),
            Unicode::new(xkb::KEY_Cyrillic_ya, '\u{44f}'),
        ); // 0x6d1
        hash.insert(
            String::from("Cyrillic_er"),
            Unicode::new(xkb::KEY_Cyrillic_er, '\u{440}'),
        ); // 0x6d2
        hash.insert(
            String::from("Cyrillic_es"),
            Unicode::new(xkb::KEY_Cyrillic_es, '\u{441}'),
        ); // 0x6d3
        hash.insert(
            String::from("Cyrillic_te"),
            Unicode::new(xkb::KEY_Cyrillic_te, '\u{442}'),
        ); // 0x6d4
        hash.insert(
            String::from("Cyrillic_u"),
            Unicode::new(xkb::KEY_Cyrillic_u, '\u{443}'),
        ); // 0x6d5
        hash.insert(
            String::from("Cyrillic_zhe"),
            Unicode::new(xkb::KEY_Cyrillic_zhe, '\u{436}'),
        ); // 0x6d6
        hash.insert(
            String::from("Cyrillic_ve"),
            Unicode::new(xkb::KEY_Cyrillic_ve, '\u{432}'),
        ); // 0x6d7
        hash.insert(
            String::from("Cyrillic_softsign"),
            Unicode::new(xkb::KEY_Cyrillic_softsign, '\u{44c}'),
        ); // 0x6d8
        hash.insert(
            String::from("Cyrillic_yeru"),
            Unicode::new(xkb::KEY_Cyrillic_yeru, '\u{44b}'),
        ); // 0x6d9
        hash.insert(
            String::from("Cyrillic_ze"),
            Unicode::new(xkb::KEY_Cyrillic_ze, '\u{437}'),
        ); // 0x6da
        hash.insert(
            String::from("Cyrillic_sha"),
            Unicode::new(xkb::KEY_Cyrillic_sha, '\u{448}'),
        ); // 0x6db
        hash.insert(
            String::from("Cyrillic_e"),
            Unicode::new(xkb::KEY_Cyrillic_e, '\u{44d}'),
        ); // 0x6dc
        hash.insert(
            String::from("Cyrillic_shcha"),
            Unicode::new(xkb::KEY_Cyrillic_shcha, '\u{449}'),
        ); // 0x6dd
        hash.insert(
            String::from("Cyrillic_che"),
            Unicode::new(xkb::KEY_Cyrillic_che, '\u{447}'),
        ); // 0x6de
        hash.insert(
            String::from("Cyrillic_hardsign"),
            Unicode::new(xkb::KEY_Cyrillic_hardsign, '\u{44a}'),
        ); // 0x6df
        hash.insert(
            String::from("Cyrillic_YU"),
            Unicode::new(xkb::KEY_Cyrillic_YU, '\u{42e}'),
        ); // 0x6e0
        hash.insert(
            String::from("Cyrillic_A"),
            Unicode::new(xkb::KEY_Cyrillic_A, '\u{410}'),
        ); // 0x6e1
        hash.insert(
            String::from("Cyrillic_BE"),
            Unicode::new(xkb::KEY_Cyrillic_BE, '\u{411}'),
        ); // 0x6e2
        hash.insert(
            String::from("Cyrillic_TSE"),
            Unicode::new(xkb::KEY_Cyrillic_TSE, '\u{426}'),
        ); // 0x6e3
        hash.insert(
            String::from("Cyrillic_DE"),
            Unicode::new(xkb::KEY_Cyrillic_DE, '\u{414}'),
        ); // 0x6e4
        hash.insert(
            String::from("Cyrillic_IE"),
            Unicode::new(xkb::KEY_Cyrillic_IE, '\u{415}'),
        ); // 0x6e5
        hash.insert(
            String::from("Cyrillic_EF"),
            Unicode::new(xkb::KEY_Cyrillic_EF, '\u{424}'),
        ); // 0x6e6
        hash.insert(
            String::from("Cyrillic_GHE"),
            Unicode::new(xkb::KEY_Cyrillic_GHE, '\u{413}'),
        ); // 0x6e7
        hash.insert(
            String::from("Cyrillic_HA"),
            Unicode::new(xkb::KEY_Cyrillic_HA, '\u{425}'),
        ); // 0x6e8
        hash.insert(
            String::from("Cyrillic_I"),
            Unicode::new(xkb::KEY_Cyrillic_I, '\u{418}'),
        ); // 0x6e9
        hash.insert(
            String::from("Cyrillic_SHORTI"),
            Unicode::new(xkb::KEY_Cyrillic_SHORTI, '\u{419}'),
        ); // 0x6ea
        hash.insert(
            String::from("Cyrillic_KA"),
            Unicode::new(xkb::KEY_Cyrillic_KA, '\u{41a}'),
        ); // 0x6eb
        hash.insert(
            String::from("Cyrillic_EL"),
            Unicode::new(xkb::KEY_Cyrillic_EL, '\u{41b}'),
        ); // 0x6ec
        hash.insert(
            String::from("Cyrillic_EM"),
            Unicode::new(xkb::KEY_Cyrillic_EM, '\u{41c}'),
        ); // 0x6ed
        hash.insert(
            String::from("Cyrillic_EN"),
            Unicode::new(xkb::KEY_Cyrillic_EN, '\u{41d}'),
        ); // 0x6ee
        hash.insert(
            String::from("Cyrillic_O"),
            Unicode::new(xkb::KEY_Cyrillic_O, '\u{41e}'),
        ); // 0x6ef
        hash.insert(
            String::from("Cyrillic_PE"),
            Unicode::new(xkb::KEY_Cyrillic_PE, '\u{41f}'),
        ); // 0x6f0
        hash.insert(
            String::from("Cyrillic_YA"),
            Unicode::new(xkb::KEY_Cyrillic_YA, '\u{42f}'),
        ); // 0x6f1
        hash.insert(
            String::from("Cyrillic_ER"),
            Unicode::new(xkb::KEY_Cyrillic_ER, '\u{420}'),
        ); // 0x6f2
        hash.insert(
            String::from("Cyrillic_ES"),
            Unicode::new(xkb::KEY_Cyrillic_ES, '\u{421}'),
        ); // 0x6f3
        hash.insert(
            String::from("Cyrillic_TE"),
            Unicode::new(xkb::KEY_Cyrillic_TE, '\u{422}'),
        ); // 0x6f4
        hash.insert(
            String::from("Cyrillic_U"),
            Unicode::new(xkb::KEY_Cyrillic_U, '\u{423}'),
        ); // 0x6f5
        hash.insert(
            String::from("Cyrillic_ZHE"),
            Unicode::new(xkb::KEY_Cyrillic_ZHE, '\u{416}'),
        ); // 0x6f6
        hash.insert(
            String::from("Cyrillic_VE"),
            Unicode::new(xkb::KEY_Cyrillic_VE, '\u{412}'),
        ); // 0x6f7
        hash.insert(
            String::from("Cyrillic_SOFTSIGN"),
            Unicode::new(xkb::KEY_Cyrillic_SOFTSIGN, '\u{42c}'),
        ); // 0x6f8
        hash.insert(
            String::from("Cyrillic_YERU"),
            Unicode::new(xkb::KEY_Cyrillic_YERU, '\u{42b}'),
        ); // 0x6f9
        hash.insert(
            String::from("Cyrillic_ZE"),
            Unicode::new(xkb::KEY_Cyrillic_ZE, '\u{417}'),
        ); // 0x6fa
        hash.insert(
            String::from("Cyrillic_SHA"),
            Unicode::new(xkb::KEY_Cyrillic_SHA, '\u{428}'),
        ); // 0x6fb
        hash.insert(
            String::from("Cyrillic_E"),
            Unicode::new(xkb::KEY_Cyrillic_E, '\u{42d}'),
        ); // 0x6fc
        hash.insert(
            String::from("Cyrillic_SHCHA"),
            Unicode::new(xkb::KEY_Cyrillic_SHCHA, '\u{429}'),
        ); // 0x6fd
        hash.insert(
            String::from("Cyrillic_CHE"),
            Unicode::new(xkb::KEY_Cyrillic_CHE, '\u{427}'),
        ); // 0x6fe
        hash.insert(
            String::from("Cyrillic_HARDSIGN"),
            Unicode::new(xkb::KEY_Cyrillic_HARDSIGN, '\u{42a}'),
        ); // 0x6ff
        hash.insert(
            String::from("Greek_ALPHAaccent"),
            Unicode::new(xkb::KEY_Greek_ALPHAaccent, '\u{386}'),
        ); // 0x7a1
        hash.insert(
            String::from("Greek_EPSILONaccent"),
            Unicode::new(xkb::KEY_Greek_EPSILONaccent, '\u{388}'),
        ); // 0x7a2
        hash.insert(
            String::from("Greek_ETAaccent"),
            Unicode::new(xkb::KEY_Greek_ETAaccent, '\u{389}'),
        ); // 0x7a3
        hash.insert(
            String::from("Greek_IOTAaccent"),
            Unicode::new(xkb::KEY_Greek_IOTAaccent, '\u{38a}'),
        ); // 0x7a4
        hash.insert(
            String::from("Greek_IOTAdiaeresis"),
            Unicode::new(xkb::KEY_Greek_IOTAdiaeresis, '\u{3aa}'),
        ); // 0x7a5
        hash.insert(
            String::from("Greek_OMICRONaccent"),
            Unicode::new(xkb::KEY_Greek_OMICRONaccent, '\u{38c}'),
        ); // 0x7a7
        hash.insert(
            String::from("Greek_UPSILONaccent"),
            Unicode::new(xkb::KEY_Greek_UPSILONaccent, '\u{38e}'),
        ); // 0x7a8
        hash.insert(
            String::from("Greek_UPSILONdieresis"),
            Unicode::new(xkb::KEY_Greek_UPSILONdieresis, '\u{3ab}'),
        ); // 0x7a9
        hash.insert(
            String::from("Greek_OMEGAaccent"),
            Unicode::new(xkb::KEY_Greek_OMEGAaccent, '\u{38f}'),
        ); // 0x7ab
        hash.insert(
            String::from("Greek_accentdieresis"),
            Unicode::new(xkb::KEY_Greek_accentdieresis, '\u{385}'),
        ); // 0x7ae
        hash.insert(
            String::from("Greek_horizbar"),
            Unicode::new(xkb::KEY_Greek_horizbar, '\u{2015}'),
        ); // 0x7af
        hash.insert(
            String::from("Greek_alphaaccent"),
            Unicode::new(xkb::KEY_Greek_alphaaccent, '\u{3ac}'),
        ); // 0x7b1
        hash.insert(
            String::from("Greek_epsilonaccent"),
            Unicode::new(xkb::KEY_Greek_epsilonaccent, '\u{3ad}'),
        ); // 0x7b2
        hash.insert(
            String::from("Greek_etaaccent"),
            Unicode::new(xkb::KEY_Greek_etaaccent, '\u{3ae}'),
        ); // 0x7b3
        hash.insert(
            String::from("Greek_iotaaccent"),
            Unicode::new(xkb::KEY_Greek_iotaaccent, '\u{3af}'),
        ); // 0x7b4
        hash.insert(
            String::from("Greek_iotadieresis"),
            Unicode::new(xkb::KEY_Greek_iotadieresis, '\u{3ca}'),
        ); // 0x7b5
        hash.insert(
            String::from("Greek_iotaaccentdieresis"),
            Unicode::new(xkb::KEY_Greek_iotaaccentdieresis, '\u{390}'),
        ); // 0x7b6
        hash.insert(
            String::from("Greek_omicronaccent"),
            Unicode::new(xkb::KEY_Greek_omicronaccent, '\u{3cc}'),
        ); // 0x7b7
        hash.insert(
            String::from("Greek_upsilonaccent"),
            Unicode::new(xkb::KEY_Greek_upsilonaccent, '\u{3cd}'),
        ); // 0x7b8
        hash.insert(
            String::from("Greek_upsilondieresis"),
            Unicode::new(xkb::KEY_Greek_upsilondieresis, '\u{3cb}'),
        ); // 0x7b9
        hash.insert(
            String::from("Greek_upsilonaccentdieresis"),
            Unicode::new(xkb::KEY_Greek_upsilonaccentdieresis, '\u{3b0}'),
        ); // 0x7ba
        hash.insert(
            String::from("Greek_omegaaccent"),
            Unicode::new(xkb::KEY_Greek_omegaaccent, '\u{3ce}'),
        ); // 0x7bb
        hash.insert(
            String::from("Greek_ALPHA"),
            Unicode::new(xkb::KEY_Greek_ALPHA, '\u{391}'),
        ); // 0x7c1
        hash.insert(
            String::from("Greek_BETA"),
            Unicode::new(xkb::KEY_Greek_BETA, '\u{392}'),
        ); // 0x7c2
        hash.insert(
            String::from("Greek_GAMMA"),
            Unicode::new(xkb::KEY_Greek_GAMMA, '\u{393}'),
        ); // 0x7c3
        hash.insert(
            String::from("Greek_DELTA"),
            Unicode::new(xkb::KEY_Greek_DELTA, '\u{394}'),
        ); // 0x7c4
        hash.insert(
            String::from("Greek_EPSILON"),
            Unicode::new(xkb::KEY_Greek_EPSILON, '\u{395}'),
        ); // 0x7c5
        hash.insert(
            String::from("Greek_ZETA"),
            Unicode::new(xkb::KEY_Greek_ZETA, '\u{396}'),
        ); // 0x7c6
        hash.insert(
            String::from("Greek_ETA"),
            Unicode::new(xkb::KEY_Greek_ETA, '\u{397}'),
        ); // 0x7c7
        hash.insert(
            String::from("Greek_THETA"),
            Unicode::new(xkb::KEY_Greek_THETA, '\u{398}'),
        ); // 0x7c8
        hash.insert(
            String::from("Greek_IOTA"),
            Unicode::new(xkb::KEY_Greek_IOTA, '\u{399}'),
        ); // 0x7c9
        hash.insert(
            String::from("Greek_KAPPA"),
            Unicode::new(xkb::KEY_Greek_KAPPA, '\u{39a}'),
        ); // 0x7ca
        hash.insert(
            String::from("Greek_LAMDA"),
            Unicode::new(xkb::KEY_Greek_LAMDA, '\u{39b}'),
        ); // 0x7cb
        hash.insert(
            String::from("Greek_LAMBDA"),
            Unicode::new(xkb::KEY_Greek_LAMBDA, '\u{39b}'),
        ); // 0x7cb
        hash.insert(
            String::from("Greek_MU"),
            Unicode::new(xkb::KEY_Greek_MU, '\u{39c}'),
        ); // 0x7cc
        hash.insert(
            String::from("Greek_NU"),
            Unicode::new(xkb::KEY_Greek_NU, '\u{39d}'),
        ); // 0x7cd
        hash.insert(
            String::from("Greek_XI"),
            Unicode::new(xkb::KEY_Greek_XI, '\u{39e}'),
        ); // 0x7ce
        hash.insert(
            String::from("Greek_OMICRON"),
            Unicode::new(xkb::KEY_Greek_OMICRON, '\u{39f}'),
        ); // 0x7cf
        hash.insert(
            String::from("Greek_PI"),
            Unicode::new(xkb::KEY_Greek_PI, '\u{3a0}'),
        ); // 0x7d0
        hash.insert(
            String::from("Greek_RHO"),
            Unicode::new(xkb::KEY_Greek_RHO, '\u{3a1}'),
        ); // 0x7d1
        hash.insert(
            String::from("Greek_SIGMA"),
            Unicode::new(xkb::KEY_Greek_SIGMA, '\u{3a3}'),
        ); // 0x7d2
        hash.insert(
            String::from("Greek_TAU"),
            Unicode::new(xkb::KEY_Greek_TAU, '\u{3a4}'),
        ); // 0x7d4
        hash.insert(
            String::from("Greek_UPSILON"),
            Unicode::new(xkb::KEY_Greek_UPSILON, '\u{3a5}'),
        ); // 0x7d5
        hash.insert(
            String::from("Greek_PHI"),
            Unicode::new(xkb::KEY_Greek_PHI, '\u{3a6}'),
        ); // 0x7d6
        hash.insert(
            String::from("Greek_CHI"),
            Unicode::new(xkb::KEY_Greek_CHI, '\u{3a7}'),
        ); // 0x7d7
        hash.insert(
            String::from("Greek_PSI"),
            Unicode::new(xkb::KEY_Greek_PSI, '\u{3a8}'),
        ); // 0x7d8
        hash.insert(
            String::from("Greek_OMEGA"),
            Unicode::new(xkb::KEY_Greek_OMEGA, '\u{3a9}'),
        ); // 0x7d9
        hash.insert(
            String::from("Greek_alpha"),
            Unicode::new(xkb::KEY_Greek_alpha, '\u{3b1}'),
        ); // 0x7e1
        hash.insert(
            String::from("Greek_beta"),
            Unicode::new(xkb::KEY_Greek_beta, '\u{3b2}'),
        ); // 0x7e2
        hash.insert(
            String::from("Greek_gamma"),
            Unicode::new(xkb::KEY_Greek_gamma, '\u{3b3}'),
        ); // 0x7e3
        hash.insert(
            String::from("Greek_delta"),
            Unicode::new(xkb::KEY_Greek_delta, '\u{3b4}'),
        ); // 0x7e4
        hash.insert(
            String::from("Greek_epsilon"),
            Unicode::new(xkb::KEY_Greek_epsilon, '\u{3b5}'),
        ); // 0x7e5
        hash.insert(
            String::from("Greek_zeta"),
            Unicode::new(xkb::KEY_Greek_zeta, '\u{3b6}'),
        ); // 0x7e6
        hash.insert(
            String::from("Greek_eta"),
            Unicode::new(xkb::KEY_Greek_eta, '\u{3b7}'),
        ); // 0x7e7
        hash.insert(
            String::from("Greek_theta"),
            Unicode::new(xkb::KEY_Greek_theta, '\u{3b8}'),
        ); // 0x7e8
        hash.insert(
            String::from("Greek_iota"),
            Unicode::new(xkb::KEY_Greek_iota, '\u{3b9}'),
        ); // 0x7e9
        hash.insert(
            String::from("Greek_kappa"),
            Unicode::new(xkb::KEY_Greek_kappa, '\u{3ba}'),
        ); // 0x7ea
        hash.insert(
            String::from("Greek_lamda"),
            Unicode::new(xkb::KEY_Greek_lamda, '\u{0}'),
        ); // 0x7eb
        hash.insert(
            String::from("Greek_lambda"),
            Unicode::new(xkb::KEY_Greek_lambda, '\u{3bb}'),
        ); // 0x7eb
        hash.insert(
            String::from("Greek_mu"),
            Unicode::new(xkb::KEY_Greek_mu, '\u{3bc}'),
        ); // 0x7ec
        hash.insert(
            String::from("Greek_nu"),
            Unicode::new(xkb::KEY_Greek_nu, '\u{3bd}'),
        ); // 0x7ed
        hash.insert(
            String::from("Greek_xi"),
            Unicode::new(xkb::KEY_Greek_xi, '\u{3be}'),
        ); // 0x7ee
        hash.insert(
            String::from("Greek_omicron"),
            Unicode::new(xkb::KEY_Greek_omicron, '\u{3bf}'),
        ); // 0x7ef
        hash.insert(
            String::from("Greek_pi"),
            Unicode::new(xkb::KEY_Greek_pi, '\u{3c0}'),
        ); // 0x7f0
        hash.insert(
            String::from("Greek_rho"),
            Unicode::new(xkb::KEY_Greek_rho, '\u{3c1}'),
        ); // 0x7f1
        hash.insert(
            String::from("Greek_sigma"),
            Unicode::new(xkb::KEY_Greek_sigma, '\u{3c3}'),
        ); // 0x7f2
        hash.insert(
            String::from("Greek_finalsmallsigma"),
            Unicode::new(xkb::KEY_Greek_finalsmallsigma, '\u{3c2}'),
        ); // 0x7f3
        hash.insert(
            String::from("Greek_tau"),
            Unicode::new(xkb::KEY_Greek_tau, '\u{3c4}'),
        ); // 0x7f4
        hash.insert(
            String::from("Greek_upsilon"),
            Unicode::new(xkb::KEY_Greek_upsilon, '\u{3c5}'),
        ); // 0x7f5
        hash.insert(
            String::from("Greek_phi"),
            Unicode::new(xkb::KEY_Greek_phi, '\u{3c6}'),
        ); // 0x7f6
        hash.insert(
            String::from("Greek_chi"),
            Unicode::new(xkb::KEY_Greek_chi, '\u{3c7}'),
        ); // 0x7f7
        hash.insert(
            String::from("Greek_psi"),
            Unicode::new(xkb::KEY_Greek_psi, '\u{3c8}'),
        ); // 0x7f8
        hash.insert(
            String::from("Greek_omega"),
            Unicode::new(xkb::KEY_Greek_omega, '\u{3c9}'),
        ); // 0x7f9
        hash.insert(
            String::from("Greek_switch"),
            Unicode::new(xkb::KEY_Greek_switch, '\u{0}'),
        ); // 0xFF7E
        hash.insert(
            String::from("leftradical"),
            Unicode::new(xkb::KEY_leftradical, '\u{23b7}'),
        ); // 0x8a1
        hash.insert(
            String::from("topleftradical"),
            Unicode::new(xkb::KEY_topleftradical, '\u{250c}'),
        ); // 0x8a2
        hash.insert(
            String::from("horizconnector"),
            Unicode::new(xkb::KEY_horizconnector, '\u{2500}'),
        ); // 0x8a3
        hash.insert(
            String::from("topintegral"),
            Unicode::new(xkb::KEY_topintegral, '\u{2320}'),
        ); // 0x8a4
        hash.insert(
            String::from("botintegral"),
            Unicode::new(xkb::KEY_botintegral, '\u{2321}'),
        ); // 0x8a5
        hash.insert(
            String::from("vertconnector"),
            Unicode::new(xkb::KEY_vertconnector, '\u{2502}'),
        ); // 0x8a6
        hash.insert(
            String::from("topleftsqbracket"),
            Unicode::new(xkb::KEY_topleftsqbracket, '\u{23a1}'),
        ); // 0x8a7
        hash.insert(
            String::from("botleftsqbracket"),
            Unicode::new(xkb::KEY_botleftsqbracket, '\u{23a3}'),
        ); // 0x8a8
        hash.insert(
            String::from("toprightsqbracket"),
            Unicode::new(xkb::KEY_toprightsqbracket, '\u{23a4}'),
        ); // 0x8a9
        hash.insert(
            String::from("botrightsqbracket"),
            Unicode::new(xkb::KEY_botrightsqbracket, '\u{23a6}'),
        ); // 0x8aa
        hash.insert(
            String::from("topleftparens"),
            Unicode::new(xkb::KEY_topleftparens, '\u{239b}'),
        ); // 0x8ab
        hash.insert(
            String::from("botleftparens"),
            Unicode::new(xkb::KEY_botleftparens, '\u{239d}'),
        ); // 0x8ac
        hash.insert(
            String::from("toprightparens"),
            Unicode::new(xkb::KEY_toprightparens, '\u{239e}'),
        ); // 0x8ad
        hash.insert(
            String::from("botrightparens"),
            Unicode::new(xkb::KEY_botrightparens, '\u{23a0}'),
        ); // 0x8ae
        hash.insert(
            String::from("leftmiddlecurlybrace"),
            Unicode::new(xkb::KEY_leftmiddlecurlybrace, '\u{23a8}'),
        ); // 0x8af
        hash.insert(
            String::from("rightmiddlecurlybrace"),
            Unicode::new(xkb::KEY_rightmiddlecurlybrace, '\u{23ac}'),
        ); // 0x8b0
        hash.insert(
            String::from("topleftsummation"),
            Unicode::new(xkb::KEY_topleftsummation, '\u{0}'),
        ); // 0x8b1
        hash.insert(
            String::from("botleftsummation"),
            Unicode::new(xkb::KEY_botleftsummation, '\u{0}'),
        ); // 0x8b2
        hash.insert(
            String::from("topvertsummationconnector"),
            Unicode::new(xkb::KEY_topvertsummationconnector, '\u{0}'),
        ); // 0x8b3
        hash.insert(
            String::from("botvertsummationconnector"),
            Unicode::new(xkb::KEY_botvertsummationconnector, '\u{0}'),
        ); // 0x8b4
        hash.insert(
            String::from("toprightsummation"),
            Unicode::new(xkb::KEY_toprightsummation, '\u{0}'),
        ); // 0x8b5
        hash.insert(
            String::from("botrightsummation"),
            Unicode::new(xkb::KEY_botrightsummation, '\u{0}'),
        ); // 0x8b6
        hash.insert(
            String::from("rightmiddlesummation"),
            Unicode::new(xkb::KEY_rightmiddlesummation, '\u{0}'),
        ); // 0x8b7
        hash.insert(
            String::from("lessthanequal"),
            Unicode::new(xkb::KEY_lessthanequal, '\u{2264}'),
        ); // 0x8bc
        hash.insert(
            String::from("notequal"),
            Unicode::new(xkb::KEY_notequal, '\u{2260}'),
        ); // 0x8bd
        hash.insert(
            String::from("greaterthanequal"),
            Unicode::new(xkb::KEY_greaterthanequal, '\u{2265}'),
        ); // 0x8be
        hash.insert(
            String::from("integral"),
            Unicode::new(xkb::KEY_integral, '\u{222b}'),
        ); // 0x8bf
        hash.insert(
            String::from("therefore"),
            Unicode::new(xkb::KEY_therefore, '\u{2234}'),
        ); // 0x8c0
        hash.insert(
            String::from("variation"),
            Unicode::new(xkb::KEY_variation, '\u{221d}'),
        ); // 0x8c1
        hash.insert(
            String::from("infinity"),
            Unicode::new(xkb::KEY_infinity, '\u{221e}'),
        ); // 0x8c2
        hash.insert(
            String::from("nabla"),
            Unicode::new(xkb::KEY_nabla, '\u{2207}'),
        ); // 0x8c5
        hash.insert(
            String::from("approximate"),
            Unicode::new(xkb::KEY_approximate, '\u{223c}'),
        ); // 0x8c8
        hash.insert(
            String::from("similarequal"),
            Unicode::new(xkb::KEY_similarequal, '\u{2243}'),
        ); // 0x8c9
        hash.insert(
            String::from("ifonlyif"),
            Unicode::new(xkb::KEY_ifonlyif, '\u{21d4}'),
        ); // 0x8cd
        hash.insert(
            String::from("implies"),
            Unicode::new(xkb::KEY_implies, '\u{21d2}'),
        ); // 0x8ce
        hash.insert(
            String::from("identical"),
            Unicode::new(xkb::KEY_identical, '\u{2261}'),
        ); // 0x8cf
        hash.insert(
            String::from("radical"),
            Unicode::new(xkb::KEY_radical, '\u{221a}'),
        ); // 0x8d6
        hash.insert(
            String::from("includedin"),
            Unicode::new(xkb::KEY_includedin, '\u{2282}'),
        ); // 0x8da
        hash.insert(
            String::from("includes"),
            Unicode::new(xkb::KEY_includes, '\u{2283}'),
        ); // 0x8db
        hash.insert(
            String::from("intersection"),
            Unicode::new(xkb::KEY_intersection, '\u{2229}'),
        ); // 0x8dc
        hash.insert(
            String::from("union"),
            Unicode::new(xkb::KEY_union, '\u{222a}'),
        ); // 0x8dd
        hash.insert(
            String::from("logicaland"),
            Unicode::new(xkb::KEY_logicaland, '\u{2227}'),
        ); // 0x8de
        hash.insert(
            String::from("logicalor"),
            Unicode::new(xkb::KEY_logicalor, '\u{2228}'),
        ); // 0x8df
        hash.insert(
            String::from("partialderivative"),
            Unicode::new(xkb::KEY_partialderivative, '\u{2202}'),
        ); // 0x8ef
        hash.insert(
            String::from("function"),
            Unicode::new(xkb::KEY_function, '\u{192}'),
        ); // 0x8f6
        hash.insert(
            String::from("leftarrow"),
            Unicode::new(xkb::KEY_leftarrow, '\u{2190}'),
        ); // 0x8fb
        hash.insert(
            String::from("uparrow"),
            Unicode::new(xkb::KEY_uparrow, '\u{2191}'),
        ); // 0x8fc
        hash.insert(
            String::from("rightarrow"),
            Unicode::new(xkb::KEY_rightarrow, '\u{2192}'),
        ); // 0x8fd
        hash.insert(
            String::from("downarrow"),
            Unicode::new(xkb::KEY_downarrow, '\u{2193}'),
        ); // 0x8fe
        hash.insert(String::from("blank"), Unicode::new(xkb::KEY_blank, '\u{0}')); // 0x9df
        hash.insert(
            String::from("soliddiamond"),
            Unicode::new(xkb::KEY_soliddiamond, '\u{25c6}'),
        ); // 0x9e0
        hash.insert(
            String::from("checkerboard"),
            Unicode::new(xkb::KEY_checkerboard, '\u{2592}'),
        ); // 0x9e1
        hash.insert(String::from("ht"), Unicode::new(xkb::KEY_ht, '\u{2409}')); // 0x9e2
        hash.insert(String::from("ff"), Unicode::new(xkb::KEY_ff, '\u{240c}')); // 0x9e3
        hash.insert(String::from("cr"), Unicode::new(xkb::KEY_cr, '\u{240d}')); // 0x9e4
        hash.insert(String::from("lf"), Unicode::new(xkb::KEY_lf, '\u{240a}')); // 0x9e5
        hash.insert(String::from("nl"), Unicode::new(xkb::KEY_nl, '\u{2424}')); // 0x9e8
        hash.insert(String::from("vt"), Unicode::new(xkb::KEY_vt, '\u{240b}')); // 0x9e9
        hash.insert(
            String::from("lowrightcorner"),
            Unicode::new(xkb::KEY_lowrightcorner, '\u{2518}'),
        ); // 0x9ea
        hash.insert(
            String::from("uprightcorner"),
            Unicode::new(xkb::KEY_uprightcorner, '\u{2510}'),
        ); // 0x9eb
        hash.insert(
            String::from("upleftcorner"),
            Unicode::new(xkb::KEY_upleftcorner, '\u{250c}'),
        ); // 0x9ec
        hash.insert(
            String::from("lowleftcorner"),
            Unicode::new(xkb::KEY_lowleftcorner, '\u{2514}'),
        ); // 0x9ed
        hash.insert(
            String::from("crossinglines"),
            Unicode::new(xkb::KEY_crossinglines, '\u{253c}'),
        ); // 0x9ee
        hash.insert(
            String::from("horizlinescan1"),
            Unicode::new(xkb::KEY_horizlinescan1, '\u{23ba}'),
        ); // 0x9ef
        hash.insert(
            String::from("horizlinescan3"),
            Unicode::new(xkb::KEY_horizlinescan3, '\u{23bb}'),
        ); // 0x9f0
        hash.insert(
            String::from("horizlinescan5"),
            Unicode::new(xkb::KEY_horizlinescan5, '\u{2500}'),
        ); // 0x9f1
        hash.insert(
            String::from("horizlinescan7"),
            Unicode::new(xkb::KEY_horizlinescan7, '\u{23bc}'),
        ); // 0x9f2
        hash.insert(
            String::from("horizlinescan9"),
            Unicode::new(xkb::KEY_horizlinescan9, '\u{23bd}'),
        ); // 0x9f3
        hash.insert(
            String::from("leftt"),
            Unicode::new(xkb::KEY_leftt, '\u{251c}'),
        ); // 0x9f4
        hash.insert(
            String::from("rightt"),
            Unicode::new(xkb::KEY_rightt, '\u{2524}'),
        ); // 0x9f5
        hash.insert(
            String::from("bott"),
            Unicode::new(xkb::KEY_bott, '\u{2534}'),
        ); // 0x9f6
        hash.insert(
            String::from("topt"),
            Unicode::new(xkb::KEY_topt, '\u{252c}'),
        ); // 0x9f7
        hash.insert(
            String::from("vertbar"),
            Unicode::new(xkb::KEY_vertbar, '\u{2502}'),
        ); // 0x9f8
        hash.insert(
            String::from("emspace"),
            Unicode::new(xkb::KEY_emspace, '\u{2003}'),
        ); // 0xaa1
        hash.insert(
            String::from("enspace"),
            Unicode::new(xkb::KEY_enspace, '\u{2002}'),
        ); // 0xaa2
        hash.insert(
            String::from("em3space"),
            Unicode::new(xkb::KEY_em3space, '\u{2004}'),
        ); // 0xaa3
        hash.insert(
            String::from("em4space"),
            Unicode::new(xkb::KEY_em4space, '\u{2005}'),
        ); // 0xaa4
        hash.insert(
            String::from("digitspace"),
            Unicode::new(xkb::KEY_digitspace, '\u{2007}'),
        ); // 0xaa5
        hash.insert(
            String::from("punctspace"),
            Unicode::new(xkb::KEY_punctspace, '\u{2008}'),
        ); // 0xaa6
        hash.insert(
            String::from("thinspace"),
            Unicode::new(xkb::KEY_thinspace, '\u{2009}'),
        ); // 0xaa7
        hash.insert(
            String::from("hairspace"),
            Unicode::new(xkb::KEY_hairspace, '\u{200a}'),
        ); // 0xaa8
        hash.insert(
            String::from("emdash"),
            Unicode::new(xkb::KEY_emdash, '\u{2014}'),
        ); // 0xaa9
        hash.insert(
            String::from("endash"),
            Unicode::new(xkb::KEY_endash, '\u{2013}'),
        ); // 0xaaa
        hash.insert(
            String::from("signifblank"),
            Unicode::new(xkb::KEY_signifblank, '\u{2423}'),
        ); // 0xaac
        hash.insert(
            String::from("ellipsis"),
            Unicode::new(xkb::KEY_ellipsis, '\u{2026}'),
        ); // 0xaae
        hash.insert(
            String::from("doubbaselinedot"),
            Unicode::new(xkb::KEY_doubbaselinedot, '\u{2025}'),
        ); // 0xaaf
        hash.insert(
            String::from("onethird"),
            Unicode::new(xkb::KEY_onethird, '\u{2153}'),
        ); // 0xab0
        hash.insert(
            String::from("twothirds"),
            Unicode::new(xkb::KEY_twothirds, '\u{2154}'),
        ); // 0xab1
        hash.insert(
            String::from("onefifth"),
            Unicode::new(xkb::KEY_onefifth, '\u{2155}'),
        ); // 0xab2
        hash.insert(
            String::from("twofifths"),
            Unicode::new(xkb::KEY_twofifths, '\u{2156}'),
        ); // 0xab3
        hash.insert(
            String::from("threefifths"),
            Unicode::new(xkb::KEY_threefifths, '\u{2157}'),
        ); // 0xab4
        hash.insert(
            String::from("fourfifths"),
            Unicode::new(xkb::KEY_fourfifths, '\u{2158}'),
        ); // 0xab5
        hash.insert(
            String::from("onesixth"),
            Unicode::new(xkb::KEY_onesixth, '\u{2159}'),
        ); // 0xab6
        hash.insert(
            String::from("fivesixths"),
            Unicode::new(xkb::KEY_fivesixths, '\u{215a}'),
        ); // 0xab7
        hash.insert(
            String::from("careof"),
            Unicode::new(xkb::KEY_careof, '\u{2105}'),
        ); // 0xab8
        hash.insert(
            String::from("figdash"),
            Unicode::new(xkb::KEY_figdash, '\u{2012}'),
        ); // 0xabb
        hash.insert(
            String::from("leftanglebracket"),
            Unicode::new(xkb::KEY_leftanglebracket, '\u{27e8}'),
        ); // 0xabc
        hash.insert(
            String::from("decimalpoint"),
            Unicode::new(xkb::KEY_decimalpoint, '\u{2e}'),
        ); // 0xabd
        hash.insert(
            String::from("rightanglebracket"),
            Unicode::new(xkb::KEY_rightanglebracket, '\u{27e9}'),
        ); // 0xabe
        hash.insert(
            String::from("marker"),
            Unicode::new(xkb::KEY_marker, '\u{0}'),
        ); // 0xabf
        hash.insert(
            String::from("oneeighth"),
            Unicode::new(xkb::KEY_oneeighth, '\u{215b}'),
        ); // 0xac3
        hash.insert(
            String::from("threeeighths"),
            Unicode::new(xkb::KEY_threeeighths, '\u{215c}'),
        ); // 0xac4
        hash.insert(
            String::from("fiveeighths"),
            Unicode::new(xkb::KEY_fiveeighths, '\u{215d}'),
        ); // 0xac5
        hash.insert(
            String::from("seveneighths"),
            Unicode::new(xkb::KEY_seveneighths, '\u{215e}'),
        ); // 0xac6
        hash.insert(
            String::from("trademark"),
            Unicode::new(xkb::KEY_trademark, '\u{2122}'),
        ); // 0xac9
        hash.insert(
            String::from("signaturemark"),
            Unicode::new(xkb::KEY_signaturemark, '\u{2613}'),
        ); // 0xaca
        hash.insert(
            String::from("trademarkincircle"),
            Unicode::new(xkb::KEY_trademarkincircle, '\u{0}'),
        ); // 0xacb
        hash.insert(
            String::from("leftopentriangle"),
            Unicode::new(xkb::KEY_leftopentriangle, '\u{25c1}'),
        ); // 0xacc
        hash.insert(
            String::from("rightopentriangle"),
            Unicode::new(xkb::KEY_rightopentriangle, '\u{25b7}'),
        ); // 0xacd
        hash.insert(
            String::from("emopencircle"),
            Unicode::new(xkb::KEY_emopencircle, '\u{25cb}'),
        ); // 0xace
        hash.insert(
            String::from("emopenrectangle"),
            Unicode::new(xkb::KEY_emopenrectangle, '\u{25af}'),
        ); // 0xacf
        hash.insert(
            String::from("leftsinglequotemark"),
            Unicode::new(xkb::KEY_leftsinglequotemark, '\u{2018}'),
        ); // 0xad0
        hash.insert(
            String::from("rightsinglequotemark"),
            Unicode::new(xkb::KEY_rightsinglequotemark, '\u{2019}'),
        ); // 0xad1
        hash.insert(
            String::from("leftdoublequotemark"),
            Unicode::new(xkb::KEY_leftdoublequotemark, '\u{201c}'),
        ); // 0xad2
        hash.insert(
            String::from("rightdoublequotemark"),
            Unicode::new(xkb::KEY_rightdoublequotemark, '\u{201d}'),
        ); // 0xad3
        hash.insert(
            String::from("prescription"),
            Unicode::new(xkb::KEY_prescription, '\u{211e}'),
        ); // 0xad4
        hash.insert(
            String::from("minutes"),
            Unicode::new(xkb::KEY_minutes, '\u{2032}'),
        ); // 0xad6
        hash.insert(
            String::from("seconds"),
            Unicode::new(xkb::KEY_seconds, '\u{2033}'),
        ); // 0xad7
        hash.insert(
            String::from("latincross"),
            Unicode::new(xkb::KEY_latincross, '\u{271d}'),
        ); // 0xad9
        hash.insert(
            String::from("hexagram"),
            Unicode::new(xkb::KEY_hexagram, '\u{0}'),
        ); // 0xada
        hash.insert(
            String::from("filledrectbullet"),
            Unicode::new(xkb::KEY_filledrectbullet, '\u{25ac}'),
        ); // 0xadb
        hash.insert(
            String::from("filledlefttribullet"),
            Unicode::new(xkb::KEY_filledlefttribullet, '\u{25c0}'),
        ); // 0xadc
        hash.insert(
            String::from("filledrighttribullet"),
            Unicode::new(xkb::KEY_filledrighttribullet, '\u{25b6}'),
        ); // 0xadd
        hash.insert(
            String::from("emfilledcircle"),
            Unicode::new(xkb::KEY_emfilledcircle, '\u{25cf}'),
        ); // 0xade
        hash.insert(
            String::from("emfilledrect"),
            Unicode::new(xkb::KEY_emfilledrect, '\u{25ae}'),
        ); // 0xadf
        hash.insert(
            String::from("enopencircbullet"),
            Unicode::new(xkb::KEY_enopencircbullet, '\u{25e6}'),
        ); // 0xae0
        hash.insert(
            String::from("enopensquarebullet"),
            Unicode::new(xkb::KEY_enopensquarebullet, '\u{25ab}'),
        ); // 0xae1
        hash.insert(
            String::from("openrectbullet"),
            Unicode::new(xkb::KEY_openrectbullet, '\u{25ad}'),
        ); // 0xae2
        hash.insert(
            String::from("opentribulletup"),
            Unicode::new(xkb::KEY_opentribulletup, '\u{25b3}'),
        ); // 0xae3
        hash.insert(
            String::from("opentribulletdown"),
            Unicode::new(xkb::KEY_opentribulletdown, '\u{25bd}'),
        ); // 0xae4
        hash.insert(
            String::from("openstar"),
            Unicode::new(xkb::KEY_openstar, '\u{2606}'),
        ); // 0xae5
        hash.insert(
            String::from("enfilledcircbullet"),
            Unicode::new(xkb::KEY_enfilledcircbullet, '\u{2022}'),
        ); // 0xae6
        hash.insert(
            String::from("enfilledsqbullet"),
            Unicode::new(xkb::KEY_enfilledsqbullet, '\u{25aa}'),
        ); // 0xae7
        hash.insert(
            String::from("filledtribulletup"),
            Unicode::new(xkb::KEY_filledtribulletup, '\u{25b2}'),
        ); // 0xae8
        hash.insert(
            String::from("filledtribulletdown"),
            Unicode::new(xkb::KEY_filledtribulletdown, '\u{25bc}'),
        ); // 0xae9
        hash.insert(
            String::from("leftpointer"),
            Unicode::new(xkb::KEY_leftpointer, '\u{261c}'),
        ); // 0xaea
        hash.insert(
            String::from("rightpointer"),
            Unicode::new(xkb::KEY_rightpointer, '\u{261e}'),
        ); // 0xaeb
        hash.insert(
            String::from("club"),
            Unicode::new(xkb::KEY_club, '\u{2663}'),
        ); // 0xaec
        hash.insert(
            String::from("diamond"),
            Unicode::new(xkb::KEY_diamond, '\u{2666}'),
        ); // 0xaed
        hash.insert(
            String::from("heart"),
            Unicode::new(xkb::KEY_heart, '\u{2665}'),
        ); // 0xaee
        hash.insert(
            String::from("maltesecross"),
            Unicode::new(xkb::KEY_maltesecross, '\u{2720}'),
        ); // 0xaf0
        hash.insert(
            String::from("dagger"),
            Unicode::new(xkb::KEY_dagger, '\u{2020}'),
        ); // 0xaf1
        hash.insert(
            String::from("doubledagger"),
            Unicode::new(xkb::KEY_doubledagger, '\u{2021}'),
        ); // 0xaf2
        hash.insert(
            String::from("checkmark"),
            Unicode::new(xkb::KEY_checkmark, '\u{2713}'),
        ); // 0xaf3
        hash.insert(
            String::from("ballotcross"),
            Unicode::new(xkb::KEY_ballotcross, '\u{2717}'),
        ); // 0xaf4
        hash.insert(
            String::from("musicalsharp"),
            Unicode::new(xkb::KEY_musicalsharp, '\u{266f}'),
        ); // 0xaf5
        hash.insert(
            String::from("musicalflat"),
            Unicode::new(xkb::KEY_musicalflat, '\u{266d}'),
        ); // 0xaf6
        hash.insert(
            String::from("malesymbol"),
            Unicode::new(xkb::KEY_malesymbol, '\u{2642}'),
        ); // 0xaf7
        hash.insert(
            String::from("femalesymbol"),
            Unicode::new(xkb::KEY_femalesymbol, '\u{2640}'),
        ); // 0xaf8
        hash.insert(
            String::from("telephone"),
            Unicode::new(xkb::KEY_telephone, '\u{260e}'),
        ); // 0xaf9
        hash.insert(
            String::from("telephonerecorder"),
            Unicode::new(xkb::KEY_telephonerecorder, '\u{2315}'),
        ); // 0xafa
        hash.insert(
            String::from("phonographcopyright"),
            Unicode::new(xkb::KEY_phonographcopyright, '\u{2117}'),
        ); // 0xafb
        hash.insert(
            String::from("caret"),
            Unicode::new(xkb::KEY_caret, '\u{2038}'),
        ); // 0xafc
        hash.insert(
            String::from("singlelowquotemark"),
            Unicode::new(xkb::KEY_singlelowquotemark, '\u{201a}'),
        ); // 0xafd
        hash.insert(
            String::from("doublelowquotemark"),
            Unicode::new(xkb::KEY_doublelowquotemark, '\u{201e}'),
        ); // 0xafe
        hash.insert(
            String::from("cursor"),
            Unicode::new(xkb::KEY_cursor, '\u{0}'),
        ); // 0xaff
        hash.insert(
            String::from("leftcaret"),
            Unicode::new(xkb::KEY_leftcaret, '\u{3c}'),
        ); // 0xba3
        hash.insert(
            String::from("rightcaret"),
            Unicode::new(xkb::KEY_rightcaret, '\u{3e}'),
        ); // 0xba6
        hash.insert(
            String::from("downcaret"),
            Unicode::new(xkb::KEY_downcaret, '\u{2228}'),
        ); // 0xba8
        hash.insert(
            String::from("upcaret"),
            Unicode::new(xkb::KEY_upcaret, '\u{2227}'),
        ); // 0xba9
        hash.insert(
            String::from("overbar"),
            Unicode::new(xkb::KEY_overbar, '\u{af}'),
        ); // 0xbc0
        hash.insert(
            String::from("downtack"),
            Unicode::new(xkb::KEY_downtack, '\u{22a5}'),
        ); // 0xbc2
        hash.insert(
            String::from("upshoe"),
            Unicode::new(xkb::KEY_upshoe, '\u{2229}'),
        ); // 0xbc3
        hash.insert(
            String::from("downstile"),
            Unicode::new(xkb::KEY_downstile, '\u{230a}'),
        ); // 0xbc4
        hash.insert(
            String::from("underbar"),
            Unicode::new(xkb::KEY_underbar, '\u{5f}'),
        ); // 0xbc6
        hash.insert(String::from("jot"), Unicode::new(xkb::KEY_jot, '\u{2218}')); // 0xbca
        hash.insert(
            String::from("quad"),
            Unicode::new(xkb::KEY_quad, '\u{2395}'),
        ); // 0xbcc
        hash.insert(
            String::from("uptack"),
            Unicode::new(xkb::KEY_uptack, '\u{22a4}'),
        ); // 0xbce
        hash.insert(
            String::from("circle"),
            Unicode::new(xkb::KEY_circle, '\u{25cb}'),
        ); // 0xbcf
        hash.insert(
            String::from("upstile"),
            Unicode::new(xkb::KEY_upstile, '\u{2308}'),
        ); // 0xbd3
        hash.insert(
            String::from("downshoe"),
            Unicode::new(xkb::KEY_downshoe, '\u{222a}'),
        ); // 0xbd6
        hash.insert(
            String::from("rightshoe"),
            Unicode::new(xkb::KEY_rightshoe, '\u{2283}'),
        ); // 0xbd8
        hash.insert(
            String::from("leftshoe"),
            Unicode::new(xkb::KEY_leftshoe, '\u{2282}'),
        ); // 0xbda
        hash.insert(
            String::from("lefttack"),
            Unicode::new(xkb::KEY_lefttack, '\u{22a2}'),
        ); // 0xbdc
        hash.insert(
            String::from("righttack"),
            Unicode::new(xkb::KEY_righttack, '\u{22a3}'),
        ); // 0xbfc
        hash.insert(
            String::from("hebrew_doublelowline"),
            Unicode::new(xkb::KEY_hebrew_doublelowline, '\u{2017}'),
        ); // 0xcdf
        hash.insert(
            String::from("hebrew_aleph"),
            Unicode::new(xkb::KEY_hebrew_aleph, '\u{5d0}'),
        ); // 0xce0
        hash.insert(
            String::from("hebrew_bet"),
            Unicode::new(xkb::KEY_hebrew_bet, '\u{5d1}'),
        ); // 0xce1
        hash.insert(
            String::from("hebrew_beth"),
            Unicode::new(xkb::KEY_hebrew_beth, '\u{5d1}'),
        ); // 0xce1
        hash.insert(
            String::from("hebrew_gimel"),
            Unicode::new(xkb::KEY_hebrew_gimel, '\u{5d2}'),
        ); // 0xce2
        hash.insert(
            String::from("hebrew_gimmel"),
            Unicode::new(xkb::KEY_hebrew_gimmel, '\u{5d2}'),
        ); // 0xce2
        hash.insert(
            String::from("hebrew_dalet"),
            Unicode::new(xkb::KEY_hebrew_dalet, '\u{5d3}'),
        ); // 0xce3
        hash.insert(
            String::from("hebrew_daleth"),
            Unicode::new(xkb::KEY_hebrew_daleth, '\u{5d3}'),
        ); // 0xce3
        hash.insert(
            String::from("hebrew_he"),
            Unicode::new(xkb::KEY_hebrew_he, '\u{5d4}'),
        ); // 0xce4
        hash.insert(
            String::from("hebrew_waw"),
            Unicode::new(xkb::KEY_hebrew_waw, '\u{5d5}'),
        ); // 0xce5
        hash.insert(
            String::from("hebrew_zain"),
            Unicode::new(xkb::KEY_hebrew_zain, '\u{5d6}'),
        ); // 0xce6
        hash.insert(
            String::from("hebrew_zayin"),
            Unicode::new(xkb::KEY_hebrew_zayin, '\u{5d6}'),
        ); // 0xce6
        hash.insert(
            String::from("hebrew_chet"),
            Unicode::new(xkb::KEY_hebrew_chet, '\u{5d7}'),
        ); // 0xce7
        hash.insert(
            String::from("hebrew_het"),
            Unicode::new(xkb::KEY_hebrew_het, '\u{5d7}'),
        ); // 0xce7
        hash.insert(
            String::from("hebrew_tet"),
            Unicode::new(xkb::KEY_hebrew_tet, '\u{5d8}'),
        ); // 0xce8
        hash.insert(
            String::from("hebrew_teth"),
            Unicode::new(xkb::KEY_hebrew_teth, '\u{5d8}'),
        ); // 0xce8
        hash.insert(
            String::from("hebrew_yod"),
            Unicode::new(xkb::KEY_hebrew_yod, '\u{5d9}'),
        ); // 0xce9
        hash.insert(
            String::from("hebrew_finalkaph"),
            Unicode::new(xkb::KEY_hebrew_finalkaph, '\u{5da}'),
        ); // 0xcea
        hash.insert(
            String::from("hebrew_kaph"),
            Unicode::new(xkb::KEY_hebrew_kaph, '\u{5db}'),
        ); // 0xceb
        hash.insert(
            String::from("hebrew_lamed"),
            Unicode::new(xkb::KEY_hebrew_lamed, '\u{5dc}'),
        ); // 0xcec
        hash.insert(
            String::from("hebrew_finalmem"),
            Unicode::new(xkb::KEY_hebrew_finalmem, '\u{5dd}'),
        ); // 0xced
        hash.insert(
            String::from("hebrew_mem"),
            Unicode::new(xkb::KEY_hebrew_mem, '\u{5de}'),
        ); // 0xcee
        hash.insert(
            String::from("hebrew_finalnun"),
            Unicode::new(xkb::KEY_hebrew_finalnun, '\u{5df}'),
        ); // 0xcef
        hash.insert(
            String::from("hebrew_nun"),
            Unicode::new(xkb::KEY_hebrew_nun, '\u{5e0}'),
        ); // 0xcf0
        hash.insert(
            String::from("hebrew_samech"),
            Unicode::new(xkb::KEY_hebrew_samech, '\u{5e1}'),
        ); // 0xcf1
        hash.insert(
            String::from("hebrew_samekh"),
            Unicode::new(xkb::KEY_hebrew_samekh, '\u{5e1}'),
        ); // 0xcf1
        hash.insert(
            String::from("hebrew_ayin"),
            Unicode::new(xkb::KEY_hebrew_ayin, '\u{5e2}'),
        ); // 0xcf2
        hash.insert(
            String::from("hebrew_finalpe"),
            Unicode::new(xkb::KEY_hebrew_finalpe, '\u{5e3}'),
        ); // 0xcf3
        hash.insert(
            String::from("hebrew_pe"),
            Unicode::new(xkb::KEY_hebrew_pe, '\u{5e4}'),
        ); // 0xcf4
        hash.insert(
            String::from("hebrew_finalzade"),
            Unicode::new(xkb::KEY_hebrew_finalzade, '\u{5e5}'),
        ); // 0xcf5
        hash.insert(
            String::from("hebrew_finalzadi"),
            Unicode::new(xkb::KEY_hebrew_finalzadi, '\u{5e5}'),
        ); // 0xcf5
        hash.insert(
            String::from("hebrew_zade"),
            Unicode::new(xkb::KEY_hebrew_zade, '\u{5e6}'),
        ); // 0xcf6
        hash.insert(
            String::from("hebrew_zadi"),
            Unicode::new(xkb::KEY_hebrew_zadi, '\u{5e6}'),
        ); // 0xcf6
        hash.insert(
            String::from("hebrew_qoph"),
            Unicode::new(xkb::KEY_hebrew_qoph, '\u{5e7}'),
        ); // 0xcf7
        hash.insert(
            String::from("hebrew_kuf"),
            Unicode::new(xkb::KEY_hebrew_kuf, '\u{5e7}'),
        ); // 0xcf7
        hash.insert(
            String::from("hebrew_resh"),
            Unicode::new(xkb::KEY_hebrew_resh, '\u{5e8}'),
        ); // 0xcf8
        hash.insert(
            String::from("hebrew_shin"),
            Unicode::new(xkb::KEY_hebrew_shin, '\u{5e9}'),
        ); // 0xcf9
        hash.insert(
            String::from("hebrew_taw"),
            Unicode::new(xkb::KEY_hebrew_taw, '\u{5ea}'),
        ); // 0xcfa
        hash.insert(
            String::from("hebrew_taf"),
            Unicode::new(xkb::KEY_hebrew_taf, '\u{5ea}'),
        ); // 0xcfa
        hash.insert(
            String::from("Hebrew_switch"),
            Unicode::new(xkb::KEY_Hebrew_switch, '\u{0}'),
        ); // 0xFF7E
        hash.insert(
            String::from("XF86ModeLock"),
            Unicode::new(xkb::KEY_XF86ModeLock, '\u{0}'),
        ); // 0x1008FF01
        hash.insert(
            String::from("XF86MonBrightnessUp"),
            Unicode::new(xkb::KEY_XF86MonBrightnessUp, '\u{0}'),
        ); // 0x1008FF02
        hash.insert(
            String::from("XF86MonBrightnessDown"),
            Unicode::new(xkb::KEY_XF86MonBrightnessDown, '\u{0}'),
        ); // 0x1008FF03
        hash.insert(
            String::from("XF86KbdLightOnOff"),
            Unicode::new(xkb::KEY_XF86KbdLightOnOff, '\u{0}'),
        ); // 0x1008FF04
        hash.insert(
            String::from("XF86KbdBrightnessUp"),
            Unicode::new(xkb::KEY_XF86KbdBrightnessUp, '\u{0}'),
        ); // 0x1008FF05
        hash.insert(
            String::from("XF86KbdBrightnessDown"),
            Unicode::new(xkb::KEY_XF86KbdBrightnessDown, '\u{0}'),
        ); // 0x1008FF06
        hash.insert(
            String::from("XF86Standby"),
            Unicode::new(xkb::KEY_XF86Standby, '\u{0}'),
        ); // 0x1008FF10
        hash.insert(
            String::from("XF86AudioLowerVolume"),
            Unicode::new(xkb::KEY_XF86AudioLowerVolume, '\u{0}'),
        ); // 0x1008FF11
        hash.insert(
            String::from("XF86AudioMute"),
            Unicode::new(xkb::KEY_XF86AudioMute, '\u{0}'),
        ); // 0x1008FF12
        hash.insert(
            String::from("XF86AudioRaiseVolume"),
            Unicode::new(xkb::KEY_XF86AudioRaiseVolume, '\u{0}'),
        ); // 0x1008FF13
        hash.insert(
            String::from("XF86AudioPlay"),
            Unicode::new(xkb::KEY_XF86AudioPlay, '\u{0}'),
        ); // 0x1008FF14
        hash.insert(
            String::from("XF86AudioStop"),
            Unicode::new(xkb::KEY_XF86AudioStop, '\u{0}'),
        ); // 0x1008FF15
        hash.insert(
            String::from("XF86AudioPrev"),
            Unicode::new(xkb::KEY_XF86AudioPrev, '\u{0}'),
        ); // 0x1008FF16
        hash.insert(
            String::from("XF86AudioNext"),
            Unicode::new(xkb::KEY_XF86AudioNext, '\u{0}'),
        ); // 0x1008FF17
        hash.insert(
            String::from("XF86HomePage"),
            Unicode::new(xkb::KEY_XF86HomePage, '\u{0}'),
        ); // 0x1008FF18
        hash.insert(
            String::from("XF86Mail"),
            Unicode::new(xkb::KEY_XF86Mail, '\u{0}'),
        ); // 0x1008FF19
        hash.insert(
            String::from("XF86Start"),
            Unicode::new(xkb::KEY_XF86Start, '\u{0}'),
        ); // 0x1008FF1A
        hash.insert(
            String::from("XF86Search"),
            Unicode::new(xkb::KEY_XF86Search, '\u{0}'),
        ); // 0x1008FF1B
        hash.insert(
            String::from("XF86AudioRecord"),
            Unicode::new(xkb::KEY_XF86AudioRecord, '\u{0}'),
        ); // 0x1008FF1C
        hash.insert(
            String::from("XF86Calculator"),
            Unicode::new(xkb::KEY_XF86Calculator, '\u{0}'),
        ); // 0x1008FF1D
        hash.insert(
            String::from("XF86Memo"),
            Unicode::new(xkb::KEY_XF86Memo, '\u{0}'),
        ); // 0x1008FF1E
        hash.insert(
            String::from("XF86ToDoList"),
            Unicode::new(xkb::KEY_XF86ToDoList, '\u{0}'),
        ); // 0x1008FF1F
        hash.insert(
            String::from("XF86Calendar"),
            Unicode::new(xkb::KEY_XF86Calendar, '\u{0}'),
        ); // 0x1008FF20
        hash.insert(
            String::from("XF86PowerDown"),
            Unicode::new(xkb::KEY_XF86PowerDown, '\u{0}'),
        ); // 0x1008FF21
        hash.insert(
            String::from("XF86ContrastAdjust"),
            Unicode::new(xkb::KEY_XF86ContrastAdjust, '\u{0}'),
        ); // 0x1008FF22
        hash.insert(
            String::from("XF86RockerUp"),
            Unicode::new(xkb::KEY_XF86RockerUp, '\u{0}'),
        ); // 0x1008FF23
        hash.insert(
            String::from("XF86RockerDown"),
            Unicode::new(xkb::KEY_XF86RockerDown, '\u{0}'),
        ); // 0x1008FF24
        hash.insert(
            String::from("XF86RockerEnter"),
            Unicode::new(xkb::KEY_XF86RockerEnter, '\u{0}'),
        ); // 0x1008FF25
        hash.insert(
            String::from("XF86Back"),
            Unicode::new(xkb::KEY_XF86Back, '\u{0}'),
        ); // 0x1008FF26
        hash.insert(
            String::from("XF86Forward"),
            Unicode::new(xkb::KEY_XF86Forward, '\u{0}'),
        ); // 0x1008FF27
        hash.insert(
            String::from("XF86Stop"),
            Unicode::new(xkb::KEY_XF86Stop, '\u{0}'),
        ); // 0x1008FF28
        hash.insert(
            String::from("XF86Refresh"),
            Unicode::new(xkb::KEY_XF86Refresh, '\u{0}'),
        ); // 0x1008FF29
        hash.insert(
            String::from("XF86PowerOff"),
            Unicode::new(xkb::KEY_XF86PowerOff, '\u{0}'),
        ); // 0x1008FF2A
        hash.insert(
            String::from("XF86WakeUp"),
            Unicode::new(xkb::KEY_XF86WakeUp, '\u{0}'),
        ); // 0x1008FF2B
        hash.insert(
            String::from("XF86Eject"),
            Unicode::new(xkb::KEY_XF86Eject, '\u{0}'),
        ); // 0x1008FF2C
        hash.insert(
            String::from("XF86ScreenSaver"),
            Unicode::new(xkb::KEY_XF86ScreenSaver, '\u{0}'),
        ); // 0x1008FF2D
        hash.insert(
            String::from("XF86WWW"),
            Unicode::new(xkb::KEY_XF86WWW, '\u{0}'),
        ); // 0x1008FF2E
        hash.insert(
            String::from("XF86Sleep"),
            Unicode::new(xkb::KEY_XF86Sleep, '\u{0}'),
        ); // 0x1008FF2F
        hash.insert(
            String::from("XF86Favorites"),
            Unicode::new(xkb::KEY_XF86Favorites, '\u{0}'),
        ); // 0x1008FF30
        hash.insert(
            String::from("XF86AudioPause"),
            Unicode::new(xkb::KEY_XF86AudioPause, '\u{0}'),
        ); // 0x1008FF31
        hash.insert(
            String::from("XF86AudioMedia"),
            Unicode::new(xkb::KEY_XF86AudioMedia, '\u{0}'),
        ); // 0x1008FF32
        hash.insert(
            String::from("XF86MyComputer"),
            Unicode::new(xkb::KEY_XF86MyComputer, '\u{0}'),
        ); // 0x1008FF33
        hash.insert(
            String::from("XF86VendorHome"),
            Unicode::new(xkb::KEY_XF86VendorHome, '\u{0}'),
        ); // 0x1008FF34
        hash.insert(
            String::from("XF86LightBulb"),
            Unicode::new(xkb::KEY_XF86LightBulb, '\u{0}'),
        ); // 0x1008FF35
        hash.insert(
            String::from("XF86Shop"),
            Unicode::new(xkb::KEY_XF86Shop, '\u{0}'),
        ); // 0x1008FF36
        hash.insert(
            String::from("XF86History"),
            Unicode::new(xkb::KEY_XF86History, '\u{0}'),
        ); // 0x1008FF37
        hash.insert(
            String::from("XF86OpenURL"),
            Unicode::new(xkb::KEY_XF86OpenURL, '\u{0}'),
        ); // 0x1008FF38
        hash.insert(
            String::from("XF86AddFavorite"),
            Unicode::new(xkb::KEY_XF86AddFavorite, '\u{0}'),
        ); // 0x1008FF39
        hash.insert(
            String::from("XF86HotLinks"),
            Unicode::new(xkb::KEY_XF86HotLinks, '\u{0}'),
        ); // 0x1008FF3A
        hash.insert(
            String::from("XF86BrightnessAdjust"),
            Unicode::new(xkb::KEY_XF86BrightnessAdjust, '\u{0}'),
        ); // 0x1008FF3B
        hash.insert(
            String::from("XF86Finance"),
            Unicode::new(xkb::KEY_XF86Finance, '\u{0}'),
        ); // 0x1008FF3C
        hash.insert(
            String::from("XF86Community"),
            Unicode::new(xkb::KEY_XF86Community, '\u{0}'),
        ); // 0x1008FF3D
        hash.insert(
            String::from("XF86AudioRewind"),
            Unicode::new(xkb::KEY_XF86AudioRewind, '\u{0}'),
        ); // 0x1008FF3E
        hash.insert(
            String::from("XF86BackForward"),
            Unicode::new(xkb::KEY_XF86BackForward, '\u{0}'),
        ); // 0x1008FF3F
        hash.insert(
            String::from("XF86Launch0"),
            Unicode::new(xkb::KEY_XF86Launch0, '\u{0}'),
        ); // 0x1008FF40
        hash.insert(
            String::from("XF86Launch1"),
            Unicode::new(xkb::KEY_XF86Launch1, '\u{0}'),
        ); // 0x1008FF41
        hash.insert(
            String::from("XF86Launch2"),
            Unicode::new(xkb::KEY_XF86Launch2, '\u{0}'),
        ); // 0x1008FF42
        hash.insert(
            String::from("XF86Launch3"),
            Unicode::new(xkb::KEY_XF86Launch3, '\u{0}'),
        ); // 0x1008FF43
        hash.insert(
            String::from("XF86Launch4"),
            Unicode::new(xkb::KEY_XF86Launch4, '\u{0}'),
        ); // 0x1008FF44
        hash.insert(
            String::from("XF86Launch5"),
            Unicode::new(xkb::KEY_XF86Launch5, '\u{0}'),
        ); // 0x1008FF45
        hash.insert(
            String::from("XF86Launch6"),
            Unicode::new(xkb::KEY_XF86Launch6, '\u{0}'),
        ); // 0x1008FF46
        hash.insert(
            String::from("XF86Launch7"),
            Unicode::new(xkb::KEY_XF86Launch7, '\u{0}'),
        ); // 0x1008FF47
        hash.insert(
            String::from("XF86Launch8"),
            Unicode::new(xkb::KEY_XF86Launch8, '\u{0}'),
        ); // 0x1008FF48
        hash.insert(
            String::from("XF86Launch9"),
            Unicode::new(xkb::KEY_XF86Launch9, '\u{0}'),
        ); // 0x1008FF49
        hash.insert(
            String::from("XF86LaunchA"),
            Unicode::new(xkb::KEY_XF86LaunchA, '\u{0}'),
        ); // 0x1008FF4A
        hash.insert(
            String::from("XF86LaunchB"),
            Unicode::new(xkb::KEY_XF86LaunchB, '\u{0}'),
        ); // 0x1008FF4B
        hash.insert(
            String::from("XF86LaunchC"),
            Unicode::new(xkb::KEY_XF86LaunchC, '\u{0}'),
        ); // 0x1008FF4C
        hash.insert(
            String::from("XF86LaunchD"),
            Unicode::new(xkb::KEY_XF86LaunchD, '\u{0}'),
        ); // 0x1008FF4D
        hash.insert(
            String::from("XF86LaunchE"),
            Unicode::new(xkb::KEY_XF86LaunchE, '\u{0}'),
        ); // 0x1008FF4E
        hash.insert(
            String::from("XF86LaunchF"),
            Unicode::new(xkb::KEY_XF86LaunchF, '\u{0}'),
        ); // 0x1008FF4F
        hash.insert(
            String::from("XF86ApplicationLeft"),
            Unicode::new(xkb::KEY_XF86ApplicationLeft, '\u{0}'),
        ); // 0x1008FF50
        hash.insert(
            String::from("XF86ApplicationRight"),
            Unicode::new(xkb::KEY_XF86ApplicationRight, '\u{0}'),
        ); // 0x1008FF51
        hash.insert(
            String::from("XF86Book"),
            Unicode::new(xkb::KEY_XF86Book, '\u{0}'),
        ); // 0x1008FF52
        hash.insert(
            String::from("XF86CD"),
            Unicode::new(xkb::KEY_XF86CD, '\u{0}'),
        ); // 0x1008FF53
        hash.insert(
            String::from("XF86Calculater"),
            Unicode::new(xkb::KEY_XF86Calculater, '\u{0}'),
        ); // 0x1008FF54
        hash.insert(
            String::from("XF86Clear"),
            Unicode::new(xkb::KEY_XF86Clear, '\u{0}'),
        ); // 0x1008FF55
        hash.insert(
            String::from("XF86Close"),
            Unicode::new(xkb::KEY_XF86Close, '\u{0}'),
        ); // 0x1008FF56
        hash.insert(
            String::from("XF86Copy"),
            Unicode::new(xkb::KEY_XF86Copy, '\u{0}'),
        ); // 0x1008FF57
        hash.insert(
            String::from("XF86Cut"),
            Unicode::new(xkb::KEY_XF86Cut, '\u{0}'),
        ); // 0x1008FF58
        hash.insert(
            String::from("XF86Display"),
            Unicode::new(xkb::KEY_XF86Display, '\u{0}'),
        ); // 0x1008FF59
        hash.insert(
            String::from("XF86DOS"),
            Unicode::new(xkb::KEY_XF86DOS, '\u{0}'),
        ); // 0x1008FF5A
        hash.insert(
            String::from("XF86Documents"),
            Unicode::new(xkb::KEY_XF86Documents, '\u{0}'),
        ); // 0x1008FF5B
        hash.insert(
            String::from("XF86Excel"),
            Unicode::new(xkb::KEY_XF86Excel, '\u{0}'),
        ); // 0x1008FF5C
        hash.insert(
            String::from("XF86Explorer"),
            Unicode::new(xkb::KEY_XF86Explorer, '\u{0}'),
        ); // 0x1008FF5D
        hash.insert(
            String::from("XF86Game"),
            Unicode::new(xkb::KEY_XF86Game, '\u{0}'),
        ); // 0x1008FF5E
        hash.insert(
            String::from("XF86Go"),
            Unicode::new(xkb::KEY_XF86Go, '\u{0}'),
        ); // 0x1008FF5F
        hash.insert(
            String::from("XF86iTouch"),
            Unicode::new(xkb::KEY_XF86iTouch, '\u{0}'),
        ); // 0x1008FF60
        hash.insert(
            String::from("XF86LogOff"),
            Unicode::new(xkb::KEY_XF86LogOff, '\u{0}'),
        ); // 0x1008FF61
        hash.insert(
            String::from("XF86Market"),
            Unicode::new(xkb::KEY_XF86Market, '\u{0}'),
        ); // 0x1008FF62
        hash.insert(
            String::from("XF86Meeting"),
            Unicode::new(xkb::KEY_XF86Meeting, '\u{0}'),
        ); // 0x1008FF63
        hash.insert(
            String::from("XF86MenuKB"),
            Unicode::new(xkb::KEY_XF86MenuKB, '\u{0}'),
        ); // 0x1008FF65
        hash.insert(
            String::from("XF86MenuPB"),
            Unicode::new(xkb::KEY_XF86MenuPB, '\u{0}'),
        ); // 0x1008FF66
        hash.insert(
            String::from("XF86MySites"),
            Unicode::new(xkb::KEY_XF86MySites, '\u{0}'),
        ); // 0x1008FF67
        hash.insert(
            String::from("XF86New"),
            Unicode::new(xkb::KEY_XF86New, '\u{0}'),
        ); // 0x1008FF68
        hash.insert(
            String::from("XF86News"),
            Unicode::new(xkb::KEY_XF86News, '\u{0}'),
        ); // 0x1008FF69
        hash.insert(
            String::from("XF86OfficeHome"),
            Unicode::new(xkb::KEY_XF86OfficeHome, '\u{0}'),
        ); // 0x1008FF6A
        hash.insert(
            String::from("XF86Open"),
            Unicode::new(xkb::KEY_XF86Open, '\u{0}'),
        ); // 0x1008FF6B
        hash.insert(
            String::from("XF86Option"),
            Unicode::new(xkb::KEY_XF86Option, '\u{0}'),
        ); // 0x1008FF6C
        hash.insert(
            String::from("XF86Paste"),
            Unicode::new(xkb::KEY_XF86Paste, '\u{0}'),
        ); // 0x1008FF6D
        hash.insert(
            String::from("XF86Phone"),
            Unicode::new(xkb::KEY_XF86Phone, '\u{0}'),
        ); // 0x1008FF6E
        hash.insert(String::from("XF86Q"), Unicode::new(xkb::KEY_XF86Q, '\u{0}')); // 0x1008FF70
        hash.insert(
            String::from("XF86Reply"),
            Unicode::new(xkb::KEY_XF86Reply, '\u{0}'),
        ); // 0x1008FF72
        hash.insert(
            String::from("XF86Reload"),
            Unicode::new(xkb::KEY_XF86Reload, '\u{0}'),
        ); // 0x1008FF73
        hash.insert(
            String::from("XF86RotateWindows"),
            Unicode::new(xkb::KEY_XF86RotateWindows, '\u{0}'),
        ); // 0x1008FF74
        hash.insert(
            String::from("XF86RotationPB"),
            Unicode::new(xkb::KEY_XF86RotationPB, '\u{0}'),
        ); // 0x1008FF75
        hash.insert(
            String::from("XF86RotationKB"),
            Unicode::new(xkb::KEY_XF86RotationKB, '\u{0}'),
        ); // 0x1008FF76
        hash.insert(
            String::from("XF86Save"),
            Unicode::new(xkb::KEY_XF86Save, '\u{0}'),
        ); // 0x1008FF77
        hash.insert(
            String::from("XF86ScrollUp"),
            Unicode::new(xkb::KEY_XF86ScrollUp, '\u{0}'),
        ); // 0x1008FF78
        hash.insert(
            String::from("XF86ScrollDown"),
            Unicode::new(xkb::KEY_XF86ScrollDown, '\u{0}'),
        ); // 0x1008FF79
        hash.insert(
            String::from("XF86ScrollClick"),
            Unicode::new(xkb::KEY_XF86ScrollClick, '\u{0}'),
        ); // 0x1008FF7A
        hash.insert(
            String::from("XF86Send"),
            Unicode::new(xkb::KEY_XF86Send, '\u{0}'),
        ); // 0x1008FF7B
        hash.insert(
            String::from("XF86Spell"),
            Unicode::new(xkb::KEY_XF86Spell, '\u{0}'),
        ); // 0x1008FF7C
        hash.insert(
            String::from("XF86SplitScreen"),
            Unicode::new(xkb::KEY_XF86SplitScreen, '\u{0}'),
        ); // 0x1008FF7D
        hash.insert(
            String::from("XF86Support"),
            Unicode::new(xkb::KEY_XF86Support, '\u{0}'),
        ); // 0x1008FF7E
        hash.insert(
            String::from("XF86TaskPane"),
            Unicode::new(xkb::KEY_XF86TaskPane, '\u{0}'),
        ); // 0x1008FF7F
        hash.insert(
            String::from("XF86Terminal"),
            Unicode::new(xkb::KEY_XF86Terminal, '\u{0}'),
        ); // 0x1008FF80
        hash.insert(
            String::from("XF86Tools"),
            Unicode::new(xkb::KEY_XF86Tools, '\u{0}'),
        ); // 0x1008FF81
        hash.insert(
            String::from("XF86Travel"),
            Unicode::new(xkb::KEY_XF86Travel, '\u{0}'),
        ); // 0x1008FF82
        hash.insert(
            String::from("XF86UserPB"),
            Unicode::new(xkb::KEY_XF86UserPB, '\u{0}'),
        ); // 0x1008FF84
        hash.insert(
            String::from("XF86User1KB"),
            Unicode::new(xkb::KEY_XF86User1KB, '\u{0}'),
        ); // 0x1008FF85
        hash.insert(
            String::from("XF86User2KB"),
            Unicode::new(xkb::KEY_XF86User2KB, '\u{0}'),
        ); // 0x1008FF86
        hash.insert(
            String::from("XF86Video"),
            Unicode::new(xkb::KEY_XF86Video, '\u{0}'),
        ); // 0x1008FF87
        hash.insert(
            String::from("XF86WheelButton"),
            Unicode::new(xkb::KEY_XF86WheelButton, '\u{0}'),
        ); // 0x1008FF88
        hash.insert(
            String::from("XF86Word"),
            Unicode::new(xkb::KEY_XF86Word, '\u{0}'),
        ); // 0x1008FF89
        hash.insert(
            String::from("XF86Xfer"),
            Unicode::new(xkb::KEY_XF86Xfer, '\u{0}'),
        ); // 0x1008FF8A
        hash.insert(
            String::from("XF86ZoomIn"),
            Unicode::new(xkb::KEY_XF86ZoomIn, '\u{0}'),
        ); // 0x1008FF8B
        hash.insert(
            String::from("XF86ZoomOut"),
            Unicode::new(xkb::KEY_XF86ZoomOut, '\u{0}'),
        ); // 0x1008FF8C
        hash.insert(
            String::from("XF86Away"),
            Unicode::new(xkb::KEY_XF86Away, '\u{0}'),
        ); // 0x1008FF8D
        hash.insert(
            String::from("XF86Messenger"),
            Unicode::new(xkb::KEY_XF86Messenger, '\u{0}'),
        ); // 0x1008FF8E
        hash.insert(
            String::from("XF86WebCam"),
            Unicode::new(xkb::KEY_XF86WebCam, '\u{0}'),
        ); // 0x1008FF8F
        hash.insert(
            String::from("XF86MailForward"),
            Unicode::new(xkb::KEY_XF86MailForward, '\u{0}'),
        ); // 0x1008FF90
        hash.insert(
            String::from("XF86Pictures"),
            Unicode::new(xkb::KEY_XF86Pictures, '\u{0}'),
        ); // 0x1008FF91
        hash.insert(
            String::from("XF86Music"),
            Unicode::new(xkb::KEY_XF86Music, '\u{0}'),
        ); // 0x1008FF92
        hash.insert(
            String::from("XF86Battery"),
            Unicode::new(xkb::KEY_XF86Battery, '\u{0}'),
        ); // 0x1008FF93
        hash.insert(
            String::from("XF86Bluetooth"),
            Unicode::new(xkb::KEY_XF86Bluetooth, '\u{0}'),
        ); // 0x1008FF94
        hash.insert(
            String::from("XF86WLAN"),
            Unicode::new(xkb::KEY_XF86WLAN, '\u{0}'),
        ); // 0x1008FF95
        hash.insert(
            String::from("XF86UWB"),
            Unicode::new(xkb::KEY_XF86UWB, '\u{0}'),
        ); // 0x1008FF96
        hash.insert(
            String::from("XF86AudioForward"),
            Unicode::new(xkb::KEY_XF86AudioForward, '\u{0}'),
        ); // 0x1008FF97
        hash.insert(
            String::from("XF86AudioRepeat"),
            Unicode::new(xkb::KEY_XF86AudioRepeat, '\u{0}'),
        ); // 0x1008FF98
        hash.insert(
            String::from("XF86AudioRandomPlay"),
            Unicode::new(xkb::KEY_XF86AudioRandomPlay, '\u{0}'),
        ); // 0x1008FF99
        hash.insert(
            String::from("XF86Subtitle"),
            Unicode::new(xkb::KEY_XF86Subtitle, '\u{0}'),
        ); // 0x1008FF9A
        hash.insert(
            String::from("XF86AudioCycleTrack"),
            Unicode::new(xkb::KEY_XF86AudioCycleTrack, '\u{0}'),
        ); // 0x1008FF9B
        hash.insert(
            String::from("XF86CycleAngle"),
            Unicode::new(xkb::KEY_XF86CycleAngle, '\u{0}'),
        ); // 0x1008FF9C
        hash.insert(
            String::from("XF86FrameBack"),
            Unicode::new(xkb::KEY_XF86FrameBack, '\u{0}'),
        ); // 0x1008FF9D
        hash.insert(
            String::from("XF86FrameForward"),
            Unicode::new(xkb::KEY_XF86FrameForward, '\u{0}'),
        ); // 0x1008FF9E
        hash.insert(
            String::from("XF86Time"),
            Unicode::new(xkb::KEY_XF86Time, '\u{0}'),
        ); // 0x1008FF9F
        hash.insert(
            String::from("XF86Select"),
            Unicode::new(xkb::KEY_XF86Select, '\u{0}'),
        ); // 0x1008FFA0
        hash.insert(
            String::from("XF86View"),
            Unicode::new(xkb::KEY_XF86View, '\u{0}'),
        ); // 0x1008FFA1
        hash.insert(
            String::from("XF86TopMenu"),
            Unicode::new(xkb::KEY_XF86TopMenu, '\u{0}'),
        ); // 0x1008FFA2
        hash.insert(
            String::from("XF86Red"),
            Unicode::new(xkb::KEY_XF86Red, '\u{0}'),
        ); // 0x1008FFA3
        hash.insert(
            String::from("XF86Green"),
            Unicode::new(xkb::KEY_XF86Green, '\u{0}'),
        ); // 0x1008FFA4
        hash.insert(
            String::from("XF86Yellow"),
            Unicode::new(xkb::KEY_XF86Yellow, '\u{0}'),
        ); // 0x1008FFA5
        hash.insert(
            String::from("XF86Blue"),
            Unicode::new(xkb::KEY_XF86Blue, '\u{0}'),
        ); // 0x1008FFA6
        hash.insert(
            String::from("XF86Suspend"),
            Unicode::new(xkb::KEY_XF86Suspend, '\u{0}'),
        ); // 0x1008FFA7
        hash.insert(
            String::from("XF86Hibernate"),
            Unicode::new(xkb::KEY_XF86Hibernate, '\u{0}'),
        ); // 0x1008FFA8
        hash.insert(
            String::from("XF86TouchpadToggle"),
            Unicode::new(xkb::KEY_XF86TouchpadToggle, '\u{0}'),
        ); // 0x1008FFA9
        hash.insert(
            String::from("XF86TouchpadOn"),
            Unicode::new(xkb::KEY_XF86TouchpadOn, '\u{0}'),
        ); // 0x1008FFB0
        hash.insert(
            String::from("XF86TouchpadOff"),
            Unicode::new(xkb::KEY_XF86TouchpadOff, '\u{0}'),
        ); // 0x1008FFB1
        hash.insert(
            String::from("XF86AudioMicMute"),
            Unicode::new(xkb::KEY_XF86AudioMicMute, '\u{0}'),
        ); // 0x1008FFB2
        hash.insert(
            String::from("XF86Switch_VT_1"),
            Unicode::new(xkb::KEY_XF86Switch_VT_1, '\u{0}'),
        ); // 0x1008FE01
        hash.insert(
            String::from("XF86Switch_VT_2"),
            Unicode::new(xkb::KEY_XF86Switch_VT_2, '\u{0}'),
        ); // 0x1008FE02
        hash.insert(
            String::from("XF86Switch_VT_3"),
            Unicode::new(xkb::KEY_XF86Switch_VT_3, '\u{0}'),
        ); // 0x1008FE03
        hash.insert(
            String::from("XF86Switch_VT_4"),
            Unicode::new(xkb::KEY_XF86Switch_VT_4, '\u{0}'),
        ); // 0x1008FE04
        hash.insert(
            String::from("XF86Switch_VT_5"),
            Unicode::new(xkb::KEY_XF86Switch_VT_5, '\u{0}'),
        ); // 0x1008FE05
        hash.insert(
            String::from("XF86Switch_VT_6"),
            Unicode::new(xkb::KEY_XF86Switch_VT_6, '\u{0}'),
        ); // 0x1008FE06
        hash.insert(
            String::from("XF86Switch_VT_7"),
            Unicode::new(xkb::KEY_XF86Switch_VT_7, '\u{0}'),
        ); // 0x1008FE07
        hash.insert(
            String::from("XF86Switch_VT_8"),
            Unicode::new(xkb::KEY_XF86Switch_VT_8, '\u{0}'),
        ); // 0x1008FE08
        hash.insert(
            String::from("XF86Switch_VT_9"),
            Unicode::new(xkb::KEY_XF86Switch_VT_9, '\u{0}'),
        ); // 0x1008FE09
        hash.insert(
            String::from("XF86Switch_VT_10"),
            Unicode::new(xkb::KEY_XF86Switch_VT_10, '\u{0}'),
        ); // 0x1008FE0A
        hash.insert(
            String::from("XF86Switch_VT_11"),
            Unicode::new(xkb::KEY_XF86Switch_VT_11, '\u{0}'),
        ); // 0x1008FE0B
        hash.insert(
            String::from("XF86Switch_VT_12"),
            Unicode::new(xkb::KEY_XF86Switch_VT_12, '\u{0}'),
        ); // 0x1008FE0C
        hash.insert(
            String::from("XF86Ungrab"),
            Unicode::new(xkb::KEY_XF86Ungrab, '\u{0}'),
        ); // 0x1008FE20
        hash.insert(
            String::from("XF86ClearGrab"),
            Unicode::new(xkb::KEY_XF86ClearGrab, '\u{0}'),
        ); // 0x1008FE21
        hash.insert(
            String::from("XF86Next_VMode"),
            Unicode::new(xkb::KEY_XF86Next_VMode, '\u{0}'),
        ); // 0x1008FE22
        hash.insert(
            String::from("XF86Prev_VMode"),
            Unicode::new(xkb::KEY_XF86Prev_VMode, '\u{0}'),
        ); // 0x1008FE23
        hash.insert(
            String::from("XF86LogWindowTree"),
            Unicode::new(xkb::KEY_XF86LogWindowTree, '\u{0}'),
        ); // 0x1008FE24
        hash.insert(
            String::from("XF86LogGrabInfo"),
            Unicode::new(xkb::KEY_XF86LogGrabInfo, '\u{0}'),
        ); // 0x1008FE25
        hash.insert(
            String::from("ISO_Lock"),
            Unicode::new(xkb::KEY_ISO_Lock, '\u{0}'),
        ); // 0xfe01
        hash.insert(
            String::from("ISO_Level2_Latch"),
            Unicode::new(xkb::KEY_ISO_Level2_Latch, '\u{0}'),
        ); // 0xfe02
        hash.insert(
            String::from("ISO_Level3_Shift"),
            Unicode::new(xkb::KEY_ISO_Level3_Shift, '\u{0}'),
        ); // 0xfe03
        hash.insert(
            String::from("ISO_Level3_Latch"),
            Unicode::new(xkb::KEY_ISO_Level3_Latch, '\u{0}'),
        ); // 0xfe04
        hash.insert(
            String::from("ISO_Level3_Lock"),
            Unicode::new(xkb::KEY_ISO_Level3_Lock, '\u{0}'),
        ); // 0xfe05
        hash.insert(
            String::from("ISO_Level5_Shift"),
            Unicode::new(xkb::KEY_ISO_Level5_Shift, '\u{0}'),
        ); // 0xfe11
        hash.insert(
            String::from("ISO_Level5_Latch"),
            Unicode::new(xkb::KEY_ISO_Level5_Latch, '\u{0}'),
        ); // 0xfe12
        hash.insert(
            String::from("ISO_Level5_Lock"),
            Unicode::new(xkb::KEY_ISO_Level5_Lock, '\u{0}'),
        ); // 0xfe13
        hash.insert(
            String::from("ISO_Group_Shift"),
            Unicode::new(xkb::KEY_ISO_Group_Shift, '\u{0}'),
        ); // 0xff7e
        hash.insert(
            String::from("ISO_Group_Latch"),
            Unicode::new(xkb::KEY_ISO_Group_Latch, '\u{0}'),
        ); // 0xfe06
        hash.insert(
            String::from("ISO_Group_Lock"),
            Unicode::new(xkb::KEY_ISO_Group_Lock, '\u{0}'),
        ); // 0xfe07
        hash.insert(
            String::from("ISO_Next_Group"),
            Unicode::new(xkb::KEY_ISO_Next_Group, '\u{0}'),
        ); // 0xfe08
        hash.insert(
            String::from("ISO_Next_Group_Lock"),
            Unicode::new(xkb::KEY_ISO_Next_Group_Lock, '\u{0}'),
        ); // 0xfe09
        hash.insert(
            String::from("ISO_Prev_Group"),
            Unicode::new(xkb::KEY_ISO_Prev_Group, '\u{0}'),
        ); // 0xfe0a
        hash.insert(
            String::from("ISO_Prev_Group_Lock"),
            Unicode::new(xkb::KEY_ISO_Prev_Group_Lock, '\u{0}'),
        ); // 0xfe0b
        hash.insert(
            String::from("ISO_First_Group"),
            Unicode::new(xkb::KEY_ISO_First_Group, '\u{0}'),
        ); // 0xfe0c
        hash.insert(
            String::from("ISO_First_Group_Lock"),
            Unicode::new(xkb::KEY_ISO_First_Group_Lock, '\u{0}'),
        ); // 0xfe0d
        hash.insert(
            String::from("ISO_Last_Group"),
            Unicode::new(xkb::KEY_ISO_Last_Group, '\u{0}'),
        ); // 0xfe0e
        hash.insert(
            String::from("ISO_Last_Group_Lock"),
            Unicode::new(xkb::KEY_ISO_Last_Group_Lock, '\u{0}'),
        ); // 0xfe0f
        hash.insert(
            String::from("ISO_Left_Tab"),
            Unicode::new(xkb::KEY_ISO_Left_Tab, '\u{0}'),
        ); // 0xfe20
        hash.insert(
            String::from("ISO_Move_Line_Up"),
            Unicode::new(xkb::KEY_ISO_Move_Line_Up, '\u{0}'),
        ); // 0xfe21
        hash.insert(
            String::from("ISO_Move_Line_Down"),
            Unicode::new(xkb::KEY_ISO_Move_Line_Down, '\u{0}'),
        ); // 0xfe22
        hash.insert(
            String::from("ISO_Partial_Line_Up"),
            Unicode::new(xkb::KEY_ISO_Partial_Line_Up, '\u{0}'),
        ); // 0xfe23
        hash.insert(
            String::from("ISO_Partial_Line_Down"),
            Unicode::new(xkb::KEY_ISO_Partial_Line_Down, '\u{0}'),
        ); // 0xfe24
        hash.insert(
            String::from("ISO_Partial_Space_Left"),
            Unicode::new(xkb::KEY_ISO_Partial_Space_Left, '\u{0}'),
        ); // 0xfe25
        hash.insert(
            String::from("ISO_Partial_Space_Right"),
            Unicode::new(xkb::KEY_ISO_Partial_Space_Right, '\u{0}'),
        ); // 0xfe26
        hash.insert(
            String::from("ISO_Set_Margin_Left"),
            Unicode::new(xkb::KEY_ISO_Set_Margin_Left, '\u{0}'),
        ); // 0xfe27
        hash.insert(
            String::from("ISO_Set_Margin_Right"),
            Unicode::new(xkb::KEY_ISO_Set_Margin_Right, '\u{0}'),
        ); // 0xfe28
        hash.insert(
            String::from("ISO_Release_Margin_Left"),
            Unicode::new(xkb::KEY_ISO_Release_Margin_Left, '\u{0}'),
        ); // 0xfe29
        hash.insert(
            String::from("ISO_Release_Margin_Right"),
            Unicode::new(xkb::KEY_ISO_Release_Margin_Right, '\u{0}'),
        ); // 0xfe2a
        hash.insert(
            String::from("ISO_Release_Both_Margins"),
            Unicode::new(xkb::KEY_ISO_Release_Both_Margins, '\u{0}'),
        ); // 0xfe2b
        hash.insert(
            String::from("ISO_Fast_Cursor_Left"),
            Unicode::new(xkb::KEY_ISO_Fast_Cursor_Left, '\u{0}'),
        ); // 0xfe2c
        hash.insert(
            String::from("ISO_Fast_Cursor_Right"),
            Unicode::new(xkb::KEY_ISO_Fast_Cursor_Right, '\u{0}'),
        ); // 0xfe2d
        hash.insert(
            String::from("ISO_Fast_Cursor_Up"),
            Unicode::new(xkb::KEY_ISO_Fast_Cursor_Up, '\u{0}'),
        ); // 0xfe2e
        hash.insert(
            String::from("ISO_Fast_Cursor_Down"),
            Unicode::new(xkb::KEY_ISO_Fast_Cursor_Down, '\u{0}'),
        ); // 0xfe2f
        hash.insert(
            String::from("ISO_Continuous_Underline"),
            Unicode::new(xkb::KEY_ISO_Continuous_Underline, '\u{0}'),
        ); // 0xfe30
        hash.insert(
            String::from("ISO_Discontinuous_Underline"),
            Unicode::new(xkb::KEY_ISO_Discontinuous_Underline, '\u{0}'),
        ); // 0xfe31
        hash.insert(
            String::from("ISO_Emphasize"),
            Unicode::new(xkb::KEY_ISO_Emphasize, '\u{0}'),
        ); // 0xfe32
        hash.insert(
            String::from("ISO_Center_Object"),
            Unicode::new(xkb::KEY_ISO_Center_Object, '\u{0}'),
        ); // 0xfe33
        hash.insert(
            String::from("ISO_Enter"),
            Unicode::new(xkb::KEY_ISO_Enter, '\u{0}'),
        ); // 0xfe34
        hash.insert(
            String::from("dead_grave"),
            Unicode::new(xkb::KEY_dead_grave, '\u{300}'),
        ); // 0xfe50
        hash.insert(
            String::from("dead_acute"),
            Unicode::new(xkb::KEY_dead_acute, '\u{301}'),
        ); // 0xfe51
        hash.insert(
            String::from("dead_circumflex"),
            Unicode::new(xkb::KEY_dead_circumflex, '\u{302}'),
        ); // 0xfe52
        hash.insert(
            String::from("dead_tilde"),
            Unicode::new(xkb::KEY_dead_tilde, '\u{303}'),
        ); // 0xfe53
        hash.insert(
            String::from("dead_perispomeni"),
            Unicode::new(xkb::KEY_dead_perispomeni, '\u{0}'),
        ); // 0xfe53
        hash.insert(
            String::from("dead_macron"),
            Unicode::new(xkb::KEY_dead_macron, '\u{304}'),
        ); // 0xfe54
        hash.insert(
            String::from("dead_breve"),
            Unicode::new(xkb::KEY_dead_breve, '\u{306}'),
        ); // 0xfe55
        hash.insert(
            String::from("dead_abovedot"),
            Unicode::new(xkb::KEY_dead_abovedot, '\u{307}'),
        ); // 0xfe56
        hash.insert(
            String::from("dead_diaeresis"),
            Unicode::new(xkb::KEY_dead_diaeresis, '\u{308}'),
        ); // 0xfe57
        hash.insert(
            String::from("dead_abovering"),
            Unicode::new(xkb::KEY_dead_abovering, '\u{30a}'),
        ); // 0xfe58
        hash.insert(
            String::from("dead_doubleacute"),
            Unicode::new(xkb::KEY_dead_doubleacute, '\u{30b}'),
        ); // 0xfe59
        hash.insert(
            String::from("dead_caron"),
            Unicode::new(xkb::KEY_dead_caron, '\u{30c}'),
        ); // 0xfe5a
        hash.insert(
            String::from("dead_cedilla"),
            Unicode::new(xkb::KEY_dead_cedilla, '\u{327}'),
        ); // 0xfe5b
        hash.insert(
            String::from("dead_ogonek"),
            Unicode::new(xkb::KEY_dead_ogonek, '\u{328}'),
        ); // 0xfe5c
        hash.insert(
            String::from("dead_iota"),
            Unicode::new(xkb::KEY_dead_iota, '\u{345}'),
        ); // 0xfe5d
        hash.insert(
            String::from("dead_voiced_sound"),
            Unicode::new(xkb::KEY_dead_voiced_sound, '\u{3099}'),
        ); // 0xfe5e
        hash.insert(
            String::from("dead_semivoiced_sound"),
            Unicode::new(xkb::KEY_dead_semivoiced_sound, '\u{309a}'),
        ); // 0xfe5f
        hash.insert(
            String::from("dead_belowdot"),
            Unicode::new(xkb::KEY_dead_belowdot, '\u{323}'),
        ); // 0xfe60
        hash.insert(
            String::from("dead_hook"),
            Unicode::new(xkb::KEY_dead_hook, '\u{309}'),
        ); // 0xfe61
        hash.insert(
            String::from("dead_horn"),
            Unicode::new(xkb::KEY_dead_horn, '\u{31b}'),
        ); // 0xfe62
        hash.insert(
            String::from("dead_stroke"),
            Unicode::new(xkb::KEY_dead_stroke, '\u{0}'),
        ); // 0xfe63
        hash.insert(
            String::from("dead_abovecomma"),
            Unicode::new(xkb::KEY_dead_abovecomma, '\u{0}'),
        ); // 0xfe64
        hash.insert(
            String::from("dead_psili"),
            Unicode::new(xkb::KEY_dead_psili, '\u{0}'),
        ); // 0xfe64
        hash.insert(
            String::from("dead_abovereversedcomma"),
            Unicode::new(xkb::KEY_dead_abovereversedcomma, '\u{0}'),
        ); // 0xfe65
        hash.insert(
            String::from("dead_dasia"),
            Unicode::new(xkb::KEY_dead_dasia, '\u{0}'),
        ); // 0xfe65
        hash.insert(
            String::from("dead_doublegrave"),
            Unicode::new(xkb::KEY_dead_doublegrave, '\u{0}'),
        ); // 0xfe66
        hash.insert(
            String::from("dead_belowring"),
            Unicode::new(xkb::KEY_dead_belowring, '\u{0}'),
        ); // 0xfe67
        hash.insert(
            String::from("dead_belowmacron"),
            Unicode::new(xkb::KEY_dead_belowmacron, '\u{0}'),
        ); // 0xfe68
        hash.insert(
            String::from("dead_belowcircumflex"),
            Unicode::new(xkb::KEY_dead_belowcircumflex, '\u{0}'),
        ); // 0xfe69
        hash.insert(
            String::from("dead_belowtilde"),
            Unicode::new(xkb::KEY_dead_belowtilde, '\u{0}'),
        ); // 0xfe6a
        hash.insert(
            String::from("dead_belowbreve"),
            Unicode::new(xkb::KEY_dead_belowbreve, '\u{0}'),
        ); // 0xfe6b
        hash.insert(
            String::from("dead_belowdiaeresis"),
            Unicode::new(xkb::KEY_dead_belowdiaeresis, '\u{0}'),
        ); // 0xfe6c
        hash.insert(
            String::from("dead_invertedbreve"),
            Unicode::new(xkb::KEY_dead_invertedbreve, '\u{0}'),
        ); // 0xfe6d
        hash.insert(
            String::from("dead_belowcomma"),
            Unicode::new(xkb::KEY_dead_belowcomma, '\u{0}'),
        ); // 0xfe6e
        hash.insert(
            String::from("dead_currency"),
            Unicode::new(xkb::KEY_dead_currency, '\u{0}'),
        ); // 0xfe6f
        hash.insert(
            String::from("dead_lowline"),
            Unicode::new(xkb::KEY_dead_lowline, '\u{0}'),
        ); // 0xfe90
        hash.insert(
            String::from("dead_aboveverticalline"),
            Unicode::new(xkb::KEY_dead_aboveverticalline, '\u{0}'),
        ); // 0xfe91
        hash.insert(
            String::from("dead_belowverticalline"),
            Unicode::new(xkb::KEY_dead_belowverticalline, '\u{0}'),
        ); // 0xfe92
        hash.insert(
            String::from("dead_longsolidusoverlay"),
            Unicode::new(xkb::KEY_dead_longsolidusoverlay, '\u{0}'),
        ); // 0xfe93
        hash.insert(
            String::from("dead_a"),
            Unicode::new(xkb::KEY_dead_a, '\u{0}'),
        ); // 0xfe80
        hash.insert(
            String::from("dead_A"),
            Unicode::new(xkb::KEY_dead_A, '\u{0}'),
        ); // 0xfe81
        hash.insert(
            String::from("dead_e"),
            Unicode::new(xkb::KEY_dead_e, '\u{0}'),
        ); // 0xfe82
        hash.insert(
            String::from("dead_E"),
            Unicode::new(xkb::KEY_dead_E, '\u{0}'),
        ); // 0xfe83
        hash.insert(
            String::from("dead_i"),
            Unicode::new(xkb::KEY_dead_i, '\u{0}'),
        ); // 0xfe84
        hash.insert(
            String::from("dead_I"),
            Unicode::new(xkb::KEY_dead_I, '\u{0}'),
        ); // 0xfe85
        hash.insert(
            String::from("dead_o"),
            Unicode::new(xkb::KEY_dead_o, '\u{0}'),
        ); // 0xfe86
        hash.insert(
            String::from("dead_O"),
            Unicode::new(xkb::KEY_dead_O, '\u{0}'),
        ); // 0xfe87
        hash.insert(
            String::from("dead_u"),
            Unicode::new(xkb::KEY_dead_u, '\u{0}'),
        ); // 0xfe88
        hash.insert(
            String::from("dead_U"),
            Unicode::new(xkb::KEY_dead_U, '\u{0}'),
        ); // 0xfe89
        hash.insert(
            String::from("dead_small_schwa"),
            Unicode::new(xkb::KEY_dead_small_schwa, '\u{0}'),
        ); // 0xfe8a
        hash.insert(
            String::from("dead_capital_schwa"),
            Unicode::new(xkb::KEY_dead_capital_schwa, '\u{0}'),
        ); // 0xfe8b
        hash.insert(
            String::from("dead_greek"),
            Unicode::new(xkb::KEY_dead_greek, '\u{0}'),
        ); // 0xfe8c
        hash.insert(
            String::from("First_Virtual_Screen"),
            Unicode::new(xkb::KEY_First_Virtual_Screen, '\u{0}'),
        ); // 0xfed0
        hash.insert(
            String::from("Prev_Virtual_Screen"),
            Unicode::new(xkb::KEY_Prev_Virtual_Screen, '\u{0}'),
        ); // 0xfed1
        hash.insert(
            String::from("Next_Virtual_Screen"),
            Unicode::new(xkb::KEY_Next_Virtual_Screen, '\u{0}'),
        ); // 0xfed2
        hash.insert(
            String::from("Last_Virtual_Screen"),
            Unicode::new(xkb::KEY_Last_Virtual_Screen, '\u{0}'),
        ); // 0xfed4
        hash.insert(
            String::from("Terminate_Server"),
            Unicode::new(xkb::KEY_Terminate_Server, '\u{0}'),
        ); // 0xfed5
        hash.insert(
            String::from("AccessX_Enable"),
            Unicode::new(xkb::KEY_AccessX_Enable, '\u{0}'),
        ); // 0xfe70
        hash.insert(
            String::from("AccessX_Feedback_Enable"),
            Unicode::new(xkb::KEY_AccessX_Feedback_Enable, '\u{0}'),
        ); // 0xfe71
        hash.insert(
            String::from("RepeatKeys_Enable"),
            Unicode::new(xkb::KEY_RepeatKeys_Enable, '\u{0}'),
        ); // 0xfe72
        hash.insert(
            String::from("SlowKeys_Enable"),
            Unicode::new(xkb::KEY_SlowKeys_Enable, '\u{0}'),
        ); // 0xfe73
        hash.insert(
            String::from("BounceKeys_Enable"),
            Unicode::new(xkb::KEY_BounceKeys_Enable, '\u{0}'),
        ); // 0xfe74
        hash.insert(
            String::from("StickyKeys_Enable"),
            Unicode::new(xkb::KEY_StickyKeys_Enable, '\u{0}'),
        ); // 0xfe75
        hash.insert(
            String::from("MouseKeys_Enable"),
            Unicode::new(xkb::KEY_MouseKeys_Enable, '\u{0}'),
        ); // 0xfe76
        hash.insert(
            String::from("MouseKeys_Accel_Enable"),
            Unicode::new(xkb::KEY_MouseKeys_Accel_Enable, '\u{0}'),
        ); // 0xfe77
        hash.insert(
            String::from("Overlay1_Enable"),
            Unicode::new(xkb::KEY_Overlay1_Enable, '\u{0}'),
        ); // 0xfe78
        hash.insert(
            String::from("Overlay2_Enable"),
            Unicode::new(xkb::KEY_Overlay2_Enable, '\u{0}'),
        ); // 0xfe79
        hash.insert(
            String::from("AudibleBell_Enable"),
            Unicode::new(xkb::KEY_AudibleBell_Enable, '\u{0}'),
        ); // 0xfe7a
        hash.insert(
            String::from("Pointer_Left"),
            Unicode::new(xkb::KEY_Pointer_Left, '\u{0}'),
        ); // 0xfee0
        hash.insert(
            String::from("Pointer_Right"),
            Unicode::new(xkb::KEY_Pointer_Right, '\u{0}'),
        ); // 0xfee1
        hash.insert(
            String::from("Pointer_Up"),
            Unicode::new(xkb::KEY_Pointer_Up, '\u{0}'),
        ); // 0xfee2
        hash.insert(
            String::from("Pointer_Down"),
            Unicode::new(xkb::KEY_Pointer_Down, '\u{0}'),
        ); // 0xfee3
        hash.insert(
            String::from("Pointer_UpLeft"),
            Unicode::new(xkb::KEY_Pointer_UpLeft, '\u{0}'),
        ); // 0xfee4
        hash.insert(
            String::from("Pointer_UpRight"),
            Unicode::new(xkb::KEY_Pointer_UpRight, '\u{0}'),
        ); // 0xfee5
        hash.insert(
            String::from("Pointer_DownLeft"),
            Unicode::new(xkb::KEY_Pointer_DownLeft, '\u{0}'),
        ); // 0xfee6
        hash.insert(
            String::from("Pointer_DownRight"),
            Unicode::new(xkb::KEY_Pointer_DownRight, '\u{0}'),
        ); // 0xfee7
        hash.insert(
            String::from("Pointer_Button_Dflt"),
            Unicode::new(xkb::KEY_Pointer_Button_Dflt, '\u{0}'),
        ); // 0xfee8
        hash.insert(
            String::from("Pointer_Button1"),
            Unicode::new(xkb::KEY_Pointer_Button1, '\u{0}'),
        ); // 0xfee9
        hash.insert(
            String::from("Pointer_Button2"),
            Unicode::new(xkb::KEY_Pointer_Button2, '\u{0}'),
        ); // 0xfeea
        hash.insert(
            String::from("Pointer_Button3"),
            Unicode::new(xkb::KEY_Pointer_Button3, '\u{0}'),
        ); // 0xfeeb
        hash.insert(
            String::from("Pointer_Button4"),
            Unicode::new(xkb::KEY_Pointer_Button4, '\u{0}'),
        ); // 0xfeec
        hash.insert(
            String::from("Pointer_Button5"),
            Unicode::new(xkb::KEY_Pointer_Button5, '\u{0}'),
        ); // 0xfeed
        hash.insert(
            String::from("Pointer_DblClick_Dflt"),
            Unicode::new(xkb::KEY_Pointer_DblClick_Dflt, '\u{0}'),
        ); // 0xfeee
        hash.insert(
            String::from("Pointer_DblClick1"),
            Unicode::new(xkb::KEY_Pointer_DblClick1, '\u{0}'),
        ); // 0xfeef
        hash.insert(
            String::from("Pointer_DblClick2"),
            Unicode::new(xkb::KEY_Pointer_DblClick2, '\u{0}'),
        ); // 0xfef0
        hash.insert(
            String::from("Pointer_DblClick3"),
            Unicode::new(xkb::KEY_Pointer_DblClick3, '\u{0}'),
        ); // 0xfef1
        hash.insert(
            String::from("Pointer_DblClick4"),
            Unicode::new(xkb::KEY_Pointer_DblClick4, '\u{0}'),
        ); // 0xfef2
        hash.insert(
            String::from("Pointer_DblClick5"),
            Unicode::new(xkb::KEY_Pointer_DblClick5, '\u{0}'),
        ); // 0xfef3
        hash.insert(
            String::from("Pointer_Drag_Dflt"),
            Unicode::new(xkb::KEY_Pointer_Drag_Dflt, '\u{0}'),
        ); // 0xfef4
        hash.insert(
            String::from("Pointer_Drag1"),
            Unicode::new(xkb::KEY_Pointer_Drag1, '\u{0}'),
        ); // 0xfef5
        hash.insert(
            String::from("Pointer_Drag2"),
            Unicode::new(xkb::KEY_Pointer_Drag2, '\u{0}'),
        ); // 0xfef6
        hash.insert(
            String::from("Pointer_Drag3"),
            Unicode::new(xkb::KEY_Pointer_Drag3, '\u{0}'),
        ); // 0xfef7
        hash.insert(
            String::from("Pointer_Drag4"),
            Unicode::new(xkb::KEY_Pointer_Drag4, '\u{0}'),
        ); // 0xfef8
        hash.insert(
            String::from("Pointer_Drag5"),
            Unicode::new(xkb::KEY_Pointer_Drag5, '\u{0}'),
        ); // 0xfefd
        hash.insert(
            String::from("Pointer_EnableKeys"),
            Unicode::new(xkb::KEY_Pointer_EnableKeys, '\u{0}'),
        ); // 0xfef9
        hash.insert(
            String::from("Pointer_Accelerate"),
            Unicode::new(xkb::KEY_Pointer_Accelerate, '\u{0}'),
        ); // 0xfefa
        hash.insert(
            String::from("Pointer_DfltBtnNext"),
            Unicode::new(xkb::KEY_Pointer_DfltBtnNext, '\u{0}'),
        ); // 0xfefb
        hash.insert(
            String::from("Pointer_DfltBtnPrev"),
            Unicode::new(xkb::KEY_Pointer_DfltBtnPrev, '\u{0}'),
        ); // 0xfefc
        hash.insert(String::from("ch"), Unicode::new(xkb::KEY_ch, '\u{0}')); // 0xfea0
        hash.insert(String::from("Ch"), Unicode::new(xkb::KEY_Ch, '\u{0}')); // 0xfea1
        hash.insert(String::from("CH"), Unicode::new(xkb::KEY_CH, '\u{0}')); // 0xfea2
        hash.insert(String::from("c_h"), Unicode::new(xkb::KEY_c_h, '\u{0}')); // 0xfea3
        hash.insert(String::from("C_h"), Unicode::new(xkb::KEY_C_h, '\u{0}')); // 0xfea4
        hash.insert(String::from("C_H"), Unicode::new(xkb::KEY_C_H, '\u{0}')); // 0xfea5
        hash
    }));

    /// Return the `Keysym` string representation based on the `Keysym` code and
    /// the codepoint. The struct that makes up these two things is
    /// [`Unicode`](self::Unicode)
    pub(crate) fn get_str_from_keysym_unicode(&self, unicode: &Unicode) -> Option<&String> {
        self.0.get_by_right(unicode)
    }

    /// Return the `Keysym` string representation based solely on the `Keysym`
    /// code
    pub(crate) fn get_str_from_keysym_code(&self, code: u32) -> Option<&String> {
        Some(self.0.iter().find(|c| c.1.keysym == code)?.0)
    }

    /// Return the `Keysym` code based on the string name
    pub(crate) fn get_keysym_code_from_str(&self, keysym: &str) -> Option<u32> {
        Some(self.0.get_by_left(&keysym.to_string())?.keysym)
    }

    /// Return the `Keysym` code based on the `char`
    pub(crate) fn get_keysym_code_from_char(&self, ch: char) -> Option<u32> {
        Some(self.0.iter().find(|c| c.1.unicode == ch)?.1.keysym)
    }

    /// Return the unicode code-point based on the name
    pub(crate) fn get_codepoint_from_str(&self, keysym: &str) -> Option<char> {
        Some(self.0.get_by_left(&keysym.to_string())?.unicode)
    }

    /// Return the UTF-8 conversion of the `Keysym`
    pub(crate) fn utf8(&self, keysym: &str) -> Result<String, Error> {
        if let Some(key) = self.get_keysym_code_from_str(keysym) {
            String::from_utf8(
                key.to_le_bytes()
                    .to_vec()
                    .into_iter()
                    .filter(|&b| b > 0)
                    .collect(),
            )
            .map_err(|_| Error::Utf8Conversion(keysym.to_string()))
        } else {
            Err(Error::InvalidKey(keysym.to_string()))
        }
    }
}
