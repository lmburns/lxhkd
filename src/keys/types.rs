// pub type KeyCodeMask = u16;
// pub type KeyCodeValue = u8;

/// Mask value for an X keypress
pub(crate) type KeyCodeMask = xkbcommon::xkb::ffi::xkb_mod_mask_t;
/// Key code value for an X keypress
pub(crate) type KeyCodeValue = xkbcommon::xkb::ffi::xkb_keycode_t;

/// Wrapper type for the event returned by `xcb`
pub(crate) type XkbEvent = xcb::Event::Xkb;
