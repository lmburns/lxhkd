use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, process::Command, str::FromStr};

/// The action that a mapping will do
///
/// It can be one of the following:
///     - A shell command    (i.e, opening a terminal)
///     - Another keymapping (i.e., Caps_Lock => Escape)
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Action {
    ShellCmd(String),
    // TODO: Change this to an enum
    Remap(String),
}

impl Action {
    pub(crate) fn run(&self) {
        match *self {
            Action::ShellCmd(cmd) => {
                let _ = Command::new("sh").args(&["-c", repr]).spawn();
            },
            Action::Remap(remap) => {
                println!("found remap: {}", remap);
            },
        }
    }
}

/////////////////////////////////////////////////////////

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
