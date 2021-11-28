use xcb::x::Timestamp;
use super::{chord::Chain, keyboard::Keyboard};
use std::{collections::BTreeMap, fmt};
use crate::parser::Action;


/// Global daemon state object.
#[derive(Debug)]
pub(crate) struct Daemon<'a> {
    /// The current keyboard setup
    kbd_state:        Keyboard<'a>,
    /// Max allowed time between keypresses
    keypress_timeout: u32,
    /// Currently chain being pressed
    current_chain:    Chain,
    /// Tracker of last keypress
    last_keypress:    Timestamp,
    /// The bindings registered in all modes.
    bindings:         BTreeMap<Chain, Action>,
}

// pub(crate) fn parse_keysym(name: &str, keysym: xcb::Keysym) -> bool {}

impl<'a> fmt::Debug for Keyboard<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Root: {:?}", self.root)
    }
}
