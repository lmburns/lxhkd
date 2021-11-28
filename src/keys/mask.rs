//! Key modifier masks

use super::{chord::KeyModifier, types::KeyCodeMask};
use xkbcommon::xkb::ModMask;
// use super::chord::{KeyCodeMask, KeyCodeValue};

// TODO: add Mode_switch, super, hyper

/// Ignore the Num_Lock modifier mask
pub(crate) const IGNORE_MASK: ModMask = xkb::ModMask(xcb::x::MOD_MASK_LOCK | xcb::MOD_MASK_2); // 0x12
pub(crate) const CTRL_MASK: ModMask = ModMask(xcb::MOD_MASK_CONTROL); // 0x04
pub(crate) const SHIFT_MASK: ModMask = ModMask(xcb::MOD_MASK_SHIFT); // 0x01
pub(crate) const SHIFT_CAPS_MASK: ModMask = ModMask(xcb::MOD_MASK_SHIFT | xcb::MOD_MASK_LOCK); // 0x03
pub(crate) const LOCK_MASK: ModMask = ModMask(xcb::MOD_MASK_LOCK); // 0x02
pub(crate) const ANY_MASK: ModMask = ModMask(xcb::MOD_MASK_ANY); // 0x8000
pub(crate) const MOD1_MASK: ModMask = ModMask(xcb::MOD_MASK_1); // 0x08
pub(crate) const MOD2_MASK: ModMask = ModMask(xcb::MOD_MASK_2); // 0x10
pub(crate) const MOD3_MASK: ModMask = ModMask(xcb::MOD_MASK_3); // 0x20
pub(crate) const MOD4_MASK: ModMask = ModMask(xcb::MOD_MASK_4); // 0x40
pub(crate) const MOD5_MASK: ModMask = ModMask(xcb::MOD_MASK_5); // 0x80

pub(crate) const NUM_MASK: xkb::ModMask = xkb::ModMask(xcb::MOD_MASK_2); // 0x10

// pub const MODS_CTRL_MASK: u8 = 0x01;
// pub const MODS_SHIFT_MASK: u8 = 0x02;
// pub const MODS_META_MASK: u8 = 0x04;
// pub const MODS_ALT_MASK: u8 = 0x08;
// pub const MODS_SUPER_MASK: u8 = 0x10;
// pub const MODS_KEY_MASK: u8 = 0x1f;

/// [`ModMask`](xkb::ModMask) wrapper to implement methods found under one
/// struct
pub(crate) struct XModMask {
    pub(crate) inner: ModMask,
}

impl XModMask {
    /// Combine masks
    pub(crate) fn combine(&mut self, other: XModMask) {
        self.inner = ModMask(self.inner.0 as KeyCodeMask | other.inner.0 as KeyCodeMask);
    }

    /// Filter out the ignored masks (NUM_LOCK) affects the overall mask, so it
    /// needs to be removed
    pub(crate) fn filter_ignored(&mut self) {
        self.inner = ModMask(self.inner.0 as KeyCodeMask & !IGNORE_MASK.0);
    }

    /// Get the mask from a string
    pub(crate) fn from_modifier(&mut self, modifier: KeyModifier) -> bool {
        let mask: XModMask = modifier.into();
        mask.filter_ignored();
        self.combine(mask);

        mask.inner.0 != 0
    }

    /// Determine if the [`Modmask`](xkb::Modmask) contains a `ctrl` modifier
    pub(crate) fn has_ctrl(&self) -> bool {
        (self.inner & CTRL_MASK) != 0
    }

    /// Determine if the [`Modmask`](xkb::Modmask) contains a `shift` modifier
    pub(crate) fn has_shift(&self) -> bool {
        (self.inner & SHIFT_MASK) != 0
    }

    /// Determine if the [`Modmask`](xkb::Modmask) contains a `lock` modifier
    pub(crate) fn has_shift(&self) -> bool {
        (self.inner & LOCK_MASK) != 0
    }

    /// Determine if the [`Modmask`](xkb::Modmask) contains a `mod1` modifier
    pub(crate) fn has_mod1(&self) -> bool {
        (self.inner & MOD1_MASK) != 0
    }

    /// Determine if the [`Modmask`](xkb::Modmask) contains a `mod2` modifier
    pub(crate) fn has_mod2(&self) -> bool {
        (self.inner & MOD2_MASK) != 0
    }

    /// Determine if the [`Modmask`](xkb::Modmask) contains a `mod3` modifier
    pub(crate) fn has_mod3(&self) -> bool {
        (self.inner & MOD3_MASK) != 0
    }

    /// Determine if the [`Modmask`](xkb::Modmask) contains a `mod4` modifier
    pub(crate) fn has_mod4(&self) -> bool {
        (self.inner & MOD4_MASK) != 0
    }

    /// Determine if the [`Modmask`](xkb::Modmask) contains a `mod5` modifier
    pub(crate) fn has_mod5(&self) -> bool {
        (self.inner & MOD5_MASK) != 0
    }

    /// Determine if the [`Modmask`](xkb::Modmask) contains `any` modifier
    pub(crate) fn has_any(&self) -> bool {
        (self.inner & ANY_MASK) != 0
    }

    // pub(crate) fn has_all(&self, fields: u8) -> bool {
    //     let fields = fields & MODS_KEY_MASK;
    //     (self.fields & fields) == fields
    // }
    // pub(crate) fn has_any(&self, fields: u8) -> bool {
    //     let fields = fields & MODS_KEY_MASK;
    //     (self.fields & fields) != 0
    // }
    // pub(crate) fn has_none(&self, fields: u8) -> bool {
    //     let fields = fields & MODS_KEY_MASK;
    //     (self.fields & fields) == 0
    // }
}

impl From<KeyModifier> for ModMask {
    fn from(modifier: KeyModifier) -> ModMask {
        match modifier {
            KeyModifier::Shift => ModMask(xcb::MOD_MASK_SHIFT),
            KeyModifier::Lock => ModMask(xcb::MOD_MASK_LOCK),
            KeyModifier::Ctrl => ModMask(xcb::MOD_MASK_CONTROL),
            KeyModifier::Alt | KeyModifier::Mod1 => ModMask(xcb::MOD_MASK_1),
            KeyModifier::Mod2 => ModMask(xcb::MOD_MASK_2),
            KeyModifier::Mod3 => ModMask(xcb::MOD_MASK_3),
            KeyModifier::Super | KeyModifier::Mod4 => ModMask(xcb::MOD_MASK_4),
            KeyModifier::Mod5 => ModMask(xcb::MOD_MASK_5),
            KeyModifier::Any => ModMask(xcb::MOD_MASK_ANY),
            KeyModifier::None => ModMask(0),
        }
    }
}

impl From<KeyModifier> for XModMask {
    fn from(modifier: KeyModifier) -> XModMask {
        match modifier {
            KeyModifier::Shift => XModMask::from(ModMask(xcb::MOD_MASK_SHIFT)),
            KeyModifier::Lock => XModMask::from(ModMask(xcb::MOD_MASK_LOCK)),
            KeyModifier::Ctrl => XModMask::from(ModMask(xcb::MOD_MASK_CONTROL)),
            KeyModifier::Alt | KeyModifier::Mod1 => XModMask::from(ModMask(xcb::MOD_MASK_1)),
            KeyModifier::Mod2 => XModMask::from(ModMask(xcb::MOD_MASK_2)),
            KeyModifier::Mod3 => XModMask::from(ModMask(xcb::MOD_MASK_3)),
            KeyModifier::Super | KeyModifier::Mod4 => XModMask::from(ModMask(xcb::MOD_MASK_4)),
            KeyModifier::Mod5 => XModMask::from(ModMask(xcb::MOD_MASK_5)),
            KeyModifier::Any => XModMask::from(ModMask(xcb::MOD_MASK_ANY)),
            KeyModifier::None => XModMask::from(ModMask(0)),
        }
    }
}

impl From<xkb::ModMask> for XModMask {
    fn from(inner: xkb::ModMask) -> XModMask {
        XModMask { inner }
    }
}
