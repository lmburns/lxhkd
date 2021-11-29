use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    process::{Command, Stdio},
    str::FromStr,
};

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
    /// Spawn a shell from the given keybind mapping
    pub(crate) fn spawn_shell<I, S>(cmd: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut builder = Command::new("sh");
        builder.arg("-c");

        for arg in cmd {
            let arg = arg.as_ref();
            builder.arg(arg);
        }

        builder
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .ok();
    }

    /// Run the given `Action`, given it is a command or a remap
    pub(crate) fn run(&self) {
        match self {
            Self::ShellCmd(cmd) => {
                Self::spawn_shell(
                    cmd.split_whitespace()
                        .map(str::trim)
                        .collect::<Vec<_>>(),
                );
            },
            Self::Remap(remap) => {
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
