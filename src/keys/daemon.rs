//! Daemon that runs the application in the background

use super::{chord::Chain, keyboard::Keyboard};
use std::{collections::BTreeMap, fmt};
use crate::parser::Action;

// =================== Daemon =====================

/// Global daemon state object.
#[derive(Debug)]
pub(crate) struct Daemon<'a> {
    /// The current keyboard setup
    keyboard:        Keyboard<'a>,
    /// Max allowed time between keypresses
    keypress_timeout: u32,
    /// Current chain being pressed
    current_chain:    Chain,
    /// The bindings registered in all modes.
    bindings:         BTreeMap<Chain, Action>,
    // /// Tracker of last keypress
    // last_keypress:    Timestamp,
}
