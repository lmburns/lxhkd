use crate::types::KEYSYMS_PER_KEYCODE;
use anyhow::{anyhow, Result};
use colored::Colorize;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use x11::keysym::{XK_Num_Lock, XK_Scroll_Lock};
use xcb::ffi::{xcb_keycode_t, xcb_mod_mask_t};
use xcb_util::keysyms::KeySymbols;

pub(crate) const LOCK_MASK: xkb::ModMask = xkb::ModMask(xcb::MOD_MASK_LOCK);
pub(crate) const NUM_MASK: xkb::ModMask = xkb::ModMask(xcb::MOD_MASK_2);
/// Ignore the Num_Lock modifier mask
pub(crate) const IGNORE_MASK: xkb::ModMask = xkb::ModMask(xcb::MOD_MASK_LOCK | xcb::MOD_MASK_2);

// use penrose_keysyms::XKeySym;

pub(crate) type KeyCodeMask = xcb_mod_mask_t;
pub(crate) type KeyCodeValue = xcb_keycode_t;

/// A key press and held modifiers
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct KeyCode {
    pub(crate) mask: KeyCodeMask,
    pub(crate) code: KeyCodeValue,
}

impl KeyCode {
    /// Filter ignored modifiers from a mask
    pub(crate) fn filter_modifier(self, mask: KeyCodeMask) -> KeyCode {
        KeyCode {
            mask: self.mask & !mask,
            code: self.code,
        }
    }
}

/// Representation of a keypress with all keys held down
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct Key {
    pub(crate) key:      String,
    pub(crate) modifier: KeyModifier,
}

/// Builtin modifiers available
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) enum KeyModifier {
    None,
    Shift,
    Lock,
    Ctrl,
    Mod1,
    Mod2,
    Mod3,
    Mod4,
    Mod5,
    Any,
}

impl From<xkb::ModMask> for KeyModifier {
    fn from(mask: xkb::ModMask) -> Self {
        match mask {
            // 0x3 is shift held down when caps_lock is on
            xkb::ModMask(xcb::MOD_MASK_SHIFT | 0x3) => KeyModifier::Shift,
            xkb::ModMask(xcb::MOD_MASK_CONTROL) => KeyModifier::Ctrl,
            xkb::ModMask(xcb::MOD_MASK_LOCK) => KeyModifier::Lock,
            xkb::ModMask(xcb::MOD_MASK_1) => KeyModifier::Mod1,
            xkb::ModMask(xcb::MOD_MASK_2) => KeyModifier::Mod2,
            xkb::ModMask(xcb::MOD_MASK_3) => KeyModifier::Mod3,
            xkb::ModMask(xcb::MOD_MASK_4) => KeyModifier::Mod4,
            xkb::ModMask(xcb::MOD_MASK_5) => KeyModifier::Mod5,
            xkb::ModMask(xcb::MOD_MASK_ANY) => KeyModifier::Any,
            _ => KeyModifier::None,
        }
    }
}

/// Return the keycode from the keysym. Examples of what it is parsing
/// can be found from the output of `xmodmap -pki`
pub(crate) fn keycodes_from_keysym(
    conn: &xcb::Connection,
    keysym: xcb::Keysym,
) -> Option<xcb::Keycode> {
    let setup = conn.get_setup();
    let min_kc = setup.min_keycode();
    let max_kc = setup.max_keycode();
    let mut result = None;
    // let mut num = 0_u8;

    // Match the last item specifying keycode within the database if it is mentioned
    // more than once
    for kc in min_kc..=max_kc {
        for col in 0..KEYSYMS_PER_KEYCODE {
            if let Ok(reply) = xcb::get_keyboard_mapping(conn, kc, 1).get_reply() {
                let keysyms = reply.keysyms();

                #[allow(clippy::cast_possible_wrap, clippy::cast_lossless)]
                let ks = KeySymbols::new(conn).get_keysym(kc, col as i32);

                if ks == keysym {
                    // num += 1;
                    // if num == 1 {
                    result = Some(kc);
                    // } else {
                    //     let mut split = result.map(|f| {
                    //         f.to_string()
                    //             .split("")
                    //             .map(ToString::to_string)
                    //             .collect::<Vec<_>>()
                    //     })?;
                    //     // .chars()
                    //     // .map(|d| d.to_digit(10).unwrap() as u8)
                    //
                    //     split[(num - 1) as usize] = kc.to_string();
                    //     split[num as usize] = xcb::NO_SYMBOL.to_string();
                    //
                    //     result = Some(split.join("").parse::<u8>().ok()?);
                    // }
                    break;
                }
            }
        }
    }

    result
}

/// Return the modifier-field code from a specified [`Keycode`](xcb::Keycode)
pub(crate) fn modfield_from_keycode(conn: &xcb::Connection, keycode: xcb::Keycode) -> u16 {
    let mut modfield = 0_u16;
    let setup = conn.get_setup();
    if let Ok(reply) = xcb::get_modifier_mapping(conn).get_reply() {
        if reply.keycodes_per_modifier() > 0 {
            let keycodes = reply.keycodes();
            let num_mods = (keycodes.iter().len() / reply.keycodes_per_modifier() as usize) as u8;

            for i in 0..num_mods {
                for j in 0..reply.keycodes_per_modifier() {
                    let mkc = keycodes[(i * reply.keycodes_per_modifier() + j) as usize];
                    if mkc == xcb::NO_SYMBOL as u8 {
                        continue;
                    }
                    if keycode == mkc {
                        modfield |= 1 << i;
                    }
                }
            }
        }
    }
    modfield
}

// int16_t modfield_from_keycode(xcb_keycode_t keycode) {
//   uint16_t modfield = 0;
//   xcb_keycode_t *mod_keycodes = NULL;
//   xcb_get_modifier_mapping_reply_t *reply = NULL;
//   if ((reply = xcb_get_modifier_mapping_reply(
//            dpy, xcb_get_modifier_mapping(dpy), NULL)) != NULL &&
//       reply->keycodes_per_modifier > 0) {
//     if ((mod_keycodes = xcb_get_modifier_mapping_keycodes(reply)) != NULL) {
//       unsigned int num_mod = xcb_get_modifier_mapping_keycodes_length(reply)
// /                              reply->keycodes_per_modifier;
//       for (unsigned int i = 0; i < num_mod; i++) {
//         for (unsigned int j = 0; j < reply->keycodes_per_modifier; j++) {
//           xcb_keycode_t mkc =
//               mod_keycodes[i * reply->keycodes_per_modifier + j];
//           if (mkc == XCB_NO_SYMBOL) continue;
//           if (keycode == mkc) modfield |= (1 << i);
//         }
//       }
//     }
//   }
//   free(reply);
//   return modfield;
// }

/// Return the modifier field based on a keysym
pub(crate) fn modfield_from_keysym(conn: &xcb::Connection, keysym: xcb::Keysym) -> u16 {
    let mut modfield = 0_u16;

    // TODO: Look into more but this works
    if let Some(mut keycodes) = keycodes_from_keysym(conn, keysym) {
        // while keycodes != xcb::NO_SYMBOL as u8 {
            modfield |= modfield_from_keycode(conn, keycodes);
            // keycodes += 1;
        // }
    }

    modfield
}

/// Print the lock-keys keysym codes
pub(crate) fn get_lock_fields(conn: &xcb::Connection) {
    let scroll_lock = modfield_from_keysym(conn, XK_Scroll_Lock);
    let num_lock = modfield_from_keysym(conn, XK_Num_Lock);
    let caps_lock = xcb::MOD_MASK_LOCK;
    println!(
        "{}:\nNum_Lock: {}\nCaps_Lock: {}\n Scroll_Lock: {}",
        "Lock Fields".yellow().bold(),
        num_lock,
        caps_lock,
        scroll_lock
    );
}

pub(crate) fn parse_keysym(name: &str, keysym: xcb::Keysym) -> bool {
}

// bool parse_keysym(char *name, xcb_keysym_t *keysym) {
//   for (unsigned int i = 0; i < LENGTH(nks_dict); i++) {
//     keysym_dict_t nks = nks_dict[i];
//     if (strcmp(name, nks.name) == 0) {
//       *keysym = nks.keysym;
//       return true;
//     }
//   }
//   return false;
// }
