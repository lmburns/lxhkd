use crate::{
    config::{Action, Config},
    keys::chord::{Chain, Chord},
};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    process::{Command, Stdio},
    str::FromStr,
};

const MODIFIER_STR: &[&str] = &[
    "alt", "lalt", "ralt", "shift", "lshift", "rshift", "cmd", "lcmd", "rcmd", "ctrl", "lctrl",
    "rctrl", "fn", "hyper", "meh",
];

/// A single line in a configuration file
pub(crate) struct SingleLine {
    /// Set of keys that will be mapped to a shell command or remapped to
    /// another set of keys
    pub(crate) chain:  Chain,
    /// Action to be executed (shell command or a remap)
    pub(crate) action: Action,
}

// pub(crate) fn parse_modifier<'a>(name: &str, field: &'a mut u16) -> (bool,
// &'a mut u16) {     if name == "shift" {
//         *field |= MOD_MASK_SHIFT;
//         (true, field)
//     } else if name == "control" || name == "ctrl" {
//         *field |= MOD_MASK_CONTROL;
//         (true, field)
//     } else if name == "mod1" {
//         *field |= MOD_MASK_1;
//         (true, field)
//     } else if name == "mod2" {
//         *field |= MOD_MASK_2;
//         (true, field)
//     } else if name == "mod3" {
//         *field |= MOD_MASK_3;
//         (true, field)
//     } else if name == "mod4" {
//         *field |= MOD_MASK_4;
//         (true, field)
//     } else if name == "mod5" {
//         *field |= MOD_MASK_5;
//         (true, field)
//     } else if name == "lock" {
//         *field |= MOD_MASK_LOCK;
//         (true, field)
//     } else if name == "any" {
//         *field |= MOD_MASK_ANY;
//         (true, field)
//     } else {
//         (false, field)
//     }
// }
