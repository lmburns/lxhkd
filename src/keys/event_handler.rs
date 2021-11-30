//! Handle `Events` sent from the X-Server

use anyhow::{Result, Context};
use super::{keyboard::Keyboard, keys::CharacterMap};
use x11rb::protocol::xproto::KeyPressEvent;
use colored::Colorize;

pub(crate) struct Handler;

impl Handler {
    pub(crate) fn handle_keypress(event: &KeyPressEvent, keyboard: &Keyboard) -> Option<CharacterMap> {
        // let kc = XKeyCode::from(ev);
        let keycode = event.detail;
        let mask = event.state;
        let charmap = CharacterMap::charmap_from_keycode(&keyboard.charmap, keycode)
            .with_context(|| format!("failed to find a `CharacterMap` for keycode={}", keycode))
            .ok()?;

        if charmap.modmask == mask {
            log::debug!("found `CharacterMap`: {:#?}", charmap);
            Some(charmap)
        } else {
            log::debug!(
                "found `CharacterMap`: {:#?}\nHowever, the mask differs from the one sent by the \
                 X-Server\n{} != {}",
                charmap,
                mask.to_string().red().bold(),
                charmap.modmask.to_string().purple().bold()
            );
            None
        }
    }
}
