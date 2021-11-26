use crate::lxhkd_fatal;
use anyhow::Result;
use colored::Colorize;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("failed to get next item in screen iterator")]
    RootNext,
}

/// Setup X11 connection
pub(crate) fn setup_connection() -> Result<(xcb::Connection, xcb::Window)> {
    let (conn, _screen_num) = xcb::Connection::connect(None)?;
    let setup = conn.get_setup();
    let screen = setup.roots().next().ok_or(Error::RootNext)?.root();

    conn.flush();

    Ok((conn, screen))
}

/// Grab the keyboard
pub(crate) fn grab_keyboard(conn: &xcb::Connection, screen: xcb::Window) {
    let cookie = xcb::grab_keyboard(
        conn,
        true, // owner events
        screen,
        xcb::CURRENT_TIME,
        xcb::GRAB_MODE_ASYNC as u8, // pointer mode
        xcb::GRAB_MODE_ASYNC as u8, // keyboard mode
    );

    match cookie.get_reply() {
        Ok(reply) =>
            if reply.status() != xcb::GRAB_STATUS_SUCCESS as u8 {
                lxhkd_fatal!("failed to grab keyboard. Reply: {}", reply.status())
            },
        Err(e) => lxhkd_fatal!("failed to grab keyboard: {}", e),
    }
}

/// Ungrab/release the keyboard
pub(crate) fn ungrab_keyboard(conn: &xcb::Connection) {
    if let Err(e) = xcb::ungrab_keyboard(conn, xcb::CURRENT_TIME).request_check() {
        lxhkd_fatal!("failed to ungrab keyboard: {}", e);
    }
}

// pub fn grab_keys(&self, window: &Window, keys: &KeyMap) {
//     let key_symbols = keysyms::KeySymbols::new(&self.connection);
//     for key in keys.key_map.keys() {
//         xcb::grab_key(
//             &self.connection,
//             false,
//             window.as_xcb_window(),
//             key.modifier as u16,
//             key_symbols
//                 .get_keycode(key.key)
//                 .next()
//                 .expect("Could not resolve keysym"),
//             xcb::GRAB_MODE_ASYNC as u8,
//             xcb::GRAB_MODE_ASYNC as u8,
//         );
//     }
// }

// /// Register intercepts for each given [KeyCode]
// pub fn grab_keys(&self, keys: &[&KeyCode]) -> Result<()> {
//     // We need to explicitly grab NumLock as an additional modifier and then
// drop it     // later on when we are passing events through to the
// WindowManager as     // NumLock alters the modifier mask when it is active.
//     let modifiers = &[0, xcb::MOD_MASK_2 as u16];
//     let mode = xcb::GRAB_MODE_ASYNC as u8;
//
//     for m in modifiers.iter() {
//         for k in keys.iter() {
//             // xcb docs: https://www.mankier.com/3/xcb_grab_key
//             xcb::grab_key_checked(
//                 &self.conn, // xcb connection to X11
//                 false,      // don't pass grabbed events through to the
// client                 self.root,  // the window to grab: in this case the
// root window                 k.mask | m, // modifiers to grab
//                 k.code,     // keycode to grab
//                 mode,       // don't lock pointer input while grabbing
//                 mode,       // don't lock keyboard input while grabbing
//             )
//             .request_check()?;
//         }
//     }
//
//     self.flush();
//     Ok(())
// }

// /// Drop all active intercepts for key combinations
// pub fn ungrab_keys(&self) -> Result<()> {
//     Ok(xcb::ungrab_key_checked(
//         &self.conn, // xcb connection to X11
//         xcb::GRAB_ANY as u8,
//         self.root, // the window to ungrab keys for
//         xcb::MOD_MASK_ANY as u16,
//     )
//     .request_check()?)
// }

/// Grab a keypress
pub(crate) fn grab_key(conn: &xcb::Connection, screen: &xcb::Window) {}
