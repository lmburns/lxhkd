#![allow(clippy::module_inception)]

pub(crate) mod chord;
pub(crate) mod event_handler;
// pub(crate) mod daemon;
pub(crate) mod keyboard;
pub(crate) mod keys;
pub(crate) mod keysym;

// Key repeat
//  - https://unix.stackexchange.com/questions/408461/where-is-the-default-repeat-rate-for-xset-stored

// XKB Overall
//  - https://www.x.org/releases/X11R7.6/doc/libX11/specs/XKB/xkblib.html
