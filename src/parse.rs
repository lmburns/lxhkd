use crate::types::{
    MOD_MASK_1, MOD_MASK_2, MOD_MASK_3, MOD_MASK_4, MOD_MASK_5, MOD_MASK_ANY, MOD_MASK_CONTROL,
    MOD_MASK_LOCK, MOD_MASK_SHIFT,
};

use xcb::ffi::{xcb_keysym_t, xcb_keycode_t, xcb_setup_t};

pub(crate) fn parse_modifier<'a>(name: &str, field: &'a mut u16) -> (bool, &'a mut u16) {
    if name == "shift" {
        *field |= MOD_MASK_SHIFT;
        (true, field)
    } else if name == "control" || name == "ctrl" {
        *field |= MOD_MASK_CONTROL;
        (true, field)
    } else if name == "mod1" {
        *field |= MOD_MASK_1;
        (true, field)
    } else if name == "mod2" {
        *field |= MOD_MASK_2;
        (true, field)
    } else if name == "mod3" {
        *field |= MOD_MASK_3;
        (true, field)
    } else if name == "mod4" {
        *field |= MOD_MASK_4;
        (true, field)
    } else if name == "mod5" {
        *field |= MOD_MASK_5;
        (true, field)
    } else if name == "lock" {
        *field |= MOD_MASK_LOCK;
        (true, field)
    } else if name == "any" {
        *field |= MOD_MASK_ANY;
        (true, field)
    } else {
        (false, field)
    }
}
