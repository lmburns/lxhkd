//! [`Keysym`](xkb::Keysym) wrapper, used to implement custom methods

use anyhow::{Context, Result};
use bimap::BiMap;
use std::{cmp::Ordering, fmt};
use thiserror::Error;
use x11rb::protocol::xproto::Keysym;
use xkbcommon::xkb;
use once_cell::sync::Lazy;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("{0} was not found in the database")]
    InvalidKey(String),
    #[error("failed to convert {0} to UTF-8")]
    Utf8Conversion(String),
}

/// A [`Keysym`](xcb::Keysym) wrapper
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct XKeysym {
    inner: Keysym,
}

impl XKeysym {
    /// Create a new instance of `XKeysym` from a [`Keysym`](xkb::Keysym)
    pub(crate) fn new(inner: xkb::Keysym) -> Self {
        Self { inner }
    }
}

impl From<Keysym> for XKeysym {
    fn from(inner: Keysym) -> XKeysym {
        XKeysym { inner }
    }
}

impl Ord for XKeysym {
    fn cmp(&self, other: &XKeysym) -> Ordering {
        let inner: u32 = self.inner;
        inner.cmp(&other.inner)
    }
}

impl PartialOrd for XKeysym {
    fn partial_cmp(&self, other: &XKeysym) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for XKeysym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

///////////////////////////////////////////////////////////////////

// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

// Taken from the `x11` crate and modified for serialization

/// Hash of available keymaps
pub(crate) struct KeysymHash(Lazy<BiMap<String, Keysym>>);

impl fmt::Debug for KeysymHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}

impl KeysymHash {
    #[allow(clippy::declare_interior_mutable_const)]
    pub(crate) const HASH: Self = Self(Lazy::new(|| {
        let mut hash = BiMap::new();

        hash.insert(String::from("BackSpace"), xkb::KEY_BackSpace); // 0xff08
        hash.insert(String::from("Tab"), xkb::KEY_Tab); // 0xff09
        hash.insert(String::from("Linefeed"), xkb::KEY_Linefeed); // 0xff0a
        hash.insert(String::from("Clear"), xkb::KEY_Clear); // 0xff0b
        hash.insert(String::from("Return"), xkb::KEY_Return); // 0xff0d
        hash.insert(String::from("Pause"), xkb::KEY_Pause); // 0xff13
        hash.insert(String::from("Scroll_Lock"), xkb::KEY_Scroll_Lock); // 0xff14
        hash.insert(String::from("Sys_Req"), xkb::KEY_Sys_Req); // 0xff15
        hash.insert(String::from("Escape"), xkb::KEY_Escape); // 0xff1b
        hash.insert(String::from("Delete"), xkb::KEY_Delete); // 0xffff
        hash.insert(String::from("Multi_key"), xkb::KEY_Multi_key); // 0xff20
        hash.insert(String::from("Codeinput"), xkb::KEY_Codeinput); // 0xff37
        hash.insert(String::from("SingleCandidate"), xkb::KEY_SingleCandidate); // 0xff3c
        hash.insert(
            String::from("MultipleCandidate"),
            xkb::KEY_MultipleCandidate,
        ); // 0xff3d
        hash.insert(
            String::from("PreviousCandidate"),
            xkb::KEY_PreviousCandidate,
        ); // 0xff3e
        hash.insert(String::from("Kanji"), xkb::KEY_Kanji); // 0xff21
        hash.insert(String::from("Muhenkan"), xkb::KEY_Muhenkan); // 0xff22
        hash.insert(String::from("Henkan_Mode"), xkb::KEY_Henkan_Mode); // 0xff23
        hash.insert(String::from("Henkan"), xkb::KEY_Henkan); // 0xff23
        hash.insert(String::from("Romaji"), xkb::KEY_Romaji); // 0xff24
        hash.insert(String::from("Hiragana"), xkb::KEY_Hiragana); // 0xff25
        hash.insert(String::from("Katakana"), xkb::KEY_Katakana); // 0xff26
        hash.insert(
            String::from("Hiragana_Katakana"),
            xkb::KEY_Hiragana_Katakana,
        ); // 0xff27
        hash.insert(String::from("Zenkaku"), xkb::KEY_Zenkaku); // 0xff28
        hash.insert(String::from("Hankaku"), xkb::KEY_Hankaku); // 0xff29
        hash.insert(String::from("Zenkaku_Hankaku"), xkb::KEY_Zenkaku_Hankaku); // 0xff2a
        hash.insert(String::from("Touroku"), xkb::KEY_Touroku); // 0xff2b
        hash.insert(String::from("Massyo"), xkb::KEY_Massyo); // 0xff2c
        hash.insert(String::from("Kana_Lock"), xkb::KEY_Kana_Lock); // 0xff2d
        hash.insert(String::from("Kana_Shift"), xkb::KEY_Kana_Shift); // 0xff2e
        hash.insert(String::from("Eisu_Shift"), xkb::KEY_Eisu_Shift); // 0xff2f
        hash.insert(String::from("Eisu_toggle"), xkb::KEY_Eisu_toggle); // 0xff30
        hash.insert(String::from("Kanji_Bangou"), xkb::KEY_Kanji_Bangou); // 0xff37
        hash.insert(String::from("Zen_Koho"), xkb::KEY_Zen_Koho); // 0xff3d
        hash.insert(String::from("Mae_Koho"), xkb::KEY_Mae_Koho); // 0xff3e
        hash.insert(String::from("Home"), xkb::KEY_Home); // 0xff50
        hash.insert(String::from("Left"), xkb::KEY_Left); // 0xff51
        hash.insert(String::from("Up"), xkb::KEY_Up); // 0xff52
        hash.insert(String::from("Right"), xkb::KEY_Right); // 0xff53
        hash.insert(String::from("Down"), xkb::KEY_Down); // 0xff54
        hash.insert(String::from("Prior"), xkb::KEY_Prior); // 0xff55
        hash.insert(String::from("Page_Up"), xkb::KEY_Page_Up); // 0xff55
        hash.insert(String::from("Next"), xkb::KEY_Next); // 0xff56
        hash.insert(String::from("Page_Down"), xkb::KEY_Page_Down); // 0xff56
        hash.insert(String::from("End"), xkb::KEY_End); // 0xff57
        hash.insert(String::from("Begin"), xkb::KEY_Begin); // 0xff58
        hash.insert(String::from("Select"), xkb::KEY_Select); // 0xff60
        hash.insert(String::from("Print"), xkb::KEY_Print); // 0xff61
        hash.insert(String::from("Execute"), xkb::KEY_Execute); // 0xff62
        hash.insert(String::from("Insert"), xkb::KEY_Insert); // 0xff63
        hash.insert(String::from("Undo"), xkb::KEY_Undo); // 0xff65
        hash.insert(String::from("Redo"), xkb::KEY_Redo); // 0xff66
        hash.insert(String::from("Menu"), xkb::KEY_Menu); // 0xff67
        hash.insert(String::from("Find"), xkb::KEY_Find); // 0xff68
        hash.insert(String::from("Cancel"), xkb::KEY_Cancel); // 0xff69
        hash.insert(String::from("Help"), xkb::KEY_Help); // 0xff6a
        hash.insert(String::from("Break"), xkb::KEY_Break); // 0xff6b
        hash.insert(String::from("Mode_switch"), xkb::KEY_Mode_switch); // 0xff7e
        hash.insert(String::from("script_switch"), xkb::KEY_script_switch); // 0xff7e
        hash.insert(String::from("Num_Lock"), xkb::KEY_Num_Lock); // 0xff7f
        hash.insert(String::from("KP_Space"), xkb::KEY_KP_Space); // 0xff80
        hash.insert(String::from("KP_Tab"), xkb::KEY_KP_Tab); // 0xff89
        hash.insert(String::from("KP_Enter"), xkb::KEY_KP_Enter); // 0xff8d
        hash.insert(String::from("KP_F1"), xkb::KEY_KP_F1); // 0xff91
        hash.insert(String::from("KP_F2"), xkb::KEY_KP_F2); // 0xff92
        hash.insert(String::from("KP_F3"), xkb::KEY_KP_F3); // 0xff93
        hash.insert(String::from("KP_F4"), xkb::KEY_KP_F4); // 0xff94
        hash.insert(String::from("KP_Home"), xkb::KEY_KP_Home); // 0xff95
        hash.insert(String::from("KP_Left"), xkb::KEY_KP_Left); // 0xff96
        hash.insert(String::from("KP_Up"), xkb::KEY_KP_Up); // 0xff97
        hash.insert(String::from("KP_Right"), xkb::KEY_KP_Right); // 0xff98
        hash.insert(String::from("KP_Down"), xkb::KEY_KP_Down); // 0xff99
        hash.insert(String::from("KP_Prior"), xkb::KEY_KP_Prior); // 0xff9a
        hash.insert(String::from("KP_Page_Up"), xkb::KEY_KP_Page_Up); // 0xff9a
        hash.insert(String::from("KP_Next"), xkb::KEY_KP_Next); // 0xff9b
        hash.insert(String::from("KP_Page_Down"), xkb::KEY_KP_Page_Down); // 0xff9b
        hash.insert(String::from("KP_End"), xkb::KEY_KP_End); // 0xff9c
        hash.insert(String::from("KP_Begin"), xkb::KEY_KP_Begin); // 0xff9d
        hash.insert(String::from("KP_Insert"), xkb::KEY_KP_Insert); // 0xff9e
        hash.insert(String::from("KP_Delete"), xkb::KEY_KP_Delete); // 0xff9f
        hash.insert(String::from("KP_Equal"), xkb::KEY_KP_Equal); // 0xffbd
        hash.insert(String::from("KP_Multiply"), xkb::KEY_KP_Multiply); // 0xffaa
        hash.insert(String::from("KP_Add"), xkb::KEY_KP_Add); // 0xffab
        hash.insert(String::from("KP_Separator"), xkb::KEY_KP_Separator); // 0xffac
        hash.insert(String::from("KP_Subtract"), xkb::KEY_KP_Subtract); // 0xffad
        hash.insert(String::from("KP_Decimal"), xkb::KEY_KP_Decimal); // 0xffae
        hash.insert(String::from("KP_Divide"), xkb::KEY_KP_Divide); // 0xffaf
        hash.insert(String::from("KP_0"), xkb::KEY_KP_0); // 0xffb0
        hash.insert(String::from("KP_1"), xkb::KEY_KP_1); // 0xffb1
        hash.insert(String::from("KP_2"), xkb::KEY_KP_2); // 0xffb2
        hash.insert(String::from("KP_3"), xkb::KEY_KP_3); // 0xffb3
        hash.insert(String::from("KP_4"), xkb::KEY_KP_4); // 0xffb4
        hash.insert(String::from("KP_5"), xkb::KEY_KP_5); // 0xffb5
        hash.insert(String::from("KP_6"), xkb::KEY_KP_6); // 0xffb6
        hash.insert(String::from("KP_7"), xkb::KEY_KP_7); // 0xffb7
        hash.insert(String::from("KP_8"), xkb::KEY_KP_8); // 0xffb8
        hash.insert(String::from("KP_9"), xkb::KEY_KP_9); // 0xffb9
        hash.insert(String::from("F1"), xkb::KEY_F1); // 0xffbe
        hash.insert(String::from("F2"), xkb::KEY_F2); // 0xffbf
        hash.insert(String::from("F3"), xkb::KEY_F3); // 0xffc0
        hash.insert(String::from("F4"), xkb::KEY_F4); // 0xffc1
        hash.insert(String::from("F5"), xkb::KEY_F5); // 0xffc2
        hash.insert(String::from("F6"), xkb::KEY_F6); // 0xffc3
        hash.insert(String::from("F7"), xkb::KEY_F7); // 0xffc4
        hash.insert(String::from("F8"), xkb::KEY_F8); // 0xffc5
        hash.insert(String::from("F9"), xkb::KEY_F9); // 0xffc6
        hash.insert(String::from("F10"), xkb::KEY_F10); // 0xffc7
        hash.insert(String::from("F11"), xkb::KEY_F11); // 0xffc8
        hash.insert(String::from("L1"), xkb::KEY_L1); // 0xffc8
        hash.insert(String::from("F12"), xkb::KEY_F12); // 0xffc9
        hash.insert(String::from("L2"), xkb::KEY_L2); // 0xffc9
        hash.insert(String::from("F13"), xkb::KEY_F13); // 0xffca
        hash.insert(String::from("L3"), xkb::KEY_L3); // 0xffca
        hash.insert(String::from("F14"), xkb::KEY_F14); // 0xffcb
        hash.insert(String::from("L4"), xkb::KEY_L4); // 0xffcb
        hash.insert(String::from("F15"), xkb::KEY_F15); // 0xffcc
        hash.insert(String::from("L5"), xkb::KEY_L5); // 0xffcc
        hash.insert(String::from("F16"), xkb::KEY_F16); // 0xffcd
        hash.insert(String::from("L6"), xkb::KEY_L6); // 0xffcd
        hash.insert(String::from("F17"), xkb::KEY_F17); // 0xffce
        hash.insert(String::from("L7"), xkb::KEY_L7); // 0xffce
        hash.insert(String::from("F18"), xkb::KEY_F18); // 0xffcf
        hash.insert(String::from("L8"), xkb::KEY_L8); // 0xffcf
        hash.insert(String::from("F19"), xkb::KEY_F19); // 0xffd0
        hash.insert(String::from("L9"), xkb::KEY_L9); // 0xffd0
        hash.insert(String::from("F20"), xkb::KEY_F20); // 0xffd1
        hash.insert(String::from("L10"), xkb::KEY_L10); // 0xffd1
        hash.insert(String::from("F21"), xkb::KEY_F21); // 0xffd2
        hash.insert(String::from("R1"), xkb::KEY_R1); // 0xffd2
        hash.insert(String::from("F22"), xkb::KEY_F22); // 0xffd3
        hash.insert(String::from("R2"), xkb::KEY_R2); // 0xffd3
        hash.insert(String::from("F23"), xkb::KEY_F23); // 0xffd4
        hash.insert(String::from("R3"), xkb::KEY_R3); // 0xffd4
        hash.insert(String::from("F24"), xkb::KEY_F24); // 0xffd5
        hash.insert(String::from("R4"), xkb::KEY_R4); // 0xffd5
        hash.insert(String::from("F25"), xkb::KEY_F25); // 0xffd6
        hash.insert(String::from("R5"), xkb::KEY_R5); // 0xffd6
        hash.insert(String::from("F26"), xkb::KEY_F26); // 0xffd7
        hash.insert(String::from("R6"), xkb::KEY_R6); // 0xffd7
        hash.insert(String::from("F27"), xkb::KEY_F27); // 0xffd8
        hash.insert(String::from("R7"), xkb::KEY_R7); // 0xffd8
        hash.insert(String::from("F28"), xkb::KEY_F28); // 0xffd9
        hash.insert(String::from("R8"), xkb::KEY_R8); // 0xffd9
        hash.insert(String::from("F29"), xkb::KEY_F29); // 0xffda
        hash.insert(String::from("R9"), xkb::KEY_R9); // 0xffda
        hash.insert(String::from("F30"), xkb::KEY_F30); // 0xffdb
        hash.insert(String::from("R10"), xkb::KEY_R10); // 0xffdb
        hash.insert(String::from("F31"), xkb::KEY_F31); // 0xffdc
        hash.insert(String::from("R11"), xkb::KEY_R11); // 0xffdc
        hash.insert(String::from("F32"), xkb::KEY_F32); // 0xffdd
        hash.insert(String::from("R12"), xkb::KEY_R12); // 0xffdd
        hash.insert(String::from("F33"), xkb::KEY_F33); // 0xffde
        hash.insert(String::from("R13"), xkb::KEY_R13); // 0xffde
        hash.insert(String::from("F34"), xkb::KEY_F34); // 0xffdf
        hash.insert(String::from("R14"), xkb::KEY_R14); // 0xffdf
        hash.insert(String::from("F35"), xkb::KEY_F35); // 0xffe0
        hash.insert(String::from("R15"), xkb::KEY_R15); // 0xffe0
        hash.insert(String::from("Shift_L"), xkb::KEY_Shift_L); // 0xffe1
        hash.insert(String::from("Shift_R"), xkb::KEY_Shift_R); // 0xffe2
        hash.insert(String::from("Control_L"), xkb::KEY_Control_L); // 0xffe3
        hash.insert(String::from("Control_R"), xkb::KEY_Control_R); // 0xffe4
        hash.insert(String::from("Caps_Lock"), xkb::KEY_Caps_Lock); // 0xffe5
        hash.insert(String::from("Shift_Lock"), xkb::KEY_Shift_Lock); // 0xffe6
        hash.insert(String::from("Meta_L"), xkb::KEY_Meta_L); // 0xffe7
        hash.insert(String::from("Meta_R"), xkb::KEY_Meta_R); // 0xffe8
        hash.insert(String::from("Alt_L"), xkb::KEY_Alt_L); // 0xffe9
        hash.insert(String::from("Alt_R"), xkb::KEY_Alt_R); // 0xffea
        hash.insert(String::from("Super_L"), xkb::KEY_Super_L); // 0xffeb
        hash.insert(String::from("Super_R"), xkb::KEY_Super_R); // 0xffec
        hash.insert(String::from("Hyper_L"), xkb::KEY_Hyper_L); // 0xffed
        hash.insert(String::from("Hyper_R"), xkb::KEY_Hyper_R); // 0xffee

        hash.insert(String::from("ISO_Lock"), xkb::KEY_ISO_Lock); // 0xfe01
        hash.insert(String::from("ISO_Level2_Latch"), xkb::KEY_ISO_Level2_Latch); // 0xfe02
        hash.insert(String::from("ISO_Level3_Shift"), xkb::KEY_ISO_Level3_Shift); // 0xfe03
        hash.insert(String::from("ISO_Level3_Latch"), xkb::KEY_ISO_Level3_Latch); // 0xfe04
        hash.insert(String::from("ISO_Level3_Lock"), xkb::KEY_ISO_Level3_Lock); // 0xfe05
        hash.insert(String::from("ISO_Level5_Shift"), xkb::KEY_ISO_Level5_Shift); // 0xfe11
        hash.insert(String::from("ISO_Level5_Latch"), xkb::KEY_ISO_Level5_Latch); // 0xfe12
        hash.insert(String::from("ISO_Level5_Lock"), xkb::KEY_ISO_Level5_Lock); // 0xfe13
        hash.insert(String::from("ISO_Group_Shift"), xkb::KEY_ISO_Group_Shift); // 0xff7e
        hash.insert(String::from("ISO_Group_Latch"), xkb::KEY_ISO_Group_Latch); // 0xfe06
        hash.insert(String::from("ISO_Group_Lock"), xkb::KEY_ISO_Group_Lock); // 0xfe07
        hash.insert(String::from("ISO_Next_Group"), xkb::KEY_ISO_Next_Group); // 0xfe08
        hash.insert(
            String::from("ISO_Next_Group_Lock"),
            xkb::KEY_ISO_Next_Group_Lock,
        ); // 0xfe09
        hash.insert(String::from("ISO_Prev_Group"), xkb::KEY_ISO_Prev_Group); // 0xfe0a
        hash.insert(
            String::from("ISO_Prev_Group_Lock"),
            xkb::KEY_ISO_Prev_Group_Lock,
        ); // 0xfe0b
        hash.insert(String::from("ISO_First_Group"), xkb::KEY_ISO_First_Group); // 0xfe0c
        hash.insert(
            String::from("ISO_First_Group_Lock"),
            xkb::KEY_ISO_First_Group_Lock,
        ); // 0xfe0d
        hash.insert(String::from("ISO_Last_Group"), xkb::KEY_ISO_Last_Group); // 0xfe0e
        hash.insert(
            String::from("ISO_Last_Group_Lock"),
            xkb::KEY_ISO_Last_Group_Lock,
        ); // 0xfe0f
        hash.insert(String::from("ISO_Left_Tab"), xkb::KEY_ISO_Left_Tab); // 0xfe20
        hash.insert(String::from("ISO_Move_Line_Up"), xkb::KEY_ISO_Move_Line_Up); // 0xfe21
        hash.insert(
            String::from("ISO_Move_Line_Down"),
            xkb::KEY_ISO_Move_Line_Down,
        ); // 0xfe22
        hash.insert(
            String::from("ISO_Partial_Line_Up"),
            xkb::KEY_ISO_Partial_Line_Up,
        ); // 0xfe23
        hash.insert(
            String::from("ISO_Partial_Line_Down"),
            xkb::KEY_ISO_Partial_Line_Down,
        ); // 0xfe24
        hash.insert(
            String::from("ISO_Partial_Space_Left"),
            xkb::KEY_ISO_Partial_Space_Left,
        ); // 0xfe25
        hash.insert(
            String::from("ISO_Partial_Space_Right"),
            xkb::KEY_ISO_Partial_Space_Right,
        ); // 0xfe26
        hash.insert(
            String::from("ISO_Set_Margin_Left"),
            xkb::KEY_ISO_Set_Margin_Left,
        ); // 0xfe27
        hash.insert(
            String::from("ISO_Set_Margin_Right"),
            xkb::KEY_ISO_Set_Margin_Right,
        ); // 0xfe28
        hash.insert(
            String::from("ISO_Release_Margin_Left"),
            xkb::KEY_ISO_Release_Margin_Left,
        ); // 0xfe29
        hash.insert(
            String::from("ISO_Release_Margin_Right"),
            xkb::KEY_ISO_Release_Margin_Right,
        ); // 0xfe2a
        hash.insert(
            String::from("ISO_Release_Both_Margins"),
            xkb::KEY_ISO_Release_Both_Margins,
        ); // 0xfe2b
        hash.insert(
            String::from("ISO_Fast_Cursor_Left"),
            xkb::KEY_ISO_Fast_Cursor_Left,
        ); // 0xfe2c
        hash.insert(
            String::from("ISO_Fast_Cursor_Right"),
            xkb::KEY_ISO_Fast_Cursor_Right,
        ); // 0xfe2d
        hash.insert(
            String::from("ISO_Fast_Cursor_Up"),
            xkb::KEY_ISO_Fast_Cursor_Up,
        ); // 0xfe2e
        hash.insert(
            String::from("ISO_Fast_Cursor_Down"),
            xkb::KEY_ISO_Fast_Cursor_Down,
        ); // 0xfe2f
        hash.insert(
            String::from("ISO_Continuous_Underline"),
            xkb::KEY_ISO_Continuous_Underline,
        ); // 0xfe30
        hash.insert(
            String::from("ISO_Discontinuous_Underline"),
            xkb::KEY_ISO_Discontinuous_Underline,
        ); // 0xfe31
        hash.insert(String::from("ISO_Emphasize"), xkb::KEY_ISO_Emphasize); // 0xfe32
        hash.insert(
            String::from("ISO_Center_Object"),
            xkb::KEY_ISO_Center_Object,
        ); // 0xfe33
        hash.insert(String::from("ISO_Enter"), xkb::KEY_ISO_Enter); // 0xfe34
        hash.insert(String::from("dead_grave"), xkb::KEY_dead_grave); // 0xfe50
        hash.insert(String::from("dead_acute"), xkb::KEY_dead_acute); // 0xfe51
        hash.insert(String::from("dead_circumflex"), xkb::KEY_dead_circumflex); // 0xfe52
        hash.insert(String::from("dead_tilde"), xkb::KEY_dead_tilde); // 0xfe53
        hash.insert(String::from("dead_perispomeni"), xkb::KEY_dead_perispomeni); // 0xfe53
        hash.insert(String::from("dead_macron"), xkb::KEY_dead_macron); // 0xfe54
        hash.insert(String::from("dead_breve"), xkb::KEY_dead_breve); // 0xfe55
        hash.insert(String::from("dead_abovedot"), xkb::KEY_dead_abovedot); // 0xfe56
        hash.insert(String::from("dead_diaeresis"), xkb::KEY_dead_diaeresis); // 0xfe57
        hash.insert(String::from("dead_abovering"), xkb::KEY_dead_abovering); // 0xfe58
        hash.insert(String::from("dead_doubleacute"), xkb::KEY_dead_doubleacute); // 0xfe59
        hash.insert(String::from("dead_caron"), xkb::KEY_dead_caron); // 0xfe5a
        hash.insert(String::from("dead_cedilla"), xkb::KEY_dead_cedilla); // 0xfe5b
        hash.insert(String::from("dead_ogonek"), xkb::KEY_dead_ogonek); // 0xfe5c
        hash.insert(String::from("dead_iota"), xkb::KEY_dead_iota); // 0xfe5d
        hash.insert(
            String::from("dead_voiced_sound"),
            xkb::KEY_dead_voiced_sound,
        ); // 0xfe5e
        hash.insert(
            String::from("dead_semivoiced_sound"),
            xkb::KEY_dead_semivoiced_sound,
        ); // 0xfe5f
        hash.insert(String::from("dead_belowdot"), xkb::KEY_dead_belowdot); // 0xfe60
        hash.insert(String::from("dead_hook"), xkb::KEY_dead_hook); // 0xfe61
        hash.insert(String::from("dead_horn"), xkb::KEY_dead_horn); // 0xfe62
        hash.insert(String::from("dead_stroke"), xkb::KEY_dead_stroke); // 0xfe63
        hash.insert(String::from("dead_abovecomma"), xkb::KEY_dead_abovecomma); // 0xfe64
        hash.insert(String::from("dead_psili"), xkb::KEY_dead_psili); // 0xfe64
        hash.insert(
            String::from("dead_abovereversedcomma"),
            xkb::KEY_dead_abovereversedcomma,
        ); // 0xfe65
        hash.insert(String::from("dead_dasia"), xkb::KEY_dead_dasia); // 0xfe65
        hash.insert(String::from("dead_doublegrave"), xkb::KEY_dead_doublegrave); // 0xfe66
        hash.insert(String::from("dead_belowring"), xkb::KEY_dead_belowring); // 0xfe67
        hash.insert(String::from("dead_belowmacron"), xkb::KEY_dead_belowmacron); // 0xfe68
        hash.insert(
            String::from("dead_belowcircumflex"),
            xkb::KEY_dead_belowcircumflex,
        ); // 0xfe69
        hash.insert(String::from("dead_belowtilde"), xkb::KEY_dead_belowtilde); // 0xfe6a
        hash.insert(String::from("dead_belowbreve"), xkb::KEY_dead_belowbreve); // 0xfe6b
        hash.insert(
            String::from("dead_belowdiaeresis"),
            xkb::KEY_dead_belowdiaeresis,
        ); // 0xfe6c
        hash.insert(
            String::from("dead_invertedbreve"),
            xkb::KEY_dead_invertedbreve,
        ); // 0xfe6d
        hash.insert(String::from("dead_belowcomma"), xkb::KEY_dead_belowcomma); // 0xfe6e
        hash.insert(String::from("dead_currency"), xkb::KEY_dead_currency); // 0xfe6f
        hash.insert(String::from("dead_a"), xkb::KEY_dead_a); // 0xfe80
        hash.insert(String::from("dead_A"), xkb::KEY_dead_A); // 0xfe81
        hash.insert(String::from("dead_e"), xkb::KEY_dead_e); // 0xfe82
        hash.insert(String::from("dead_E"), xkb::KEY_dead_E); // 0xfe83
        hash.insert(String::from("dead_i"), xkb::KEY_dead_i); // 0xfe84
        hash.insert(String::from("dead_I"), xkb::KEY_dead_I); // 0xfe85
        hash.insert(String::from("dead_o"), xkb::KEY_dead_o); // 0xfe86
        hash.insert(String::from("dead_O"), xkb::KEY_dead_O); // 0xfe87
        hash.insert(String::from("dead_u"), xkb::KEY_dead_u); // 0xfe88
        hash.insert(String::from("dead_U"), xkb::KEY_dead_U); // 0xfe89
        hash.insert(String::from("dead_small_schwa"), xkb::KEY_dead_small_schwa); // 0xfe8a
        hash.insert(
            String::from("dead_capital_schwa"),
            xkb::KEY_dead_capital_schwa,
        ); // 0xfe8b
        hash.insert(String::from("dead_greek"), xkb::KEY_dead_greek); // 0xfe8c
        hash.insert(
            String::from("First_Virtual_Screen"),
            xkb::KEY_First_Virtual_Screen,
        ); // 0xfed0
        hash.insert(
            String::from("Prev_Virtual_Screen"),
            xkb::KEY_Prev_Virtual_Screen,
        ); // 0xfed1
        hash.insert(
            String::from("Next_Virtual_Screen"),
            xkb::KEY_Next_Virtual_Screen,
        ); // 0xfed2
        hash.insert(
            String::from("Last_Virtual_Screen"),
            xkb::KEY_Last_Virtual_Screen,
        ); // 0xfed4
        hash.insert(String::from("Terminate_Server"), xkb::KEY_Terminate_Server); // 0xfed5
        hash.insert(String::from("AccessX_Enable"), xkb::KEY_AccessX_Enable); // 0xfe70
        hash.insert(
            String::from("AccessX_Feedback_Enable"),
            xkb::KEY_AccessX_Feedback_Enable,
        ); // 0xfe71
        hash.insert(
            String::from("RepeatKeys_Enable"),
            xkb::KEY_RepeatKeys_Enable,
        ); // 0xfe72
        hash.insert(String::from("SlowKeys_Enable"), xkb::KEY_SlowKeys_Enable); // 0xfe73
        hash.insert(
            String::from("BounceKeys_Enable"),
            xkb::KEY_BounceKeys_Enable,
        ); // 0xfe74
        hash.insert(
            String::from("StickyKeys_Enable"),
            xkb::KEY_StickyKeys_Enable,
        ); // 0xfe75
        hash.insert(String::from("MouseKeys_Enable"), xkb::KEY_MouseKeys_Enable); // 0xfe76
        hash.insert(
            String::from("MouseKeys_Accel_Enable"),
            xkb::KEY_MouseKeys_Accel_Enable,
        ); // 0xfe77
        hash.insert(String::from("Overlay1_Enable"), xkb::KEY_Overlay1_Enable); // 0xfe78
        hash.insert(String::from("Overlay2_Enable"), xkb::KEY_Overlay2_Enable); // 0xfe79
        hash.insert(
            String::from("AudibleBell_Enable"),
            xkb::KEY_AudibleBell_Enable,
        ); // 0xfe7a
        hash.insert(String::from("Pointer_Left"), xkb::KEY_Pointer_Left); // 0xfee0
        hash.insert(String::from("Pointer_Right"), xkb::KEY_Pointer_Right); // 0xfee1
        hash.insert(String::from("Pointer_Up"), xkb::KEY_Pointer_Up); // 0xfee2
        hash.insert(String::from("Pointer_Down"), xkb::KEY_Pointer_Down); // 0xfee3
        hash.insert(String::from("Pointer_UpLeft"), xkb::KEY_Pointer_UpLeft); // 0xfee4
        hash.insert(String::from("Pointer_UpRight"), xkb::KEY_Pointer_UpRight); // 0xfee5
        hash.insert(String::from("Pointer_DownLeft"), xkb::KEY_Pointer_DownLeft); // 0xfee6
        hash.insert(
            String::from("Pointer_DownRight"),
            xkb::KEY_Pointer_DownRight,
        ); // 0xfee7
        hash.insert(
            String::from("Pointer_Button_Dflt"),
            xkb::KEY_Pointer_Button_Dflt,
        ); // 0xfee8
        hash.insert(String::from("Pointer_Button1"), xkb::KEY_Pointer_Button1); // 0xfee9
        hash.insert(String::from("Pointer_Button2"), xkb::KEY_Pointer_Button2); // 0xfeea
        hash.insert(String::from("Pointer_Button3"), xkb::KEY_Pointer_Button3); // 0xfeeb
        hash.insert(String::from("Pointer_Button4"), xkb::KEY_Pointer_Button4); // 0xfeec
        hash.insert(String::from("Pointer_Button5"), xkb::KEY_Pointer_Button5); // 0xfeed
        hash.insert(
            String::from("Pointer_DblClick_Dflt"),
            xkb::KEY_Pointer_DblClick_Dflt,
        ); // 0xfeee
        hash.insert(
            String::from("Pointer_DblClick1"),
            xkb::KEY_Pointer_DblClick1,
        ); // 0xfeef
        hash.insert(
            String::from("Pointer_DblClick2"),
            xkb::KEY_Pointer_DblClick2,
        ); // 0xfef0
        hash.insert(
            String::from("Pointer_DblClick3"),
            xkb::KEY_Pointer_DblClick3,
        ); // 0xfef1
        hash.insert(
            String::from("Pointer_DblClick4"),
            xkb::KEY_Pointer_DblClick4,
        ); // 0xfef2
        hash.insert(
            String::from("Pointer_DblClick5"),
            xkb::KEY_Pointer_DblClick5,
        ); // 0xfef3
        hash.insert(
            String::from("Pointer_Drag_Dflt"),
            xkb::KEY_Pointer_Drag_Dflt,
        ); // 0xfef4
        hash.insert(String::from("Pointer_Drag1"), xkb::KEY_Pointer_Drag1); // 0xfef5
        hash.insert(String::from("Pointer_Drag2"), xkb::KEY_Pointer_Drag2); // 0xfef6
        hash.insert(String::from("Pointer_Drag3"), xkb::KEY_Pointer_Drag3); // 0xfef7
        hash.insert(String::from("Pointer_Drag4"), xkb::KEY_Pointer_Drag4); // 0xfef8
        hash.insert(String::from("Pointer_Drag5"), xkb::KEY_Pointer_Drag5); // 0xfefd
        hash.insert(
            String::from("Pointer_EnableKeys"),
            xkb::KEY_Pointer_EnableKeys,
        ); // 0xfef9
        hash.insert(
            String::from("Pointer_Accelerate"),
            xkb::KEY_Pointer_Accelerate,
        ); // 0xfefa
        hash.insert(
            String::from("Pointer_DfltBtnNext"),
            xkb::KEY_Pointer_DfltBtnNext,
        ); // 0xfefb
        hash.insert(
            String::from("Pointer_DfltBtnPrev"),
            xkb::KEY_Pointer_DfltBtnPrev,
        ); // 0xfefc
        hash.insert(String::from("ch"), xkb::KEY_ch); // 0xfea0
        hash.insert(String::from("Ch"), xkb::KEY_Ch); // 0xfea1
        hash.insert(String::from("CH"), xkb::KEY_CH); // 0xfea2
        hash.insert(String::from("c_h"), xkb::KEY_c_h); // 0xfea3
        hash.insert(String::from("C_h"), xkb::KEY_C_h); // 0xfea4
        hash.insert(String::from("C_H"), xkb::KEY_C_H); // 0xfea5

        hash.insert(String::from("3270_Duplicate"), xkb::KEY_3270_Duplicate); // 0xfd01
        hash.insert(String::from("3270_FieldMark"), xkb::KEY_3270_FieldMark); // 0xfd02
        hash.insert(String::from("3270_Right2"), xkb::KEY_3270_Right2); // 0xfd03
        hash.insert(String::from("3270_Left2"), xkb::KEY_3270_Left2); // 0xfd04
        hash.insert(String::from("3270_BackTab"), xkb::KEY_3270_BackTab); // 0xfd05
        hash.insert(String::from("3270_EraseEOF"), xkb::KEY_3270_EraseEOF); // 0xfd06
        hash.insert(String::from("3270_EraseInput"), xkb::KEY_3270_EraseInput); // 0xfd07
        hash.insert(String::from("3270_Reset"), xkb::KEY_3270_Reset); // 0xfd08
        hash.insert(String::from("3270_Quit"), xkb::KEY_3270_Quit); // 0xfd09
        hash.insert(String::from("3270_PA1"), xkb::KEY_3270_PA1); // 0xfd0a
        hash.insert(String::from("3270_PA2"), xkb::KEY_3270_PA2); // 0xfd0b
        hash.insert(String::from("3270_PA3"), xkb::KEY_3270_PA3); // 0xfd0c
        hash.insert(String::from("3270_Test"), xkb::KEY_3270_Test); // 0xfd0d
        hash.insert(String::from("3270_Attn"), xkb::KEY_3270_Attn); // 0xfd0e
        hash.insert(String::from("3270_CursorBlink"), xkb::KEY_3270_CursorBlink); // 0xfd0f
        hash.insert(String::from("3270_AltCursor"), xkb::KEY_3270_AltCursor); // 0xfd10
        hash.insert(String::from("3270_KeyClick"), xkb::KEY_3270_KeyClick); // 0xfd11
        hash.insert(String::from("3270_Jump"), xkb::KEY_3270_Jump); // 0xfd12
        hash.insert(String::from("3270_Ident"), xkb::KEY_3270_Ident); // 0xfd13
        hash.insert(String::from("3270_Rule"), xkb::KEY_3270_Rule); // 0xfd14
        hash.insert(String::from("3270_Copy"), xkb::KEY_3270_Copy); // 0xfd15
        hash.insert(String::from("3270_Play"), xkb::KEY_3270_Play); // 0xfd16
        hash.insert(String::from("3270_Setup"), xkb::KEY_3270_Setup); // 0xfd17
        hash.insert(String::from("3270_Record"), xkb::KEY_3270_Record); // 0xfd18
        hash.insert(
            String::from("3270_ChangeScreen"),
            xkb::KEY_3270_ChangeScreen,
        ); // 0xfd19
        hash.insert(String::from("3270_DeleteWord"), xkb::KEY_3270_DeleteWord); // 0xfd1a
        hash.insert(String::from("3270_ExSelect"), xkb::KEY_3270_ExSelect); // 0xfd1b
        hash.insert(
            String::from("3270_CursorSelect"),
            xkb::KEY_3270_CursorSelect,
        ); // 0xfd1c
        hash.insert(String::from("3270_PrintScreen"), xkb::KEY_3270_PrintScreen); // 0xfd1d
        hash.insert(String::from("3270_Enter"), xkb::KEY_3270_Enter); // 0xfd1e

        hash.insert(String::from("space"), xkb::KEY_space); // 0x0020
        hash.insert(String::from("exclam"), xkb::KEY_exclam); // 0x0021
        hash.insert(String::from("quotedbl"), xkb::KEY_quotedbl); // 0x0022
        hash.insert(String::from("numbersign"), xkb::KEY_numbersign); // 0x0023
        hash.insert(String::from("dollar"), xkb::KEY_dollar); // 0x0024
        hash.insert(String::from("percent"), xkb::KEY_percent); // 0x0025
        hash.insert(String::from("ampersand"), xkb::KEY_ampersand); // 0x0026
        hash.insert(String::from("apostrophe"), xkb::KEY_apostrophe); // 0x0027
        hash.insert(String::from("quoteright"), xkb::KEY_quoteright); // 0x0027
        hash.insert(String::from("parenleft"), xkb::KEY_parenleft); // 0x0028
        hash.insert(String::from("parenright"), xkb::KEY_parenright); // 0x0029
        hash.insert(String::from("asterisk"), xkb::KEY_asterisk); // 0x002a
        hash.insert(String::from("plus"), xkb::KEY_plus); // 0x002b
        hash.insert(String::from("comma"), xkb::KEY_comma); // 0x002c
        hash.insert(String::from("minus"), xkb::KEY_minus); // 0x002d
        hash.insert(String::from("period"), xkb::KEY_period); // 0x002e
        hash.insert(String::from("slash"), xkb::KEY_slash); // 0x002f
        hash.insert(String::from("0"), xkb::KEY_0); // 0x0030
        hash.insert(String::from("1"), xkb::KEY_1); // 0x0031
        hash.insert(String::from("2"), xkb::KEY_2); // 0x0032
        hash.insert(String::from("3"), xkb::KEY_3); // 0x0033
        hash.insert(String::from("4"), xkb::KEY_4); // 0x0034
        hash.insert(String::from("5"), xkb::KEY_5); // 0x0035
        hash.insert(String::from("6"), xkb::KEY_6); // 0x0036
        hash.insert(String::from("7"), xkb::KEY_7); // 0x0037
        hash.insert(String::from("8"), xkb::KEY_8); // 0x0038
        hash.insert(String::from("9"), xkb::KEY_9); // 0x0039
        hash.insert(String::from("colon"), xkb::KEY_colon); // 0x003a
        hash.insert(String::from("semicolon"), xkb::KEY_semicolon); // 0x003b
        hash.insert(String::from("less"), xkb::KEY_less); // 0x003c
        hash.insert(String::from("equal"), xkb::KEY_equal); // 0x003d
        hash.insert(String::from("greater"), xkb::KEY_greater); // 0x003e
        hash.insert(String::from("question"), xkb::KEY_question); // 0x003f
        hash.insert(String::from("at"), xkb::KEY_at); // 0x0040
        hash.insert(String::from("A"), xkb::KEY_A); // 0x0041
        hash.insert(String::from("B"), xkb::KEY_B); // 0x0042
        hash.insert(String::from("C"), xkb::KEY_C); // 0x0043
        hash.insert(String::from("D"), xkb::KEY_D); // 0x0044
        hash.insert(String::from("E"), xkb::KEY_E); // 0x0045
        hash.insert(String::from("F"), xkb::KEY_F); // 0x0046
        hash.insert(String::from("G"), xkb::KEY_G); // 0x0047
        hash.insert(String::from("H"), xkb::KEY_H); // 0x0048
        hash.insert(String::from("I"), xkb::KEY_I); // 0x0049
        hash.insert(String::from("J"), xkb::KEY_J); // 0x004a
        hash.insert(String::from("K"), xkb::KEY_K); // 0x004b
        hash.insert(String::from("L"), xkb::KEY_L); // 0x004c
        hash.insert(String::from("M"), xkb::KEY_M); // 0x004d
        hash.insert(String::from("N"), xkb::KEY_N); // 0x004e
        hash.insert(String::from("O"), xkb::KEY_O); // 0x004f
        hash.insert(String::from("P"), xkb::KEY_P); // 0x0050
        hash.insert(String::from("Q"), xkb::KEY_Q); // 0x0051
        hash.insert(String::from("R"), xkb::KEY_R); // 0x0052
        hash.insert(String::from("S"), xkb::KEY_S); // 0x0053
        hash.insert(String::from("T"), xkb::KEY_T); // 0x0054
        hash.insert(String::from("U"), xkb::KEY_U); // 0x0055
        hash.insert(String::from("V"), xkb::KEY_V); // 0x0056
        hash.insert(String::from("W"), xkb::KEY_W); // 0x0057
        hash.insert(String::from("X"), xkb::KEY_X); // 0x0058
        hash.insert(String::from("Y"), xkb::KEY_Y); // 0x0059
        hash.insert(String::from("Z"), xkb::KEY_Z); // 0x005a
        hash.insert(String::from("bracketleft"), xkb::KEY_bracketleft); // 0x005b
        hash.insert(String::from("backslash"), xkb::KEY_backslash); // 0x005c
        hash.insert(String::from("bracketright"), xkb::KEY_bracketright); // 0x005d
        hash.insert(String::from("asciicircum"), xkb::KEY_asciicircum); // 0x005e
        hash.insert(String::from("underscore"), xkb::KEY_underscore); // 0x005f
        hash.insert(String::from("grave"), xkb::KEY_grave); // 0x0060
        hash.insert(String::from("quoteleft"), xkb::KEY_quoteleft); // 0x0060
        hash.insert(String::from("a"), xkb::KEY_a); // 0x0061
        hash.insert(String::from("b"), xkb::KEY_b); // 0x0062
        hash.insert(String::from("c"), xkb::KEY_c); // 0x0063
        hash.insert(String::from("d"), xkb::KEY_d); // 0x0064
        hash.insert(String::from("e"), xkb::KEY_e); // 0x0065
        hash.insert(String::from("f"), xkb::KEY_f); // 0x0066
        hash.insert(String::from("g"), xkb::KEY_g); // 0x0067
        hash.insert(String::from("h"), xkb::KEY_h); // 0x0068
        hash.insert(String::from("i"), xkb::KEY_i); // 0x0069
        hash.insert(String::from("j"), xkb::KEY_j); // 0x006a
        hash.insert(String::from("k"), xkb::KEY_k); // 0x006b
        hash.insert(String::from("l"), xkb::KEY_l); // 0x006c
        hash.insert(String::from("m"), xkb::KEY_m); // 0x006d
        hash.insert(String::from("n"), xkb::KEY_n); // 0x006e
        hash.insert(String::from("o"), xkb::KEY_o); // 0x006f
        hash.insert(String::from("p"), xkb::KEY_p); // 0x0070
        hash.insert(String::from("q"), xkb::KEY_q); // 0x0071
        hash.insert(String::from("r"), xkb::KEY_r); // 0x0072
        hash.insert(String::from("s"), xkb::KEY_s); // 0x0073
        hash.insert(String::from("t"), xkb::KEY_t); // 0x0074
        hash.insert(String::from("u"), xkb::KEY_u); // 0x0075
        hash.insert(String::from("v"), xkb::KEY_v); // 0x0076
        hash.insert(String::from("w"), xkb::KEY_w); // 0x0077
        hash.insert(String::from("x"), xkb::KEY_x); // 0x0078
        hash.insert(String::from("y"), xkb::KEY_y); // 0x0079
        hash.insert(String::from("z"), xkb::KEY_z); // 0x007a
        hash.insert(String::from("braceleft"), xkb::KEY_braceleft); // 0x007b
        hash.insert(String::from("bar"), xkb::KEY_bar); // 0x007c
        hash.insert(String::from("braceright"), xkb::KEY_braceright); // 0x007d
        hash.insert(String::from("asciitilde"), xkb::KEY_asciitilde); // 0x007e
        hash.insert(String::from("nobreakspace"), xkb::KEY_nobreakspace); // 0x00a0
        hash.insert(String::from("exclamdown"), xkb::KEY_exclamdown); // 0x00a1
        hash.insert(String::from("cent"), xkb::KEY_cent); // 0x00a2
        hash.insert(String::from("sterling"), xkb::KEY_sterling); // 0x00a3
        hash.insert(String::from("currency"), xkb::KEY_currency); // 0x00a4
        hash.insert(String::from("yen"), xkb::KEY_yen); // 0x00a5
        hash.insert(String::from("brokenbar"), xkb::KEY_brokenbar); // 0x00a6
        hash.insert(String::from("section"), xkb::KEY_section); // 0x00a7
        hash.insert(String::from("diaeresis"), xkb::KEY_diaeresis); // 0x00a8
        hash.insert(String::from("copyright"), xkb::KEY_copyright); // 0x00a9
        hash.insert(String::from("ordfeminine"), xkb::KEY_ordfeminine); // 0x00aa
        hash.insert(String::from("guillemotleft"), xkb::KEY_guillemotleft); // 0x00ab
        hash.insert(String::from("notsign"), xkb::KEY_notsign); // 0x00ac
        hash.insert(String::from("hyphen"), xkb::KEY_hyphen); // 0x00ad
        hash.insert(String::from("registered"), xkb::KEY_registered); // 0x00ae
        hash.insert(String::from("macron"), xkb::KEY_macron); // 0x00af
        hash.insert(String::from("degree"), xkb::KEY_degree); // 0x00b0
        hash.insert(String::from("plusminus"), xkb::KEY_plusminus); // 0x00b1
        hash.insert(String::from("twosuperior"), xkb::KEY_twosuperior); // 0x00b2
        hash.insert(String::from("threesuperior"), xkb::KEY_threesuperior); // 0x00b3
        hash.insert(String::from("acute"), xkb::KEY_acute); // 0x00b4
        hash.insert(String::from("mu"), xkb::KEY_mu); // 0x00b5
        hash.insert(String::from("paragraph"), xkb::KEY_paragraph); // 0x00b6
        hash.insert(String::from("periodcentered"), xkb::KEY_periodcentered); // 0x00b7
        hash.insert(String::from("cedilla"), xkb::KEY_cedilla); // 0x00b8
        hash.insert(String::from("onesuperior"), xkb::KEY_onesuperior); // 0x00b9
        hash.insert(String::from("masculine"), xkb::KEY_masculine); // 0x00ba
        hash.insert(String::from("guillemotright"), xkb::KEY_guillemotright); // 0x00bb
        hash.insert(String::from("onequarter"), xkb::KEY_onequarter); // 0x00bc
        hash.insert(String::from("onehalf"), xkb::KEY_onehalf); // 0x00bd
        hash.insert(String::from("threequarters"), xkb::KEY_threequarters); // 0x00be
        hash.insert(String::from("questiondown"), xkb::KEY_questiondown); // 0x00bf
        hash.insert(String::from("Agrave"), xkb::KEY_Agrave); // 0x00c0
        hash.insert(String::from("Aacute"), xkb::KEY_Aacute); // 0x00c1
        hash.insert(String::from("Acircumflex"), xkb::KEY_Acircumflex); // 0x00c2
        hash.insert(String::from("Atilde"), xkb::KEY_Atilde); // 0x00c3
        hash.insert(String::from("Adiaeresis"), xkb::KEY_Adiaeresis); // 0x00c4
        hash.insert(String::from("Aring"), xkb::KEY_Aring); // 0x00c5
        hash.insert(String::from("AE"), xkb::KEY_AE); // 0x00c6
        hash.insert(String::from("Ccedilla"), xkb::KEY_Ccedilla); // 0x00c7
        hash.insert(String::from("Egrave"), xkb::KEY_Egrave); // 0x00c8
        hash.insert(String::from("Eacute"), xkb::KEY_Eacute); // 0x00c9
        hash.insert(String::from("Ecircumflex"), xkb::KEY_Ecircumflex); // 0x00ca
        hash.insert(String::from("Ediaeresis"), xkb::KEY_Ediaeresis); // 0x00cb
        hash.insert(String::from("Igrave"), xkb::KEY_Igrave); // 0x00cc
        hash.insert(String::from("Iacute"), xkb::KEY_Iacute); // 0x00cd
        hash.insert(String::from("Icircumflex"), xkb::KEY_Icircumflex); // 0x00ce
        hash.insert(String::from("Idiaeresis"), xkb::KEY_Idiaeresis); // 0x00cf
        hash.insert(String::from("ETH"), xkb::KEY_ETH); // 0x00d0
        hash.insert(String::from("Eth"), xkb::KEY_Eth); // 0x00d0
        hash.insert(String::from("Ntilde"), xkb::KEY_Ntilde); // 0x00d1
        hash.insert(String::from("Ograve"), xkb::KEY_Ograve); // 0x00d2
        hash.insert(String::from("Oacute"), xkb::KEY_Oacute); // 0x00d3
        hash.insert(String::from("Ocircumflex"), xkb::KEY_Ocircumflex); // 0x00d4
        hash.insert(String::from("Otilde"), xkb::KEY_Otilde); // 0x00d5
        hash.insert(String::from("Odiaeresis"), xkb::KEY_Odiaeresis); // 0x00d6
        hash.insert(String::from("multiply"), xkb::KEY_multiply); // 0x00d7
        hash.insert(String::from("Oslash"), xkb::KEY_Oslash); // 0x00d8
        hash.insert(String::from("Ooblique"), xkb::KEY_Ooblique); // 0x00d8
        hash.insert(String::from("Ugrave"), xkb::KEY_Ugrave); // 0x00d9
        hash.insert(String::from("Uacute"), xkb::KEY_Uacute); // 0x00da
        hash.insert(String::from("Ucircumflex"), xkb::KEY_Ucircumflex); // 0x00db
        hash.insert(String::from("Udiaeresis"), xkb::KEY_Udiaeresis); // 0x00dc
        hash.insert(String::from("Yacute"), xkb::KEY_Yacute); // 0x00dd
        hash.insert(String::from("THORN"), xkb::KEY_THORN); // 0x00de
        hash.insert(String::from("Thorn"), xkb::KEY_Thorn); // 0x00de
        hash.insert(String::from("ssharp"), xkb::KEY_ssharp); // 0x00df
        hash.insert(String::from("agrave"), xkb::KEY_agrave); // 0x00e0
        hash.insert(String::from("aacute"), xkb::KEY_aacute); // 0x00e1
        hash.insert(String::from("acircumflex"), xkb::KEY_acircumflex); // 0x00e2
        hash.insert(String::from("atilde"), xkb::KEY_atilde); // 0x00e3
        hash.insert(String::from("adiaeresis"), xkb::KEY_adiaeresis); // 0x00e4
        hash.insert(String::from("aring"), xkb::KEY_aring); // 0x00e5
        hash.insert(String::from("ae"), xkb::KEY_ae); // 0x00e6
        hash.insert(String::from("ccedilla"), xkb::KEY_ccedilla); // 0x00e7
        hash.insert(String::from("egrave"), xkb::KEY_egrave); // 0x00e8
        hash.insert(String::from("eacute"), xkb::KEY_eacute); // 0x00e9
        hash.insert(String::from("ecircumflex"), xkb::KEY_ecircumflex); // 0x00ea
        hash.insert(String::from("ediaeresis"), xkb::KEY_ediaeresis); // 0x00eb
        hash.insert(String::from("igrave"), xkb::KEY_igrave); // 0x00ec
        hash.insert(String::from("iacute"), xkb::KEY_iacute); // 0x00ed
        hash.insert(String::from("icircumflex"), xkb::KEY_icircumflex); // 0x00ee
        hash.insert(String::from("idiaeresis"), xkb::KEY_idiaeresis); // 0x00ef
        hash.insert(String::from("eth"), xkb::KEY_eth); // 0x00f0
        hash.insert(String::from("ntilde"), xkb::KEY_ntilde); // 0x00f1
        hash.insert(String::from("ograve"), xkb::KEY_ograve); // 0x00f2
        hash.insert(String::from("oacute"), xkb::KEY_oacute); // 0x00f3
        hash.insert(String::from("ocircumflex"), xkb::KEY_ocircumflex); // 0x00f4
        hash.insert(String::from("otilde"), xkb::KEY_otilde); // 0x00f5
        hash.insert(String::from("odiaeresis"), xkb::KEY_odiaeresis); // 0x00f6
        hash.insert(String::from("division"), xkb::KEY_division); // 0x00f7
        hash.insert(String::from("oslash"), xkb::KEY_oslash); // 0x00f8
        hash.insert(String::from("ooblique"), xkb::KEY_ooblique); // 0x00f8
        hash.insert(String::from("ugrave"), xkb::KEY_ugrave); // 0x00f9
        hash.insert(String::from("uacute"), xkb::KEY_uacute); // 0x00fa
        hash.insert(String::from("ucircumflex"), xkb::KEY_ucircumflex); // 0x00fb
        hash.insert(String::from("udiaeresis"), xkb::KEY_udiaeresis); // 0x00fc
        hash.insert(String::from("yacute"), xkb::KEY_yacute); // 0x00fd
        hash.insert(String::from("thorn"), xkb::KEY_thorn); // 0x00fe
        hash.insert(String::from("ydiaeresis"), xkb::KEY_ydiaeresis); // 0x00ff

        hash.insert(String::from("Aogonek"), xkb::KEY_Aogonek); // 0x01a1
        hash.insert(String::from("breve"), xkb::KEY_breve); // 0x01a2
        hash.insert(String::from("Lstroke"), xkb::KEY_Lstroke); // 0x01a3
        hash.insert(String::from("Lcaron"), xkb::KEY_Lcaron); // 0x01a5
        hash.insert(String::from("Sacute"), xkb::KEY_Sacute); // 0x01a6
        hash.insert(String::from("Scaron"), xkb::KEY_Scaron); // 0x01a9
        hash.insert(String::from("Scedilla"), xkb::KEY_Scedilla); // 0x01aa
        hash.insert(String::from("Tcaron"), xkb::KEY_Tcaron); // 0x01ab
        hash.insert(String::from("Zacute"), xkb::KEY_Zacute); // 0x01ac
        hash.insert(String::from("Zcaron"), xkb::KEY_Zcaron); // 0x01ae
        hash.insert(String::from("Zabovedot"), xkb::KEY_Zabovedot); // 0x01af
        hash.insert(String::from("aogonek"), xkb::KEY_aogonek); // 0x01b1
        hash.insert(String::from("ogonek"), xkb::KEY_ogonek); // 0x01b2
        hash.insert(String::from("lstroke"), xkb::KEY_lstroke); // 0x01b3
        hash.insert(String::from("lcaron"), xkb::KEY_lcaron); // 0x01b5
        hash.insert(String::from("sacute"), xkb::KEY_sacute); // 0x01b6
        hash.insert(String::from("caron"), xkb::KEY_caron); // 0x01b7
        hash.insert(String::from("scaron"), xkb::KEY_scaron); // 0x01b9
        hash.insert(String::from("scedilla"), xkb::KEY_scedilla); // 0x01ba
        hash.insert(String::from("tcaron"), xkb::KEY_tcaron); // 0x01bb
        hash.insert(String::from("zacute"), xkb::KEY_zacute); // 0x01bc
        hash.insert(String::from("doubleacute"), xkb::KEY_doubleacute); // 0x01bd
        hash.insert(String::from("zcaron"), xkb::KEY_zcaron); // 0x01be
        hash.insert(String::from("zabovedot"), xkb::KEY_zabovedot); // 0x01bf
        hash.insert(String::from("Racute"), xkb::KEY_Racute); // 0x01c0
        hash.insert(String::from("Abreve"), xkb::KEY_Abreve); // 0x01c3
        hash.insert(String::from("Lacute"), xkb::KEY_Lacute); // 0x01c5
        hash.insert(String::from("Cacute"), xkb::KEY_Cacute); // 0x01c6
        hash.insert(String::from("Ccaron"), xkb::KEY_Ccaron); // 0x01c8
        hash.insert(String::from("Eogonek"), xkb::KEY_Eogonek); // 0x01ca
        hash.insert(String::from("Ecaron"), xkb::KEY_Ecaron); // 0x01cc
        hash.insert(String::from("Dcaron"), xkb::KEY_Dcaron); // 0x01cf
        hash.insert(String::from("Dstroke"), xkb::KEY_Dstroke); // 0x01d0
        hash.insert(String::from("Nacute"), xkb::KEY_Nacute); // 0x01d1
        hash.insert(String::from("Ncaron"), xkb::KEY_Ncaron); // 0x01d2
        hash.insert(String::from("Odoubleacute"), xkb::KEY_Odoubleacute); // 0x01d5
        hash.insert(String::from("Rcaron"), xkb::KEY_Rcaron); // 0x01d8
        hash.insert(String::from("Uring"), xkb::KEY_Uring); // 0x01d9
        hash.insert(String::from("Udoubleacute"), xkb::KEY_Udoubleacute); // 0x01db
        hash.insert(String::from("Tcedilla"), xkb::KEY_Tcedilla); // 0x01de
        hash.insert(String::from("racute"), xkb::KEY_racute); // 0x01e0
        hash.insert(String::from("abreve"), xkb::KEY_abreve); // 0x01e3
        hash.insert(String::from("lacute"), xkb::KEY_lacute); // 0x01e5
        hash.insert(String::from("cacute"), xkb::KEY_cacute); // 0x01e6
        hash.insert(String::from("ccaron"), xkb::KEY_ccaron); // 0x01e8
        hash.insert(String::from("eogonek"), xkb::KEY_eogonek); // 0x01ea
        hash.insert(String::from("ecaron"), xkb::KEY_ecaron); // 0x01ec
        hash.insert(String::from("dcaron"), xkb::KEY_dcaron); // 0x01ef
        hash.insert(String::from("dstroke"), xkb::KEY_dstroke); // 0x01f0
        hash.insert(String::from("nacute"), xkb::KEY_nacute); // 0x01f1
        hash.insert(String::from("ncaron"), xkb::KEY_ncaron); // 0x01f2
        hash.insert(String::from("odoubleacute"), xkb::KEY_odoubleacute); // 0x01f5
        hash.insert(String::from("rcaron"), xkb::KEY_rcaron); // 0x01f8
        hash.insert(String::from("uring"), xkb::KEY_uring); // 0x01f9
        hash.insert(String::from("udoubleacute"), xkb::KEY_udoubleacute); // 0x01fb
        hash.insert(String::from("tcedilla"), xkb::KEY_tcedilla); // 0x01fe
        hash.insert(String::from("abovedot"), xkb::KEY_abovedot); // 0x01ff

        hash.insert(String::from("Hstroke"), xkb::KEY_Hstroke); // 0x02a1
        hash.insert(String::from("Hcircumflex"), xkb::KEY_Hcircumflex); // 0x02a6
        hash.insert(String::from("Iabovedot"), xkb::KEY_Iabovedot); // 0x02a9
        hash.insert(String::from("Gbreve"), xkb::KEY_Gbreve); // 0x02ab
        hash.insert(String::from("Jcircumflex"), xkb::KEY_Jcircumflex); // 0x02ac
        hash.insert(String::from("hstroke"), xkb::KEY_hstroke); // 0x02b1
        hash.insert(String::from("hcircumflex"), xkb::KEY_hcircumflex); // 0x02b6
        hash.insert(String::from("idotless"), xkb::KEY_idotless); // 0x02b9
        hash.insert(String::from("gbreve"), xkb::KEY_gbreve); // 0x02bb
        hash.insert(String::from("jcircumflex"), xkb::KEY_jcircumflex); // 0x02bc
        hash.insert(String::from("Cabovedot"), xkb::KEY_Cabovedot); // 0x02c5
        hash.insert(String::from("Ccircumflex"), xkb::KEY_Ccircumflex); // 0x02c6
        hash.insert(String::from("Gabovedot"), xkb::KEY_Gabovedot); // 0x02d5
        hash.insert(String::from("Gcircumflex"), xkb::KEY_Gcircumflex); // 0x02d8
        hash.insert(String::from("Ubreve"), xkb::KEY_Ubreve); // 0x02dd
        hash.insert(String::from("Scircumflex"), xkb::KEY_Scircumflex); // 0x02de
        hash.insert(String::from("cabovedot"), xkb::KEY_cabovedot); // 0x02e5
        hash.insert(String::from("ccircumflex"), xkb::KEY_ccircumflex); // 0x02e6
        hash.insert(String::from("gabovedot"), xkb::KEY_gabovedot); // 0x02f5
        hash.insert(String::from("gcircumflex"), xkb::KEY_gcircumflex); // 0x02f8
        hash.insert(String::from("ubreve"), xkb::KEY_ubreve); // 0x02fd
        hash.insert(String::from("scircumflex"), xkb::KEY_scircumflex); // 0x02fe

        hash.insert(String::from("kra"), xkb::KEY_kra); // 0x03a2
        hash.insert(String::from("kappa"), xkb::KEY_kappa); // 0x03a2
        hash.insert(String::from("Rcedilla"), xkb::KEY_Rcedilla); // 0x03a3
        hash.insert(String::from("Itilde"), xkb::KEY_Itilde); // 0x03a5
        hash.insert(String::from("Lcedilla"), xkb::KEY_Lcedilla); // 0x03a6
        hash.insert(String::from("Emacron"), xkb::KEY_Emacron); // 0x03aa
        hash.insert(String::from("Gcedilla"), xkb::KEY_Gcedilla); // 0x03ab
        hash.insert(String::from("Tslash"), xkb::KEY_Tslash); // 0x03ac
        hash.insert(String::from("rcedilla"), xkb::KEY_rcedilla); // 0x03b3
        hash.insert(String::from("itilde"), xkb::KEY_itilde); // 0x03b5
        hash.insert(String::from("lcedilla"), xkb::KEY_lcedilla); // 0x03b6
        hash.insert(String::from("emacron"), xkb::KEY_emacron); // 0x03ba
        hash.insert(String::from("gcedilla"), xkb::KEY_gcedilla); // 0x03bb
        hash.insert(String::from("tslash"), xkb::KEY_tslash); // 0x03bc
        hash.insert(String::from("ENG"), xkb::KEY_ENG); // 0x03bd
        hash.insert(String::from("eng"), xkb::KEY_eng); // 0x03bf
        hash.insert(String::from("Amacron"), xkb::KEY_Amacron); // 0x03c0
        hash.insert(String::from("Iogonek"), xkb::KEY_Iogonek); // 0x03c7
        hash.insert(String::from("Eabovedot"), xkb::KEY_Eabovedot); // 0x03cc
        hash.insert(String::from("Imacron"), xkb::KEY_Imacron); // 0x03cf
        hash.insert(String::from("Ncedilla"), xkb::KEY_Ncedilla); // 0x03d1
        hash.insert(String::from("Omacron"), xkb::KEY_Omacron); // 0x03d2
        hash.insert(String::from("Kcedilla"), xkb::KEY_Kcedilla); // 0x03d3
        hash.insert(String::from("Uogonek"), xkb::KEY_Uogonek); // 0x03d9
        hash.insert(String::from("Utilde"), xkb::KEY_Utilde); // 0x03dd
        hash.insert(String::from("Umacron"), xkb::KEY_Umacron); // 0x03de
        hash.insert(String::from("amacron"), xkb::KEY_amacron); // 0x03e0
        hash.insert(String::from("iogonek"), xkb::KEY_iogonek); // 0x03e7
        hash.insert(String::from("eabovedot"), xkb::KEY_eabovedot); // 0x03ec
        hash.insert(String::from("imacron"), xkb::KEY_imacron); // 0x03ef
        hash.insert(String::from("ncedilla"), xkb::KEY_ncedilla); // 0x03f1
        hash.insert(String::from("omacron"), xkb::KEY_omacron); // 0x03f2
        hash.insert(String::from("kcedilla"), xkb::KEY_kcedilla); // 0x03f3
        hash.insert(String::from("uogonek"), xkb::KEY_uogonek); // 0x03f9
        hash.insert(String::from("utilde"), xkb::KEY_utilde); // 0x03fd
        hash.insert(String::from("umacron"), xkb::KEY_umacron); // 0x03fe

        hash.insert(String::from("Wcircumflex"), xkb::KEY_Wcircumflex); // 0x1000174
        hash.insert(String::from("wcircumflex"), xkb::KEY_wcircumflex); // 0x1000175
        hash.insert(String::from("Ycircumflex"), xkb::KEY_Ycircumflex); // 0x1000176
        hash.insert(String::from("ycircumflex"), xkb::KEY_ycircumflex); // 0x1000177
        hash.insert(String::from("Babovedot"), xkb::KEY_Babovedot); // 0x1001e02
        hash.insert(String::from("babovedot"), xkb::KEY_babovedot); // 0x1001e03
        hash.insert(String::from("Dabovedot"), xkb::KEY_Dabovedot); // 0x1001e0a
        hash.insert(String::from("dabovedot"), xkb::KEY_dabovedot); // 0x1001e0b
        hash.insert(String::from("Fabovedot"), xkb::KEY_Fabovedot); // 0x1001e1e
        hash.insert(String::from("fabovedot"), xkb::KEY_fabovedot); // 0x1001e1f
        hash.insert(String::from("Mabovedot"), xkb::KEY_Mabovedot); // 0x1001e40
        hash.insert(String::from("mabovedot"), xkb::KEY_mabovedot); // 0x1001e41
        hash.insert(String::from("Pabovedot"), xkb::KEY_Pabovedot); // 0x1001e56
        hash.insert(String::from("pabovedot"), xkb::KEY_pabovedot); // 0x1001e57
        hash.insert(String::from("Sabovedot"), xkb::KEY_Sabovedot); // 0x1001e60
        hash.insert(String::from("sabovedot"), xkb::KEY_sabovedot); // 0x1001e61
        hash.insert(String::from("Tabovedot"), xkb::KEY_Tabovedot); // 0x1001e6a
        hash.insert(String::from("tabovedot"), xkb::KEY_tabovedot); // 0x1001e6b
        hash.insert(String::from("Wgrave"), xkb::KEY_Wgrave); // 0x1001e80
        hash.insert(String::from("wgrave"), xkb::KEY_wgrave); // 0x1001e81
        hash.insert(String::from("Wacute"), xkb::KEY_Wacute); // 0x1001e82
        hash.insert(String::from("wacute"), xkb::KEY_wacute); // 0x1001e83
        hash.insert(String::from("Wdiaeresis"), xkb::KEY_Wdiaeresis); // 0x1001e84
        hash.insert(String::from("wdiaeresis"), xkb::KEY_wdiaeresis); // 0x1001e85
        hash.insert(String::from("Ygrave"), xkb::KEY_Ygrave); // 0x1001ef2
        hash.insert(String::from("ygrave"), xkb::KEY_ygrave); // 0x1001ef3

        hash.insert(String::from("OE"), xkb::KEY_OE); // 0x13bc
        hash.insert(String::from("oe"), xkb::KEY_oe); // 0x13bd
        hash.insert(String::from("Ydiaeresis"), xkb::KEY_Ydiaeresis); // 0x13be

        hash.insert(String::from("overline"), xkb::KEY_overline); // 0x047e
        hash.insert(String::from("kana_fullstop"), xkb::KEY_kana_fullstop); // 0x04a1
        hash.insert(
            String::from("kana_openingbracket"),
            xkb::KEY_kana_openingbracket,
        ); // 0x04a2
        hash.insert(
            String::from("kana_closingbracket"),
            xkb::KEY_kana_closingbracket,
        ); // 0x04a3
        hash.insert(String::from("kana_comma"), xkb::KEY_kana_comma); // 0x04a4
        hash.insert(String::from("kana_conjunctive"), xkb::KEY_kana_conjunctive); // 0x04a5
        hash.insert(String::from("kana_middledot"), xkb::KEY_kana_middledot); // 0x04a5
        hash.insert(String::from("kana_WO"), xkb::KEY_kana_WO); // 0x04a6
        hash.insert(String::from("kana_a"), xkb::KEY_kana_a); // 0x04a7
        hash.insert(String::from("kana_i"), xkb::KEY_kana_i); // 0x04a8
        hash.insert(String::from("kana_u"), xkb::KEY_kana_u); // 0x04a9
        hash.insert(String::from("kana_e"), xkb::KEY_kana_e); // 0x04aa
        hash.insert(String::from("kana_o"), xkb::KEY_kana_o); // 0x04ab
        hash.insert(String::from("kana_ya"), xkb::KEY_kana_ya); // 0x04ac
        hash.insert(String::from("kana_yu"), xkb::KEY_kana_yu); // 0x04ad
        hash.insert(String::from("kana_yo"), xkb::KEY_kana_yo); // 0x04ae
        hash.insert(String::from("kana_tsu"), xkb::KEY_kana_tsu); // 0x04af
        hash.insert(String::from("kana_tu"), xkb::KEY_kana_tu); // 0x04af
        hash.insert(String::from("prolongedsound"), xkb::KEY_prolongedsound); // 0x04b0
        hash.insert(String::from("kana_A"), xkb::KEY_kana_A); // 0x04b1
        hash.insert(String::from("kana_I"), xkb::KEY_kana_I); // 0x04b2
        hash.insert(String::from("kana_U"), xkb::KEY_kana_U); // 0x04b3
        hash.insert(String::from("kana_E"), xkb::KEY_kana_E); // 0x04b4
        hash.insert(String::from("kana_O"), xkb::KEY_kana_O); // 0x04b5
        hash.insert(String::from("kana_KA"), xkb::KEY_kana_KA); // 0x04b6
        hash.insert(String::from("kana_KI"), xkb::KEY_kana_KI); // 0x04b7
        hash.insert(String::from("kana_KU"), xkb::KEY_kana_KU); // 0x04b8
        hash.insert(String::from("kana_KE"), xkb::KEY_kana_KE); // 0x04b9
        hash.insert(String::from("kana_KO"), xkb::KEY_kana_KO); // 0x04ba
        hash.insert(String::from("kana_SA"), xkb::KEY_kana_SA); // 0x04bb
        hash.insert(String::from("kana_SHI"), xkb::KEY_kana_SHI); // 0x04bc
        hash.insert(String::from("kana_SU"), xkb::KEY_kana_SU); // 0x04bd
        hash.insert(String::from("kana_SE"), xkb::KEY_kana_SE); // 0x04be
        hash.insert(String::from("kana_SO"), xkb::KEY_kana_SO); // 0x04bf
        hash.insert(String::from("kana_TA"), xkb::KEY_kana_TA); // 0x04c0
        hash.insert(String::from("kana_CHI"), xkb::KEY_kana_CHI); // 0x04c1
        hash.insert(String::from("kana_TI"), xkb::KEY_kana_TI); // 0x04c1
        hash.insert(String::from("kana_TSU"), xkb::KEY_kana_TSU); // 0x04c2
        hash.insert(String::from("kana_TU"), xkb::KEY_kana_TU); // 0x04c2
        hash.insert(String::from("kana_TE"), xkb::KEY_kana_TE); // 0x04c3
        hash.insert(String::from("kana_TO"), xkb::KEY_kana_TO); // 0x04c4
        hash.insert(String::from("kana_NA"), xkb::KEY_kana_NA); // 0x04c5
        hash.insert(String::from("kana_NI"), xkb::KEY_kana_NI); // 0x04c6
        hash.insert(String::from("kana_NU"), xkb::KEY_kana_NU); // 0x04c7
        hash.insert(String::from("kana_NE"), xkb::KEY_kana_NE); // 0x04c8
        hash.insert(String::from("kana_NO"), xkb::KEY_kana_NO); // 0x04c9
        hash.insert(String::from("kana_HA"), xkb::KEY_kana_HA); // 0x04ca
        hash.insert(String::from("kana_HI"), xkb::KEY_kana_HI); // 0x04cb
        hash.insert(String::from("kana_FU"), xkb::KEY_kana_FU); // 0x04cc
        hash.insert(String::from("kana_HU"), xkb::KEY_kana_HU); // 0x04cc
        hash.insert(String::from("kana_HE"), xkb::KEY_kana_HE); // 0x04cd
        hash.insert(String::from("kana_HO"), xkb::KEY_kana_HO); // 0x04ce
        hash.insert(String::from("kana_MA"), xkb::KEY_kana_MA); // 0x04cf
        hash.insert(String::from("kana_MI"), xkb::KEY_kana_MI); // 0x04d0
        hash.insert(String::from("kana_MU"), xkb::KEY_kana_MU); // 0x04d1
        hash.insert(String::from("kana_ME"), xkb::KEY_kana_ME); // 0x04d2
        hash.insert(String::from("kana_MO"), xkb::KEY_kana_MO); // 0x04d3
        hash.insert(String::from("kana_YA"), xkb::KEY_kana_YA); // 0x04d4
        hash.insert(String::from("kana_YU"), xkb::KEY_kana_YU); // 0x04d5
        hash.insert(String::from("kana_YO"), xkb::KEY_kana_YO); // 0x04d6
        hash.insert(String::from("kana_RA"), xkb::KEY_kana_RA); // 0x04d7
        hash.insert(String::from("kana_RI"), xkb::KEY_kana_RI); // 0x04d8
        hash.insert(String::from("kana_RU"), xkb::KEY_kana_RU); // 0x04d9
        hash.insert(String::from("kana_RE"), xkb::KEY_kana_RE); // 0x04da
        hash.insert(String::from("kana_RO"), xkb::KEY_kana_RO); // 0x04db
        hash.insert(String::from("kana_WA"), xkb::KEY_kana_WA); // 0x04dc
        hash.insert(String::from("kana_N"), xkb::KEY_kana_N); // 0x04dd
        hash.insert(String::from("voicedsound"), xkb::KEY_voicedsound); // 0x04de
        hash.insert(String::from("semivoicedsound"), xkb::KEY_semivoicedsound); // 0x04df
        hash.insert(String::from("kana_switch"), xkb::KEY_kana_switch); // 0xff7e

        hash.insert(String::from("Farsi_0"), xkb::KEY_Farsi_0); // 0x10006f0
        hash.insert(String::from("Farsi_1"), xkb::KEY_Farsi_1); // 0x10006f1
        hash.insert(String::from("Farsi_2"), xkb::KEY_Farsi_2); // 0x10006f2
        hash.insert(String::from("Farsi_3"), xkb::KEY_Farsi_3); // 0x10006f3
        hash.insert(String::from("Farsi_4"), xkb::KEY_Farsi_4); // 0x10006f4
        hash.insert(String::from("Farsi_5"), xkb::KEY_Farsi_5); // 0x10006f5
        hash.insert(String::from("Farsi_6"), xkb::KEY_Farsi_6); // 0x10006f6
        hash.insert(String::from("Farsi_7"), xkb::KEY_Farsi_7); // 0x10006f7
        hash.insert(String::from("Farsi_8"), xkb::KEY_Farsi_8); // 0x10006f8
        hash.insert(String::from("Farsi_9"), xkb::KEY_Farsi_9); // 0x10006f9
        hash.insert(String::from("Arabic_percent"), xkb::KEY_Arabic_percent); // 0x100066a
        hash.insert(
            String::from("Arabic_superscript_alef"),
            xkb::KEY_Arabic_superscript_alef,
        ); // 0x1000670
        hash.insert(String::from("Arabic_tteh"), xkb::KEY_Arabic_tteh); // 0x1000679
        hash.insert(String::from("Arabic_peh"), xkb::KEY_Arabic_peh); // 0x100067e
        hash.insert(String::from("Arabic_tcheh"), xkb::KEY_Arabic_tcheh); // 0x1000686
        hash.insert(String::from("Arabic_ddal"), xkb::KEY_Arabic_ddal); // 0x1000688
        hash.insert(String::from("Arabic_rreh"), xkb::KEY_Arabic_rreh); // 0x1000691
        hash.insert(String::from("Arabic_comma"), xkb::KEY_Arabic_comma); // 0x05ac
        hash.insert(String::from("Arabic_fullstop"), xkb::KEY_Arabic_fullstop); // 0x10006d4
        hash.insert(String::from("Arabic_0"), xkb::KEY_Arabic_0); // 0x1000660
        hash.insert(String::from("Arabic_1"), xkb::KEY_Arabic_1); // 0x1000661
        hash.insert(String::from("Arabic_2"), xkb::KEY_Arabic_2); // 0x1000662
        hash.insert(String::from("Arabic_3"), xkb::KEY_Arabic_3); // 0x1000663
        hash.insert(String::from("Arabic_4"), xkb::KEY_Arabic_4); // 0x1000664
        hash.insert(String::from("Arabic_5"), xkb::KEY_Arabic_5); // 0x1000665
        hash.insert(String::from("Arabic_6"), xkb::KEY_Arabic_6); // 0x1000666
        hash.insert(String::from("Arabic_7"), xkb::KEY_Arabic_7); // 0x1000667
        hash.insert(String::from("Arabic_8"), xkb::KEY_Arabic_8); // 0x1000668
        hash.insert(String::from("Arabic_9"), xkb::KEY_Arabic_9); // 0x1000669
        hash.insert(String::from("Arabic_semicolon"), xkb::KEY_Arabic_semicolon); // 0x05bb
        hash.insert(
            String::from("Arabic_question_mark"),
            xkb::KEY_Arabic_question_mark,
        ); // 0x05bf
        hash.insert(String::from("Arabic_hamza"), xkb::KEY_Arabic_hamza); // 0x05c1
        hash.insert(
            String::from("Arabic_maddaonalef"),
            xkb::KEY_Arabic_maddaonalef,
        ); // 0x05c2
        hash.insert(
            String::from("Arabic_hamzaonalef"),
            xkb::KEY_Arabic_hamzaonalef,
        ); // 0x05c3
        hash.insert(
            String::from("Arabic_hamzaonwaw"),
            xkb::KEY_Arabic_hamzaonwaw,
        ); // 0x05c4
        hash.insert(
            String::from("Arabic_hamzaunderalef"),
            xkb::KEY_Arabic_hamzaunderalef,
        ); // 0x05c5
        hash.insert(
            String::from("Arabic_hamzaonyeh"),
            xkb::KEY_Arabic_hamzaonyeh,
        ); // 0x05c6
        hash.insert(String::from("Arabic_alef"), xkb::KEY_Arabic_alef); // 0x05c7
        hash.insert(String::from("Arabic_beh"), xkb::KEY_Arabic_beh); // 0x05c8
        hash.insert(
            String::from("Arabic_tehmarbuta"),
            xkb::KEY_Arabic_tehmarbuta,
        ); // 0x05c9
        hash.insert(String::from("Arabic_teh"), xkb::KEY_Arabic_teh); // 0x05ca
        hash.insert(String::from("Arabic_theh"), xkb::KEY_Arabic_theh); // 0x05cb
        hash.insert(String::from("Arabic_jeem"), xkb::KEY_Arabic_jeem); // 0x05cc
        hash.insert(String::from("Arabic_hah"), xkb::KEY_Arabic_hah); // 0x05cd
        hash.insert(String::from("Arabic_khah"), xkb::KEY_Arabic_khah); // 0x05ce
        hash.insert(String::from("Arabic_dal"), xkb::KEY_Arabic_dal); // 0x05cf
        hash.insert(String::from("Arabic_thal"), xkb::KEY_Arabic_thal); // 0x05d0
        hash.insert(String::from("Arabic_ra"), xkb::KEY_Arabic_ra); // 0x05d1
        hash.insert(String::from("Arabic_zain"), xkb::KEY_Arabic_zain); // 0x05d2
        hash.insert(String::from("Arabic_seen"), xkb::KEY_Arabic_seen); // 0x05d3
        hash.insert(String::from("Arabic_sheen"), xkb::KEY_Arabic_sheen); // 0x05d4
        hash.insert(String::from("Arabic_sad"), xkb::KEY_Arabic_sad); // 0x05d5
        hash.insert(String::from("Arabic_dad"), xkb::KEY_Arabic_dad); // 0x05d6
        hash.insert(String::from("Arabic_tah"), xkb::KEY_Arabic_tah); // 0x05d7
        hash.insert(String::from("Arabic_zah"), xkb::KEY_Arabic_zah); // 0x05d8
        hash.insert(String::from("Arabic_ain"), xkb::KEY_Arabic_ain); // 0x05d9
        hash.insert(String::from("Arabic_ghain"), xkb::KEY_Arabic_ghain); // 0x05da
        hash.insert(String::from("Arabic_tatweel"), xkb::KEY_Arabic_tatweel); // 0x05e0
        hash.insert(String::from("Arabic_feh"), xkb::KEY_Arabic_feh); // 0x05e1
        hash.insert(String::from("Arabic_qaf"), xkb::KEY_Arabic_qaf); // 0x05e2
        hash.insert(String::from("Arabic_kaf"), xkb::KEY_Arabic_kaf); // 0x05e3
        hash.insert(String::from("Arabic_lam"), xkb::KEY_Arabic_lam); // 0x05e4
        hash.insert(String::from("Arabic_meem"), xkb::KEY_Arabic_meem); // 0x05e5
        hash.insert(String::from("Arabic_noon"), xkb::KEY_Arabic_noon); // 0x05e6
        hash.insert(String::from("Arabic_ha"), xkb::KEY_Arabic_ha); // 0x05e7
        hash.insert(String::from("Arabic_heh"), xkb::KEY_Arabic_heh); // 0x05e7
        hash.insert(String::from("Arabic_waw"), xkb::KEY_Arabic_waw); // 0x05e8
        hash.insert(
            String::from("Arabic_alefmaksura"),
            xkb::KEY_Arabic_alefmaksura,
        ); // 0x05e9
        hash.insert(String::from("Arabic_yeh"), xkb::KEY_Arabic_yeh); // 0x05ea
        hash.insert(String::from("Arabic_fathatan"), xkb::KEY_Arabic_fathatan); // 0x05eb
        hash.insert(String::from("Arabic_dammatan"), xkb::KEY_Arabic_dammatan); // 0x05ec
        hash.insert(String::from("Arabic_kasratan"), xkb::KEY_Arabic_kasratan); // 0x05ed
        hash.insert(String::from("Arabic_fatha"), xkb::KEY_Arabic_fatha); // 0x05ee
        hash.insert(String::from("Arabic_damma"), xkb::KEY_Arabic_damma); // 0x05ef
        hash.insert(String::from("Arabic_kasra"), xkb::KEY_Arabic_kasra); // 0x05f0
        hash.insert(String::from("Arabic_shadda"), xkb::KEY_Arabic_shadda); // 0x05f1
        hash.insert(String::from("Arabic_sukun"), xkb::KEY_Arabic_sukun); // 0x05f2
        hash.insert(
            String::from("Arabic_madda_above"),
            xkb::KEY_Arabic_madda_above,
        ); // 0x1000653
        hash.insert(
            String::from("Arabic_hamza_above"),
            xkb::KEY_Arabic_hamza_above,
        ); // 0x1000654
        hash.insert(
            String::from("Arabic_hamza_below"),
            xkb::KEY_Arabic_hamza_below,
        ); // 0x1000655
        hash.insert(String::from("Arabic_jeh"), xkb::KEY_Arabic_jeh); // 0x1000698
        hash.insert(String::from("Arabic_veh"), xkb::KEY_Arabic_veh); // 0x10006a4
        hash.insert(String::from("Arabic_keheh"), xkb::KEY_Arabic_keheh); // 0x10006a9
        hash.insert(String::from("Arabic_gaf"), xkb::KEY_Arabic_gaf); // 0x10006af
        hash.insert(
            String::from("Arabic_noon_ghunna"),
            xkb::KEY_Arabic_noon_ghunna,
        ); // 0x10006ba
        hash.insert(
            String::from("Arabic_heh_doachashmee"),
            xkb::KEY_Arabic_heh_doachashmee,
        ); // 0x10006be
        hash.insert(String::from("Farsi_yeh"), xkb::KEY_Farsi_yeh); // 0x10006cc
        hash.insert(String::from("Arabic_farsi_yeh"), xkb::KEY_Arabic_farsi_yeh); // 0x10006cc
        hash.insert(String::from("Arabic_yeh_baree"), xkb::KEY_Arabic_yeh_baree); // 0x10006d2
        hash.insert(String::from("Arabic_heh_goal"), xkb::KEY_Arabic_heh_goal); // 0x10006c1
        hash.insert(String::from("Arabic_switch"), xkb::KEY_Arabic_switch); // 0xff7e

        hash.insert(String::from("Cyrillic_GHE_bar"), xkb::KEY_Cyrillic_GHE_bar); // 0x1000492
        hash.insert(String::from("Cyrillic_ghe_bar"), xkb::KEY_Cyrillic_ghe_bar); // 0x1000493
        hash.insert(
            String::from("Cyrillic_ZHE_descender"),
            xkb::KEY_Cyrillic_ZHE_descender,
        ); // 0x1000496
        hash.insert(
            String::from("Cyrillic_zhe_descender"),
            xkb::KEY_Cyrillic_zhe_descender,
        ); // 0x1000497
        hash.insert(
            String::from("Cyrillic_KA_descender"),
            xkb::KEY_Cyrillic_KA_descender,
        ); // 0x100049a
        hash.insert(
            String::from("Cyrillic_ka_descender"),
            xkb::KEY_Cyrillic_ka_descender,
        ); // 0x100049b
        hash.insert(
            String::from("Cyrillic_KA_vertstroke"),
            xkb::KEY_Cyrillic_KA_vertstroke,
        ); // 0x100049c
        hash.insert(
            String::from("Cyrillic_ka_vertstroke"),
            xkb::KEY_Cyrillic_ka_vertstroke,
        ); // 0x100049d
        hash.insert(
            String::from("Cyrillic_EN_descender"),
            xkb::KEY_Cyrillic_EN_descender,
        ); // 0x10004a2
        hash.insert(
            String::from("Cyrillic_en_descender"),
            xkb::KEY_Cyrillic_en_descender,
        ); // 0x10004a3
        hash.insert(
            String::from("Cyrillic_U_straight"),
            xkb::KEY_Cyrillic_U_straight,
        ); // 0x10004ae
        hash.insert(
            String::from("Cyrillic_u_straight"),
            xkb::KEY_Cyrillic_u_straight,
        ); // 0x10004af
        hash.insert(
            String::from("Cyrillic_U_straight_bar"),
            xkb::KEY_Cyrillic_U_straight_bar,
        ); // 0x10004b0
        hash.insert(
            String::from("Cyrillic_u_straight_bar"),
            xkb::KEY_Cyrillic_u_straight_bar,
        ); // 0x10004b1
        hash.insert(
            String::from("Cyrillic_HA_descender"),
            xkb::KEY_Cyrillic_HA_descender,
        ); // 0x10004b2
        hash.insert(
            String::from("Cyrillic_ha_descender"),
            xkb::KEY_Cyrillic_ha_descender,
        ); // 0x10004b3
        hash.insert(
            String::from("Cyrillic_CHE_descender"),
            xkb::KEY_Cyrillic_CHE_descender,
        ); // 0x10004b6
        hash.insert(
            String::from("Cyrillic_che_descender"),
            xkb::KEY_Cyrillic_che_descender,
        ); // 0x10004b7
        hash.insert(
            String::from("Cyrillic_CHE_vertstroke"),
            xkb::KEY_Cyrillic_CHE_vertstroke,
        ); // 0x10004b8
        hash.insert(
            String::from("Cyrillic_che_vertstroke"),
            xkb::KEY_Cyrillic_che_vertstroke,
        ); // 0x10004b9
        hash.insert(String::from("Cyrillic_SHHA"), xkb::KEY_Cyrillic_SHHA); // 0x10004ba
        hash.insert(String::from("Cyrillic_shha"), xkb::KEY_Cyrillic_shha); // 0x10004bb
        hash.insert(String::from("Cyrillic_SCHWA"), xkb::KEY_Cyrillic_SCHWA); // 0x10004d8
        hash.insert(String::from("Cyrillic_schwa"), xkb::KEY_Cyrillic_schwa); // 0x10004d9
        hash.insert(
            String::from("Cyrillic_I_macron"),
            xkb::KEY_Cyrillic_I_macron,
        ); // 0x10004e2
        hash.insert(
            String::from("Cyrillic_i_macron"),
            xkb::KEY_Cyrillic_i_macron,
        ); // 0x10004e3
        hash.insert(String::from("Cyrillic_O_bar"), xkb::KEY_Cyrillic_O_bar); // 0x10004e8
        hash.insert(String::from("Cyrillic_o_bar"), xkb::KEY_Cyrillic_o_bar); // 0x10004e9
        hash.insert(
            String::from("Cyrillic_U_macron"),
            xkb::KEY_Cyrillic_U_macron,
        ); // 0x10004ee
        hash.insert(
            String::from("Cyrillic_u_macron"),
            xkb::KEY_Cyrillic_u_macron,
        ); // 0x10004ef
        hash.insert(String::from("Serbian_dje"), xkb::KEY_Serbian_dje); // 0x06a1
        hash.insert(String::from("Macedonia_gje"), xkb::KEY_Macedonia_gje); // 0x06a2
        hash.insert(String::from("Cyrillic_io"), xkb::KEY_Cyrillic_io); // 0x06a3
        hash.insert(String::from("Ukrainian_ie"), xkb::KEY_Ukrainian_ie); // 0x06a4
        hash.insert(String::from("Ukranian_je"), xkb::KEY_Ukranian_je); // 0x06a4
        hash.insert(String::from("Macedonia_dse"), xkb::KEY_Macedonia_dse); // 0x06a5
        hash.insert(String::from("Ukrainian_i"), xkb::KEY_Ukrainian_i); // 0x06a6
        hash.insert(String::from("Ukranian_i"), xkb::KEY_Ukranian_i); // 0x06a6
        hash.insert(String::from("Ukrainian_yi"), xkb::KEY_Ukrainian_yi); // 0x06a7
        hash.insert(String::from("Ukranian_yi"), xkb::KEY_Ukranian_yi); // 0x06a7
        hash.insert(String::from("Cyrillic_je"), xkb::KEY_Cyrillic_je); // 0x06a8
        hash.insert(String::from("Serbian_je"), xkb::KEY_Serbian_je); // 0x06a8
        hash.insert(String::from("Cyrillic_lje"), xkb::KEY_Cyrillic_lje); // 0x06a9
        hash.insert(String::from("Serbian_lje"), xkb::KEY_Serbian_lje); // 0x06a9
        hash.insert(String::from("Cyrillic_nje"), xkb::KEY_Cyrillic_nje); // 0x06aa
        hash.insert(String::from("Serbian_nje"), xkb::KEY_Serbian_nje); // 0x06aa
        hash.insert(String::from("Serbian_tshe"), xkb::KEY_Serbian_tshe); // 0x06ab
        hash.insert(String::from("Macedonia_kje"), xkb::KEY_Macedonia_kje); // 0x06ac
        hash.insert(
            String::from("Ukrainian_ghe_with_upturn"),
            xkb::KEY_Ukrainian_ghe_with_upturn,
        ); // 0x06ad
        hash.insert(
            String::from("Byelorussian_shortu"),
            xkb::KEY_Byelorussian_shortu,
        ); // 0x06ae
        hash.insert(String::from("Cyrillic_dzhe"), xkb::KEY_Cyrillic_dzhe); // 0x06af
        hash.insert(String::from("Serbian_dze"), xkb::KEY_Serbian_dze); // 0x06af
        hash.insert(String::from("numerosign"), xkb::KEY_numerosign); // 0x06b0
        hash.insert(String::from("Serbian_DJE"), xkb::KEY_Serbian_DJE); // 0x06b1
        hash.insert(String::from("Macedonia_GJE"), xkb::KEY_Macedonia_GJE); // 0x06b2
        hash.insert(String::from("Cyrillic_IO"), xkb::KEY_Cyrillic_IO); // 0x06b3
        hash.insert(String::from("Ukrainian_IE"), xkb::KEY_Ukrainian_IE); // 0x06b4
        hash.insert(String::from("Ukranian_JE"), xkb::KEY_Ukranian_JE); // 0x06b4
        hash.insert(String::from("Macedonia_DSE"), xkb::KEY_Macedonia_DSE); // 0x06b5
        hash.insert(String::from("Ukrainian_I"), xkb::KEY_Ukrainian_I); // 0x06b6
        hash.insert(String::from("Ukranian_I"), xkb::KEY_Ukranian_I); // 0x06b6
        hash.insert(String::from("Ukrainian_YI"), xkb::KEY_Ukrainian_YI); // 0x06b7
        hash.insert(String::from("Ukranian_YI"), xkb::KEY_Ukranian_YI); // 0x06b7
        hash.insert(String::from("Cyrillic_JE"), xkb::KEY_Cyrillic_JE); // 0x06b8
        hash.insert(String::from("Serbian_JE"), xkb::KEY_Serbian_JE); // 0x06b8
        hash.insert(String::from("Cyrillic_LJE"), xkb::KEY_Cyrillic_LJE); // 0x06b9
        hash.insert(String::from("Serbian_LJE"), xkb::KEY_Serbian_LJE); // 0x06b9
        hash.insert(String::from("Cyrillic_NJE"), xkb::KEY_Cyrillic_NJE); // 0x06ba
        hash.insert(String::from("Serbian_NJE"), xkb::KEY_Serbian_NJE); // 0x06ba
        hash.insert(String::from("Serbian_TSHE"), xkb::KEY_Serbian_TSHE); // 0x06bb
        hash.insert(String::from("Macedonia_KJE"), xkb::KEY_Macedonia_KJE); // 0x06bc
        hash.insert(
            String::from("Ukrainian_GHE_WITH_UPTURN"),
            xkb::KEY_Ukrainian_GHE_WITH_UPTURN,
        ); // 0x06bd
        hash.insert(
            String::from("Byelorussian_SHORTU"),
            xkb::KEY_Byelorussian_SHORTU,
        ); // 0x06be
        hash.insert(String::from("Cyrillic_DZHE"), xkb::KEY_Cyrillic_DZHE); // 0x06bf
        hash.insert(String::from("Serbian_DZE"), xkb::KEY_Serbian_DZE); // 0x06bf
        hash.insert(String::from("Cyrillic_yu"), xkb::KEY_Cyrillic_yu); // 0x06c0
        hash.insert(String::from("Cyrillic_a"), xkb::KEY_Cyrillic_a); // 0x06c1
        hash.insert(String::from("Cyrillic_be"), xkb::KEY_Cyrillic_be); // 0x06c2
        hash.insert(String::from("Cyrillic_tse"), xkb::KEY_Cyrillic_tse); // 0x06c3
        hash.insert(String::from("Cyrillic_de"), xkb::KEY_Cyrillic_de); // 0x06c4
        hash.insert(String::from("Cyrillic_ie"), xkb::KEY_Cyrillic_ie); // 0x06c5
        hash.insert(String::from("Cyrillic_ef"), xkb::KEY_Cyrillic_ef); // 0x06c6
        hash.insert(String::from("Cyrillic_ghe"), xkb::KEY_Cyrillic_ghe); // 0x06c7
        hash.insert(String::from("Cyrillic_ha"), xkb::KEY_Cyrillic_ha); // 0x06c8
        hash.insert(String::from("Cyrillic_i"), xkb::KEY_Cyrillic_i); // 0x06c9
        hash.insert(String::from("Cyrillic_shorti"), xkb::KEY_Cyrillic_shorti); // 0x06ca
        hash.insert(String::from("Cyrillic_ka"), xkb::KEY_Cyrillic_ka); // 0x06cb
        hash.insert(String::from("Cyrillic_el"), xkb::KEY_Cyrillic_el); // 0x06cc
        hash.insert(String::from("Cyrillic_em"), xkb::KEY_Cyrillic_em); // 0x06cd
        hash.insert(String::from("Cyrillic_en"), xkb::KEY_Cyrillic_en); // 0x06ce
        hash.insert(String::from("Cyrillic_o"), xkb::KEY_Cyrillic_o); // 0x06cf
        hash.insert(String::from("Cyrillic_pe"), xkb::KEY_Cyrillic_pe); // 0x06d0
        hash.insert(String::from("Cyrillic_ya"), xkb::KEY_Cyrillic_ya); // 0x06d1
        hash.insert(String::from("Cyrillic_er"), xkb::KEY_Cyrillic_er); // 0x06d2
        hash.insert(String::from("Cyrillic_es"), xkb::KEY_Cyrillic_es); // 0x06d3
        hash.insert(String::from("Cyrillic_te"), xkb::KEY_Cyrillic_te); // 0x06d4
        hash.insert(String::from("Cyrillic_u"), xkb::KEY_Cyrillic_u); // 0x06d5
        hash.insert(String::from("Cyrillic_zhe"), xkb::KEY_Cyrillic_zhe); // 0x06d6
        hash.insert(String::from("Cyrillic_ve"), xkb::KEY_Cyrillic_ve); // 0x06d7
        hash.insert(
            String::from("Cyrillic_softsign"),
            xkb::KEY_Cyrillic_softsign,
        ); // 0x06d8
        hash.insert(String::from("Cyrillic_yeru"), xkb::KEY_Cyrillic_yeru); // 0x06d9
        hash.insert(String::from("Cyrillic_ze"), xkb::KEY_Cyrillic_ze); // 0x06da
        hash.insert(String::from("Cyrillic_sha"), xkb::KEY_Cyrillic_sha); // 0x06db
        hash.insert(String::from("Cyrillic_e"), xkb::KEY_Cyrillic_e); // 0x06dc
        hash.insert(String::from("Cyrillic_shcha"), xkb::KEY_Cyrillic_shcha); // 0x06dd
        hash.insert(String::from("Cyrillic_che"), xkb::KEY_Cyrillic_che); // 0x06de
        hash.insert(
            String::from("Cyrillic_hardsign"),
            xkb::KEY_Cyrillic_hardsign,
        ); // 0x06df
        hash.insert(String::from("Cyrillic_YU"), xkb::KEY_Cyrillic_YU); // 0x06e0
        hash.insert(String::from("Cyrillic_A"), xkb::KEY_Cyrillic_A); // 0x06e1
        hash.insert(String::from("Cyrillic_BE"), xkb::KEY_Cyrillic_BE); // 0x06e2
        hash.insert(String::from("Cyrillic_TSE"), xkb::KEY_Cyrillic_TSE); // 0x06e3
        hash.insert(String::from("Cyrillic_DE"), xkb::KEY_Cyrillic_DE); // 0x06e4
        hash.insert(String::from("Cyrillic_IE"), xkb::KEY_Cyrillic_IE); // 0x06e5
        hash.insert(String::from("Cyrillic_EF"), xkb::KEY_Cyrillic_EF); // 0x06e6
        hash.insert(String::from("Cyrillic_GHE"), xkb::KEY_Cyrillic_GHE); // 0x06e7
        hash.insert(String::from("Cyrillic_HA"), xkb::KEY_Cyrillic_HA); // 0x06e8
        hash.insert(String::from("Cyrillic_I"), xkb::KEY_Cyrillic_I); // 0x06e9
        hash.insert(String::from("Cyrillic_SHORTI"), xkb::KEY_Cyrillic_SHORTI); // 0x06ea
        hash.insert(String::from("Cyrillic_KA"), xkb::KEY_Cyrillic_KA); // 0x06eb
        hash.insert(String::from("Cyrillic_EL"), xkb::KEY_Cyrillic_EL); // 0x06ec
        hash.insert(String::from("Cyrillic_EM"), xkb::KEY_Cyrillic_EM); // 0x06ed
        hash.insert(String::from("Cyrillic_EN"), xkb::KEY_Cyrillic_EN); // 0x06ee
        hash.insert(String::from("Cyrillic_O"), xkb::KEY_Cyrillic_O); // 0x06ef
        hash.insert(String::from("Cyrillic_PE"), xkb::KEY_Cyrillic_PE); // 0x06f0
        hash.insert(String::from("Cyrillic_YA"), xkb::KEY_Cyrillic_YA); // 0x06f1
        hash.insert(String::from("Cyrillic_ER"), xkb::KEY_Cyrillic_ER); // 0x06f2
        hash.insert(String::from("Cyrillic_ES"), xkb::KEY_Cyrillic_ES); // 0x06f3
        hash.insert(String::from("Cyrillic_TE"), xkb::KEY_Cyrillic_TE); // 0x06f4
        hash.insert(String::from("Cyrillic_U"), xkb::KEY_Cyrillic_U); // 0x06f5
        hash.insert(String::from("Cyrillic_ZHE"), xkb::KEY_Cyrillic_ZHE); // 0x06f6
        hash.insert(String::from("Cyrillic_VE"), xkb::KEY_Cyrillic_VE); // 0x06f7
        hash.insert(
            String::from("Cyrillic_SOFTSIGN"),
            xkb::KEY_Cyrillic_SOFTSIGN,
        ); // 0x06f8
        hash.insert(String::from("Cyrillic_YERU"), xkb::KEY_Cyrillic_YERU); // 0x06f9
        hash.insert(String::from("Cyrillic_ZE"), xkb::KEY_Cyrillic_ZE); // 0x06fa
        hash.insert(String::from("Cyrillic_SHA"), xkb::KEY_Cyrillic_SHA); // 0x06fb
        hash.insert(String::from("Cyrillic_E"), xkb::KEY_Cyrillic_E); // 0x06fc
        hash.insert(String::from("Cyrillic_SHCHA"), xkb::KEY_Cyrillic_SHCHA); // 0x06fd
        hash.insert(String::from("Cyrillic_CHE"), xkb::KEY_Cyrillic_CHE); // 0x06fe
        hash.insert(
            String::from("Cyrillic_HARDSIGN"),
            xkb::KEY_Cyrillic_HARDSIGN,
        ); // 0x06ff

        hash.insert(
            String::from("Greek_ALPHAaccent"),
            xkb::KEY_Greek_ALPHAaccent,
        ); // 0x07a1
        hash.insert(
            String::from("Greek_EPSILONaccent"),
            xkb::KEY_Greek_EPSILONaccent,
        ); // 0x07a2
        hash.insert(String::from("Greek_ETAaccent"), xkb::KEY_Greek_ETAaccent); // 0x07a3
        hash.insert(String::from("Greek_IOTAaccent"), xkb::KEY_Greek_IOTAaccent); // 0x07a4
        hash.insert(
            String::from("Greek_IOTAdieresis"),
            xkb::KEY_Greek_IOTAdieresis,
        ); // 0x07a5
        hash.insert(
            String::from("Greek_IOTAdiaeresis"),
            xkb::KEY_Greek_IOTAdiaeresis,
        ); // 0x07a5
        hash.insert(
            String::from("Greek_OMICRONaccent"),
            xkb::KEY_Greek_OMICRONaccent,
        ); // 0x07a7
        hash.insert(
            String::from("Greek_UPSILONaccent"),
            xkb::KEY_Greek_UPSILONaccent,
        ); // 0x07a8
        hash.insert(
            String::from("Greek_UPSILONdieresis"),
            xkb::KEY_Greek_UPSILONdieresis,
        ); // 0x07a9
        hash.insert(
            String::from("Greek_OMEGAaccent"),
            xkb::KEY_Greek_OMEGAaccent,
        ); // 0x07ab
        hash.insert(
            String::from("Greek_accentdieresis"),
            xkb::KEY_Greek_accentdieresis,
        ); // 0x07ae
        hash.insert(String::from("Greek_horizbar"), xkb::KEY_Greek_horizbar); // 0x07af
        hash.insert(
            String::from("Greek_alphaaccent"),
            xkb::KEY_Greek_alphaaccent,
        ); // 0x07b1
        hash.insert(
            String::from("Greek_epsilonaccent"),
            xkb::KEY_Greek_epsilonaccent,
        ); // 0x07b2
        hash.insert(String::from("Greek_etaaccent"), xkb::KEY_Greek_etaaccent); // 0x07b3
        hash.insert(String::from("Greek_iotaaccent"), xkb::KEY_Greek_iotaaccent); // 0x07b4
        hash.insert(
            String::from("Greek_iotadieresis"),
            xkb::KEY_Greek_iotadieresis,
        ); // 0x07b5
        hash.insert(
            String::from("Greek_iotaaccentdieresis"),
            xkb::KEY_Greek_iotaaccentdieresis,
        ); // 0x07b6
        hash.insert(
            String::from("Greek_omicronaccent"),
            xkb::KEY_Greek_omicronaccent,
        ); // 0x07b7
        hash.insert(
            String::from("Greek_upsilonaccent"),
            xkb::KEY_Greek_upsilonaccent,
        ); // 0x07b8
        hash.insert(
            String::from("Greek_upsilondieresis"),
            xkb::KEY_Greek_upsilondieresis,
        ); // 0x07b9
        hash.insert(
            String::from("Greek_upsilonaccentdieresis"),
            xkb::KEY_Greek_upsilonaccentdieresis,
        ); // 0x07ba
        hash.insert(
            String::from("Greek_omegaaccent"),
            xkb::KEY_Greek_omegaaccent,
        ); // 0x07bb
        hash.insert(String::from("Greek_ALPHA"), xkb::KEY_Greek_ALPHA); // 0x07c1
        hash.insert(String::from("Greek_BETA"), xkb::KEY_Greek_BETA); // 0x07c2
        hash.insert(String::from("Greek_GAMMA"), xkb::KEY_Greek_GAMMA); // 0x07c3
        hash.insert(String::from("Greek_DELTA"), xkb::KEY_Greek_DELTA); // 0x07c4
        hash.insert(String::from("Greek_EPSILON"), xkb::KEY_Greek_EPSILON); // 0x07c5
        hash.insert(String::from("Greek_ZETA"), xkb::KEY_Greek_ZETA); // 0x07c6
        hash.insert(String::from("Greek_ETA"), xkb::KEY_Greek_ETA); // 0x07c7
        hash.insert(String::from("Greek_THETA"), xkb::KEY_Greek_THETA); // 0x07c8
        hash.insert(String::from("Greek_IOTA"), xkb::KEY_Greek_IOTA); // 0x07c9
        hash.insert(String::from("Greek_KAPPA"), xkb::KEY_Greek_KAPPA); // 0x07ca
        hash.insert(String::from("Greek_LAMDA"), xkb::KEY_Greek_LAMDA); // 0x07cb
        hash.insert(String::from("Greek_LAMBDA"), xkb::KEY_Greek_LAMBDA); // 0x07cb
        hash.insert(String::from("Greek_MU"), xkb::KEY_Greek_MU); // 0x07cc
        hash.insert(String::from("Greek_NU"), xkb::KEY_Greek_NU); // 0x07cd
        hash.insert(String::from("Greek_XI"), xkb::KEY_Greek_XI); // 0x07ce
        hash.insert(String::from("Greek_OMICRON"), xkb::KEY_Greek_OMICRON); // 0x07cf
        hash.insert(String::from("Greek_PI"), xkb::KEY_Greek_PI); // 0x07d0
        hash.insert(String::from("Greek_RHO"), xkb::KEY_Greek_RHO); // 0x07d1
        hash.insert(String::from("Greek_SIGMA"), xkb::KEY_Greek_SIGMA); // 0x07d2
        hash.insert(String::from("Greek_TAU"), xkb::KEY_Greek_TAU); // 0x07d4
        hash.insert(String::from("Greek_UPSILON"), xkb::KEY_Greek_UPSILON); // 0x07d5
        hash.insert(String::from("Greek_PHI"), xkb::KEY_Greek_PHI); // 0x07d6
        hash.insert(String::from("Greek_CHI"), xkb::KEY_Greek_CHI); // 0x07d7
        hash.insert(String::from("Greek_PSI"), xkb::KEY_Greek_PSI); // 0x07d8
        hash.insert(String::from("Greek_OMEGA"), xkb::KEY_Greek_OMEGA); // 0x07d9
        hash.insert(String::from("Greek_alpha"), xkb::KEY_Greek_alpha); // 0x07e1
        hash.insert(String::from("Greek_beta"), xkb::KEY_Greek_beta); // 0x07e2
        hash.insert(String::from("Greek_gamma"), xkb::KEY_Greek_gamma); // 0x07e3
        hash.insert(String::from("Greek_delta"), xkb::KEY_Greek_delta); // 0x07e4
        hash.insert(String::from("Greek_epsilon"), xkb::KEY_Greek_epsilon); // 0x07e5
        hash.insert(String::from("Greek_zeta"), xkb::KEY_Greek_zeta); // 0x07e6
        hash.insert(String::from("Greek_eta"), xkb::KEY_Greek_eta); // 0x07e7
        hash.insert(String::from("Greek_theta"), xkb::KEY_Greek_theta); // 0x07e8
        hash.insert(String::from("Greek_iota"), xkb::KEY_Greek_iota); // 0x07e9
        hash.insert(String::from("Greek_kappa"), xkb::KEY_Greek_kappa); // 0x07ea
        hash.insert(String::from("Greek_lamda"), xkb::KEY_Greek_lamda); // 0x07eb
        hash.insert(String::from("Greek_lambda"), xkb::KEY_Greek_lambda); // 0x07eb
        hash.insert(String::from("Greek_mu"), xkb::KEY_Greek_mu); // 0x07ec
        hash.insert(String::from("Greek_nu"), xkb::KEY_Greek_nu); // 0x07ed
        hash.insert(String::from("Greek_xi"), xkb::KEY_Greek_xi); // 0x07ee
        hash.insert(String::from("Greek_omicron"), xkb::KEY_Greek_omicron); // 0x07ef
        hash.insert(String::from("Greek_pi"), xkb::KEY_Greek_pi); // 0x07f0
        hash.insert(String::from("Greek_rho"), xkb::KEY_Greek_rho); // 0x07f1
        hash.insert(String::from("Greek_sigma"), xkb::KEY_Greek_sigma); // 0x07f2
        hash.insert(
            String::from("Greek_finalsmallsigma"),
            xkb::KEY_Greek_finalsmallsigma,
        ); // 0x07f3
        hash.insert(String::from("Greek_tau"), xkb::KEY_Greek_tau); // 0x07f4
        hash.insert(String::from("Greek_upsilon"), xkb::KEY_Greek_upsilon); // 0x07f5
        hash.insert(String::from("Greek_phi"), xkb::KEY_Greek_phi); // 0x07f6
        hash.insert(String::from("Greek_chi"), xkb::KEY_Greek_chi); // 0x07f7
        hash.insert(String::from("Greek_psi"), xkb::KEY_Greek_psi); // 0x07f8
        hash.insert(String::from("Greek_omega"), xkb::KEY_Greek_omega); // 0x07f9
        hash.insert(String::from("Greek_switch"), xkb::KEY_Greek_switch); // 0xff7e

        hash.insert(String::from("leftradical"), xkb::KEY_leftradical); // 0x08a1
        hash.insert(String::from("topleftradical"), xkb::KEY_topleftradical); // 0x08a2
        hash.insert(String::from("horizconnector"), xkb::KEY_horizconnector); // 0x08a3
        hash.insert(String::from("topintegral"), xkb::KEY_topintegral); // 0x08a4
        hash.insert(String::from("botintegral"), xkb::KEY_botintegral); // 0x08a5
        hash.insert(String::from("vertconnector"), xkb::KEY_vertconnector); // 0x08a6
        hash.insert(String::from("topleftsqbracket"), xkb::KEY_topleftsqbracket); // 0x08a7
        hash.insert(String::from("botleftsqbracket"), xkb::KEY_botleftsqbracket); // 0x08a8
        hash.insert(
            String::from("toprightsqbracket"),
            xkb::KEY_toprightsqbracket,
        ); // 0x08a9
        hash.insert(
            String::from("botrightsqbracket"),
            xkb::KEY_botrightsqbracket,
        ); // 0x08aa
        hash.insert(String::from("topleftparens"), xkb::KEY_topleftparens); // 0x08ab
        hash.insert(String::from("botleftparens"), xkb::KEY_botleftparens); // 0x08ac
        hash.insert(String::from("toprightparens"), xkb::KEY_toprightparens); // 0x08ad
        hash.insert(String::from("botrightparens"), xkb::KEY_botrightparens); // 0x08ae
        hash.insert(
            String::from("leftmiddlecurlybrace"),
            xkb::KEY_leftmiddlecurlybrace,
        ); // 0x08af
        hash.insert(
            String::from("rightmiddlecurlybrace"),
            xkb::KEY_rightmiddlecurlybrace,
        ); // 0x08b0
        hash.insert(String::from("topleftsummation"), xkb::KEY_topleftsummation); // 0x08b1
        hash.insert(String::from("botleftsummation"), xkb::KEY_botleftsummation); // 0x08b2
        hash.insert(
            String::from("topvertsummationconnector"),
            xkb::KEY_topvertsummationconnector,
        ); // 0x08b3
        hash.insert(
            String::from("botvertsummationconnector"),
            xkb::KEY_botvertsummationconnector,
        ); // 0x08b4
        hash.insert(
            String::from("toprightsummation"),
            xkb::KEY_toprightsummation,
        ); // 0x08b5
        hash.insert(
            String::from("botrightsummation"),
            xkb::KEY_botrightsummation,
        ); // 0x08b6
        hash.insert(
            String::from("rightmiddlesummation"),
            xkb::KEY_rightmiddlesummation,
        ); // 0x08b7
        hash.insert(String::from("lessthanequal"), xkb::KEY_lessthanequal); // 0x08bc
        hash.insert(String::from("notequal"), xkb::KEY_notequal); // 0x08bd
        hash.insert(String::from("greaterthanequal"), xkb::KEY_greaterthanequal); // 0x08be
        hash.insert(String::from("integral"), xkb::KEY_integral); // 0x08bf
        hash.insert(String::from("therefore"), xkb::KEY_therefore); // 0x08c0
        hash.insert(String::from("variation"), xkb::KEY_variation); // 0x08c1
        hash.insert(String::from("infinity"), xkb::KEY_infinity); // 0x08c2
        hash.insert(String::from("nabla"), xkb::KEY_nabla); // 0x08c5
        hash.insert(String::from("approximate"), xkb::KEY_approximate); // 0x08c8
        hash.insert(String::from("similarequal"), xkb::KEY_similarequal); // 0x08c9
        hash.insert(String::from("ifonlyif"), xkb::KEY_ifonlyif); // 0x08cd
        hash.insert(String::from("implies"), xkb::KEY_implies); // 0x08ce
        hash.insert(String::from("identical"), xkb::KEY_identical); // 0x08cf
        hash.insert(String::from("radical"), xkb::KEY_radical); // 0x08d6
        hash.insert(String::from("includedin"), xkb::KEY_includedin); // 0x08da
        hash.insert(String::from("includes"), xkb::KEY_includes); // 0x08db
        hash.insert(String::from("intersection"), xkb::KEY_intersection); // 0x08dc
        hash.insert(String::from("union"), xkb::KEY_union); // 0x08dd
        hash.insert(String::from("logicaland"), xkb::KEY_logicaland); // 0x08de
        hash.insert(String::from("logicalor"), xkb::KEY_logicalor); // 0x08df
        hash.insert(
            String::from("partialderivative"),
            xkb::KEY_partialderivative,
        ); // 0x08ef
        hash.insert(String::from("function"), xkb::KEY_function); // 0x08f6
        hash.insert(String::from("leftarrow"), xkb::KEY_leftarrow); // 0x08fb
        hash.insert(String::from("uparrow"), xkb::KEY_uparrow); // 0x08fc
        hash.insert(String::from("rightarrow"), xkb::KEY_rightarrow); // 0x08fd
        hash.insert(String::from("downarrow"), xkb::KEY_downarrow); // 0x08fe

        hash.insert(String::from("blank"), xkb::KEY_blank); // 0x09df
        hash.insert(String::from("soliddiamond"), xkb::KEY_soliddiamond); // 0x09e0
        hash.insert(String::from("checkerboard"), xkb::KEY_checkerboard); // 0x09e1
        hash.insert(String::from("ht"), xkb::KEY_ht); // 0x09e2
        hash.insert(String::from("ff"), xkb::KEY_ff); // 0x09e3
        hash.insert(String::from("cr"), xkb::KEY_cr); // 0x09e4
        hash.insert(String::from("lf"), xkb::KEY_lf); // 0x09e5
        hash.insert(String::from("nl"), xkb::KEY_nl); // 0x09e8
        hash.insert(String::from("vt"), xkb::KEY_vt); // 0x09e9
        hash.insert(String::from("lowrightcorner"), xkb::KEY_lowrightcorner); // 0x09ea
        hash.insert(String::from("uprightcorner"), xkb::KEY_uprightcorner); // 0x09eb
        hash.insert(String::from("upleftcorner"), xkb::KEY_upleftcorner); // 0x09ec
        hash.insert(String::from("lowleftcorner"), xkb::KEY_lowleftcorner); // 0x09ed
        hash.insert(String::from("crossinglines"), xkb::KEY_crossinglines); // 0x09ee
        hash.insert(String::from("horizlinescan1"), xkb::KEY_horizlinescan1); // 0x09ef
        hash.insert(String::from("horizlinescan3"), xkb::KEY_horizlinescan3); // 0x09f0
        hash.insert(String::from("horizlinescan5"), xkb::KEY_horizlinescan5); // 0x09f1
        hash.insert(String::from("horizlinescan7"), xkb::KEY_horizlinescan7); // 0x09f2
        hash.insert(String::from("horizlinescan9"), xkb::KEY_horizlinescan9); // 0x09f3
        hash.insert(String::from("leftt"), xkb::KEY_leftt); // 0x09f4
        hash.insert(String::from("rightt"), xkb::KEY_rightt); // 0x09f5
        hash.insert(String::from("bott"), xkb::KEY_bott); // 0x09f6
        hash.insert(String::from("topt"), xkb::KEY_topt); // 0x09f7
        hash.insert(String::from("vertbar"), xkb::KEY_vertbar); // 0x09f8

        hash.insert(String::from("emspace"), xkb::KEY_emspace); // 0x0aa1
        hash.insert(String::from("enspace"), xkb::KEY_enspace); // 0x0aa2
        hash.insert(String::from("em3space"), xkb::KEY_em3space); // 0x0aa3
        hash.insert(String::from("em4space"), xkb::KEY_em4space); // 0x0aa4
        hash.insert(String::from("digitspace"), xkb::KEY_digitspace); // 0x0aa5
        hash.insert(String::from("punctspace"), xkb::KEY_punctspace); // 0x0aa6
        hash.insert(String::from("thinspace"), xkb::KEY_thinspace); // 0x0aa7
        hash.insert(String::from("hairspace"), xkb::KEY_hairspace); // 0x0aa8
        hash.insert(String::from("emdash"), xkb::KEY_emdash); // 0x0aa9
        hash.insert(String::from("endash"), xkb::KEY_endash); // 0x0aaa
        hash.insert(String::from("signifblank"), xkb::KEY_signifblank); // 0x0aac
        hash.insert(String::from("ellipsis"), xkb::KEY_ellipsis); // 0x0aae
        hash.insert(String::from("doubbaselinedot"), xkb::KEY_doubbaselinedot); // 0x0aaf
        hash.insert(String::from("onethird"), xkb::KEY_onethird); // 0x0ab0
        hash.insert(String::from("twothirds"), xkb::KEY_twothirds); // 0x0ab1
        hash.insert(String::from("onefifth"), xkb::KEY_onefifth); // 0x0ab2
        hash.insert(String::from("twofifths"), xkb::KEY_twofifths); // 0x0ab3
        hash.insert(String::from("threefifths"), xkb::KEY_threefifths); // 0x0ab4
        hash.insert(String::from("fourfifths"), xkb::KEY_fourfifths); // 0x0ab5
        hash.insert(String::from("onesixth"), xkb::KEY_onesixth); // 0x0ab6
        hash.insert(String::from("fivesixths"), xkb::KEY_fivesixths); // 0x0ab7
        hash.insert(String::from("careof"), xkb::KEY_careof); // 0x0ab8
        hash.insert(String::from("figdash"), xkb::KEY_figdash); // 0x0abb
        hash.insert(String::from("leftanglebracket"), xkb::KEY_leftanglebracket); // 0x0abc
        hash.insert(String::from("decimalpoint"), xkb::KEY_decimalpoint); // 0x0abd
        hash.insert(
            String::from("rightanglebracket"),
            xkb::KEY_rightanglebracket,
        ); // 0x0abe
        hash.insert(String::from("marker"), xkb::KEY_marker); // 0x0abf
        hash.insert(String::from("oneeighth"), xkb::KEY_oneeighth); // 0x0ac3
        hash.insert(String::from("threeeighths"), xkb::KEY_threeeighths); // 0x0ac4
        hash.insert(String::from("fiveeighths"), xkb::KEY_fiveeighths); // 0x0ac5
        hash.insert(String::from("seveneighths"), xkb::KEY_seveneighths); // 0x0ac6
        hash.insert(String::from("trademark"), xkb::KEY_trademark); // 0x0ac9
        hash.insert(String::from("signaturemark"), xkb::KEY_signaturemark); // 0x0aca
        hash.insert(
            String::from("trademarkincircle"),
            xkb::KEY_trademarkincircle,
        ); // 0x0acb
        hash.insert(String::from("leftopentriangle"), xkb::KEY_leftopentriangle); // 0x0acc
        hash.insert(
            String::from("rightopentriangle"),
            xkb::KEY_rightopentriangle,
        ); // 0x0acd
        hash.insert(String::from("emopencircle"), xkb::KEY_emopencircle); // 0x0ace
        hash.insert(String::from("emopenrectangle"), xkb::KEY_emopenrectangle); // 0x0acf
        hash.insert(
            String::from("leftsinglequotemark"),
            xkb::KEY_leftsinglequotemark,
        ); // 0x0ad0
        hash.insert(
            String::from("rightsinglequotemark"),
            xkb::KEY_rightsinglequotemark,
        ); // 0x0ad1
        hash.insert(
            String::from("leftdoublequotemark"),
            xkb::KEY_leftdoublequotemark,
        ); // 0x0ad2
        hash.insert(
            String::from("rightdoublequotemark"),
            xkb::KEY_rightdoublequotemark,
        ); // 0x0ad3
        hash.insert(String::from("prescription"), xkb::KEY_prescription); // 0x0ad4
        hash.insert(String::from("permille"), xkb::KEY_permille); // 0x0ad5
        hash.insert(String::from("minutes"), xkb::KEY_minutes); // 0x0ad6
        hash.insert(String::from("seconds"), xkb::KEY_seconds); // 0x0ad7
        hash.insert(String::from("latincross"), xkb::KEY_latincross); // 0x0ad9
        hash.insert(String::from("hexagram"), xkb::KEY_hexagram); // 0x0ada
        hash.insert(String::from("filledrectbullet"), xkb::KEY_filledrectbullet); // 0x0adb
        hash.insert(
            String::from("filledlefttribullet"),
            xkb::KEY_filledlefttribullet,
        ); // 0x0adc
        hash.insert(
            String::from("filledrighttribullet"),
            xkb::KEY_filledrighttribullet,
        ); // 0x0add
        hash.insert(String::from("emfilledcircle"), xkb::KEY_emfilledcircle); // 0x0ade
        hash.insert(String::from("emfilledrect"), xkb::KEY_emfilledrect); // 0x0adf
        hash.insert(String::from("enopencircbullet"), xkb::KEY_enopencircbullet); // 0x0ae0
        hash.insert(
            String::from("enopensquarebullet"),
            xkb::KEY_enopensquarebullet,
        ); // 0x0ae1
        hash.insert(String::from("openrectbullet"), xkb::KEY_openrectbullet); // 0x0ae2
        hash.insert(String::from("opentribulletup"), xkb::KEY_opentribulletup); // 0x0ae3
        hash.insert(
            String::from("opentribulletdown"),
            xkb::KEY_opentribulletdown,
        ); // 0x0ae4
        hash.insert(String::from("openstar"), xkb::KEY_openstar); // 0x0ae5
        hash.insert(
            String::from("enfilledcircbullet"),
            xkb::KEY_enfilledcircbullet,
        ); // 0x0ae6
        hash.insert(String::from("enfilledsqbullet"), xkb::KEY_enfilledsqbullet); // 0x0ae7
        hash.insert(
            String::from("filledtribulletup"),
            xkb::KEY_filledtribulletup,
        ); // 0x0ae8
        hash.insert(
            String::from("filledtribulletdown"),
            xkb::KEY_filledtribulletdown,
        ); // 0x0ae9
        hash.insert(String::from("leftpointer"), xkb::KEY_leftpointer); // 0x0aea
        hash.insert(String::from("rightpointer"), xkb::KEY_rightpointer); // 0x0aeb
        hash.insert(String::from("club"), xkb::KEY_club); // 0x0aec
        hash.insert(String::from("diamond"), xkb::KEY_diamond); // 0x0aed
        hash.insert(String::from("heart"), xkb::KEY_heart); // 0x0aee
        hash.insert(String::from("maltesecross"), xkb::KEY_maltesecross); // 0x0af0
        hash.insert(String::from("dagger"), xkb::KEY_dagger); // 0x0af1
        hash.insert(String::from("doubledagger"), xkb::KEY_doubledagger); // 0x0af2
        hash.insert(String::from("checkmark"), xkb::KEY_checkmark); // 0x0af3
        hash.insert(String::from("ballotcross"), xkb::KEY_ballotcross); // 0x0af4
        hash.insert(String::from("musicalsharp"), xkb::KEY_musicalsharp); // 0x0af5
        hash.insert(String::from("musicalflat"), xkb::KEY_musicalflat); // 0x0af6
        hash.insert(String::from("malesymbol"), xkb::KEY_malesymbol); // 0x0af7
        hash.insert(String::from("femalesymbol"), xkb::KEY_femalesymbol); // 0x0af8
        hash.insert(String::from("telephone"), xkb::KEY_telephone); // 0x0af9
        hash.insert(
            String::from("telephonerecorder"),
            xkb::KEY_telephonerecorder,
        ); // 0x0afa
        hash.insert(
            String::from("phonographcopyright"),
            xkb::KEY_phonographcopyright,
        ); // 0x0afb
        hash.insert(String::from("caret"), xkb::KEY_caret); // 0x0afc
        hash.insert(
            String::from("singlelowquotemark"),
            xkb::KEY_singlelowquotemark,
        ); // 0x0afd
        hash.insert(
            String::from("doublelowquotemark"),
            xkb::KEY_doublelowquotemark,
        ); // 0x0afe
        hash.insert(String::from("cursor"), xkb::KEY_cursor); // 0x0aff

        hash.insert(String::from("leftcaret"), xkb::KEY_leftcaret); // 0x0ba3
        hash.insert(String::from("rightcaret"), xkb::KEY_rightcaret); // 0x0ba6
        hash.insert(String::from("downcaret"), xkb::KEY_downcaret); // 0x0ba8
        hash.insert(String::from("upcaret"), xkb::KEY_upcaret); // 0x0ba9
        hash.insert(String::from("overbar"), xkb::KEY_overbar); // 0x0bc0
        hash.insert(String::from("downtack"), xkb::KEY_downtack); // 0x0bc2
        hash.insert(String::from("upshoe"), xkb::KEY_upshoe); // 0x0bc3
        hash.insert(String::from("downstile"), xkb::KEY_downstile); // 0x0bc4
        hash.insert(String::from("underbar"), xkb::KEY_underbar); // 0x0bc6
        hash.insert(String::from("jot"), xkb::KEY_jot); // 0x0bca
        hash.insert(String::from("quad"), xkb::KEY_quad); // 0x0bcc
        hash.insert(String::from("uptack"), xkb::KEY_uptack); // 0x0bce
        hash.insert(String::from("circle"), xkb::KEY_circle); // 0x0bcf
        hash.insert(String::from("upstile"), xkb::KEY_upstile); // 0x0bd3
        hash.insert(String::from("downshoe"), xkb::KEY_downshoe); // 0x0bd6
        hash.insert(String::from("rightshoe"), xkb::KEY_rightshoe); // 0x0bd8
        hash.insert(String::from("leftshoe"), xkb::KEY_leftshoe); // 0x0bda
        hash.insert(String::from("lefttack"), xkb::KEY_lefttack); // 0x0bdc
        hash.insert(String::from("righttack"), xkb::KEY_righttack); // 0x0bfc

        hash.insert(
            String::from("hebrew_doublelowline"),
            xkb::KEY_hebrew_doublelowline,
        ); // 0x0cdf
        hash.insert(String::from("hebrew_aleph"), xkb::KEY_hebrew_aleph); // 0x0ce0
        hash.insert(String::from("hebrew_bet"), xkb::KEY_hebrew_bet); // 0x0ce1
        hash.insert(String::from("hebrew_beth"), xkb::KEY_hebrew_beth); // 0x0ce1
        hash.insert(String::from("hebrew_gimel"), xkb::KEY_hebrew_gimel); // 0x0ce2
        hash.insert(String::from("hebrew_gimmel"), xkb::KEY_hebrew_gimmel); // 0x0ce2
        hash.insert(String::from("hebrew_dalet"), xkb::KEY_hebrew_dalet); // 0x0ce3
        hash.insert(String::from("hebrew_daleth"), xkb::KEY_hebrew_daleth); // 0x0ce3
        hash.insert(String::from("hebrew_he"), xkb::KEY_hebrew_he); // 0x0ce4
        hash.insert(String::from("hebrew_waw"), xkb::KEY_hebrew_waw); // 0x0ce5
        hash.insert(String::from("hebrew_zain"), xkb::KEY_hebrew_zain); // 0x0ce6
        hash.insert(String::from("hebrew_zayin"), xkb::KEY_hebrew_zayin); // 0x0ce6
        hash.insert(String::from("hebrew_chet"), xkb::KEY_hebrew_chet); // 0x0ce7
        hash.insert(String::from("hebrew_het"), xkb::KEY_hebrew_het); // 0x0ce7
        hash.insert(String::from("hebrew_tet"), xkb::KEY_hebrew_tet); // 0x0ce8
        hash.insert(String::from("hebrew_teth"), xkb::KEY_hebrew_teth); // 0x0ce8
        hash.insert(String::from("hebrew_yod"), xkb::KEY_hebrew_yod); // 0x0ce9
        hash.insert(String::from("hebrew_finalkaph"), xkb::KEY_hebrew_finalkaph); // 0x0cea
        hash.insert(String::from("hebrew_kaph"), xkb::KEY_hebrew_kaph); // 0x0ceb
        hash.insert(String::from("hebrew_lamed"), xkb::KEY_hebrew_lamed); // 0x0cec
        hash.insert(String::from("hebrew_finalmem"), xkb::KEY_hebrew_finalmem); // 0x0ced
        hash.insert(String::from("hebrew_mem"), xkb::KEY_hebrew_mem); // 0x0cee
        hash.insert(String::from("hebrew_finalnun"), xkb::KEY_hebrew_finalnun); // 0x0cef
        hash.insert(String::from("hebrew_nun"), xkb::KEY_hebrew_nun); // 0x0cf0
        hash.insert(String::from("hebrew_samech"), xkb::KEY_hebrew_samech); // 0x0cf1
        hash.insert(String::from("hebrew_samekh"), xkb::KEY_hebrew_samekh); // 0x0cf1
        hash.insert(String::from("hebrew_ayin"), xkb::KEY_hebrew_ayin); // 0x0cf2
        hash.insert(String::from("hebrew_finalpe"), xkb::KEY_hebrew_finalpe); // 0x0cf3
        hash.insert(String::from("hebrew_pe"), xkb::KEY_hebrew_pe); // 0x0cf4
        hash.insert(String::from("hebrew_finalzade"), xkb::KEY_hebrew_finalzade); // 0x0cf5
        hash.insert(String::from("hebrew_finalzadi"), xkb::KEY_hebrew_finalzadi); // 0x0cf5
        hash.insert(String::from("hebrew_zade"), xkb::KEY_hebrew_zade); // 0x0cf6
        hash.insert(String::from("hebrew_zadi"), xkb::KEY_hebrew_zadi); // 0x0cf6
        hash.insert(String::from("hebrew_qoph"), xkb::KEY_hebrew_qoph); // 0x0cf7
        hash.insert(String::from("hebrew_kuf"), xkb::KEY_hebrew_kuf); // 0x0cf7
        hash.insert(String::from("hebrew_resh"), xkb::KEY_hebrew_resh); // 0x0cf8
        hash.insert(String::from("hebrew_shin"), xkb::KEY_hebrew_shin); // 0x0cf9
        hash.insert(String::from("hebrew_taw"), xkb::KEY_hebrew_taw); // 0x0cfa
        hash.insert(String::from("hebrew_taf"), xkb::KEY_hebrew_taf); // 0x0cfa
        hash.insert(String::from("Hebrew_switch"), xkb::KEY_Hebrew_switch); // 0xff7e

        hash.insert(String::from("Thai_kokai"), xkb::KEY_Thai_kokai); // 0x0da1
        hash.insert(String::from("Thai_khokhai"), xkb::KEY_Thai_khokhai); // 0x0da2
        hash.insert(String::from("Thai_khokhuat"), xkb::KEY_Thai_khokhuat); // 0x0da3
        hash.insert(String::from("Thai_khokhwai"), xkb::KEY_Thai_khokhwai); // 0x0da4
        hash.insert(String::from("Thai_khokhon"), xkb::KEY_Thai_khokhon); // 0x0da5
        hash.insert(String::from("Thai_khorakhang"), xkb::KEY_Thai_khorakhang); // 0x0da6
        hash.insert(String::from("Thai_ngongu"), xkb::KEY_Thai_ngongu); // 0x0da7
        hash.insert(String::from("Thai_chochan"), xkb::KEY_Thai_chochan); // 0x0da8
        hash.insert(String::from("Thai_choching"), xkb::KEY_Thai_choching); // 0x0da9
        hash.insert(String::from("Thai_chochang"), xkb::KEY_Thai_chochang); // 0x0daa
        hash.insert(String::from("Thai_soso"), xkb::KEY_Thai_soso); // 0x0dab
        hash.insert(String::from("Thai_chochoe"), xkb::KEY_Thai_chochoe); // 0x0dac
        hash.insert(String::from("Thai_yoying"), xkb::KEY_Thai_yoying); // 0x0dad
        hash.insert(String::from("Thai_dochada"), xkb::KEY_Thai_dochada); // 0x0dae
        hash.insert(String::from("Thai_topatak"), xkb::KEY_Thai_topatak); // 0x0daf
        hash.insert(String::from("Thai_thothan"), xkb::KEY_Thai_thothan); // 0x0db0
        hash.insert(
            String::from("Thai_thonangmontho"),
            xkb::KEY_Thai_thonangmontho,
        ); // 0x0db1
        hash.insert(String::from("Thai_thophuthao"), xkb::KEY_Thai_thophuthao); // 0x0db2
        hash.insert(String::from("Thai_nonen"), xkb::KEY_Thai_nonen); // 0x0db3
        hash.insert(String::from("Thai_dodek"), xkb::KEY_Thai_dodek); // 0x0db4
        hash.insert(String::from("Thai_totao"), xkb::KEY_Thai_totao); // 0x0db5
        hash.insert(String::from("Thai_thothung"), xkb::KEY_Thai_thothung); // 0x0db6
        hash.insert(String::from("Thai_thothahan"), xkb::KEY_Thai_thothahan); // 0x0db7
        hash.insert(String::from("Thai_thothong"), xkb::KEY_Thai_thothong); // 0x0db8
        hash.insert(String::from("Thai_nonu"), xkb::KEY_Thai_nonu); // 0x0db9
        hash.insert(String::from("Thai_bobaimai"), xkb::KEY_Thai_bobaimai); // 0x0dba
        hash.insert(String::from("Thai_popla"), xkb::KEY_Thai_popla); // 0x0dbb
        hash.insert(String::from("Thai_phophung"), xkb::KEY_Thai_phophung); // 0x0dbc
        hash.insert(String::from("Thai_fofa"), xkb::KEY_Thai_fofa); // 0x0dbd
        hash.insert(String::from("Thai_phophan"), xkb::KEY_Thai_phophan); // 0x0dbe
        hash.insert(String::from("Thai_fofan"), xkb::KEY_Thai_fofan); // 0x0dbf
        hash.insert(String::from("Thai_phosamphao"), xkb::KEY_Thai_phosamphao); // 0x0dc0
        hash.insert(String::from("Thai_moma"), xkb::KEY_Thai_moma); // 0x0dc1
        hash.insert(String::from("Thai_yoyak"), xkb::KEY_Thai_yoyak); // 0x0dc2
        hash.insert(String::from("Thai_rorua"), xkb::KEY_Thai_rorua); // 0x0dc3
        hash.insert(String::from("Thai_ru"), xkb::KEY_Thai_ru); // 0x0dc4
        hash.insert(String::from("Thai_loling"), xkb::KEY_Thai_loling); // 0x0dc5
        hash.insert(String::from("Thai_lu"), xkb::KEY_Thai_lu); // 0x0dc6
        hash.insert(String::from("Thai_wowaen"), xkb::KEY_Thai_wowaen); // 0x0dc7
        hash.insert(String::from("Thai_sosala"), xkb::KEY_Thai_sosala); // 0x0dc8
        hash.insert(String::from("Thai_sorusi"), xkb::KEY_Thai_sorusi); // 0x0dc9
        hash.insert(String::from("Thai_sosua"), xkb::KEY_Thai_sosua); // 0x0dca
        hash.insert(String::from("Thai_hohip"), xkb::KEY_Thai_hohip); // 0x0dcb
        hash.insert(String::from("Thai_lochula"), xkb::KEY_Thai_lochula); // 0x0dcc
        hash.insert(String::from("Thai_oang"), xkb::KEY_Thai_oang); // 0x0dcd
        hash.insert(String::from("Thai_honokhuk"), xkb::KEY_Thai_honokhuk); // 0x0dce
        hash.insert(String::from("Thai_paiyannoi"), xkb::KEY_Thai_paiyannoi); // 0x0dcf
        hash.insert(String::from("Thai_saraa"), xkb::KEY_Thai_saraa); // 0x0dd0
        hash.insert(String::from("Thai_maihanakat"), xkb::KEY_Thai_maihanakat); // 0x0dd1
        hash.insert(String::from("Thai_saraaa"), xkb::KEY_Thai_saraaa); // 0x0dd2
        hash.insert(String::from("Thai_saraam"), xkb::KEY_Thai_saraam); // 0x0dd3
        hash.insert(String::from("Thai_sarai"), xkb::KEY_Thai_sarai); // 0x0dd4
        hash.insert(String::from("Thai_saraii"), xkb::KEY_Thai_saraii); // 0x0dd5
        hash.insert(String::from("Thai_saraue"), xkb::KEY_Thai_saraue); // 0x0dd6
        hash.insert(String::from("Thai_sarauee"), xkb::KEY_Thai_sarauee); // 0x0dd7
        hash.insert(String::from("Thai_sarau"), xkb::KEY_Thai_sarau); // 0x0dd8
        hash.insert(String::from("Thai_sarauu"), xkb::KEY_Thai_sarauu); // 0x0dd9
        hash.insert(String::from("Thai_phinthu"), xkb::KEY_Thai_phinthu); // 0x0dda
        hash.insert(
            String::from("Thai_maihanakat_maitho"),
            xkb::KEY_Thai_maihanakat_maitho,
        ); // 0x0dde
        hash.insert(String::from("Thai_baht"), xkb::KEY_Thai_baht); // 0x0ddf
        hash.insert(String::from("Thai_sarae"), xkb::KEY_Thai_sarae); // 0x0de0
        hash.insert(String::from("Thai_saraae"), xkb::KEY_Thai_saraae); // 0x0de1
        hash.insert(String::from("Thai_sarao"), xkb::KEY_Thai_sarao); // 0x0de2
        hash.insert(
            String::from("Thai_saraaimaimuan"),
            xkb::KEY_Thai_saraaimaimuan,
        ); // 0x0de3
        hash.insert(
            String::from("Thai_saraaimaimalai"),
            xkb::KEY_Thai_saraaimaimalai,
        ); // 0x0de4
        hash.insert(String::from("Thai_lakkhangyao"), xkb::KEY_Thai_lakkhangyao); // 0x0de5
        hash.insert(String::from("Thai_maiyamok"), xkb::KEY_Thai_maiyamok); // 0x0de6
        hash.insert(String::from("Thai_maitaikhu"), xkb::KEY_Thai_maitaikhu); // 0x0de7
        hash.insert(String::from("Thai_maiek"), xkb::KEY_Thai_maiek); // 0x0de8
        hash.insert(String::from("Thai_maitho"), xkb::KEY_Thai_maitho); // 0x0de9
        hash.insert(String::from("Thai_maitri"), xkb::KEY_Thai_maitri); // 0x0dea
        hash.insert(String::from("Thai_maichattawa"), xkb::KEY_Thai_maichattawa); // 0x0deb
        hash.insert(String::from("Thai_thanthakhat"), xkb::KEY_Thai_thanthakhat); // 0x0dec
        hash.insert(String::from("Thai_nikhahit"), xkb::KEY_Thai_nikhahit); // 0x0ded
        hash.insert(String::from("Thai_leksun"), xkb::KEY_Thai_leksun); // 0x0df0
        hash.insert(String::from("Thai_leknung"), xkb::KEY_Thai_leknung); // 0x0df1
        hash.insert(String::from("Thai_leksong"), xkb::KEY_Thai_leksong); // 0x0df2
        hash.insert(String::from("Thai_leksam"), xkb::KEY_Thai_leksam); // 0x0df3
        hash.insert(String::from("Thai_leksi"), xkb::KEY_Thai_leksi); // 0x0df4
        hash.insert(String::from("Thai_lekha"), xkb::KEY_Thai_lekha); // 0x0df5
        hash.insert(String::from("Thai_lekhok"), xkb::KEY_Thai_lekhok); // 0x0df6
        hash.insert(String::from("Thai_lekchet"), xkb::KEY_Thai_lekchet); // 0x0df7
        hash.insert(String::from("Thai_lekpaet"), xkb::KEY_Thai_lekpaet); // 0x0df8
        hash.insert(String::from("Thai_lekkao"), xkb::KEY_Thai_lekkao); // 0x0df9

        hash.insert(String::from("Hangul"), xkb::KEY_Hangul); // 0xff31
        hash.insert(String::from("Hangul_Start"), xkb::KEY_Hangul_Start); // 0xff32
        hash.insert(String::from("Hangul_End"), xkb::KEY_Hangul_End); // 0xff33
        hash.insert(String::from("Hangul_Hanja"), xkb::KEY_Hangul_Hanja); // 0xff34
        hash.insert(String::from("Hangul_Jamo"), xkb::KEY_Hangul_Jamo); // 0xff35
        hash.insert(String::from("Hangul_Romaja"), xkb::KEY_Hangul_Romaja); // 0xff36
        hash.insert(String::from("Hangul_Codeinput"), xkb::KEY_Hangul_Codeinput); // 0xff37
        hash.insert(String::from("Hangul_Jeonja"), xkb::KEY_Hangul_Jeonja); // 0xff38
        hash.insert(String::from("Hangul_Banja"), xkb::KEY_Hangul_Banja); // 0xff39
        hash.insert(String::from("Hangul_PreHanja"), xkb::KEY_Hangul_PreHanja); // 0xff3a
        hash.insert(String::from("Hangul_PostHanja"), xkb::KEY_Hangul_PostHanja); // 0xff3b
        hash.insert(
            String::from("Hangul_SingleCandidate"),
            xkb::KEY_Hangul_SingleCandidate,
        ); // 0xff3c
        hash.insert(
            String::from("Hangul_MultipleCandidate"),
            xkb::KEY_Hangul_MultipleCandidate,
        ); // 0xff3d
        hash.insert(
            String::from("Hangul_PreviousCandidate"),
            xkb::KEY_Hangul_PreviousCandidate,
        ); // 0xff3e
        hash.insert(String::from("Hangul_Special"), xkb::KEY_Hangul_Special); // 0xff3f
        hash.insert(String::from("Hangul_switch"), xkb::KEY_Hangul_switch); // 0xff7e
        hash.insert(String::from("Hangul_Kiyeog"), xkb::KEY_Hangul_Kiyeog); // 0x0ea1
        hash.insert(
            String::from("Hangul_SsangKiyeog"),
            xkb::KEY_Hangul_SsangKiyeog,
        ); // 0x0ea2
        hash.insert(
            String::from("Hangul_KiyeogSios"),
            xkb::KEY_Hangul_KiyeogSios,
        ); // 0x0ea3
        hash.insert(String::from("Hangul_Nieun"), xkb::KEY_Hangul_Nieun); // 0x0ea4
        hash.insert(
            String::from("Hangul_NieunJieuj"),
            xkb::KEY_Hangul_NieunJieuj,
        ); // 0x0ea5
        hash.insert(
            String::from("Hangul_NieunHieuh"),
            xkb::KEY_Hangul_NieunHieuh,
        ); // 0x0ea6
        hash.insert(String::from("Hangul_Dikeud"), xkb::KEY_Hangul_Dikeud); // 0x0ea7
        hash.insert(
            String::from("Hangul_SsangDikeud"),
            xkb::KEY_Hangul_SsangDikeud,
        ); // 0x0ea8
        hash.insert(String::from("Hangul_Rieul"), xkb::KEY_Hangul_Rieul); // 0x0ea9
        hash.insert(
            String::from("Hangul_RieulKiyeog"),
            xkb::KEY_Hangul_RieulKiyeog,
        ); // 0x0eaa
        hash.insert(
            String::from("Hangul_RieulMieum"),
            xkb::KEY_Hangul_RieulMieum,
        ); // 0x0eab
        hash.insert(
            String::from("Hangul_RieulPieub"),
            xkb::KEY_Hangul_RieulPieub,
        ); // 0x0eac
        hash.insert(String::from("Hangul_RieulSios"), xkb::KEY_Hangul_RieulSios); // 0x0ead
        hash.insert(
            String::from("Hangul_RieulTieut"),
            xkb::KEY_Hangul_RieulTieut,
        ); // 0x0eae
        hash.insert(
            String::from("Hangul_RieulPhieuf"),
            xkb::KEY_Hangul_RieulPhieuf,
        ); // 0x0eaf
        hash.insert(
            String::from("Hangul_RieulHieuh"),
            xkb::KEY_Hangul_RieulHieuh,
        ); // 0x0eb0
        hash.insert(String::from("Hangul_Mieum"), xkb::KEY_Hangul_Mieum); // 0x0eb1
        hash.insert(String::from("Hangul_Pieub"), xkb::KEY_Hangul_Pieub); // 0x0eb2
        hash.insert(
            String::from("Hangul_SsangPieub"),
            xkb::KEY_Hangul_SsangPieub,
        ); // 0x0eb3
        hash.insert(String::from("Hangul_PieubSios"), xkb::KEY_Hangul_PieubSios); // 0x0eb4
        hash.insert(String::from("Hangul_Sios"), xkb::KEY_Hangul_Sios); // 0x0eb5
        hash.insert(String::from("Hangul_SsangSios"), xkb::KEY_Hangul_SsangSios); // 0x0eb6
        hash.insert(String::from("Hangul_Ieung"), xkb::KEY_Hangul_Ieung); // 0x0eb7
        hash.insert(String::from("Hangul_Jieuj"), xkb::KEY_Hangul_Jieuj); // 0x0eb8
        hash.insert(
            String::from("Hangul_SsangJieuj"),
            xkb::KEY_Hangul_SsangJieuj,
        ); // 0x0eb9
        hash.insert(String::from("Hangul_Cieuc"), xkb::KEY_Hangul_Cieuc); // 0x0eba
        hash.insert(String::from("Hangul_Khieuq"), xkb::KEY_Hangul_Khieuq); // 0x0ebb
        hash.insert(String::from("Hangul_Tieut"), xkb::KEY_Hangul_Tieut); // 0x0ebc
        hash.insert(String::from("Hangul_Phieuf"), xkb::KEY_Hangul_Phieuf); // 0x0ebd
        hash.insert(String::from("Hangul_Hieuh"), xkb::KEY_Hangul_Hieuh); // 0x0ebe
        hash.insert(String::from("Hangul_A"), xkb::KEY_Hangul_A); // 0x0ebf
        hash.insert(String::from("Hangul_AE"), xkb::KEY_Hangul_AE); // 0x0ec0
        hash.insert(String::from("Hangul_YA"), xkb::KEY_Hangul_YA); // 0x0ec1
        hash.insert(String::from("Hangul_YAE"), xkb::KEY_Hangul_YAE); // 0x0ec2
        hash.insert(String::from("Hangul_EO"), xkb::KEY_Hangul_EO); // 0x0ec3
        hash.insert(String::from("Hangul_E"), xkb::KEY_Hangul_E); // 0x0ec4
        hash.insert(String::from("Hangul_YEO"), xkb::KEY_Hangul_YEO); // 0x0ec5
        hash.insert(String::from("Hangul_YE"), xkb::KEY_Hangul_YE); // 0x0ec6
        hash.insert(String::from("Hangul_O"), xkb::KEY_Hangul_O); // 0x0ec7
        hash.insert(String::from("Hangul_WA"), xkb::KEY_Hangul_WA); // 0x0ec8
        hash.insert(String::from("Hangul_WAE"), xkb::KEY_Hangul_WAE); // 0x0ec9
        hash.insert(String::from("Hangul_OE"), xkb::KEY_Hangul_OE); // 0x0eca
        hash.insert(String::from("Hangul_YO"), xkb::KEY_Hangul_YO); // 0x0ecb
        hash.insert(String::from("Hangul_U"), xkb::KEY_Hangul_U); // 0x0ecc
        hash.insert(String::from("Hangul_WEO"), xkb::KEY_Hangul_WEO); // 0x0ecd
        hash.insert(String::from("Hangul_WE"), xkb::KEY_Hangul_WE); // 0x0ece
        hash.insert(String::from("Hangul_WI"), xkb::KEY_Hangul_WI); // 0x0ecf
        hash.insert(String::from("Hangul_YU"), xkb::KEY_Hangul_YU); // 0x0ed0
        hash.insert(String::from("Hangul_EU"), xkb::KEY_Hangul_EU); // 0x0ed1
        hash.insert(String::from("Hangul_YI"), xkb::KEY_Hangul_YI); // 0x0ed2
        hash.insert(String::from("Hangul_I"), xkb::KEY_Hangul_I); // 0x0ed3
        hash.insert(String::from("Hangul_J_Kiyeog"), xkb::KEY_Hangul_J_Kiyeog); // 0x0ed4
        hash.insert(
            String::from("Hangul_J_SsangKiyeog"),
            xkb::KEY_Hangul_J_SsangKiyeog,
        ); // 0x0ed5
        hash.insert(
            String::from("Hangul_J_KiyeogSios"),
            xkb::KEY_Hangul_J_KiyeogSios,
        ); // 0x0ed6
        hash.insert(String::from("Hangul_J_Nieun"), xkb::KEY_Hangul_J_Nieun); // 0x0ed7
        hash.insert(
            String::from("Hangul_J_NieunJieuj"),
            xkb::KEY_Hangul_J_NieunJieuj,
        ); // 0x0ed8
        hash.insert(
            String::from("Hangul_J_NieunHieuh"),
            xkb::KEY_Hangul_J_NieunHieuh,
        ); // 0x0ed9
        hash.insert(String::from("Hangul_J_Dikeud"), xkb::KEY_Hangul_J_Dikeud); // 0x0eda
        hash.insert(String::from("Hangul_J_Rieul"), xkb::KEY_Hangul_J_Rieul); // 0x0edb
        hash.insert(
            String::from("Hangul_J_RieulKiyeog"),
            xkb::KEY_Hangul_J_RieulKiyeog,
        ); // 0x0edc
        hash.insert(
            String::from("Hangul_J_RieulMieum"),
            xkb::KEY_Hangul_J_RieulMieum,
        ); // 0x0edd
        hash.insert(
            String::from("Hangul_J_RieulPieub"),
            xkb::KEY_Hangul_J_RieulPieub,
        ); // 0x0ede
        hash.insert(
            String::from("Hangul_J_RieulSios"),
            xkb::KEY_Hangul_J_RieulSios,
        ); // 0x0edf
        hash.insert(
            String::from("Hangul_J_RieulTieut"),
            xkb::KEY_Hangul_J_RieulTieut,
        ); // 0x0ee0
        hash.insert(
            String::from("Hangul_J_RieulPhieuf"),
            xkb::KEY_Hangul_J_RieulPhieuf,
        ); // 0x0ee1
        hash.insert(
            String::from("Hangul_J_RieulHieuh"),
            xkb::KEY_Hangul_J_RieulHieuh,
        ); // 0x0ee2
        hash.insert(String::from("Hangul_J_Mieum"), xkb::KEY_Hangul_J_Mieum); // 0x0ee3
        hash.insert(String::from("Hangul_J_Pieub"), xkb::KEY_Hangul_J_Pieub); // 0x0ee4
        hash.insert(
            String::from("Hangul_J_PieubSios"),
            xkb::KEY_Hangul_J_PieubSios,
        ); // 0x0ee5
        hash.insert(String::from("Hangul_J_Sios"), xkb::KEY_Hangul_J_Sios); // 0x0ee6
        hash.insert(
            String::from("Hangul_J_SsangSios"),
            xkb::KEY_Hangul_J_SsangSios,
        ); // 0x0ee7
        hash.insert(String::from("Hangul_J_Ieung"), xkb::KEY_Hangul_J_Ieung); // 0x0ee8
        hash.insert(String::from("Hangul_J_Jieuj"), xkb::KEY_Hangul_J_Jieuj); // 0x0ee9
        hash.insert(String::from("Hangul_J_Cieuc"), xkb::KEY_Hangul_J_Cieuc); // 0x0eea
        hash.insert(String::from("Hangul_J_Khieuq"), xkb::KEY_Hangul_J_Khieuq); // 0x0eeb
        hash.insert(String::from("Hangul_J_Tieut"), xkb::KEY_Hangul_J_Tieut); // 0x0eec
        hash.insert(String::from("Hangul_J_Phieuf"), xkb::KEY_Hangul_J_Phieuf); // 0x0eed
        hash.insert(String::from("Hangul_J_Hieuh"), xkb::KEY_Hangul_J_Hieuh); // 0x0eee
        hash.insert(
            String::from("Hangul_RieulYeorinHieuh"),
            xkb::KEY_Hangul_RieulYeorinHieuh,
        ); // 0x0eef
        hash.insert(
            String::from("Hangul_SunkyeongeumMieum"),
            xkb::KEY_Hangul_SunkyeongeumMieum,
        ); // 0x0ef0
        hash.insert(
            String::from("Hangul_SunkyeongeumPieub"),
            xkb::KEY_Hangul_SunkyeongeumPieub,
        ); // 0x0ef1
        hash.insert(String::from("Hangul_PanSios"), xkb::KEY_Hangul_PanSios); // 0x0ef2
        hash.insert(
            String::from("Hangul_KkogjiDalrinIeung"),
            xkb::KEY_Hangul_KkogjiDalrinIeung,
        ); // 0x0ef3
        hash.insert(
            String::from("Hangul_SunkyeongeumPhieuf"),
            xkb::KEY_Hangul_SunkyeongeumPhieuf,
        ); // 0x0ef4
        hash.insert(
            String::from("Hangul_YeorinHieuh"),
            xkb::KEY_Hangul_YeorinHieuh,
        ); // 0x0ef5
        hash.insert(String::from("Hangul_AraeA"), xkb::KEY_Hangul_AraeA); // 0x0ef6
        hash.insert(String::from("Hangul_AraeAE"), xkb::KEY_Hangul_AraeAE); // 0x0ef7
        hash.insert(String::from("Hangul_J_PanSios"), xkb::KEY_Hangul_J_PanSios); // 0x0ef8
        hash.insert(
            String::from("Hangul_J_KkogjiDalrinIeung"),
            xkb::KEY_Hangul_J_KkogjiDalrinIeung,
        ); // 0x0ef9
        hash.insert(
            String::from("Hangul_J_YeorinHieuh"),
            xkb::KEY_Hangul_J_YeorinHieuh,
        ); // 0x0efa
        hash.insert(String::from("Korean_Won"), xkb::KEY_Korean_Won); // 0x0eff

        hash.insert(
            String::from("Armenian_ligature_ew"),
            xkb::KEY_Armenian_ligature_ew,
        ); // 0x1000587
        hash.insert(
            String::from("Armenian_full_stop"),
            xkb::KEY_Armenian_full_stop,
        ); // 0x1000589
        hash.insert(
            String::from("Armenian_verjaket"),
            xkb::KEY_Armenian_verjaket,
        ); // 0x1000589
        hash.insert(
            String::from("Armenian_separation_mark"),
            xkb::KEY_Armenian_separation_mark,
        ); // 0x100055d
        hash.insert(String::from("Armenian_but"), xkb::KEY_Armenian_but); // 0x100055d
        hash.insert(String::from("Armenian_hyphen"), xkb::KEY_Armenian_hyphen); // 0x100058a
        hash.insert(
            String::from("Armenian_yentamna"),
            xkb::KEY_Armenian_yentamna,
        ); // 0x100058a
        hash.insert(String::from("Armenian_exclam"), xkb::KEY_Armenian_exclam); // 0x100055c
        hash.insert(String::from("Armenian_amanak"), xkb::KEY_Armenian_amanak); // 0x100055c
        hash.insert(String::from("Armenian_accent"), xkb::KEY_Armenian_accent); // 0x100055b
        hash.insert(String::from("Armenian_shesht"), xkb::KEY_Armenian_shesht); // 0x100055b
        hash.insert(
            String::from("Armenian_question"),
            xkb::KEY_Armenian_question,
        ); // 0x100055e
        hash.insert(String::from("Armenian_paruyk"), xkb::KEY_Armenian_paruyk); // 0x100055e
        hash.insert(String::from("Armenian_AYB"), xkb::KEY_Armenian_AYB); // 0x1000531
        hash.insert(String::from("Armenian_ayb"), xkb::KEY_Armenian_ayb); // 0x1000561
        hash.insert(String::from("Armenian_BEN"), xkb::KEY_Armenian_BEN); // 0x1000532
        hash.insert(String::from("Armenian_ben"), xkb::KEY_Armenian_ben); // 0x1000562
        hash.insert(String::from("Armenian_GIM"), xkb::KEY_Armenian_GIM); // 0x1000533
        hash.insert(String::from("Armenian_gim"), xkb::KEY_Armenian_gim); // 0x1000563
        hash.insert(String::from("Armenian_DA"), xkb::KEY_Armenian_DA); // 0x1000534
        hash.insert(String::from("Armenian_da"), xkb::KEY_Armenian_da); // 0x1000564
        hash.insert(String::from("Armenian_YECH"), xkb::KEY_Armenian_YECH); // 0x1000535
        hash.insert(String::from("Armenian_yech"), xkb::KEY_Armenian_yech); // 0x1000565
        hash.insert(String::from("Armenian_ZA"), xkb::KEY_Armenian_ZA); // 0x1000536
        hash.insert(String::from("Armenian_za"), xkb::KEY_Armenian_za); // 0x1000566
        hash.insert(String::from("Armenian_E"), xkb::KEY_Armenian_E); // 0x1000537
        hash.insert(String::from("Armenian_e"), xkb::KEY_Armenian_e); // 0x1000567
        hash.insert(String::from("Armenian_AT"), xkb::KEY_Armenian_AT); // 0x1000538
        hash.insert(String::from("Armenian_at"), xkb::KEY_Armenian_at); // 0x1000568
        hash.insert(String::from("Armenian_TO"), xkb::KEY_Armenian_TO); // 0x1000539
        hash.insert(String::from("Armenian_to"), xkb::KEY_Armenian_to); // 0x1000569
        hash.insert(String::from("Armenian_ZHE"), xkb::KEY_Armenian_ZHE); // 0x100053a
        hash.insert(String::from("Armenian_zhe"), xkb::KEY_Armenian_zhe); // 0x100056a
        hash.insert(String::from("Armenian_INI"), xkb::KEY_Armenian_INI); // 0x100053b
        hash.insert(String::from("Armenian_ini"), xkb::KEY_Armenian_ini); // 0x100056b
        hash.insert(String::from("Armenian_LYUN"), xkb::KEY_Armenian_LYUN); // 0x100053c
        hash.insert(String::from("Armenian_lyun"), xkb::KEY_Armenian_lyun); // 0x100056c
        hash.insert(String::from("Armenian_KHE"), xkb::KEY_Armenian_KHE); // 0x100053d
        hash.insert(String::from("Armenian_khe"), xkb::KEY_Armenian_khe); // 0x100056d
        hash.insert(String::from("Armenian_TSA"), xkb::KEY_Armenian_TSA); // 0x100053e
        hash.insert(String::from("Armenian_tsa"), xkb::KEY_Armenian_tsa); // 0x100056e
        hash.insert(String::from("Armenian_KEN"), xkb::KEY_Armenian_KEN); // 0x100053f
        hash.insert(String::from("Armenian_ken"), xkb::KEY_Armenian_ken); // 0x100056f
        hash.insert(String::from("Armenian_HO"), xkb::KEY_Armenian_HO); // 0x1000540
        hash.insert(String::from("Armenian_ho"), xkb::KEY_Armenian_ho); // 0x1000570
        hash.insert(String::from("Armenian_DZA"), xkb::KEY_Armenian_DZA); // 0x1000541
        hash.insert(String::from("Armenian_dza"), xkb::KEY_Armenian_dza); // 0x1000571
        hash.insert(String::from("Armenian_GHAT"), xkb::KEY_Armenian_GHAT); // 0x1000542
        hash.insert(String::from("Armenian_ghat"), xkb::KEY_Armenian_ghat); // 0x1000572
        hash.insert(String::from("Armenian_TCHE"), xkb::KEY_Armenian_TCHE); // 0x1000543
        hash.insert(String::from("Armenian_tche"), xkb::KEY_Armenian_tche); // 0x1000573
        hash.insert(String::from("Armenian_MEN"), xkb::KEY_Armenian_MEN); // 0x1000544
        hash.insert(String::from("Armenian_men"), xkb::KEY_Armenian_men); // 0x1000574
        hash.insert(String::from("Armenian_HI"), xkb::KEY_Armenian_HI); // 0x1000545
        hash.insert(String::from("Armenian_hi"), xkb::KEY_Armenian_hi); // 0x1000575
        hash.insert(String::from("Armenian_NU"), xkb::KEY_Armenian_NU); // 0x1000546
        hash.insert(String::from("Armenian_nu"), xkb::KEY_Armenian_nu); // 0x1000576
        hash.insert(String::from("Armenian_SHA"), xkb::KEY_Armenian_SHA); // 0x1000547
        hash.insert(String::from("Armenian_sha"), xkb::KEY_Armenian_sha); // 0x1000577
        hash.insert(String::from("Armenian_VO"), xkb::KEY_Armenian_VO); // 0x1000548
        hash.insert(String::from("Armenian_vo"), xkb::KEY_Armenian_vo); // 0x1000578
        hash.insert(String::from("Armenian_CHA"), xkb::KEY_Armenian_CHA); // 0x1000549
        hash.insert(String::from("Armenian_cha"), xkb::KEY_Armenian_cha); // 0x1000579
        hash.insert(String::from("Armenian_PE"), xkb::KEY_Armenian_PE); // 0x100054a
        hash.insert(String::from("Armenian_pe"), xkb::KEY_Armenian_pe); // 0x100057a
        hash.insert(String::from("Armenian_JE"), xkb::KEY_Armenian_JE); // 0x100054b
        hash.insert(String::from("Armenian_je"), xkb::KEY_Armenian_je); // 0x100057b
        hash.insert(String::from("Armenian_RA"), xkb::KEY_Armenian_RA); // 0x100054c
        hash.insert(String::from("Armenian_ra"), xkb::KEY_Armenian_ra); // 0x100057c
        hash.insert(String::from("Armenian_SE"), xkb::KEY_Armenian_SE); // 0x100054d
        hash.insert(String::from("Armenian_se"), xkb::KEY_Armenian_se); // 0x100057d
        hash.insert(String::from("Armenian_VEV"), xkb::KEY_Armenian_VEV); // 0x100054e
        hash.insert(String::from("Armenian_vev"), xkb::KEY_Armenian_vev); // 0x100057e
        hash.insert(String::from("Armenian_TYUN"), xkb::KEY_Armenian_TYUN); // 0x100054f
        hash.insert(String::from("Armenian_tyun"), xkb::KEY_Armenian_tyun); // 0x100057f
        hash.insert(String::from("Armenian_RE"), xkb::KEY_Armenian_RE); // 0x1000550
        hash.insert(String::from("Armenian_re"), xkb::KEY_Armenian_re); // 0x1000580
        hash.insert(String::from("Armenian_TSO"), xkb::KEY_Armenian_TSO); // 0x1000551
        hash.insert(String::from("Armenian_tso"), xkb::KEY_Armenian_tso); // 0x1000581
        hash.insert(String::from("Armenian_VYUN"), xkb::KEY_Armenian_VYUN); // 0x1000552
        hash.insert(String::from("Armenian_vyun"), xkb::KEY_Armenian_vyun); // 0x1000582
        hash.insert(String::from("Armenian_PYUR"), xkb::KEY_Armenian_PYUR); // 0x1000553
        hash.insert(String::from("Armenian_pyur"), xkb::KEY_Armenian_pyur); // 0x1000583
        hash.insert(String::from("Armenian_KE"), xkb::KEY_Armenian_KE); // 0x1000554
        hash.insert(String::from("Armenian_ke"), xkb::KEY_Armenian_ke); // 0x1000584
        hash.insert(String::from("Armenian_O"), xkb::KEY_Armenian_O); // 0x1000555
        hash.insert(String::from("Armenian_o"), xkb::KEY_Armenian_o); // 0x1000585
        hash.insert(String::from("Armenian_FE"), xkb::KEY_Armenian_FE); // 0x1000556
        hash.insert(String::from("Armenian_fe"), xkb::KEY_Armenian_fe); // 0x1000586
        hash.insert(
            String::from("Armenian_apostrophe"),
            xkb::KEY_Armenian_apostrophe,
        ); // 0x100055a

        hash.insert(String::from("Georgian_an"), xkb::KEY_Georgian_an); // 0x10010d0
        hash.insert(String::from("Georgian_ban"), xkb::KEY_Georgian_ban); // 0x10010d1
        hash.insert(String::from("Georgian_gan"), xkb::KEY_Georgian_gan); // 0x10010d2
        hash.insert(String::from("Georgian_don"), xkb::KEY_Georgian_don); // 0x10010d3
        hash.insert(String::from("Georgian_en"), xkb::KEY_Georgian_en); // 0x10010d4
        hash.insert(String::from("Georgian_vin"), xkb::KEY_Georgian_vin); // 0x10010d5
        hash.insert(String::from("Georgian_zen"), xkb::KEY_Georgian_zen); // 0x10010d6
        hash.insert(String::from("Georgian_tan"), xkb::KEY_Georgian_tan); // 0x10010d7
        hash.insert(String::from("Georgian_in"), xkb::KEY_Georgian_in); // 0x10010d8
        hash.insert(String::from("Georgian_kan"), xkb::KEY_Georgian_kan); // 0x10010d9
        hash.insert(String::from("Georgian_las"), xkb::KEY_Georgian_las); // 0x10010da
        hash.insert(String::from("Georgian_man"), xkb::KEY_Georgian_man); // 0x10010db
        hash.insert(String::from("Georgian_nar"), xkb::KEY_Georgian_nar); // 0x10010dc
        hash.insert(String::from("Georgian_on"), xkb::KEY_Georgian_on); // 0x10010dd
        hash.insert(String::from("Georgian_par"), xkb::KEY_Georgian_par); // 0x10010de
        hash.insert(String::from("Georgian_zhar"), xkb::KEY_Georgian_zhar); // 0x10010df
        hash.insert(String::from("Georgian_rae"), xkb::KEY_Georgian_rae); // 0x10010e0
        hash.insert(String::from("Georgian_san"), xkb::KEY_Georgian_san); // 0x10010e1
        hash.insert(String::from("Georgian_tar"), xkb::KEY_Georgian_tar); // 0x10010e2
        hash.insert(String::from("Georgian_un"), xkb::KEY_Georgian_un); // 0x10010e3
        hash.insert(String::from("Georgian_phar"), xkb::KEY_Georgian_phar); // 0x10010e4
        hash.insert(String::from("Georgian_khar"), xkb::KEY_Georgian_khar); // 0x10010e5
        hash.insert(String::from("Georgian_ghan"), xkb::KEY_Georgian_ghan); // 0x10010e6
        hash.insert(String::from("Georgian_qar"), xkb::KEY_Georgian_qar); // 0x10010e7
        hash.insert(String::from("Georgian_shin"), xkb::KEY_Georgian_shin); // 0x10010e8
        hash.insert(String::from("Georgian_chin"), xkb::KEY_Georgian_chin); // 0x10010e9
        hash.insert(String::from("Georgian_can"), xkb::KEY_Georgian_can); // 0x10010ea
        hash.insert(String::from("Georgian_jil"), xkb::KEY_Georgian_jil); // 0x10010eb
        hash.insert(String::from("Georgian_cil"), xkb::KEY_Georgian_cil); // 0x10010ec
        hash.insert(String::from("Georgian_char"), xkb::KEY_Georgian_char); // 0x10010ed
        hash.insert(String::from("Georgian_xan"), xkb::KEY_Georgian_xan); // 0x10010ee
        hash.insert(String::from("Georgian_jhan"), xkb::KEY_Georgian_jhan); // 0x10010ef
        hash.insert(String::from("Georgian_hae"), xkb::KEY_Georgian_hae); // 0x10010f0
        hash.insert(String::from("Georgian_he"), xkb::KEY_Georgian_he); // 0x10010f1
        hash.insert(String::from("Georgian_hie"), xkb::KEY_Georgian_hie); // 0x10010f2
        hash.insert(String::from("Georgian_we"), xkb::KEY_Georgian_we); // 0x10010f3
        hash.insert(String::from("Georgian_har"), xkb::KEY_Georgian_har); // 0x10010f4
        hash.insert(String::from("Georgian_hoe"), xkb::KEY_Georgian_hoe); // 0x10010f5
        hash.insert(String::from("Georgian_fi"), xkb::KEY_Georgian_fi); // 0x10010f6

        hash.insert(String::from("Xabovedot"), xkb::KEY_Xabovedot); // 0x1001e8a
        hash.insert(String::from("Ibreve"), xkb::KEY_Ibreve); // 0x100012c
        hash.insert(String::from("Zstroke"), xkb::KEY_Zstroke); // 0x10001b5
        hash.insert(String::from("Gcaron"), xkb::KEY_Gcaron); // 0x10001e6
        hash.insert(String::from("Ocaron"), xkb::KEY_Ocaron); // 0x10001d1
        hash.insert(String::from("Obarred"), xkb::KEY_Obarred); // 0x100019f
        hash.insert(String::from("xabovedot"), xkb::KEY_xabovedot); // 0x1001e8b
        hash.insert(String::from("ibreve"), xkb::KEY_ibreve); // 0x100012d
        hash.insert(String::from("zstroke"), xkb::KEY_zstroke); // 0x10001b6
        hash.insert(String::from("gcaron"), xkb::KEY_gcaron); // 0x10001e7
        hash.insert(String::from("ocaron"), xkb::KEY_ocaron); // 0x10001d2
        hash.insert(String::from("obarred"), xkb::KEY_obarred); // 0x1000275
        hash.insert(String::from("SCHWA"), xkb::KEY_SCHWA); // 0x100018f
        hash.insert(String::from("schwa"), xkb::KEY_schwa); // 0x1000259
        hash.insert(String::from("EZH"), xkb::KEY_EZH); // 0x10001b7
        hash.insert(String::from("ezh"), xkb::KEY_ezh); // 0x1000292
        hash.insert(String::from("Lbelowdot"), xkb::KEY_Lbelowdot); // 0x1001e36
        hash.insert(String::from("lbelowdot"), xkb::KEY_lbelowdot); // 0x1001e37

        hash.insert(String::from("Abelowdot"), xkb::KEY_Abelowdot); // 0x1001ea0
        hash.insert(String::from("abelowdot"), xkb::KEY_abelowdot); // 0x1001ea1
        hash.insert(String::from("Ahook"), xkb::KEY_Ahook); // 0x1001ea2
        hash.insert(String::from("ahook"), xkb::KEY_ahook); // 0x1001ea3
        hash.insert(String::from("Acircumflexacute"), xkb::KEY_Acircumflexacute); // 0x1001ea4
        hash.insert(String::from("acircumflexacute"), xkb::KEY_acircumflexacute); // 0x1001ea5
        hash.insert(String::from("Acircumflexgrave"), xkb::KEY_Acircumflexgrave); // 0x1001ea6
        hash.insert(String::from("acircumflexgrave"), xkb::KEY_acircumflexgrave); // 0x1001ea7
        hash.insert(String::from("Acircumflexhook"), xkb::KEY_Acircumflexhook); // 0x1001ea8
        hash.insert(String::from("acircumflexhook"), xkb::KEY_acircumflexhook); // 0x1001ea9
        hash.insert(String::from("Acircumflextilde"), xkb::KEY_Acircumflextilde); // 0x1001eaa
        hash.insert(String::from("acircumflextilde"), xkb::KEY_acircumflextilde); // 0x1001eab
        hash.insert(
            String::from("Acircumflexbelowdot"),
            xkb::KEY_Acircumflexbelowdot,
        ); // 0x1001eac
        hash.insert(
            String::from("acircumflexbelowdot"),
            xkb::KEY_acircumflexbelowdot,
        ); // 0x1001ead
        hash.insert(String::from("Abreveacute"), xkb::KEY_Abreveacute); // 0x1001eae
        hash.insert(String::from("abreveacute"), xkb::KEY_abreveacute); // 0x1001eaf
        hash.insert(String::from("Abrevegrave"), xkb::KEY_Abrevegrave); // 0x1001eb0
        hash.insert(String::from("abrevegrave"), xkb::KEY_abrevegrave); // 0x1001eb1
        hash.insert(String::from("Abrevehook"), xkb::KEY_Abrevehook); // 0x1001eb2
        hash.insert(String::from("abrevehook"), xkb::KEY_abrevehook); // 0x1001eb3
        hash.insert(String::from("Abrevetilde"), xkb::KEY_Abrevetilde); // 0x1001eb4
        hash.insert(String::from("abrevetilde"), xkb::KEY_abrevetilde); // 0x1001eb5
        hash.insert(String::from("Abrevebelowdot"), xkb::KEY_Abrevebelowdot); // 0x1001eb6
        hash.insert(String::from("abrevebelowdot"), xkb::KEY_abrevebelowdot); // 0x1001eb7
        hash.insert(String::from("Ebelowdot"), xkb::KEY_Ebelowdot); // 0x1001eb8
        hash.insert(String::from("ebelowdot"), xkb::KEY_ebelowdot); // 0x1001eb9
        hash.insert(String::from("Ehook"), xkb::KEY_Ehook); // 0x1001eba
        hash.insert(String::from("ehook"), xkb::KEY_ehook); // 0x1001ebb
        hash.insert(String::from("Etilde"), xkb::KEY_Etilde); // 0x1001ebc
        hash.insert(String::from("etilde"), xkb::KEY_etilde); // 0x1001ebd
        hash.insert(String::from("Ecircumflexacute"), xkb::KEY_Ecircumflexacute); // 0x1001ebe
        hash.insert(String::from("ecircumflexacute"), xkb::KEY_ecircumflexacute); // 0x1001ebf
        hash.insert(String::from("Ecircumflexgrave"), xkb::KEY_Ecircumflexgrave); // 0x1001ec0
        hash.insert(String::from("ecircumflexgrave"), xkb::KEY_ecircumflexgrave); // 0x1001ec1
        hash.insert(String::from("Ecircumflexhook"), xkb::KEY_Ecircumflexhook); // 0x1001ec2
        hash.insert(String::from("ecircumflexhook"), xkb::KEY_ecircumflexhook); // 0x1001ec3
        hash.insert(String::from("Ecircumflextilde"), xkb::KEY_Ecircumflextilde); // 0x1001ec4
        hash.insert(String::from("ecircumflextilde"), xkb::KEY_ecircumflextilde); // 0x1001ec5
        hash.insert(
            String::from("Ecircumflexbelowdot"),
            xkb::KEY_Ecircumflexbelowdot,
        ); // 0x1001ec6
        hash.insert(
            String::from("ecircumflexbelowdot"),
            xkb::KEY_ecircumflexbelowdot,
        ); // 0x1001ec7
        hash.insert(String::from("Ihook"), xkb::KEY_Ihook); // 0x1001ec8
        hash.insert(String::from("ihook"), xkb::KEY_ihook); // 0x1001ec9
        hash.insert(String::from("Ibelowdot"), xkb::KEY_Ibelowdot); // 0x1001eca
        hash.insert(String::from("ibelowdot"), xkb::KEY_ibelowdot); // 0x1001ecb
        hash.insert(String::from("Obelowdot"), xkb::KEY_Obelowdot); // 0x1001ecc
        hash.insert(String::from("obelowdot"), xkb::KEY_obelowdot); // 0x1001ecd
        hash.insert(String::from("Ohook"), xkb::KEY_Ohook); // 0x1001ece
        hash.insert(String::from("ohook"), xkb::KEY_ohook); // 0x1001ecf
        hash.insert(String::from("Ocircumflexacute"), xkb::KEY_Ocircumflexacute); // 0x1001ed0
        hash.insert(String::from("ocircumflexacute"), xkb::KEY_ocircumflexacute); // 0x1001ed1
        hash.insert(String::from("Ocircumflexgrave"), xkb::KEY_Ocircumflexgrave); // 0x1001ed2
        hash.insert(String::from("ocircumflexgrave"), xkb::KEY_ocircumflexgrave); // 0x1001ed3
        hash.insert(String::from("Ocircumflexhook"), xkb::KEY_Ocircumflexhook); // 0x1001ed4
        hash.insert(String::from("ocircumflexhook"), xkb::KEY_ocircumflexhook); // 0x1001ed5
        hash.insert(String::from("Ocircumflextilde"), xkb::KEY_Ocircumflextilde); // 0x1001ed6
        hash.insert(String::from("ocircumflextilde"), xkb::KEY_ocircumflextilde); // 0x1001ed7
        hash.insert(
            String::from("Ocircumflexbelowdot"),
            xkb::KEY_Ocircumflexbelowdot,
        ); // 0x1001ed8
        hash.insert(
            String::from("ocircumflexbelowdot"),
            xkb::KEY_ocircumflexbelowdot,
        ); // 0x1001ed9
        hash.insert(String::from("Ohornacute"), xkb::KEY_Ohornacute); // 0x1001eda
        hash.insert(String::from("ohornacute"), xkb::KEY_ohornacute); // 0x1001edb
        hash.insert(String::from("Ohorngrave"), xkb::KEY_Ohorngrave); // 0x1001edc
        hash.insert(String::from("ohorngrave"), xkb::KEY_ohorngrave); // 0x1001edd
        hash.insert(String::from("Ohornhook"), xkb::KEY_Ohornhook); // 0x1001ede
        hash.insert(String::from("ohornhook"), xkb::KEY_ohornhook); // 0x1001edf
        hash.insert(String::from("Ohorntilde"), xkb::KEY_Ohorntilde); // 0x1001ee0
        hash.insert(String::from("ohorntilde"), xkb::KEY_ohorntilde); // 0x1001ee1
        hash.insert(String::from("Ohornbelowdot"), xkb::KEY_Ohornbelowdot); // 0x1001ee2
        hash.insert(String::from("ohornbelowdot"), xkb::KEY_ohornbelowdot); // 0x1001ee3
        hash.insert(String::from("Ubelowdot"), xkb::KEY_Ubelowdot); // 0x1001ee4
        hash.insert(String::from("ubelowdot"), xkb::KEY_ubelowdot); // 0x1001ee5
        hash.insert(String::from("Uhook"), xkb::KEY_Uhook); // 0x1001ee6
        hash.insert(String::from("uhook"), xkb::KEY_uhook); // 0x1001ee7
        hash.insert(String::from("Uhornacute"), xkb::KEY_Uhornacute); // 0x1001ee8
        hash.insert(String::from("uhornacute"), xkb::KEY_uhornacute); // 0x1001ee9
        hash.insert(String::from("Uhorngrave"), xkb::KEY_Uhorngrave); // 0x1001eea
        hash.insert(String::from("uhorngrave"), xkb::KEY_uhorngrave); // 0x1001eeb
        hash.insert(String::from("Uhornhook"), xkb::KEY_Uhornhook); // 0x1001eec
        hash.insert(String::from("uhornhook"), xkb::KEY_uhornhook); // 0x1001eed
        hash.insert(String::from("Uhorntilde"), xkb::KEY_Uhorntilde); // 0x1001eee
        hash.insert(String::from("uhorntilde"), xkb::KEY_uhorntilde); // 0x1001eef
        hash.insert(String::from("Uhornbelowdot"), xkb::KEY_Uhornbelowdot); // 0x1001ef0
        hash.insert(String::from("uhornbelowdot"), xkb::KEY_uhornbelowdot); // 0x1001ef1
        hash.insert(String::from("Ybelowdot"), xkb::KEY_Ybelowdot); // 0x1001ef4
        hash.insert(String::from("ybelowdot"), xkb::KEY_ybelowdot); // 0x1001ef5
        hash.insert(String::from("Yhook"), xkb::KEY_Yhook); // 0x1001ef6
        hash.insert(String::from("yhook"), xkb::KEY_yhook); // 0x1001ef7
        hash.insert(String::from("Ytilde"), xkb::KEY_Ytilde); // 0x1001ef8
        hash.insert(String::from("ytilde"), xkb::KEY_ytilde); // 0x1001ef9
        hash.insert(String::from("Ohorn"), xkb::KEY_Ohorn); // 0x10001a0
        hash.insert(String::from("ohorn"), xkb::KEY_ohorn); // 0x10001a1
        hash.insert(String::from("Uhorn"), xkb::KEY_Uhorn); // 0x10001af
        hash.insert(String::from("uhorn"), xkb::KEY_uhorn); // 0x10001b0

        hash.insert(String::from("EcuSign"), xkb::KEY_EcuSign); // 0x10020a0
        hash.insert(String::from("ColonSign"), xkb::KEY_ColonSign); // 0x10020a1
        hash.insert(String::from("CruzeiroSign"), xkb::KEY_CruzeiroSign); // 0x10020a2
        hash.insert(String::from("FFrancSign"), xkb::KEY_FFrancSign); // 0x10020a3
        hash.insert(String::from("LiraSign"), xkb::KEY_LiraSign); // 0x10020a4
        hash.insert(String::from("MillSign"), xkb::KEY_MillSign); // 0x10020a5
        hash.insert(String::from("NairaSign"), xkb::KEY_NairaSign); // 0x10020a6
        hash.insert(String::from("PesetaSign"), xkb::KEY_PesetaSign); // 0x10020a7
        hash.insert(String::from("RupeeSign"), xkb::KEY_RupeeSign); // 0x10020a8
        hash.insert(String::from("WonSign"), xkb::KEY_WonSign); // 0x10020a9
        hash.insert(String::from("NewSheqelSign"), xkb::KEY_NewSheqelSign); // 0x10020aa
        hash.insert(String::from("DongSign"), xkb::KEY_DongSign); // 0x10020ab
        hash.insert(String::from("EuroSign"), xkb::KEY_EuroSign); // 0x20ac

        hash.insert(String::from("zerosuperior"), xkb::KEY_zerosuperior); // 0x1002070
        hash.insert(String::from("foursuperior"), xkb::KEY_foursuperior); // 0x1002074
        hash.insert(String::from("fivesuperior"), xkb::KEY_fivesuperior); // 0x1002075
        hash.insert(String::from("sixsuperior"), xkb::KEY_sixsuperior); // 0x1002076
        hash.insert(String::from("sevensuperior"), xkb::KEY_sevensuperior); // 0x1002077
        hash.insert(String::from("eightsuperior"), xkb::KEY_eightsuperior); // 0x1002078
        hash.insert(String::from("ninesuperior"), xkb::KEY_ninesuperior); // 0x1002079
        hash.insert(String::from("zerosubscript"), xkb::KEY_zerosubscript); // 0x1002080
        hash.insert(String::from("onesubscript"), xkb::KEY_onesubscript); // 0x1002081
        hash.insert(String::from("twosubscript"), xkb::KEY_twosubscript); // 0x1002082
        hash.insert(String::from("threesubscript"), xkb::KEY_threesubscript); // 0x1002083
        hash.insert(String::from("foursubscript"), xkb::KEY_foursubscript); // 0x1002084
        hash.insert(String::from("fivesubscript"), xkb::KEY_fivesubscript); // 0x1002085
        hash.insert(String::from("sixsubscript"), xkb::KEY_sixsubscript); // 0x1002086
        hash.insert(String::from("sevensubscript"), xkb::KEY_sevensubscript); // 0x1002087
        hash.insert(String::from("eightsubscript"), xkb::KEY_eightsubscript); // 0x1002088
        hash.insert(String::from("ninesubscript"), xkb::KEY_ninesubscript); // 0x1002089
        hash.insert(String::from("partdifferential"), xkb::KEY_partdifferential); // 0x1002202
        hash.insert(String::from("emptyset"), xkb::KEY_emptyset); // 0x1002205
        hash.insert(String::from("elementof"), xkb::KEY_elementof); // 0x1002208
        hash.insert(String::from("notelementof"), xkb::KEY_notelementof); // 0x1002209
        hash.insert(String::from("containsas"), xkb::KEY_containsas); // 0x100220B
        hash.insert(String::from("squareroot"), xkb::KEY_squareroot); // 0x100221A
        hash.insert(String::from("cuberoot"), xkb::KEY_cuberoot); // 0x100221B
        hash.insert(String::from("fourthroot"), xkb::KEY_fourthroot); // 0x100221C
        hash.insert(String::from("dintegral"), xkb::KEY_dintegral); // 0x100222C
        hash.insert(String::from("tintegral"), xkb::KEY_tintegral); // 0x100222D
        hash.insert(String::from("because"), xkb::KEY_because); // 0x1002235
        hash.insert(String::from("approxeq"), xkb::KEY_approxeq); // 0x1002248
        hash.insert(String::from("notapproxeq"), xkb::KEY_notapproxeq); // 0x1002247
        hash.insert(String::from("notidentical"), xkb::KEY_notidentical); // 0x1002262
        hash.insert(String::from("stricteq"), xkb::KEY_stricteq); // 0x1002263

        hash.insert(String::from("braille_dot_1"), xkb::KEY_braille_dot_1); // 0xfff1
        hash.insert(String::from("braille_dot_2"), xkb::KEY_braille_dot_2); // 0xfff2
        hash.insert(String::from("braille_dot_3"), xkb::KEY_braille_dot_3); // 0xfff3
        hash.insert(String::from("braille_dot_4"), xkb::KEY_braille_dot_4); // 0xfff4
        hash.insert(String::from("braille_dot_5"), xkb::KEY_braille_dot_5); // 0xfff5
        hash.insert(String::from("braille_dot_6"), xkb::KEY_braille_dot_6); // 0xfff6
        hash.insert(String::from("braille_dot_7"), xkb::KEY_braille_dot_7); // 0xfff7
        hash.insert(String::from("braille_dot_8"), xkb::KEY_braille_dot_8); // 0xfff8
        hash.insert(String::from("braille_dot_9"), xkb::KEY_braille_dot_9); // 0xfff9
        hash.insert(String::from("braille_dot_10"), xkb::KEY_braille_dot_10); // 0xfffa
        hash.insert(String::from("braille_blank"), xkb::KEY_braille_blank); // 0x1002800
        hash.insert(String::from("braille_dots_1"), xkb::KEY_braille_dots_1); // 0x1002801
        hash.insert(String::from("braille_dots_2"), xkb::KEY_braille_dots_2); // 0x1002802
        hash.insert(String::from("braille_dots_12"), xkb::KEY_braille_dots_12); // 0x1002803
        hash.insert(String::from("braille_dots_3"), xkb::KEY_braille_dots_3); // 0x1002804
        hash.insert(String::from("braille_dots_13"), xkb::KEY_braille_dots_13); // 0x1002805
        hash.insert(String::from("braille_dots_23"), xkb::KEY_braille_dots_23); // 0x1002806
        hash.insert(String::from("braille_dots_123"), xkb::KEY_braille_dots_123); // 0x1002807
        hash.insert(String::from("braille_dots_4"), xkb::KEY_braille_dots_4); // 0x1002808
        hash.insert(String::from("braille_dots_14"), xkb::KEY_braille_dots_14); // 0x1002809
        hash.insert(String::from("braille_dots_24"), xkb::KEY_braille_dots_24); // 0x100280a
        hash.insert(String::from("braille_dots_124"), xkb::KEY_braille_dots_124); // 0x100280b
        hash.insert(String::from("braille_dots_34"), xkb::KEY_braille_dots_34); // 0x100280c
        hash.insert(String::from("braille_dots_134"), xkb::KEY_braille_dots_134); // 0x100280d
        hash.insert(String::from("braille_dots_234"), xkb::KEY_braille_dots_234); // 0x100280e
        hash.insert(
            String::from("braille_dots_1234"),
            xkb::KEY_braille_dots_1234,
        ); // 0x100280f
        hash.insert(String::from("braille_dots_5"), xkb::KEY_braille_dots_5); // 0x1002810
        hash.insert(String::from("braille_dots_15"), xkb::KEY_braille_dots_15); // 0x1002811
        hash.insert(String::from("braille_dots_25"), xkb::KEY_braille_dots_25); // 0x1002812
        hash.insert(String::from("braille_dots_125"), xkb::KEY_braille_dots_125); // 0x1002813
        hash.insert(String::from("braille_dots_35"), xkb::KEY_braille_dots_35); // 0x1002814
        hash.insert(String::from("braille_dots_135"), xkb::KEY_braille_dots_135); // 0x1002815
        hash.insert(String::from("braille_dots_235"), xkb::KEY_braille_dots_235); // 0x1002816
        hash.insert(
            String::from("braille_dots_1235"),
            xkb::KEY_braille_dots_1235,
        ); // 0x1002817
        hash.insert(String::from("braille_dots_45"), xkb::KEY_braille_dots_45); // 0x1002818
        hash.insert(String::from("braille_dots_145"), xkb::KEY_braille_dots_145); // 0x1002819
        hash.insert(String::from("braille_dots_245"), xkb::KEY_braille_dots_245); // 0x100281a
        hash.insert(
            String::from("braille_dots_1245"),
            xkb::KEY_braille_dots_1245,
        ); // 0x100281b
        hash.insert(String::from("braille_dots_345"), xkb::KEY_braille_dots_345); // 0x100281c
        hash.insert(
            String::from("braille_dots_1345"),
            xkb::KEY_braille_dots_1345,
        ); // 0x100281d
        hash.insert(
            String::from("braille_dots_2345"),
            xkb::KEY_braille_dots_2345,
        ); // 0x100281e
        hash.insert(
            String::from("braille_dots_12345"),
            xkb::KEY_braille_dots_12345,
        ); // 0x100281f
        hash.insert(String::from("braille_dots_6"), xkb::KEY_braille_dots_6); // 0x1002820
        hash.insert(String::from("braille_dots_16"), xkb::KEY_braille_dots_16); // 0x1002821
        hash.insert(String::from("braille_dots_26"), xkb::KEY_braille_dots_26); // 0x1002822
        hash.insert(String::from("braille_dots_126"), xkb::KEY_braille_dots_126); // 0x1002823
        hash.insert(String::from("braille_dots_36"), xkb::KEY_braille_dots_36); // 0x1002824
        hash.insert(String::from("braille_dots_136"), xkb::KEY_braille_dots_136); // 0x1002825
        hash.insert(String::from("braille_dots_236"), xkb::KEY_braille_dots_236); // 0x1002826
        hash.insert(
            String::from("braille_dots_1236"),
            xkb::KEY_braille_dots_1236,
        ); // 0x1002827
        hash.insert(String::from("braille_dots_46"), xkb::KEY_braille_dots_46); // 0x1002828
        hash.insert(String::from("braille_dots_146"), xkb::KEY_braille_dots_146); // 0x1002829
        hash.insert(String::from("braille_dots_246"), xkb::KEY_braille_dots_246); // 0x100282a
        hash.insert(
            String::from("braille_dots_1246"),
            xkb::KEY_braille_dots_1246,
        ); // 0x100282b
        hash.insert(String::from("braille_dots_346"), xkb::KEY_braille_dots_346); // 0x100282c
        hash.insert(
            String::from("braille_dots_1346"),
            xkb::KEY_braille_dots_1346,
        ); // 0x100282d
        hash.insert(
            String::from("braille_dots_2346"),
            xkb::KEY_braille_dots_2346,
        ); // 0x100282e
        hash.insert(
            String::from("braille_dots_12346"),
            xkb::KEY_braille_dots_12346,
        ); // 0x100282f
        hash.insert(String::from("braille_dots_56"), xkb::KEY_braille_dots_56); // 0x1002830
        hash.insert(String::from("braille_dots_156"), xkb::KEY_braille_dots_156); // 0x1002831
        hash.insert(String::from("braille_dots_256"), xkb::KEY_braille_dots_256); // 0x1002832
        hash.insert(
            String::from("braille_dots_1256"),
            xkb::KEY_braille_dots_1256,
        ); // 0x1002833
        hash.insert(String::from("braille_dots_356"), xkb::KEY_braille_dots_356); // 0x1002834
        hash.insert(
            String::from("braille_dots_1356"),
            xkb::KEY_braille_dots_1356,
        ); // 0x1002835
        hash.insert(
            String::from("braille_dots_2356"),
            xkb::KEY_braille_dots_2356,
        ); // 0x1002836
        hash.insert(
            String::from("braille_dots_12356"),
            xkb::KEY_braille_dots_12356,
        ); // 0x1002837
        hash.insert(String::from("braille_dots_456"), xkb::KEY_braille_dots_456); // 0x1002838
        hash.insert(
            String::from("braille_dots_1456"),
            xkb::KEY_braille_dots_1456,
        ); // 0x1002839
        hash.insert(
            String::from("braille_dots_2456"),
            xkb::KEY_braille_dots_2456,
        ); // 0x100283a
        hash.insert(
            String::from("braille_dots_12456"),
            xkb::KEY_braille_dots_12456,
        ); // 0x100283b
        hash.insert(
            String::from("braille_dots_3456"),
            xkb::KEY_braille_dots_3456,
        ); // 0x100283c
        hash.insert(
            String::from("braille_dots_13456"),
            xkb::KEY_braille_dots_13456,
        ); // 0x100283d
        hash.insert(
            String::from("braille_dots_23456"),
            xkb::KEY_braille_dots_23456,
        ); // 0x100283e
        hash.insert(
            String::from("braille_dots_123456"),
            xkb::KEY_braille_dots_123456,
        ); // 0x100283f
        hash.insert(String::from("braille_dots_7"), xkb::KEY_braille_dots_7); // 0x1002840
        hash.insert(String::from("braille_dots_17"), xkb::KEY_braille_dots_17); // 0x1002841
        hash.insert(String::from("braille_dots_27"), xkb::KEY_braille_dots_27); // 0x1002842
        hash.insert(String::from("braille_dots_127"), xkb::KEY_braille_dots_127); // 0x1002843
        hash.insert(String::from("braille_dots_37"), xkb::KEY_braille_dots_37); // 0x1002844
        hash.insert(String::from("braille_dots_137"), xkb::KEY_braille_dots_137); // 0x1002845
        hash.insert(String::from("braille_dots_237"), xkb::KEY_braille_dots_237); // 0x1002846
        hash.insert(
            String::from("braille_dots_1237"),
            xkb::KEY_braille_dots_1237,
        ); // 0x1002847
        hash.insert(String::from("braille_dots_47"), xkb::KEY_braille_dots_47); // 0x1002848
        hash.insert(String::from("braille_dots_147"), xkb::KEY_braille_dots_147); // 0x1002849
        hash.insert(String::from("braille_dots_247"), xkb::KEY_braille_dots_247); // 0x100284a
        hash.insert(
            String::from("braille_dots_1247"),
            xkb::KEY_braille_dots_1247,
        ); // 0x100284b
        hash.insert(String::from("braille_dots_347"), xkb::KEY_braille_dots_347); // 0x100284c
        hash.insert(
            String::from("braille_dots_1347"),
            xkb::KEY_braille_dots_1347,
        ); // 0x100284d
        hash.insert(
            String::from("braille_dots_2347"),
            xkb::KEY_braille_dots_2347,
        ); // 0x100284e
        hash.insert(
            String::from("braille_dots_12347"),
            xkb::KEY_braille_dots_12347,
        ); // 0x100284f
        hash.insert(String::from("braille_dots_57"), xkb::KEY_braille_dots_57); // 0x1002850
        hash.insert(String::from("braille_dots_157"), xkb::KEY_braille_dots_157); // 0x1002851
        hash.insert(String::from("braille_dots_257"), xkb::KEY_braille_dots_257); // 0x1002852
        hash.insert(
            String::from("braille_dots_1257"),
            xkb::KEY_braille_dots_1257,
        ); // 0x1002853
        hash.insert(String::from("braille_dots_357"), xkb::KEY_braille_dots_357); // 0x1002854
        hash.insert(
            String::from("braille_dots_1357"),
            xkb::KEY_braille_dots_1357,
        ); // 0x1002855
        hash.insert(
            String::from("braille_dots_2357"),
            xkb::KEY_braille_dots_2357,
        ); // 0x1002856
        hash.insert(
            String::from("braille_dots_12357"),
            xkb::KEY_braille_dots_12357,
        ); // 0x1002857
        hash.insert(String::from("braille_dots_457"), xkb::KEY_braille_dots_457); // 0x1002858
        hash.insert(
            String::from("braille_dots_1457"),
            xkb::KEY_braille_dots_1457,
        ); // 0x1002859
        hash.insert(
            String::from("braille_dots_2457"),
            xkb::KEY_braille_dots_2457,
        ); // 0x100285a
        hash.insert(
            String::from("braille_dots_12457"),
            xkb::KEY_braille_dots_12457,
        ); // 0x100285b
        hash.insert(
            String::from("braille_dots_3457"),
            xkb::KEY_braille_dots_3457,
        ); // 0x100285c
        hash.insert(
            String::from("braille_dots_13457"),
            xkb::KEY_braille_dots_13457,
        ); // 0x100285d
        hash.insert(
            String::from("braille_dots_23457"),
            xkb::KEY_braille_dots_23457,
        ); // 0x100285e
        hash.insert(
            String::from("braille_dots_123457"),
            xkb::KEY_braille_dots_123457,
        ); // 0x100285f
        hash.insert(String::from("braille_dots_67"), xkb::KEY_braille_dots_67); // 0x1002860
        hash.insert(String::from("braille_dots_167"), xkb::KEY_braille_dots_167); // 0x1002861
        hash.insert(String::from("braille_dots_267"), xkb::KEY_braille_dots_267); // 0x1002862
        hash.insert(
            String::from("braille_dots_1267"),
            xkb::KEY_braille_dots_1267,
        ); // 0x1002863
        hash.insert(String::from("braille_dots_367"), xkb::KEY_braille_dots_367); // 0x1002864
        hash.insert(
            String::from("braille_dots_1367"),
            xkb::KEY_braille_dots_1367,
        ); // 0x1002865
        hash.insert(
            String::from("braille_dots_2367"),
            xkb::KEY_braille_dots_2367,
        ); // 0x1002866
        hash.insert(
            String::from("braille_dots_12367"),
            xkb::KEY_braille_dots_12367,
        ); // 0x1002867
        hash.insert(String::from("braille_dots_467"), xkb::KEY_braille_dots_467); // 0x1002868
        hash.insert(
            String::from("braille_dots_1467"),
            xkb::KEY_braille_dots_1467,
        ); // 0x1002869
        hash.insert(
            String::from("braille_dots_2467"),
            xkb::KEY_braille_dots_2467,
        ); // 0x100286a
        hash.insert(
            String::from("braille_dots_12467"),
            xkb::KEY_braille_dots_12467,
        ); // 0x100286b
        hash.insert(
            String::from("braille_dots_3467"),
            xkb::KEY_braille_dots_3467,
        ); // 0x100286c
        hash.insert(
            String::from("braille_dots_13467"),
            xkb::KEY_braille_dots_13467,
        ); // 0x100286d
        hash.insert(
            String::from("braille_dots_23467"),
            xkb::KEY_braille_dots_23467,
        ); // 0x100286e
        hash.insert(
            String::from("braille_dots_123467"),
            xkb::KEY_braille_dots_123467,
        ); // 0x100286f
        hash.insert(String::from("braille_dots_567"), xkb::KEY_braille_dots_567); // 0x1002870
        hash.insert(
            String::from("braille_dots_1567"),
            xkb::KEY_braille_dots_1567,
        ); // 0x1002871
        hash.insert(
            String::from("braille_dots_2567"),
            xkb::KEY_braille_dots_2567,
        ); // 0x1002872
        hash.insert(
            String::from("braille_dots_12567"),
            xkb::KEY_braille_dots_12567,
        ); // 0x1002873
        hash.insert(
            String::from("braille_dots_3567"),
            xkb::KEY_braille_dots_3567,
        ); // 0x1002874
        hash.insert(
            String::from("braille_dots_13567"),
            xkb::KEY_braille_dots_13567,
        ); // 0x1002875
        hash.insert(
            String::from("braille_dots_23567"),
            xkb::KEY_braille_dots_23567,
        ); // 0x1002876
        hash.insert(
            String::from("braille_dots_123567"),
            xkb::KEY_braille_dots_123567,
        ); // 0x1002877
        hash.insert(
            String::from("braille_dots_4567"),
            xkb::KEY_braille_dots_4567,
        ); // 0x1002878
        hash.insert(
            String::from("braille_dots_14567"),
            xkb::KEY_braille_dots_14567,
        ); // 0x1002879
        hash.insert(
            String::from("braille_dots_24567"),
            xkb::KEY_braille_dots_24567,
        ); // 0x100287a
        hash.insert(
            String::from("braille_dots_124567"),
            xkb::KEY_braille_dots_124567,
        ); // 0x100287b
        hash.insert(
            String::from("braille_dots_34567"),
            xkb::KEY_braille_dots_34567,
        ); // 0x100287c
        hash.insert(
            String::from("braille_dots_134567"),
            xkb::KEY_braille_dots_134567,
        ); // 0x100287d
        hash.insert(
            String::from("braille_dots_234567"),
            xkb::KEY_braille_dots_234567,
        ); // 0x100287e
        hash.insert(
            String::from("braille_dots_1234567"),
            xkb::KEY_braille_dots_1234567,
        ); // 0x100287f
        hash.insert(String::from("braille_dots_8"), xkb::KEY_braille_dots_8); // 0x1002880
        hash.insert(String::from("braille_dots_18"), xkb::KEY_braille_dots_18); // 0x1002881
        hash.insert(String::from("braille_dots_28"), xkb::KEY_braille_dots_28); // 0x1002882
        hash.insert(String::from("braille_dots_128"), xkb::KEY_braille_dots_128); // 0x1002883
        hash.insert(String::from("braille_dots_38"), xkb::KEY_braille_dots_38); // 0x1002884
        hash.insert(String::from("braille_dots_138"), xkb::KEY_braille_dots_138); // 0x1002885
        hash.insert(String::from("braille_dots_238"), xkb::KEY_braille_dots_238); // 0x1002886
        hash.insert(
            String::from("braille_dots_1238"),
            xkb::KEY_braille_dots_1238,
        ); // 0x1002887
        hash.insert(String::from("braille_dots_48"), xkb::KEY_braille_dots_48); // 0x1002888
        hash.insert(String::from("braille_dots_148"), xkb::KEY_braille_dots_148); // 0x1002889
        hash.insert(String::from("braille_dots_248"), xkb::KEY_braille_dots_248); // 0x100288a
        hash.insert(
            String::from("braille_dots_1248"),
            xkb::KEY_braille_dots_1248,
        ); // 0x100288b
        hash.insert(String::from("braille_dots_348"), xkb::KEY_braille_dots_348); // 0x100288c
        hash.insert(
            String::from("braille_dots_1348"),
            xkb::KEY_braille_dots_1348,
        ); // 0x100288d
        hash.insert(
            String::from("braille_dots_2348"),
            xkb::KEY_braille_dots_2348,
        ); // 0x100288e
        hash.insert(
            String::from("braille_dots_12348"),
            xkb::KEY_braille_dots_12348,
        ); // 0x100288f
        hash.insert(String::from("braille_dots_58"), xkb::KEY_braille_dots_58); // 0x1002890
        hash.insert(String::from("braille_dots_158"), xkb::KEY_braille_dots_158); // 0x1002891
        hash.insert(String::from("braille_dots_258"), xkb::KEY_braille_dots_258); // 0x1002892
        hash.insert(
            String::from("braille_dots_1258"),
            xkb::KEY_braille_dots_1258,
        ); // 0x1002893
        hash.insert(String::from("braille_dots_358"), xkb::KEY_braille_dots_358); // 0x1002894
        hash.insert(
            String::from("braille_dots_1358"),
            xkb::KEY_braille_dots_1358,
        ); // 0x1002895
        hash.insert(
            String::from("braille_dots_2358"),
            xkb::KEY_braille_dots_2358,
        ); // 0x1002896
        hash.insert(
            String::from("braille_dots_12358"),
            xkb::KEY_braille_dots_12358,
        ); // 0x1002897
        hash.insert(String::from("braille_dots_458"), xkb::KEY_braille_dots_458); // 0x1002898
        hash.insert(
            String::from("braille_dots_1458"),
            xkb::KEY_braille_dots_1458,
        ); // 0x1002899
        hash.insert(
            String::from("braille_dots_2458"),
            xkb::KEY_braille_dots_2458,
        ); // 0x100289a
        hash.insert(
            String::from("braille_dots_12458"),
            xkb::KEY_braille_dots_12458,
        ); // 0x100289b
        hash.insert(
            String::from("braille_dots_3458"),
            xkb::KEY_braille_dots_3458,
        ); // 0x100289c
        hash.insert(
            String::from("braille_dots_13458"),
            xkb::KEY_braille_dots_13458,
        ); // 0x100289d
        hash.insert(
            String::from("braille_dots_23458"),
            xkb::KEY_braille_dots_23458,
        ); // 0x100289e
        hash.insert(
            String::from("braille_dots_123458"),
            xkb::KEY_braille_dots_123458,
        ); // 0x100289f
        hash.insert(String::from("braille_dots_68"), xkb::KEY_braille_dots_68); // 0x10028a0
        hash.insert(String::from("braille_dots_168"), xkb::KEY_braille_dots_168); // 0x10028a1
        hash.insert(String::from("braille_dots_268"), xkb::KEY_braille_dots_268); // 0x10028a2
        hash.insert(
            String::from("braille_dots_1268"),
            xkb::KEY_braille_dots_1268,
        ); // 0x10028a3
        hash.insert(String::from("braille_dots_368"), xkb::KEY_braille_dots_368); // 0x10028a4
        hash.insert(
            String::from("braille_dots_1368"),
            xkb::KEY_braille_dots_1368,
        ); // 0x10028a5
        hash.insert(
            String::from("braille_dots_2368"),
            xkb::KEY_braille_dots_2368,
        ); // 0x10028a6
        hash.insert(
            String::from("braille_dots_12368"),
            xkb::KEY_braille_dots_12368,
        ); // 0x10028a7
        hash.insert(String::from("braille_dots_468"), xkb::KEY_braille_dots_468); // 0x10028a8
        hash.insert(
            String::from("braille_dots_1468"),
            xkb::KEY_braille_dots_1468,
        ); // 0x10028a9
        hash.insert(
            String::from("braille_dots_2468"),
            xkb::KEY_braille_dots_2468,
        ); // 0x10028aa
        hash.insert(
            String::from("braille_dots_12468"),
            xkb::KEY_braille_dots_12468,
        ); // 0x10028ab
        hash.insert(
            String::from("braille_dots_3468"),
            xkb::KEY_braille_dots_3468,
        ); // 0x10028ac
        hash.insert(
            String::from("braille_dots_13468"),
            xkb::KEY_braille_dots_13468,
        ); // 0x10028ad
        hash.insert(
            String::from("braille_dots_23468"),
            xkb::KEY_braille_dots_23468,
        ); // 0x10028ae
        hash.insert(
            String::from("braille_dots_123468"),
            xkb::KEY_braille_dots_123468,
        ); // 0x10028af
        hash.insert(String::from("braille_dots_568"), xkb::KEY_braille_dots_568); // 0x10028b0
        hash.insert(
            String::from("braille_dots_1568"),
            xkb::KEY_braille_dots_1568,
        ); // 0x10028b1
        hash.insert(
            String::from("braille_dots_2568"),
            xkb::KEY_braille_dots_2568,
        ); // 0x10028b2
        hash.insert(
            String::from("braille_dots_12568"),
            xkb::KEY_braille_dots_12568,
        ); // 0x10028b3
        hash.insert(
            String::from("braille_dots_3568"),
            xkb::KEY_braille_dots_3568,
        ); // 0x10028b4
        hash.insert(
            String::from("braille_dots_13568"),
            xkb::KEY_braille_dots_13568,
        ); // 0x10028b5
        hash.insert(
            String::from("braille_dots_23568"),
            xkb::KEY_braille_dots_23568,
        ); // 0x10028b6
        hash.insert(
            String::from("braille_dots_123568"),
            xkb::KEY_braille_dots_123568,
        ); // 0x10028b7
        hash.insert(
            String::from("braille_dots_4568"),
            xkb::KEY_braille_dots_4568,
        ); // 0x10028b8
        hash.insert(
            String::from("braille_dots_14568"),
            xkb::KEY_braille_dots_14568,
        ); // 0x10028b9
        hash.insert(
            String::from("braille_dots_24568"),
            xkb::KEY_braille_dots_24568,
        ); // 0x10028ba
        hash.insert(
            String::from("braille_dots_124568"),
            xkb::KEY_braille_dots_124568,
        ); // 0x10028bb
        hash.insert(
            String::from("braille_dots_34568"),
            xkb::KEY_braille_dots_34568,
        ); // 0x10028bc
        hash.insert(
            String::from("braille_dots_134568"),
            xkb::KEY_braille_dots_134568,
        ); // 0x10028bd
        hash.insert(
            String::from("braille_dots_234568"),
            xkb::KEY_braille_dots_234568,
        ); // 0x10028be
        hash.insert(
            String::from("braille_dots_1234568"),
            xkb::KEY_braille_dots_1234568,
        ); // 0x10028bf
        hash.insert(String::from("braille_dots_78"), xkb::KEY_braille_dots_78); // 0x10028c0
        hash.insert(String::from("braille_dots_178"), xkb::KEY_braille_dots_178); // 0x10028c1
        hash.insert(String::from("braille_dots_278"), xkb::KEY_braille_dots_278); // 0x10028c2
        hash.insert(
            String::from("braille_dots_1278"),
            xkb::KEY_braille_dots_1278,
        ); // 0x10028c3
        hash.insert(String::from("braille_dots_378"), xkb::KEY_braille_dots_378); // 0x10028c4
        hash.insert(
            String::from("braille_dots_1378"),
            xkb::KEY_braille_dots_1378,
        ); // 0x10028c5
        hash.insert(
            String::from("braille_dots_2378"),
            xkb::KEY_braille_dots_2378,
        ); // 0x10028c6
        hash.insert(
            String::from("braille_dots_12378"),
            xkb::KEY_braille_dots_12378,
        ); // 0x10028c7
        hash.insert(String::from("braille_dots_478"), xkb::KEY_braille_dots_478); // 0x10028c8
        hash.insert(
            String::from("braille_dots_1478"),
            xkb::KEY_braille_dots_1478,
        ); // 0x10028c9
        hash.insert(
            String::from("braille_dots_2478"),
            xkb::KEY_braille_dots_2478,
        ); // 0x10028ca
        hash.insert(
            String::from("braille_dots_12478"),
            xkb::KEY_braille_dots_12478,
        ); // 0x10028cb
        hash.insert(
            String::from("braille_dots_3478"),
            xkb::KEY_braille_dots_3478,
        ); // 0x10028cc
        hash.insert(
            String::from("braille_dots_13478"),
            xkb::KEY_braille_dots_13478,
        ); // 0x10028cd
        hash.insert(
            String::from("braille_dots_23478"),
            xkb::KEY_braille_dots_23478,
        ); // 0x10028ce
        hash.insert(
            String::from("braille_dots_123478"),
            xkb::KEY_braille_dots_123478,
        ); // 0x10028cf
        hash.insert(String::from("braille_dots_578"), xkb::KEY_braille_dots_578); // 0x10028d0
        hash.insert(
            String::from("braille_dots_1578"),
            xkb::KEY_braille_dots_1578,
        ); // 0x10028d1
        hash.insert(
            String::from("braille_dots_2578"),
            xkb::KEY_braille_dots_2578,
        ); // 0x10028d2
        hash.insert(
            String::from("braille_dots_12578"),
            xkb::KEY_braille_dots_12578,
        ); // 0x10028d3
        hash.insert(
            String::from("braille_dots_3578"),
            xkb::KEY_braille_dots_3578,
        ); // 0x10028d4
        hash.insert(
            String::from("braille_dots_13578"),
            xkb::KEY_braille_dots_13578,
        ); // 0x10028d5
        hash.insert(
            String::from("braille_dots_23578"),
            xkb::KEY_braille_dots_23578,
        ); // 0x10028d6
        hash.insert(
            String::from("braille_dots_123578"),
            xkb::KEY_braille_dots_123578,
        ); // 0x10028d7
        hash.insert(
            String::from("braille_dots_4578"),
            xkb::KEY_braille_dots_4578,
        ); // 0x10028d8
        hash.insert(
            String::from("braille_dots_14578"),
            xkb::KEY_braille_dots_14578,
        ); // 0x10028d9
        hash.insert(
            String::from("braille_dots_24578"),
            xkb::KEY_braille_dots_24578,
        ); // 0x10028da
        hash.insert(
            String::from("braille_dots_124578"),
            xkb::KEY_braille_dots_124578,
        ); // 0x10028db
        hash.insert(
            String::from("braille_dots_34578"),
            xkb::KEY_braille_dots_34578,
        ); // 0x10028dc
        hash.insert(
            String::from("braille_dots_134578"),
            xkb::KEY_braille_dots_134578,
        ); // 0x10028dd
        hash.insert(
            String::from("braille_dots_234578"),
            xkb::KEY_braille_dots_234578,
        ); // 0x10028de
        hash.insert(
            String::from("braille_dots_1234578"),
            xkb::KEY_braille_dots_1234578,
        ); // 0x10028df
        hash.insert(String::from("braille_dots_678"), xkb::KEY_braille_dots_678); // 0x10028e0
        hash.insert(
            String::from("braille_dots_1678"),
            xkb::KEY_braille_dots_1678,
        ); // 0x10028e1
        hash.insert(
            String::from("braille_dots_2678"),
            xkb::KEY_braille_dots_2678,
        ); // 0x10028e2
        hash.insert(
            String::from("braille_dots_12678"),
            xkb::KEY_braille_dots_12678,
        ); // 0x10028e3
        hash.insert(
            String::from("braille_dots_3678"),
            xkb::KEY_braille_dots_3678,
        ); // 0x10028e4
        hash.insert(
            String::from("braille_dots_13678"),
            xkb::KEY_braille_dots_13678,
        ); // 0x10028e5
        hash.insert(
            String::from("braille_dots_23678"),
            xkb::KEY_braille_dots_23678,
        ); // 0x10028e6
        hash.insert(
            String::from("braille_dots_123678"),
            xkb::KEY_braille_dots_123678,
        ); // 0x10028e7
        hash.insert(
            String::from("braille_dots_4678"),
            xkb::KEY_braille_dots_4678,
        ); // 0x10028e8
        hash.insert(
            String::from("braille_dots_14678"),
            xkb::KEY_braille_dots_14678,
        ); // 0x10028e9
        hash.insert(
            String::from("braille_dots_24678"),
            xkb::KEY_braille_dots_24678,
        ); // 0x10028ea
        hash.insert(
            String::from("braille_dots_124678"),
            xkb::KEY_braille_dots_124678,
        ); // 0x10028eb
        hash.insert(
            String::from("braille_dots_34678"),
            xkb::KEY_braille_dots_34678,
        ); // 0x10028ec
        hash.insert(
            String::from("braille_dots_134678"),
            xkb::KEY_braille_dots_134678,
        ); // 0x10028ed
        hash.insert(
            String::from("braille_dots_234678"),
            xkb::KEY_braille_dots_234678,
        ); // 0x10028ee
        hash.insert(
            String::from("braille_dots_1234678"),
            xkb::KEY_braille_dots_1234678,
        ); // 0x10028ef
        hash.insert(
            String::from("braille_dots_5678"),
            xkb::KEY_braille_dots_5678,
        ); // 0x10028f0
        hash.insert(
            String::from("braille_dots_15678"),
            xkb::KEY_braille_dots_15678,
        ); // 0x10028f1
        hash.insert(
            String::from("braille_dots_25678"),
            xkb::KEY_braille_dots_25678,
        ); // 0x10028f2
        hash.insert(
            String::from("braille_dots_125678"),
            xkb::KEY_braille_dots_125678,
        ); // 0x10028f3
        hash.insert(
            String::from("braille_dots_35678"),
            xkb::KEY_braille_dots_35678,
        ); // 0x10028f4
        hash.insert(
            String::from("braille_dots_135678"),
            xkb::KEY_braille_dots_135678,
        ); // 0x10028f5
        hash.insert(
            String::from("braille_dots_235678"),
            xkb::KEY_braille_dots_235678,
        ); // 0x10028f6
        hash.insert(
            String::from("braille_dots_1235678"),
            xkb::KEY_braille_dots_1235678,
        ); // 0x10028f7
        hash.insert(
            String::from("braille_dots_45678"),
            xkb::KEY_braille_dots_45678,
        ); // 0x10028f8
        hash.insert(
            String::from("braille_dots_145678"),
            xkb::KEY_braille_dots_145678,
        ); // 0x10028f9
        hash.insert(
            String::from("braille_dots_245678"),
            xkb::KEY_braille_dots_245678,
        ); // 0x10028fa
        hash.insert(
            String::from("braille_dots_1245678"),
            xkb::KEY_braille_dots_1245678,
        ); // 0x10028fb
        hash.insert(
            String::from("braille_dots_345678"),
            xkb::KEY_braille_dots_345678,
        ); // 0x10028fc
        hash.insert(
            String::from("braille_dots_1345678"),
            xkb::KEY_braille_dots_1345678,
        ); // 0x10028fd
        hash.insert(
            String::from("braille_dots_2345678"),
            xkb::KEY_braille_dots_2345678,
        ); // 0x10028fe
        hash.insert(
            String::from("braille_dots_12345678"),
            xkb::KEY_braille_dots_12345678,
        ); // 0x10028ff

        hash.insert(String::from("Sinh_ng"), xkb::KEY_Sinh_ng); // 0x1000d82
        hash.insert(String::from("Sinh_h2"), xkb::KEY_Sinh_h2); // 0x1000d83
        hash.insert(String::from("Sinh_a"), xkb::KEY_Sinh_a); // 0x1000d85
        hash.insert(String::from("Sinh_aa"), xkb::KEY_Sinh_aa); // 0x1000d86
        hash.insert(String::from("Sinh_ae"), xkb::KEY_Sinh_ae); // 0x1000d87
        hash.insert(String::from("Sinh_aee"), xkb::KEY_Sinh_aee); // 0x1000d88
        hash.insert(String::from("Sinh_i"), xkb::KEY_Sinh_i); // 0x1000d89
        hash.insert(String::from("Sinh_ii"), xkb::KEY_Sinh_ii); // 0x1000d8a
        hash.insert(String::from("Sinh_u"), xkb::KEY_Sinh_u); // 0x1000d8b
        hash.insert(String::from("Sinh_uu"), xkb::KEY_Sinh_uu); // 0x1000d8c
        hash.insert(String::from("Sinh_ri"), xkb::KEY_Sinh_ri); // 0x1000d8d
        hash.insert(String::from("Sinh_rii"), xkb::KEY_Sinh_rii); // 0x1000d8e
        hash.insert(String::from("Sinh_lu"), xkb::KEY_Sinh_lu); // 0x1000d8f
        hash.insert(String::from("Sinh_luu"), xkb::KEY_Sinh_luu); // 0x1000d90
        hash.insert(String::from("Sinh_e"), xkb::KEY_Sinh_e); // 0x1000d91
        hash.insert(String::from("Sinh_ee"), xkb::KEY_Sinh_ee); // 0x1000d92
        hash.insert(String::from("Sinh_ai"), xkb::KEY_Sinh_ai); // 0x1000d93
        hash.insert(String::from("Sinh_o"), xkb::KEY_Sinh_o); // 0x1000d94
        hash.insert(String::from("Sinh_oo"), xkb::KEY_Sinh_oo); // 0x1000d95
        hash.insert(String::from("Sinh_au"), xkb::KEY_Sinh_au); // 0x1000d96
        hash.insert(String::from("Sinh_ka"), xkb::KEY_Sinh_ka); // 0x1000d9a
        hash.insert(String::from("Sinh_kha"), xkb::KEY_Sinh_kha); // 0x1000d9b
        hash.insert(String::from("Sinh_ga"), xkb::KEY_Sinh_ga); // 0x1000d9c
        hash.insert(String::from("Sinh_gha"), xkb::KEY_Sinh_gha); // 0x1000d9d
        hash.insert(String::from("Sinh_ng2"), xkb::KEY_Sinh_ng2); // 0x1000d9e
        hash.insert(String::from("Sinh_nga"), xkb::KEY_Sinh_nga); // 0x1000d9f
        hash.insert(String::from("Sinh_ca"), xkb::KEY_Sinh_ca); // 0x1000da0
        hash.insert(String::from("Sinh_cha"), xkb::KEY_Sinh_cha); // 0x1000da1
        hash.insert(String::from("Sinh_ja"), xkb::KEY_Sinh_ja); // 0x1000da2
        hash.insert(String::from("Sinh_jha"), xkb::KEY_Sinh_jha); // 0x1000da3
        hash.insert(String::from("Sinh_nya"), xkb::KEY_Sinh_nya); // 0x1000da4
        hash.insert(String::from("Sinh_jnya"), xkb::KEY_Sinh_jnya); // 0x1000da5
        hash.insert(String::from("Sinh_nja"), xkb::KEY_Sinh_nja); // 0x1000da6
        hash.insert(String::from("Sinh_tta"), xkb::KEY_Sinh_tta); // 0x1000da7
        hash.insert(String::from("Sinh_ttha"), xkb::KEY_Sinh_ttha); // 0x1000da8
        hash.insert(String::from("Sinh_dda"), xkb::KEY_Sinh_dda); // 0x1000da9
        hash.insert(String::from("Sinh_ddha"), xkb::KEY_Sinh_ddha); // 0x1000daa
        hash.insert(String::from("Sinh_nna"), xkb::KEY_Sinh_nna); // 0x1000dab
        hash.insert(String::from("Sinh_ndda"), xkb::KEY_Sinh_ndda); // 0x1000dac
        hash.insert(String::from("Sinh_tha"), xkb::KEY_Sinh_tha); // 0x1000dad
        hash.insert(String::from("Sinh_thha"), xkb::KEY_Sinh_thha); // 0x1000dae
        hash.insert(String::from("Sinh_dha"), xkb::KEY_Sinh_dha); // 0x1000daf
        hash.insert(String::from("Sinh_dhha"), xkb::KEY_Sinh_dhha); // 0x1000db0
        hash.insert(String::from("Sinh_na"), xkb::KEY_Sinh_na); // 0x1000db1
        hash.insert(String::from("Sinh_ndha"), xkb::KEY_Sinh_ndha); // 0x1000db3
        hash.insert(String::from("Sinh_pa"), xkb::KEY_Sinh_pa); // 0x1000db4
        hash.insert(String::from("Sinh_pha"), xkb::KEY_Sinh_pha); // 0x1000db5
        hash.insert(String::from("Sinh_ba"), xkb::KEY_Sinh_ba); // 0x1000db6
        hash.insert(String::from("Sinh_bha"), xkb::KEY_Sinh_bha); // 0x1000db7
        hash.insert(String::from("Sinh_ma"), xkb::KEY_Sinh_ma); // 0x1000db8
        hash.insert(String::from("Sinh_mba"), xkb::KEY_Sinh_mba); // 0x1000db9
        hash.insert(String::from("Sinh_ya"), xkb::KEY_Sinh_ya); // 0x1000dba
        hash.insert(String::from("Sinh_ra"), xkb::KEY_Sinh_ra); // 0x1000dbb
        hash.insert(String::from("Sinh_la"), xkb::KEY_Sinh_la); // 0x1000dbd
        hash.insert(String::from("Sinh_va"), xkb::KEY_Sinh_va); // 0x1000dc0
        hash.insert(String::from("Sinh_sha"), xkb::KEY_Sinh_sha); // 0x1000dc1
        hash.insert(String::from("Sinh_ssha"), xkb::KEY_Sinh_ssha); // 0x1000dc2
        hash.insert(String::from("Sinh_sa"), xkb::KEY_Sinh_sa); // 0x1000dc3
        hash.insert(String::from("Sinh_ha"), xkb::KEY_Sinh_ha); // 0x1000dc4
        hash.insert(String::from("Sinh_lla"), xkb::KEY_Sinh_lla); // 0x1000dc5
        hash.insert(String::from("Sinh_fa"), xkb::KEY_Sinh_fa); // 0x1000dc6
        hash.insert(String::from("Sinh_al"), xkb::KEY_Sinh_al); // 0x1000dca
        hash.insert(String::from("Sinh_aa2"), xkb::KEY_Sinh_aa2); // 0x1000dcf
        hash.insert(String::from("Sinh_ae2"), xkb::KEY_Sinh_ae2); // 0x1000dd0
        hash.insert(String::from("Sinh_aee2"), xkb::KEY_Sinh_aee2); // 0x1000dd1
        hash.insert(String::from("Sinh_i2"), xkb::KEY_Sinh_i2); // 0x1000dd2
        hash.insert(String::from("Sinh_ii2"), xkb::KEY_Sinh_ii2); // 0x1000dd3
        hash.insert(String::from("Sinh_u2"), xkb::KEY_Sinh_u2); // 0x1000dd4
        hash.insert(String::from("Sinh_uu2"), xkb::KEY_Sinh_uu2); // 0x1000dd6
        hash.insert(String::from("Sinh_ru2"), xkb::KEY_Sinh_ru2); // 0x1000dd8
        hash.insert(String::from("Sinh_e2"), xkb::KEY_Sinh_e2); // 0x1000dd9
        hash.insert(String::from("Sinh_ee2"), xkb::KEY_Sinh_ee2); // 0x1000dda
        hash.insert(String::from("Sinh_ai2"), xkb::KEY_Sinh_ai2); // 0x1000ddb
        hash.insert(String::from("Sinh_o2"), xkb::KEY_Sinh_o2); // 0x1000ddc
        hash.insert(String::from("Sinh_oo2"), xkb::KEY_Sinh_oo2); // 0x1000ddd
        hash.insert(String::from("Sinh_au2"), xkb::KEY_Sinh_au2); // 0x1000dde
        hash.insert(String::from("Sinh_lu2"), xkb::KEY_Sinh_lu2); // 0x1000ddf
        hash.insert(String::from("Sinh_ruu2"), xkb::KEY_Sinh_ruu2); // 0x1000df2
        hash.insert(String::from("Sinh_luu2"), xkb::KEY_Sinh_luu2); // 0x1000df3
        hash.insert(String::from("Sinh_kunddaliya"), xkb::KEY_Sinh_kunddaliya); // 0x1000df4
        hash.insert(String::from("XF86ModeLock"), xkb::KEY_XF86ModeLock); // 0x1008FF01
        hash.insert(
            String::from("XF86MonBrightnessUp"),
            xkb::KEY_XF86MonBrightnessUp,
        ); // 0x1008FF02
        hash.insert(
            String::from("XF86MonBrightnessDown"),
            xkb::KEY_XF86MonBrightnessDown,
        ); // 0x1008FF03
        hash.insert(
            String::from("XF86KbdLightOnOff"),
            xkb::KEY_XF86KbdLightOnOff,
        ); // 0x1008FF04
        hash.insert(
            String::from("XF86KbdBrightnessUp"),
            xkb::KEY_XF86KbdBrightnessUp,
        ); // 0x1008FF05
        hash.insert(
            String::from("XF86KbdBrightnessDown"),
            xkb::KEY_XF86KbdBrightnessDown,
        ); // 0x1008FF06
        hash.insert(String::from("XF86Standby"), xkb::KEY_XF86Standby); // 0x1008FF10
        hash.insert(
            String::from("XF86AudioLowerVolume"),
            xkb::KEY_XF86AudioLowerVolume,
        ); // 0x1008FF11
        hash.insert(String::from("XF86AudioMute"), xkb::KEY_XF86AudioMute); // 0x1008FF12
        hash.insert(
            String::from("XF86AudioRaiseVolume"),
            xkb::KEY_XF86AudioRaiseVolume,
        ); // 0x1008FF13
        hash.insert(String::from("XF86AudioPlay"), xkb::KEY_XF86AudioPlay); // 0x1008FF14
        hash.insert(String::from("XF86AudioStop"), xkb::KEY_XF86AudioStop); // 0x1008FF15
        hash.insert(String::from("XF86AudioPrev"), xkb::KEY_XF86AudioPrev); // 0x1008FF16
        hash.insert(String::from("XF86AudioNext"), xkb::KEY_XF86AudioNext); // 0x1008FF17
        hash.insert(String::from("XF86HomePage"), xkb::KEY_XF86HomePage); // 0x1008FF18
        hash.insert(String::from("XF86Mail"), xkb::KEY_XF86Mail); // 0x1008FF19
        hash.insert(String::from("XF86Start"), xkb::KEY_XF86Start); // 0x1008FF1A
        hash.insert(String::from("XF86Search"), xkb::KEY_XF86Search); // 0x1008FF1B
        hash.insert(String::from("XF86AudioRecord"), xkb::KEY_XF86AudioRecord); // 0x1008FF1C
        hash.insert(String::from("XF86Calculator"), xkb::KEY_XF86Calculator); // 0x1008FF1D
        hash.insert(String::from("XF86Memo"), xkb::KEY_XF86Memo); // 0x1008FF1E
        hash.insert(String::from("XF86ToDoList"), xkb::KEY_XF86ToDoList); // 0x1008FF1F
        hash.insert(String::from("XF86Calendar"), xkb::KEY_XF86Calendar); // 0x1008FF20
        hash.insert(String::from("XF86PowerDown"), xkb::KEY_XF86PowerDown); // 0x1008FF21
        hash.insert(
            String::from("XF86ContrastAdjust"),
            xkb::KEY_XF86ContrastAdjust,
        ); // 0x1008FF22
        hash.insert(String::from("XF86RockerUp"), xkb::KEY_XF86RockerUp); // 0x1008FF23
        hash.insert(String::from("XF86RockerDown"), xkb::KEY_XF86RockerDown); // 0x1008FF24
        hash.insert(String::from("XF86RockerEnter"), xkb::KEY_XF86RockerEnter); // 0x1008FF25
        hash.insert(String::from("XF86Back"), xkb::KEY_XF86Back); // 0x1008FF26
        hash.insert(String::from("XF86Forward"), xkb::KEY_XF86Forward); // 0x1008FF27
        hash.insert(String::from("XF86Stop"), xkb::KEY_XF86Stop); // 0x1008FF28
        hash.insert(String::from("XF86Refresh"), xkb::KEY_XF86Refresh); // 0x1008FF29
        hash.insert(String::from("XF86PowerOff"), xkb::KEY_XF86PowerOff); // 0x1008FF2A
        hash.insert(String::from("XF86WakeUp"), xkb::KEY_XF86WakeUp); // 0x1008FF2B
        hash.insert(String::from("XF86Eject"), xkb::KEY_XF86Eject); // 0x1008FF2C
        hash.insert(String::from("XF86ScreenSaver"), xkb::KEY_XF86ScreenSaver); // 0x1008FF2D
        hash.insert(String::from("XF86WWW"), xkb::KEY_XF86WWW); // 0x1008FF2E
        hash.insert(String::from("XF86Sleep"), xkb::KEY_XF86Sleep); // 0x1008FF2F
        hash.insert(String::from("XF86Favorites"), xkb::KEY_XF86Favorites); // 0x1008FF30
        hash.insert(String::from("XF86AudioPause"), xkb::KEY_XF86AudioPause); // 0x1008FF31
        hash.insert(String::from("XF86AudioMedia"), xkb::KEY_XF86AudioMedia); // 0x1008FF32
        hash.insert(String::from("XF86MyComputer"), xkb::KEY_XF86MyComputer); // 0x1008FF33
        hash.insert(String::from("XF86VendorHome"), xkb::KEY_XF86VendorHome); // 0x1008FF34
        hash.insert(String::from("XF86LightBulb"), xkb::KEY_XF86LightBulb); // 0x1008FF35
        hash.insert(String::from("XF86Shop"), xkb::KEY_XF86Shop); // 0x1008FF36
        hash.insert(String::from("XF86History"), xkb::KEY_XF86History); // 0x1008FF37
        hash.insert(String::from("XF86OpenURL"), xkb::KEY_XF86OpenURL); // 0x1008FF38
        hash.insert(String::from("XF86AddFavorite"), xkb::KEY_XF86AddFavorite); // 0x1008FF39
        hash.insert(String::from("XF86HotLinks"), xkb::KEY_XF86HotLinks); // 0x1008FF3A
        hash.insert(
            String::from("XF86BrightnessAdjust"),
            xkb::KEY_XF86BrightnessAdjust,
        ); // 0x1008FF3B
        hash.insert(String::from("XF86Finance"), xkb::KEY_XF86Finance); // 0x1008FF3C
        hash.insert(String::from("XF86Community"), xkb::KEY_XF86Community); // 0x1008FF3D
        hash.insert(String::from("XF86AudioRewind"), xkb::KEY_XF86AudioRewind); // 0x1008FF3E
        hash.insert(String::from("XF86BackForward"), xkb::KEY_XF86BackForward); // 0x1008FF3F
        hash.insert(String::from("XF86Launch0"), xkb::KEY_XF86Launch0); // 0x1008FF40
        hash.insert(String::from("XF86Launch1"), xkb::KEY_XF86Launch1); // 0x1008FF41
        hash.insert(String::from("XF86Launch2"), xkb::KEY_XF86Launch2); // 0x1008FF42
        hash.insert(String::from("XF86Launch3"), xkb::KEY_XF86Launch3); // 0x1008FF43
        hash.insert(String::from("XF86Launch4"), xkb::KEY_XF86Launch4); // 0x1008FF44
        hash.insert(String::from("XF86Launch5"), xkb::KEY_XF86Launch5); // 0x1008FF45
        hash.insert(String::from("XF86Launch6"), xkb::KEY_XF86Launch6); // 0x1008FF46
        hash.insert(String::from("XF86Launch7"), xkb::KEY_XF86Launch7); // 0x1008FF47
        hash.insert(String::from("XF86Launch8"), xkb::KEY_XF86Launch8); // 0x1008FF48
        hash.insert(String::from("XF86Launch9"), xkb::KEY_XF86Launch9); // 0x1008FF49
        hash.insert(String::from("XF86LaunchA"), xkb::KEY_XF86LaunchA); // 0x1008FF4A
        hash.insert(String::from("XF86LaunchB"), xkb::KEY_XF86LaunchB); // 0x1008FF4B
        hash.insert(String::from("XF86LaunchC"), xkb::KEY_XF86LaunchC); // 0x1008FF4C
        hash.insert(String::from("XF86LaunchD"), xkb::KEY_XF86LaunchD); // 0x1008FF4D
        hash.insert(String::from("XF86LaunchE"), xkb::KEY_XF86LaunchE); // 0x1008FF4E
        hash.insert(String::from("XF86LaunchF"), xkb::KEY_XF86LaunchF); // 0x1008FF4F
        hash.insert(
            String::from("XF86ApplicationLeft"),
            xkb::KEY_XF86ApplicationLeft,
        ); // 0x1008FF50
        hash.insert(
            String::from("XF86ApplicationRight"),
            xkb::KEY_XF86ApplicationRight,
        ); // 0x1008FF51
        hash.insert(String::from("XF86Book"), xkb::KEY_XF86Book); // 0x1008FF52
        hash.insert(String::from("XF86CD"), xkb::KEY_XF86CD); // 0x1008FF53
        hash.insert(String::from("XF86Calculater"), xkb::KEY_XF86Calculater); // 0x1008FF54
        hash.insert(String::from("XF86Clear"), xkb::KEY_XF86Clear); // 0x1008FF55
        hash.insert(String::from("XF86Close"), xkb::KEY_XF86Close); // 0x1008FF56
        hash.insert(String::from("XF86Copy"), xkb::KEY_XF86Copy); // 0x1008FF57
        hash.insert(String::from("XF86Cut"), xkb::KEY_XF86Cut); // 0x1008FF58
        hash.insert(String::from("XF86Display"), xkb::KEY_XF86Display); // 0x1008FF59
        hash.insert(String::from("XF86DOS"), xkb::KEY_XF86DOS); // 0x1008FF5A
        hash.insert(String::from("XF86Documents"), xkb::KEY_XF86Documents); // 0x1008FF5B
        hash.insert(String::from("XF86Excel"), xkb::KEY_XF86Excel); // 0x1008FF5C
        hash.insert(String::from("XF86Explorer"), xkb::KEY_XF86Explorer); // 0x1008FF5D
        hash.insert(String::from("XF86Game"), xkb::KEY_XF86Game); // 0x1008FF5E
        hash.insert(String::from("XF86Go"), xkb::KEY_XF86Go); // 0x1008FF5F
        hash.insert(String::from("XF86iTouch"), xkb::KEY_XF86iTouch); // 0x1008FF60
        hash.insert(String::from("XF86LogOff"), xkb::KEY_XF86LogOff); // 0x1008FF61
        hash.insert(String::from("XF86Market"), xkb::KEY_XF86Market); // 0x1008FF62
        hash.insert(String::from("XF86Meeting"), xkb::KEY_XF86Meeting); // 0x1008FF63
        hash.insert(String::from("XF86MenuKB"), xkb::KEY_XF86MenuKB); // 0x1008FF65
        hash.insert(String::from("XF86MenuPB"), xkb::KEY_XF86MenuPB); // 0x1008FF66
        hash.insert(String::from("XF86MySites"), xkb::KEY_XF86MySites); // 0x1008FF67
        hash.insert(String::from("XF86New"), xkb::KEY_XF86New); // 0x1008FF68
        hash.insert(String::from("XF86News"), xkb::KEY_XF86News); // 0x1008FF69
        hash.insert(String::from("XF86OfficeHome"), xkb::KEY_XF86OfficeHome); // 0x1008FF6A
        hash.insert(String::from("XF86Open"), xkb::KEY_XF86Open); // 0x1008FF6B
        hash.insert(String::from("XF86Option"), xkb::KEY_XF86Option); // 0x1008FF6C
        hash.insert(String::from("XF86Paste"), xkb::KEY_XF86Paste); // 0x1008FF6D
        hash.insert(String::from("XF86Phone"), xkb::KEY_XF86Phone); // 0x1008FF6E
        hash.insert(String::from("XF86Q"), xkb::KEY_XF86Q); // 0x1008FF70
        hash.insert(String::from("XF86Reply"), xkb::KEY_XF86Reply); // 0x1008FF72
        hash.insert(String::from("XF86Reload"), xkb::KEY_XF86Reload); // 0x1008FF73
        hash.insert(
            String::from("XF86RotateWindows"),
            xkb::KEY_XF86RotateWindows,
        ); // 0x1008FF74
        hash.insert(String::from("XF86RotationPB"), xkb::KEY_XF86RotationPB); // 0x1008FF75
        hash.insert(String::from("XF86RotationKB"), xkb::KEY_XF86RotationKB); // 0x1008FF76
        hash.insert(String::from("XF86Save"), xkb::KEY_XF86Save); // 0x1008FF77
        hash.insert(String::from("XF86ScrollUp"), xkb::KEY_XF86ScrollUp); // 0x1008FF78
        hash.insert(String::from("XF86ScrollDown"), xkb::KEY_XF86ScrollDown); // 0x1008FF79
        hash.insert(String::from("XF86ScrollClick"), xkb::KEY_XF86ScrollClick); // 0x1008FF7A
        hash.insert(String::from("XF86Send"), xkb::KEY_XF86Send); // 0x1008FF7B
        hash.insert(String::from("XF86Spell"), xkb::KEY_XF86Spell); // 0x1008FF7C
        hash.insert(String::from("XF86SplitScreen"), xkb::KEY_XF86SplitScreen); // 0x1008FF7D
        hash.insert(String::from("XF86Support"), xkb::KEY_XF86Support); // 0x1008FF7E
        hash.insert(String::from("XF86TaskPane"), xkb::KEY_XF86TaskPane); // 0x1008FF7F
        hash.insert(String::from("XF86Terminal"), xkb::KEY_XF86Terminal); // 0x1008FF80
        hash.insert(String::from("XF86Tools"), xkb::KEY_XF86Tools); // 0x1008FF81
        hash.insert(String::from("XF86Travel"), xkb::KEY_XF86Travel); // 0x1008FF82
        hash.insert(String::from("XF86UserPB"), xkb::KEY_XF86UserPB); // 0x1008FF84
        hash.insert(String::from("XF86User1KB"), xkb::KEY_XF86User1KB); // 0x1008FF85
        hash.insert(String::from("XF86User2KB"), xkb::KEY_XF86User2KB); // 0x1008FF86
        hash.insert(String::from("XF86Video"), xkb::KEY_XF86Video); // 0x1008FF87
        hash.insert(String::from("XF86WheelButton"), xkb::KEY_XF86WheelButton); // 0x1008FF88
        hash.insert(String::from("XF86Word"), xkb::KEY_XF86Word); // 0x1008FF89
        hash.insert(String::from("XF86Xfer"), xkb::KEY_XF86Xfer); // 0x1008FF8A
        hash.insert(String::from("XF86ZoomIn"), xkb::KEY_XF86ZoomIn); // 0x1008FF8B
        hash.insert(String::from("XF86ZoomOut"), xkb::KEY_XF86ZoomOut); // 0x1008FF8C
        hash.insert(String::from("XF86Away"), xkb::KEY_XF86Away); // 0x1008FF8D
        hash.insert(String::from("XF86Messenger"), xkb::KEY_XF86Messenger); // 0x1008FF8E
        hash.insert(String::from("XF86WebCam"), xkb::KEY_XF86WebCam); // 0x1008FF8F
        hash.insert(String::from("XF86MailForward"), xkb::KEY_XF86MailForward); // 0x1008FF90
        hash.insert(String::from("XF86Pictures"), xkb::KEY_XF86Pictures); // 0x1008FF91
        hash.insert(String::from("XF86Music"), xkb::KEY_XF86Music); // 0x1008FF92
        hash.insert(String::from("XF86Battery"), xkb::KEY_XF86Battery); // 0x1008FF93
        hash.insert(String::from("XF86Bluetooth"), xkb::KEY_XF86Bluetooth); // 0x1008FF94
        hash.insert(String::from("XF86WLAN"), xkb::KEY_XF86WLAN); // 0x1008FF95
        hash.insert(String::from("XF86UWB"), xkb::KEY_XF86UWB); // 0x1008FF96
        hash.insert(String::from("XF86AudioForward"), xkb::KEY_XF86AudioForward); // 0x1008FF97
        hash.insert(String::from("XF86AudioRepeat"), xkb::KEY_XF86AudioRepeat); // 0x1008FF98
        hash.insert(
            String::from("XF86AudioRandomPlay"),
            xkb::KEY_XF86AudioRandomPlay,
        ); // 0x1008FF99
        hash.insert(String::from("XF86Subtitle"), xkb::KEY_XF86Subtitle); // 0x1008FF9A
        hash.insert(
            String::from("XF86AudioCycleTrack"),
            xkb::KEY_XF86AudioCycleTrack,
        ); // 0x1008FF9B
        hash.insert(String::from("XF86CycleAngle"), xkb::KEY_XF86CycleAngle); // 0x1008FF9C
        hash.insert(String::from("XF86FrameBack"), xkb::KEY_XF86FrameBack); // 0x1008FF9D
        hash.insert(String::from("XF86FrameForward"), xkb::KEY_XF86FrameForward); // 0x1008FF9E
        hash.insert(String::from("XF86Time"), xkb::KEY_XF86Time); // 0x1008FF9F
        hash.insert(String::from("XF86Select"), xkb::KEY_XF86Select); // 0x1008FFA0
        hash.insert(String::from("XF86View"), xkb::KEY_XF86View); // 0x1008FFA1
        hash.insert(String::from("XF86TopMenu"), xkb::KEY_XF86TopMenu); // 0x1008FFA2
        hash.insert(String::from("XF86Red"), xkb::KEY_XF86Red); // 0x1008FFA3
        hash.insert(String::from("XF86Green"), xkb::KEY_XF86Green); // 0x1008FFA4
        hash.insert(String::from("XF86Yellow"), xkb::KEY_XF86Yellow); // 0x1008FFA5
        hash.insert(String::from("XF86Blue"), xkb::KEY_XF86Blue); // 0x1008FFA6
        hash.insert(String::from("XF86Suspend"), xkb::KEY_XF86Suspend); // 0x1008FFA7
        hash.insert(String::from("XF86Hibernate"), xkb::KEY_XF86Hibernate); // 0x1008FFA8
        hash.insert(
            String::from("XF86TouchpadToggle"),
            xkb::KEY_XF86TouchpadToggle,
        ); // 0x1008FFA9
        hash.insert(String::from("XF86TouchpadOn"), xkb::KEY_XF86TouchpadOn); // 0x1008FFB0
        hash.insert(String::from("XF86TouchpadOff"), xkb::KEY_XF86TouchpadOff); // 0x1008FFB1
        hash.insert(String::from("XF86AudioMicMute"), xkb::KEY_XF86AudioMicMute); // 0x1008FFB2
        hash.insert(String::from("XF86Switch_VT_1"), xkb::KEY_XF86Switch_VT_1); // 0x1008FE01
        hash.insert(String::from("XF86Switch_VT_2"), xkb::KEY_XF86Switch_VT_2); // 0x1008FE02
        hash.insert(String::from("XF86Switch_VT_3"), xkb::KEY_XF86Switch_VT_3); // 0x1008FE03
        hash.insert(String::from("XF86Switch_VT_4"), xkb::KEY_XF86Switch_VT_4); // 0x1008FE04
        hash.insert(String::from("XF86Switch_VT_5"), xkb::KEY_XF86Switch_VT_5); // 0x1008FE05
        hash.insert(String::from("XF86Switch_VT_6"), xkb::KEY_XF86Switch_VT_6); // 0x1008FE06
        hash.insert(String::from("XF86Switch_VT_7"), xkb::KEY_XF86Switch_VT_7); // 0x1008FE07
        hash.insert(String::from("XF86Switch_VT_8"), xkb::KEY_XF86Switch_VT_8); // 0x1008FE08
        hash.insert(String::from("XF86Switch_VT_9"), xkb::KEY_XF86Switch_VT_9); // 0x1008FE09
        hash.insert(String::from("XF86Switch_VT_10"), xkb::KEY_XF86Switch_VT_10); // 0x1008FE0A
        hash.insert(String::from("XF86Switch_VT_11"), xkb::KEY_XF86Switch_VT_11); // 0x1008FE0B
        hash.insert(String::from("XF86Switch_VT_12"), xkb::KEY_XF86Switch_VT_12); // 0x1008FE0C
        hash.insert(String::from("XF86Ungrab"), xkb::KEY_XF86Ungrab); // 0x1008FE20
        hash.insert(String::from("XF86ClearGrab"), xkb::KEY_XF86ClearGrab); // 0x1008FE21
        hash.insert(String::from("XF86Next_VMode"), xkb::KEY_XF86Next_VMode); // 0x1008FE22
        hash.insert(String::from("XF86Prev_VMode"), xkb::KEY_XF86Prev_VMode); // 0x1008FE23
        hash.insert(
            String::from("XF86LogWindowTree"),
            xkb::KEY_XF86LogWindowTree,
        ); // 0x1008FE24
        hash.insert(String::from("XF86LogGrabInfo"), xkb::KEY_XF86LogGrabInfo); // 0x1008FE25
        hash
    }));

    /// Return the Keysym name based on the code
    pub(crate) fn get_keysym(&self, code: u32) -> Option<&String> {
        self.0.get_by_right(&code)
    }

    /// Return the Keysym code based on the name
    pub(crate) fn get_code(&self, keysym: &str) -> Option<&u32> {
        self.0.get_by_left(&keysym.to_string())
    }

    /// Return the UTF-8 conversion of the `Keysym`
    pub(crate) fn utf8(&self, keysym: &str) -> Result<String, Error> {
        if let Some(key) = self.get_code(keysym) {
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
