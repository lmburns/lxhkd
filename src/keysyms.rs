use std::convert::AsRef;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString};

/// X keysym mappings.
#[allow(non_camel_case_types)]
#[derive(AsRefStr, EnumString, EnumIter, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum XKeySym {
    #[strum(serialize = "BackSpace")]
    XK_BackSpace,
    #[strum(serialize = "Tab")]
    XK_Tab,
    #[strum(serialize = "Linefeed")]
    XK_Linefeed,
    #[strum(serialize = "Clear")]
    XK_Clear,
    #[strum(serialize = "Return")]
    XK_Return,
    #[strum(serialize = "Pause")]
    XK_Pause,
    #[strum(serialize = "Scroll_Lock")]
    XK_Scroll_Lock,
    #[strum(serialize = "Sys_Req")]
    XK_Sys_Req,
    #[strum(serialize = "Escape")]
    XK_Escape,
    #[strum(serialize = "Delete")]
    XK_Delete,
    #[strum(serialize = "Multi_key")]
    XK_Multi_key,
    #[strum(serialize = "Kanji")]
    XK_Kanji,
    #[strum(serialize = "Muhenkan")]
    XK_Muhenkan,
    #[strum(serialize = "Henkan_Mode")]
    XK_Henkan_Mode,
    #[strum(serialize = "Henkan")]
    XK_Henkan,
    #[strum(serialize = "Romaji")]
    XK_Romaji,
    #[strum(serialize = "Hiragana")]
    XK_Hiragana,
    #[strum(serialize = "Katakana")]
    XK_Katakana,
    #[strum(serialize = "Hiragana_Katakana")]
    XK_Hiragana_Katakana,
    #[strum(serialize = "Zenkaku")]
    XK_Zenkaku,
    #[strum(serialize = "Hankaku")]
    XK_Hankaku,
    #[strum(serialize = "Zenkaku_Hankaku")]
    XK_Zenkaku_Hankaku,
    #[strum(serialize = "Touroku")]
    XK_Touroku,
    #[strum(serialize = "Massyo")]
    XK_Massyo,
    #[strum(serialize = "Kana_Lock")]
    XK_Kana_Lock,
    #[strum(serialize = "Kana_Shift")]
    XK_Kana_Shift,
    #[strum(serialize = "Eisu_Shift")]
    XK_Eisu_Shift,
    #[strum(serialize = "Eisu_toggle")]
    XK_Eisu_toggle,
    #[strum(serialize = "Home")]
    XK_Home,
    #[strum(serialize = "Left")]
    XK_Left,
    #[strum(serialize = "Up")]
    XK_Up,
    #[strum(serialize = "Right")]
    XK_Right,
    #[strum(serialize = "Down")]
    XK_Down,
    #[strum(serialize = "Prior")]
    XK_Prior,
    #[strum(serialize = "Page_Up")]
    XK_Page_Up,
    #[strum(serialize = "Next")]
    XK_Next,
    #[strum(serialize = "Page_Down")]
    XK_Page_Down,
    #[strum(serialize = "End")]
    XK_End,
    #[strum(serialize = "Begin")]
    XK_Begin,
    #[strum(serialize = "Win_L")]
    XK_Win_L,
    #[strum(serialize = "Win_R")]
    XK_Win_R,
    #[strum(serialize = "App")]
    XK_App,
    #[strum(serialize = "Select")]
    XK_Select,
    #[strum(serialize = "Print")]
    XK_Print,
    #[strum(serialize = "Execute")]
    XK_Execute,
    #[strum(serialize = "Insert")]
    XK_Insert,
    #[strum(serialize = "Undo")]
    XK_Undo,
    #[strum(serialize = "Redo")]
    XK_Redo,
    #[strum(serialize = "Menu")]
    XK_Menu,
    #[strum(serialize = "Find")]
    XK_Find,
    #[strum(serialize = "Cancel")]
    XK_Cancel,
    #[strum(serialize = "Help")]
    XK_Help,
    #[strum(serialize = "Break")]
    XK_Break,
    #[strum(serialize = "Mode_switch")]
    XK_Mode_switch,
    #[strum(serialize = "script_switch")]
    XK_script_switch,
    #[strum(serialize = "Num_Lock")]
    XK_Num_Lock,
    #[strum(serialize = "KP_Space")]
    XK_KP_Space,
    #[strum(serialize = "KP_Tab")]
    XK_KP_Tab,
    #[strum(serialize = "KP_Enter")]
    XK_KP_Enter,
    #[strum(serialize = "KP_F1")]
    XK_KP_F1,
    #[strum(serialize = "KP_F2")]
    XK_KP_F2,
    #[strum(serialize = "KP_F3")]
    XK_KP_F3,
    #[strum(serialize = "KP_F4")]
    XK_KP_F4,
    #[strum(serialize = "KP_Home")]
    XK_KP_Home,
    #[strum(serialize = "KP_Left")]
    XK_KP_Left,
    #[strum(serialize = "KP_Up")]
    XK_KP_Up,
    #[strum(serialize = "KP_Right")]
    XK_KP_Right,
    #[strum(serialize = "KP_Down")]
    XK_KP_Down,
    #[strum(serialize = "KP_Prior")]
    XK_KP_Prior,
    #[strum(serialize = "KP_Page_Up")]
    XK_KP_Page_Up,
    #[strum(serialize = "KP_Next")]
    XK_KP_Next,
    #[strum(serialize = "KP_Page_Down")]
    XK_KP_Page_Down,
    #[strum(serialize = "KP_End")]
    XK_KP_End,
    #[strum(serialize = "KP_Begin")]
    XK_KP_Begin,
    #[strum(serialize = "KP_Insert")]
    XK_KP_Insert,
    #[strum(serialize = "KP_Delete")]
    XK_KP_Delete,
    #[strum(serialize = "KP_Equal")]
    XK_KP_Equal,
    #[strum(serialize = "KP_Multiply")]
    XK_KP_Multiply,
    #[strum(serialize = "KP_Add")]
    XK_KP_Add,
    #[strum(serialize = "KP_Separator")]
    XK_KP_Separator,
    #[strum(serialize = "KP_Subtract")]
    XK_KP_Subtract,
    #[strum(serialize = "KP_Decimal")]
    XK_KP_Decimal,
    #[strum(serialize = "KP_Divide")]
    XK_KP_Divide,
    #[strum(serialize = "KP_0")]
    XK_KP_0,
    #[strum(serialize = "KP_1")]
    XK_KP_1,
    #[strum(serialize = "KP_2")]
    XK_KP_2,
    #[strum(serialize = "KP_3")]
    XK_KP_3,
    #[strum(serialize = "KP_4")]
    XK_KP_4,
    #[strum(serialize = "KP_5")]
    XK_KP_5,
    #[strum(serialize = "KP_6")]
    XK_KP_6,
    #[strum(serialize = "KP_7")]
    XK_KP_7,
    #[strum(serialize = "KP_8")]
    XK_KP_8,
    #[strum(serialize = "KP_9")]
    XK_KP_9,
    #[strum(serialize = "F1")]
    XK_F1,
    #[strum(serialize = "F2")]
    XK_F2,
    #[strum(serialize = "F3")]
    XK_F3,
    #[strum(serialize = "F4")]
    XK_F4,
    #[strum(serialize = "F5")]
    XK_F5,
    #[strum(serialize = "F6")]
    XK_F6,
    #[strum(serialize = "F7")]
    XK_F7,
    #[strum(serialize = "F8")]
    XK_F8,
    #[strum(serialize = "F9")]
    XK_F9,
    #[strum(serialize = "F10")]
    XK_F10,
    #[strum(serialize = "F11")]
    XK_F11,
    #[strum(serialize = "L1")]
    XK_L1,
    #[strum(serialize = "F12")]
    XK_F12,
    #[strum(serialize = "L2")]
    XK_L2,
    #[strum(serialize = "F13")]
    XK_F13,
    #[strum(serialize = "L3")]
    XK_L3,
    #[strum(serialize = "F14")]
    XK_F14,
    #[strum(serialize = "L4")]
    XK_L4,
    #[strum(serialize = "F15")]
    XK_F15,
    #[strum(serialize = "L5")]
    XK_L5,
    #[strum(serialize = "F16")]
    XK_F16,
    #[strum(serialize = "L6")]
    XK_L6,
    #[strum(serialize = "F17")]
    XK_F17,
    #[strum(serialize = "L7")]
    XK_L7,
    #[strum(serialize = "F18")]
    XK_F18,
    #[strum(serialize = "L8")]
    XK_L8,
    #[strum(serialize = "F19")]
    XK_F19,
    #[strum(serialize = "L9")]
    XK_L9,
    #[strum(serialize = "F20")]
    XK_F20,
    #[strum(serialize = "L10")]
    XK_L10,
    #[strum(serialize = "F21")]
    XK_F21,
    #[strum(serialize = "R1")]
    XK_R1,
    #[strum(serialize = "F22")]
    XK_F22,
    #[strum(serialize = "R2")]
    XK_R2,
    #[strum(serialize = "F23")]
    XK_F23,
    #[strum(serialize = "R3")]
    XK_R3,
    #[strum(serialize = "F24")]
    XK_F24,
    #[strum(serialize = "R4")]
    XK_R4,
    #[strum(serialize = "F25")]
    XK_F25,
    #[strum(serialize = "R5")]
    XK_R5,
    #[strum(serialize = "F26")]
    XK_F26,
    #[strum(serialize = "R6")]
    XK_R6,
    #[strum(serialize = "F27")]
    XK_F27,
    #[strum(serialize = "R7")]
    XK_R7,
    #[strum(serialize = "F28")]
    XK_F28,
    #[strum(serialize = "R8")]
    XK_R8,
    #[strum(serialize = "F29")]
    XK_F29,
    #[strum(serialize = "R9")]
    XK_R9,
    #[strum(serialize = "F30")]
    XK_F30,
    #[strum(serialize = "R10")]
    XK_R10,
    #[strum(serialize = "F31")]
    XK_F31,
    #[strum(serialize = "R11")]
    XK_R11,
    #[strum(serialize = "F32")]
    XK_F32,
    #[strum(serialize = "R12")]
    XK_R12,
    #[strum(serialize = "F33")]
    XK_F33,
    #[strum(serialize = "R13")]
    XK_R13,
    #[strum(serialize = "F34")]
    XK_F34,
    #[strum(serialize = "R14")]
    XK_R14,
    #[strum(serialize = "F35")]
    XK_F35,
    #[strum(serialize = "R15")]
    XK_R15,
    #[strum(serialize = "Shift_L")]
    XK_Shift_L,
    #[strum(serialize = "Shift_R")]
    XK_Shift_R,
    #[strum(serialize = "Control_L")]
    XK_Control_L,
    #[strum(serialize = "Control_R")]
    XK_Control_R,
    #[strum(serialize = "Caps_Lock")]
    XK_Caps_Lock,
    #[strum(serialize = "Shift_Lock")]
    XK_Shift_Lock,
    #[strum(serialize = "Meta_L")]
    XK_Meta_L,
    #[strum(serialize = "Meta_R")]
    XK_Meta_R,
    #[strum(serialize = "Alt_L")]
    XK_Alt_L,
    #[strum(serialize = "Alt_R")]
    XK_Alt_R,
    #[strum(serialize = "Super_L")]
    XK_Super_L,
    #[strum(serialize = "Super_R")]
    XK_Super_R,
    #[strum(serialize = "Hyper_L")]
    XK_Hyper_L,
    #[strum(serialize = "Hyper_R")]
    XK_Hyper_R,
    #[strum(serialize = "space")]
    XK_space,
    #[strum(serialize = "exclam")]
    XK_exclam,
    #[strum(serialize = "quotedbl")]
    XK_quotedbl,
    #[strum(serialize = "numbersign")]
    XK_numbersign,
    #[strum(serialize = "dollar")]
    XK_dollar,
    #[strum(serialize = "percent")]
    XK_percent,
    #[strum(serialize = "ampersand")]
    XK_ampersand,
    #[strum(serialize = "apostrophe")]
    XK_apostrophe,
    #[strum(serialize = "quoteright")]
    XK_quoteright,
    #[strum(serialize = "parenleft")]
    XK_parenleft,
    #[strum(serialize = "parenright")]
    XK_parenright,
    #[strum(serialize = "asterisk")]
    XK_asterisk,
    #[strum(serialize = "plus")]
    XK_plus,
    #[strum(serialize = "comma")]
    XK_comma,
    #[strum(serialize = "minus")]
    XK_minus,
    #[strum(serialize = "period")]
    XK_period,
    #[strum(serialize = "slash")]
    XK_slash,
    #[strum(serialize = "0")]
    XK_0,
    #[strum(serialize = "1")]
    XK_1,
    #[strum(serialize = "2")]
    XK_2,
    #[strum(serialize = "3")]
    XK_3,
    #[strum(serialize = "4")]
    XK_4,
    #[strum(serialize = "5")]
    XK_5,
    #[strum(serialize = "6")]
    XK_6,
    #[strum(serialize = "7")]
    XK_7,
    #[strum(serialize = "8")]
    XK_8,
    #[strum(serialize = "9")]
    XK_9,
    #[strum(serialize = "colon")]
    XK_colon,
    #[strum(serialize = "semicolon")]
    XK_semicolon,
    #[strum(serialize = "less")]
    XK_less,
    #[strum(serialize = "equal")]
    XK_equal,
    #[strum(serialize = "greater")]
    XK_greater,
    #[strum(serialize = "question")]
    XK_question,
    #[strum(serialize = "at")]
    XK_at,
    #[strum(serialize = "A")]
    XK_A,
    #[strum(serialize = "B")]
    XK_B,
    #[strum(serialize = "C")]
    XK_C,
    #[strum(serialize = "D")]
    XK_D,
    #[strum(serialize = "E")]
    XK_E,
    #[strum(serialize = "F")]
    XK_F,
    #[strum(serialize = "G")]
    XK_G,
    #[strum(serialize = "H")]
    XK_H,
    #[strum(serialize = "I")]
    XK_I,
    #[strum(serialize = "J")]
    XK_J,
    #[strum(serialize = "K")]
    XK_K,
    #[strum(serialize = "L")]
    XK_L,
    #[strum(serialize = "M")]
    XK_M,
    #[strum(serialize = "N")]
    XK_N,
    #[strum(serialize = "O")]
    XK_O,
    #[strum(serialize = "P")]
    XK_P,
    #[strum(serialize = "Q")]
    XK_Q,
    #[strum(serialize = "R")]
    XK_R,
    #[strum(serialize = "S")]
    XK_S,
    #[strum(serialize = "T")]
    XK_T,
    #[strum(serialize = "U")]
    XK_U,
    #[strum(serialize = "V")]
    XK_V,
    #[strum(serialize = "W")]
    XK_W,
    #[strum(serialize = "X")]
    XK_X,
    #[strum(serialize = "Y")]
    XK_Y,
    #[strum(serialize = "Z")]
    XK_Z,
    #[strum(serialize = "bracketleft")]
    XK_bracketleft,
    #[strum(serialize = "backslash")]
    XK_backslash,
    #[strum(serialize = "bracketright")]
    XK_bracketright,
    #[strum(serialize = "asciicircum")]
    XK_asciicircum,
    #[strum(serialize = "underscore")]
    XK_underscore,
    #[strum(serialize = "grave")]
    XK_grave,
    #[strum(serialize = "quoteleft")]
    XK_quoteleft,
    #[strum(serialize = "a")]
    XK_a,
    #[strum(serialize = "b")]
    XK_b,
    #[strum(serialize = "c")]
    XK_c,
    #[strum(serialize = "d")]
    XK_d,
    #[strum(serialize = "e")]
    XK_e,
    #[strum(serialize = "f")]
    XK_f,
    #[strum(serialize = "g")]
    XK_g,
    #[strum(serialize = "h")]
    XK_h,
    #[strum(serialize = "i")]
    XK_i,
    #[strum(serialize = "j")]
    XK_j,
    #[strum(serialize = "k")]
    XK_k,
    #[strum(serialize = "l")]
    XK_l,
    #[strum(serialize = "m")]
    XK_m,
    #[strum(serialize = "n")]
    XK_n,
    #[strum(serialize = "o")]
    XK_o,
    #[strum(serialize = "p")]
    XK_p,
    #[strum(serialize = "q")]
    XK_q,
    #[strum(serialize = "r")]
    XK_r,
    #[strum(serialize = "s")]
    XK_s,
    #[strum(serialize = "t")]
    XK_t,
    #[strum(serialize = "u")]
    XK_u,
    #[strum(serialize = "v")]
    XK_v,
    #[strum(serialize = "w")]
    XK_w,
    #[strum(serialize = "x")]
    XK_x,
    #[strum(serialize = "y")]
    XK_y,
    #[strum(serialize = "z")]
    XK_z,
    #[strum(serialize = "braceleft")]
    XK_braceleft,
    #[strum(serialize = "bar")]
    XK_bar,
    #[strum(serialize = "braceright")]
    XK_braceright,
    #[strum(serialize = "asciitilde")]
    XK_asciitilde,
    #[strum(serialize = "nobreakspace")]
    XK_nobreakspace,
    #[strum(serialize = "exclamdown")]
    XK_exclamdown,
    #[strum(serialize = "cent")]
    XK_cent,
    #[strum(serialize = "sterling")]
    XK_sterling,
    #[strum(serialize = "currency")]
    XK_currency,
    #[strum(serialize = "yen")]
    XK_yen,
    #[strum(serialize = "brokenbar")]
    XK_brokenbar,
    #[strum(serialize = "section")]
    XK_section,
    #[strum(serialize = "diaeresis")]
    XK_diaeresis,
    #[strum(serialize = "copyright")]
    XK_copyright,
    #[strum(serialize = "ordfeminine")]
    XK_ordfeminine,
    #[strum(serialize = "guillemotleft")]
    XK_guillemotleft,
    #[strum(serialize = "notsign")]
    XK_notsign,
    #[strum(serialize = "hyphen")]
    XK_hyphen,
    #[strum(serialize = "registered")]
    XK_registered,
    #[strum(serialize = "macron")]
    XK_macron,
    #[strum(serialize = "degree")]
    XK_degree,
    #[strum(serialize = "plusminus")]
    XK_plusminus,
    #[strum(serialize = "twosuperior")]
    XK_twosuperior,
    #[strum(serialize = "threesuperior")]
    XK_threesuperior,
    #[strum(serialize = "acute")]
    XK_acute,
    #[strum(serialize = "mu")]
    XK_mu,
    #[strum(serialize = "paragraph")]
    XK_paragraph,
    #[strum(serialize = "periodcentered")]
    XK_periodcentered,
    #[strum(serialize = "cedilla")]
    XK_cedilla,
    #[strum(serialize = "onesuperior")]
    XK_onesuperior,
    #[strum(serialize = "masculine")]
    XK_masculine,
    #[strum(serialize = "guillemotright")]
    XK_guillemotright,
    #[strum(serialize = "onequarter")]
    XK_onequarter,
    #[strum(serialize = "onehalf")]
    XK_onehalf,
    #[strum(serialize = "threequarters")]
    XK_threequarters,
    #[strum(serialize = "questiondown")]
    XK_questiondown,
    #[strum(serialize = "Agrave")]
    XK_Agrave,
    #[strum(serialize = "Aacute")]
    XK_Aacute,
    #[strum(serialize = "Acircumflex")]
    XK_Acircumflex,
    #[strum(serialize = "Atilde")]
    XK_Atilde,
    #[strum(serialize = "Adiaeresis")]
    XK_Adiaeresis,
    #[strum(serialize = "Aring")]
    XK_Aring,
    #[strum(serialize = "AE")]
    XK_AE,
    #[strum(serialize = "Ccedilla")]
    XK_Ccedilla,
    #[strum(serialize = "Egrave")]
    XK_Egrave,
    #[strum(serialize = "Eacute")]
    XK_Eacute,
    #[strum(serialize = "Ecircumflex")]
    XK_Ecircumflex,
    #[strum(serialize = "Ediaeresis")]
    XK_Ediaeresis,
    #[strum(serialize = "Igrave")]
    XK_Igrave,
    #[strum(serialize = "Iacute")]
    XK_Iacute,
    #[strum(serialize = "Icircumflex")]
    XK_Icircumflex,
    #[strum(serialize = "Idiaeresis")]
    XK_Idiaeresis,
    #[strum(serialize = "ETH")]
    XK_ETH,
    #[strum(serialize = "Eth")]
    XK_Eth,
    #[strum(serialize = "Ntilde")]
    XK_Ntilde,
    #[strum(serialize = "Ograve")]
    XK_Ograve,
    #[strum(serialize = "Oacute")]
    XK_Oacute,
    #[strum(serialize = "Ocircumflex")]
    XK_Ocircumflex,
    #[strum(serialize = "Otilde")]
    XK_Otilde,
    #[strum(serialize = "Odiaeresis")]
    XK_Odiaeresis,
    #[strum(serialize = "multiply")]
    XK_multiply,
    #[strum(serialize = "Ooblique")]
    XK_Ooblique,
    #[strum(serialize = "Ugrave")]
    XK_Ugrave,
    #[strum(serialize = "Uacute")]
    XK_Uacute,
    #[strum(serialize = "Ucircumflex")]
    XK_Ucircumflex,
    #[strum(serialize = "Udiaeresis")]
    XK_Udiaeresis,
    #[strum(serialize = "Yacute")]
    XK_Yacute,
    #[strum(serialize = "THORN")]
    XK_THORN,
    #[strum(serialize = "Thorn")]
    XK_Thorn,
    #[strum(serialize = "ssharp")]
    XK_ssharp,
    #[strum(serialize = "agrave")]
    XK_agrave,
    #[strum(serialize = "aacute")]
    XK_aacute,
    #[strum(serialize = "acircumflex")]
    XK_acircumflex,
    #[strum(serialize = "atilde")]
    XK_atilde,
    #[strum(serialize = "adiaeresis")]
    XK_adiaeresis,
    #[strum(serialize = "aring")]
    XK_aring,
    #[strum(serialize = "ae")]
    XK_ae,
    #[strum(serialize = "ccedilla")]
    XK_ccedilla,
    #[strum(serialize = "egrave")]
    XK_egrave,
    #[strum(serialize = "eacute")]
    XK_eacute,
    #[strum(serialize = "ecircumflex")]
    XK_ecircumflex,
    #[strum(serialize = "ediaeresis")]
    XK_ediaeresis,
    #[strum(serialize = "igrave")]
    XK_igrave,
    #[strum(serialize = "iacute")]
    XK_iacute,
    #[strum(serialize = "icircumflex")]
    XK_icircumflex,
    #[strum(serialize = "idiaeresis")]
    XK_idiaeresis,
    #[strum(serialize = "eth")]
    XK_eth,
    #[strum(serialize = "ntilde")]
    XK_ntilde,
    #[strum(serialize = "ograve")]
    XK_ograve,
    #[strum(serialize = "oacute")]
    XK_oacute,
    #[strum(serialize = "ocircumflex")]
    XK_ocircumflex,
    #[strum(serialize = "otilde")]
    XK_otilde,
    #[strum(serialize = "odiaeresis")]
    XK_odiaeresis,
    #[strum(serialize = "division")]
    XK_division,
    #[strum(serialize = "oslash")]
    XK_oslash,
    #[strum(serialize = "ugrave")]
    XK_ugrave,
    #[strum(serialize = "uacute")]
    XK_uacute,
    #[strum(serialize = "ucircumflex")]
    XK_ucircumflex,
    #[strum(serialize = "udiaeresis")]
    XK_udiaeresis,
    #[strum(serialize = "yacute")]
    XK_yacute,
    #[strum(serialize = "thorn")]
    XK_thorn,
    #[strum(serialize = "ydiaeresis")]
    XK_ydiaeresis,
    #[strum(serialize = "Aogonek")]
    XK_Aogonek,
    #[strum(serialize = "breve")]
    XK_breve,
    #[strum(serialize = "Lstroke")]
    XK_Lstroke,
    #[strum(serialize = "Lcaron")]
    XK_Lcaron,
    #[strum(serialize = "Sacute")]
    XK_Sacute,
    #[strum(serialize = "Scaron")]
    XK_Scaron,
    #[strum(serialize = "Scedilla")]
    XK_Scedilla,
    #[strum(serialize = "Tcaron")]
    XK_Tcaron,
    #[strum(serialize = "Zacute")]
    XK_Zacute,
    #[strum(serialize = "Zcaron")]
    XK_Zcaron,
    #[strum(serialize = "Zabovedot")]
    XK_Zabovedot,
    #[strum(serialize = "aogonek")]
    XK_aogonek,
    #[strum(serialize = "ogonek")]
    XK_ogonek,
    #[strum(serialize = "lstroke")]
    XK_lstroke,
    #[strum(serialize = "lcaron")]
    XK_lcaron,
    #[strum(serialize = "sacute")]
    XK_sacute,
    #[strum(serialize = "caron")]
    XK_caron,
    #[strum(serialize = "scaron")]
    XK_scaron,
    #[strum(serialize = "scedilla")]
    XK_scedilla,
    #[strum(serialize = "tcaron")]
    XK_tcaron,
    #[strum(serialize = "zacute")]
    XK_zacute,
    #[strum(serialize = "doubleacute")]
    XK_doubleacute,
    #[strum(serialize = "zcaron")]
    XK_zcaron,
    #[strum(serialize = "zabovedot")]
    XK_zabovedot,
    #[strum(serialize = "Racute")]
    XK_Racute,
    #[strum(serialize = "Abreve")]
    XK_Abreve,
    #[strum(serialize = "Lacute")]
    XK_Lacute,
    #[strum(serialize = "Cacute")]
    XK_Cacute,
    #[strum(serialize = "Ccaron")]
    XK_Ccaron,
    #[strum(serialize = "Eogonek")]
    XK_Eogonek,
    #[strum(serialize = "Ecaron")]
    XK_Ecaron,
    #[strum(serialize = "Dcaron")]
    XK_Dcaron,
    #[strum(serialize = "Dstroke")]
    XK_Dstroke,
    #[strum(serialize = "Nacute")]
    XK_Nacute,
    #[strum(serialize = "Ncaron")]
    XK_Ncaron,
    #[strum(serialize = "Odoubleacute")]
    XK_Odoubleacute,
    #[strum(serialize = "Rcaron")]
    XK_Rcaron,
    #[strum(serialize = "Uring")]
    XK_Uring,
    #[strum(serialize = "Udoubleacute")]
    XK_Udoubleacute,
    #[strum(serialize = "Tcedilla")]
    XK_Tcedilla,
    #[strum(serialize = "racute")]
    XK_racute,
    #[strum(serialize = "abreve")]
    XK_abreve,
    #[strum(serialize = "lacute")]
    XK_lacute,
    #[strum(serialize = "cacute")]
    XK_cacute,
    #[strum(serialize = "ccaron")]
    XK_ccaron,
    #[strum(serialize = "eogonek")]
    XK_eogonek,
    #[strum(serialize = "ecaron")]
    XK_ecaron,
    #[strum(serialize = "dcaron")]
    XK_dcaron,
    #[strum(serialize = "dstroke")]
    XK_dstroke,
    #[strum(serialize = "nacute")]
    XK_nacute,
    #[strum(serialize = "ncaron")]
    XK_ncaron,
    #[strum(serialize = "odoubleacute")]
    XK_odoubleacute,
    #[strum(serialize = "udoubleacute")]
    XK_udoubleacute,
    #[strum(serialize = "rcaron")]
    XK_rcaron,
    #[strum(serialize = "uring")]
    XK_uring,
    #[strum(serialize = "tcedilla")]
    XK_tcedilla,
    #[strum(serialize = "abovedot")]
    XK_abovedot,
    #[strum(serialize = "Hstroke")]
    XK_Hstroke,
    #[strum(serialize = "Hcircumflex")]
    XK_Hcircumflex,
    #[strum(serialize = "Iabovedot")]
    XK_Iabovedot,
    #[strum(serialize = "Gbreve")]
    XK_Gbreve,
    #[strum(serialize = "Jcircumflex")]
    XK_Jcircumflex,
    #[strum(serialize = "hstroke")]
    XK_hstroke,
    #[strum(serialize = "hcircumflex")]
    XK_hcircumflex,
    #[strum(serialize = "idotless")]
    XK_idotless,
    #[strum(serialize = "gbreve")]
    XK_gbreve,
    #[strum(serialize = "jcircumflex")]
    XK_jcircumflex,
    #[strum(serialize = "Cabovedot")]
    XK_Cabovedot,
    #[strum(serialize = "Ccircumflex")]
    XK_Ccircumflex,
    #[strum(serialize = "Gabovedot")]
    XK_Gabovedot,
    #[strum(serialize = "Gcircumflex")]
    XK_Gcircumflex,
    #[strum(serialize = "Ubreve")]
    XK_Ubreve,
    #[strum(serialize = "Scircumflex")]
    XK_Scircumflex,
    #[strum(serialize = "cabovedot")]
    XK_cabovedot,
    #[strum(serialize = "ccircumflex")]
    XK_ccircumflex,
    #[strum(serialize = "gabovedot")]
    XK_gabovedot,
    #[strum(serialize = "gcircumflex")]
    XK_gcircumflex,
    #[strum(serialize = "ubreve")]
    XK_ubreve,
    #[strum(serialize = "scircumflex")]
    XK_scircumflex,
    #[strum(serialize = "kra")]
    XK_kra,
    #[strum(serialize = "kappa")]
    XK_kappa,
    #[strum(serialize = "Rcedilla")]
    XK_Rcedilla,
    #[strum(serialize = "Itilde")]
    XK_Itilde,
    #[strum(serialize = "Lcedilla")]
    XK_Lcedilla,
    #[strum(serialize = "Emacron")]
    XK_Emacron,
    #[strum(serialize = "Gcedilla")]
    XK_Gcedilla,
    #[strum(serialize = "Tslash")]
    XK_Tslash,
    #[strum(serialize = "rcedilla")]
    XK_rcedilla,
    #[strum(serialize = "itilde")]
    XK_itilde,
    #[strum(serialize = "lcedilla")]
    XK_lcedilla,
    #[strum(serialize = "emacron")]
    XK_emacron,
    #[strum(serialize = "gcedilla")]
    XK_gcedilla,
    #[strum(serialize = "tslash")]
    XK_tslash,
    #[strum(serialize = "ENG")]
    XK_ENG,
    #[strum(serialize = "eng")]
    XK_eng,
    #[strum(serialize = "Amacron")]
    XK_Amacron,
    #[strum(serialize = "Iogonek")]
    XK_Iogonek,
    #[strum(serialize = "Eabovedot")]
    XK_Eabovedot,
    #[strum(serialize = "Imacron")]
    XK_Imacron,
    #[strum(serialize = "Ncedilla")]
    XK_Ncedilla,
    #[strum(serialize = "Omacron")]
    XK_Omacron,
    #[strum(serialize = "Kcedilla")]
    XK_Kcedilla,
    #[strum(serialize = "Uogonek")]
    XK_Uogonek,
    #[strum(serialize = "Utilde")]
    XK_Utilde,
    #[strum(serialize = "Umacron")]
    XK_Umacron,
    #[strum(serialize = "amacron")]
    XK_amacron,
    #[strum(serialize = "iogonek")]
    XK_iogonek,
    #[strum(serialize = "eabovedot")]
    XK_eabovedot,
    #[strum(serialize = "imacron")]
    XK_imacron,
    #[strum(serialize = "ncedilla")]
    XK_ncedilla,
    #[strum(serialize = "omacron")]
    XK_omacron,
    #[strum(serialize = "kcedilla")]
    XK_kcedilla,
    #[strum(serialize = "uogonek")]
    XK_uogonek,
    #[strum(serialize = "utilde")]
    XK_utilde,
    #[strum(serialize = "umacron")]
    XK_umacron,
    #[strum(serialize = "overline")]
    XK_overline,
    #[strum(serialize = "kana_fullstop")]
    XK_kana_fullstop,
    #[strum(serialize = "kana_openingbracket")]
    XK_kana_openingbracket,
    #[strum(serialize = "kana_closingbracket")]
    XK_kana_closingbracket,
    #[strum(serialize = "kana_comma")]
    XK_kana_comma,
    #[strum(serialize = "kana_conjunctive")]
    XK_kana_conjunctive,
    #[strum(serialize = "kana_middledot")]
    XK_kana_middledot,
    #[strum(serialize = "kana_WO")]
    XK_kana_WO,
    #[strum(serialize = "kana_a")]
    XK_kana_a,
    #[strum(serialize = "kana_i")]
    XK_kana_i,
    #[strum(serialize = "kana_u")]
    XK_kana_u,
    #[strum(serialize = "kana_e")]
    XK_kana_e,
    #[strum(serialize = "kana_o")]
    XK_kana_o,
    #[strum(serialize = "kana_ya")]
    XK_kana_ya,
    #[strum(serialize = "kana_yu")]
    XK_kana_yu,
    #[strum(serialize = "kana_yo")]
    XK_kana_yo,
    #[strum(serialize = "kana_tsu")]
    XK_kana_tsu,
    #[strum(serialize = "kana_tu")]
    XK_kana_tu,
    #[strum(serialize = "prolongedsound")]
    XK_prolongedsound,
    #[strum(serialize = "kana_A")]
    XK_kana_A,
    #[strum(serialize = "kana_I")]
    XK_kana_I,
    #[strum(serialize = "kana_U")]
    XK_kana_U,
    #[strum(serialize = "kana_E")]
    XK_kana_E,
    #[strum(serialize = "kana_O")]
    XK_kana_O,
    #[strum(serialize = "kana_KA")]
    XK_kana_KA,
    #[strum(serialize = "kana_KI")]
    XK_kana_KI,
    #[strum(serialize = "kana_KU")]
    XK_kana_KU,
    #[strum(serialize = "kana_KE")]
    XK_kana_KE,
    #[strum(serialize = "kana_KO")]
    XK_kana_KO,
    #[strum(serialize = "kana_SA")]
    XK_kana_SA,
    #[strum(serialize = "kana_SHI")]
    XK_kana_SHI,
    #[strum(serialize = "kana_SU")]
    XK_kana_SU,
    #[strum(serialize = "kana_SE")]
    XK_kana_SE,
    #[strum(serialize = "kana_SO")]
    XK_kana_SO,
    #[strum(serialize = "kana_TA")]
    XK_kana_TA,
    #[strum(serialize = "kana_CHI")]
    XK_kana_CHI,
    #[strum(serialize = "kana_TI")]
    XK_kana_TI,
    #[strum(serialize = "kana_TSU")]
    XK_kana_TSU,
    #[strum(serialize = "kana_TU")]
    XK_kana_TU,
    #[strum(serialize = "kana_TE")]
    XK_kana_TE,
    #[strum(serialize = "kana_TO")]
    XK_kana_TO,
    #[strum(serialize = "kana_NA")]
    XK_kana_NA,
    #[strum(serialize = "kana_NI")]
    XK_kana_NI,
    #[strum(serialize = "kana_NU")]
    XK_kana_NU,
    #[strum(serialize = "kana_NE")]
    XK_kana_NE,
    #[strum(serialize = "kana_NO")]
    XK_kana_NO,
    #[strum(serialize = "kana_HA")]
    XK_kana_HA,
    #[strum(serialize = "kana_HI")]
    XK_kana_HI,
    #[strum(serialize = "kana_FU")]
    XK_kana_FU,
    #[strum(serialize = "kana_HU")]
    XK_kana_HU,
    #[strum(serialize = "kana_HE")]
    XK_kana_HE,
    #[strum(serialize = "kana_HO")]
    XK_kana_HO,
    #[strum(serialize = "kana_MA")]
    XK_kana_MA,
    #[strum(serialize = "kana_MI")]
    XK_kana_MI,
    #[strum(serialize = "kana_MU")]
    XK_kana_MU,
    #[strum(serialize = "kana_ME")]
    XK_kana_ME,
    #[strum(serialize = "kana_MO")]
    XK_kana_MO,
    #[strum(serialize = "kana_YA")]
    XK_kana_YA,
    #[strum(serialize = "kana_YU")]
    XK_kana_YU,
    #[strum(serialize = "kana_YO")]
    XK_kana_YO,
    #[strum(serialize = "kana_RA")]
    XK_kana_RA,
    #[strum(serialize = "kana_RI")]
    XK_kana_RI,
    #[strum(serialize = "kana_RU")]
    XK_kana_RU,
    #[strum(serialize = "kana_RE")]
    XK_kana_RE,
    #[strum(serialize = "kana_RO")]
    XK_kana_RO,
    #[strum(serialize = "kana_WA")]
    XK_kana_WA,
    #[strum(serialize = "kana_N")]
    XK_kana_N,
    #[strum(serialize = "voicedsound")]
    XK_voicedsound,
    #[strum(serialize = "semivoicedsound")]
    XK_semivoicedsound,
    #[strum(serialize = "kana_switch")]
    XK_kana_switch,
    #[strum(serialize = "Arabic_comma")]
    XK_Arabic_comma,
    #[strum(serialize = "Arabic_semicolon")]
    XK_Arabic_semicolon,
    #[strum(serialize = "Arabic_question_mark")]
    XK_Arabic_question_mark,
    #[strum(serialize = "Arabic_hamza")]
    XK_Arabic_hamza,
    #[strum(serialize = "Arabic_maddaonalef")]
    XK_Arabic_maddaonalef,
    #[strum(serialize = "Arabic_hamzaonalef")]
    XK_Arabic_hamzaonalef,
    #[strum(serialize = "Arabic_hamzaonwaw")]
    XK_Arabic_hamzaonwaw,
    #[strum(serialize = "Arabic_hamzaunderalef")]
    XK_Arabic_hamzaunderalef,
    #[strum(serialize = "Arabic_hamzaonyeh")]
    XK_Arabic_hamzaonyeh,
    #[strum(serialize = "Arabic_alef")]
    XK_Arabic_alef,
    #[strum(serialize = "Arabic_beh")]
    XK_Arabic_beh,
    #[strum(serialize = "Arabic_tehmarbuta")]
    XK_Arabic_tehmarbuta,
    #[strum(serialize = "Arabic_teh")]
    XK_Arabic_teh,
    #[strum(serialize = "Arabic_theh")]
    XK_Arabic_theh,
    #[strum(serialize = "Arabic_jeem")]
    XK_Arabic_jeem,
    #[strum(serialize = "Arabic_hah")]
    XK_Arabic_hah,
    #[strum(serialize = "Arabic_khah")]
    XK_Arabic_khah,
    #[strum(serialize = "Arabic_dal")]
    XK_Arabic_dal,
    #[strum(serialize = "Arabic_thal")]
    XK_Arabic_thal,
    #[strum(serialize = "Arabic_ra")]
    XK_Arabic_ra,
    #[strum(serialize = "Arabic_zain")]
    XK_Arabic_zain,
    #[strum(serialize = "Arabic_seen")]
    XK_Arabic_seen,
    #[strum(serialize = "Arabic_sheen")]
    XK_Arabic_sheen,
    #[strum(serialize = "Arabic_sad")]
    XK_Arabic_sad,
    #[strum(serialize = "Arabic_dad")]
    XK_Arabic_dad,
    #[strum(serialize = "Arabic_tah")]
    XK_Arabic_tah,
    #[strum(serialize = "Arabic_zah")]
    XK_Arabic_zah,
    #[strum(serialize = "Arabic_ain")]
    XK_Arabic_ain,
    #[strum(serialize = "Arabic_ghain")]
    XK_Arabic_ghain,
    #[strum(serialize = "Arabic_tatweel")]
    XK_Arabic_tatweel,
    #[strum(serialize = "Arabic_feh")]
    XK_Arabic_feh,
    #[strum(serialize = "Arabic_qaf")]
    XK_Arabic_qaf,
    #[strum(serialize = "Arabic_kaf")]
    XK_Arabic_kaf,
    #[strum(serialize = "Arabic_lam")]
    XK_Arabic_lam,
    #[strum(serialize = "Arabic_meem")]
    XK_Arabic_meem,
    #[strum(serialize = "Arabic_noon")]
    XK_Arabic_noon,
    #[strum(serialize = "Arabic_ha")]
    XK_Arabic_ha,
    #[strum(serialize = "Arabic_heh")]
    XK_Arabic_heh,
    #[strum(serialize = "Arabic_waw")]
    XK_Arabic_waw,
    #[strum(serialize = "Arabic_alefmaksura")]
    XK_Arabic_alefmaksura,
    #[strum(serialize = "Arabic_yeh")]
    XK_Arabic_yeh,
    #[strum(serialize = "Arabic_fathatan")]
    XK_Arabic_fathatan,
    #[strum(serialize = "Arabic_dammatan")]
    XK_Arabic_dammatan,
    #[strum(serialize = "Arabic_kasratan")]
    XK_Arabic_kasratan,
    #[strum(serialize = "Arabic_fatha")]
    XK_Arabic_fatha,
    #[strum(serialize = "Arabic_damma")]
    XK_Arabic_damma,
    #[strum(serialize = "Arabic_kasra")]
    XK_Arabic_kasra,
    #[strum(serialize = "Arabic_shadda")]
    XK_Arabic_shadda,
    #[strum(serialize = "Arabic_sukun")]
    XK_Arabic_sukun,
    #[strum(serialize = "Arabic_switch")]
    XK_Arabic_switch,
    #[strum(serialize = "Serbian_dje")]
    XK_Serbian_dje,
    #[strum(serialize = "Macedonia_gje")]
    XK_Macedonia_gje,
    #[strum(serialize = "Cyrillic_io")]
    XK_Cyrillic_io,
    #[strum(serialize = "Ukrainian_ie")]
    XK_Ukrainian_ie,
    #[strum(serialize = "Ukranian_je")]
    XK_Ukranian_je,
    #[strum(serialize = "Macedonia_dse")]
    XK_Macedonia_dse,
    #[strum(serialize = "Ukrainian_i")]
    XK_Ukrainian_i,
    #[strum(serialize = "Ukranian_i")]
    XK_Ukranian_i,
    #[strum(serialize = "Ukrainian_yi")]
    XK_Ukrainian_yi,
    #[strum(serialize = "Ukranian_yi")]
    XK_Ukranian_yi,
    #[strum(serialize = "Cyrillic_je")]
    XK_Cyrillic_je,
    #[strum(serialize = "Serbian_je")]
    XK_Serbian_je,
    #[strum(serialize = "Cyrillic_lje")]
    XK_Cyrillic_lje,
    #[strum(serialize = "Serbian_lje")]
    XK_Serbian_lje,
    #[strum(serialize = "Cyrillic_nje")]
    XK_Cyrillic_nje,
    #[strum(serialize = "Serbian_nje")]
    XK_Serbian_nje,
    #[strum(serialize = "Serbian_tshe")]
    XK_Serbian_tshe,
    #[strum(serialize = "Macedonia_kje")]
    XK_Macedonia_kje,
    #[strum(serialize = "Byelorussian_shortu")]
    XK_Byelorussian_shortu,
    #[strum(serialize = "Cyrillic_dzhe")]
    XK_Cyrillic_dzhe,
    #[strum(serialize = "Serbian_dze")]
    XK_Serbian_dze,
    #[strum(serialize = "numerosign")]
    XK_numerosign,
    #[strum(serialize = "Serbian_DJE")]
    XK_Serbian_DJE,
    #[strum(serialize = "Macedonia_GJE")]
    XK_Macedonia_GJE,
    #[strum(serialize = "Cyrillic_IO")]
    XK_Cyrillic_IO,
    #[strum(serialize = "Ukrainian_IE")]
    XK_Ukrainian_IE,
    #[strum(serialize = "Ukranian_JE")]
    XK_Ukranian_JE,
    #[strum(serialize = "Macedonia_DSE")]
    XK_Macedonia_DSE,
    #[strum(serialize = "Ukrainian_I")]
    XK_Ukrainian_I,
    #[strum(serialize = "Ukranian_I")]
    XK_Ukranian_I,
    #[strum(serialize = "Ukrainian_YI")]
    XK_Ukrainian_YI,
    #[strum(serialize = "Ukranian_YI")]
    XK_Ukranian_YI,
    #[strum(serialize = "Cyrillic_JE")]
    XK_Cyrillic_JE,
    #[strum(serialize = "Serbian_JE")]
    XK_Serbian_JE,
    #[strum(serialize = "Cyrillic_LJE")]
    XK_Cyrillic_LJE,
    #[strum(serialize = "Serbian_LJE")]
    XK_Serbian_LJE,
    #[strum(serialize = "Cyrillic_NJE")]
    XK_Cyrillic_NJE,
    #[strum(serialize = "Serbian_NJE")]
    XK_Serbian_NJE,
    #[strum(serialize = "Serbian_TSHE")]
    XK_Serbian_TSHE,
    #[strum(serialize = "Macedonia_KJE")]
    XK_Macedonia_KJE,
    #[strum(serialize = "Byelorussian_SHORTU")]
    XK_Byelorussian_SHORTU,
    #[strum(serialize = "Cyrillic_DZHE")]
    XK_Cyrillic_DZHE,
    #[strum(serialize = "Serbian_DZE")]
    XK_Serbian_DZE,
    #[strum(serialize = "Cyrillic_yu")]
    XK_Cyrillic_yu,
    #[strum(serialize = "Cyrillic_a")]
    XK_Cyrillic_a,
    #[strum(serialize = "Cyrillic_be")]
    XK_Cyrillic_be,
    #[strum(serialize = "Cyrillic_tse")]
    XK_Cyrillic_tse,
    #[strum(serialize = "Cyrillic_de")]
    XK_Cyrillic_de,
    #[strum(serialize = "Cyrillic_ie")]
    XK_Cyrillic_ie,
    #[strum(serialize = "Cyrillic_ef")]
    XK_Cyrillic_ef,
    #[strum(serialize = "Cyrillic_ghe")]
    XK_Cyrillic_ghe,
    #[strum(serialize = "Cyrillic_ha")]
    XK_Cyrillic_ha,
    #[strum(serialize = "Cyrillic_i")]
    XK_Cyrillic_i,
    #[strum(serialize = "Cyrillic_shorti")]
    XK_Cyrillic_shorti,
    #[strum(serialize = "Cyrillic_ka")]
    XK_Cyrillic_ka,
    #[strum(serialize = "Cyrillic_el")]
    XK_Cyrillic_el,
    #[strum(serialize = "Cyrillic_em")]
    XK_Cyrillic_em,
    #[strum(serialize = "Cyrillic_en")]
    XK_Cyrillic_en,
    #[strum(serialize = "Cyrillic_o")]
    XK_Cyrillic_o,
    #[strum(serialize = "Cyrillic_pe")]
    XK_Cyrillic_pe,
    #[strum(serialize = "Cyrillic_ya")]
    XK_Cyrillic_ya,
    #[strum(serialize = "Cyrillic_er")]
    XK_Cyrillic_er,
    #[strum(serialize = "Cyrillic_es")]
    XK_Cyrillic_es,
    #[strum(serialize = "Cyrillic_te")]
    XK_Cyrillic_te,
    #[strum(serialize = "Cyrillic_u")]
    XK_Cyrillic_u,
    #[strum(serialize = "Cyrillic_zhe")]
    XK_Cyrillic_zhe,
    #[strum(serialize = "Cyrillic_ve")]
    XK_Cyrillic_ve,
    #[strum(serialize = "Cyrillic_softsign")]
    XK_Cyrillic_softsign,
    #[strum(serialize = "Cyrillic_yeru")]
    XK_Cyrillic_yeru,
    #[strum(serialize = "Cyrillic_ze")]
    XK_Cyrillic_ze,
    #[strum(serialize = "Cyrillic_sha")]
    XK_Cyrillic_sha,
    #[strum(serialize = "Cyrillic_e")]
    XK_Cyrillic_e,
    #[strum(serialize = "Cyrillic_shcha")]
    XK_Cyrillic_shcha,
    #[strum(serialize = "Cyrillic_che")]
    XK_Cyrillic_che,
    #[strum(serialize = "Cyrillic_hardsign")]
    XK_Cyrillic_hardsign,
    #[strum(serialize = "Cyrillic_YU")]
    XK_Cyrillic_YU,
    #[strum(serialize = "Cyrillic_A")]
    XK_Cyrillic_A,
    #[strum(serialize = "Cyrillic_BE")]
    XK_Cyrillic_BE,
    #[strum(serialize = "Cyrillic_TSE")]
    XK_Cyrillic_TSE,
    #[strum(serialize = "Cyrillic_DE")]
    XK_Cyrillic_DE,
    #[strum(serialize = "Cyrillic_IE")]
    XK_Cyrillic_IE,
    #[strum(serialize = "Cyrillic_EF")]
    XK_Cyrillic_EF,
    #[strum(serialize = "Cyrillic_GHE")]
    XK_Cyrillic_GHE,
    #[strum(serialize = "Cyrillic_HA")]
    XK_Cyrillic_HA,
    #[strum(serialize = "Cyrillic_I")]
    XK_Cyrillic_I,
    #[strum(serialize = "Cyrillic_SHORTI")]
    XK_Cyrillic_SHORTI,
    #[strum(serialize = "Cyrillic_KA")]
    XK_Cyrillic_KA,
    #[strum(serialize = "Cyrillic_EL")]
    XK_Cyrillic_EL,
    #[strum(serialize = "Cyrillic_EM")]
    XK_Cyrillic_EM,
    #[strum(serialize = "Cyrillic_EN")]
    XK_Cyrillic_EN,
    #[strum(serialize = "Cyrillic_O")]
    XK_Cyrillic_O,
    #[strum(serialize = "Cyrillic_PE")]
    XK_Cyrillic_PE,
    #[strum(serialize = "Cyrillic_YA")]
    XK_Cyrillic_YA,
    #[strum(serialize = "Cyrillic_ER")]
    XK_Cyrillic_ER,
    #[strum(serialize = "Cyrillic_ES")]
    XK_Cyrillic_ES,
    #[strum(serialize = "Cyrillic_TE")]
    XK_Cyrillic_TE,
    #[strum(serialize = "Cyrillic_U")]
    XK_Cyrillic_U,
    #[strum(serialize = "Cyrillic_ZHE")]
    XK_Cyrillic_ZHE,
    #[strum(serialize = "Cyrillic_VE")]
    XK_Cyrillic_VE,
    #[strum(serialize = "Cyrillic_SOFTSIGN")]
    XK_Cyrillic_SOFTSIGN,
    #[strum(serialize = "Cyrillic_YERU")]
    XK_Cyrillic_YERU,
    #[strum(serialize = "Cyrillic_ZE")]
    XK_Cyrillic_ZE,
    #[strum(serialize = "Cyrillic_SHA")]
    XK_Cyrillic_SHA,
    #[strum(serialize = "Cyrillic_E")]
    XK_Cyrillic_E,
    #[strum(serialize = "Cyrillic_SHCHA")]
    XK_Cyrillic_SHCHA,
    #[strum(serialize = "Cyrillic_CHE")]
    XK_Cyrillic_CHE,
    #[strum(serialize = "Cyrillic_HARDSIGN")]
    XK_Cyrillic_HARDSIGN,
    #[strum(serialize = "Greek_ALPHAaccent")]
    XK_Greek_ALPHAaccent,
    #[strum(serialize = "Greek_EPSILONaccent")]
    XK_Greek_EPSILONaccent,
    #[strum(serialize = "Greek_ETAaccent")]
    XK_Greek_ETAaccent,
    #[strum(serialize = "Greek_IOTAaccent")]
    XK_Greek_IOTAaccent,
    #[strum(serialize = "Greek_IOTAdiaeresis")]
    XK_Greek_IOTAdiaeresis,
    #[strum(serialize = "Greek_OMICRONaccent")]
    XK_Greek_OMICRONaccent,
    #[strum(serialize = "Greek_UPSILONaccent")]
    XK_Greek_UPSILONaccent,
    #[strum(serialize = "Greek_UPSILONdieresis")]
    XK_Greek_UPSILONdieresis,
    #[strum(serialize = "Greek_OMEGAaccent")]
    XK_Greek_OMEGAaccent,
    #[strum(serialize = "Greek_accentdieresis")]
    XK_Greek_accentdieresis,
    #[strum(serialize = "Greek_horizbar")]
    XK_Greek_horizbar,
    #[strum(serialize = "Greek_alphaaccent")]
    XK_Greek_alphaaccent,
    #[strum(serialize = "Greek_epsilonaccent")]
    XK_Greek_epsilonaccent,
    #[strum(serialize = "Greek_etaaccent")]
    XK_Greek_etaaccent,
    #[strum(serialize = "Greek_iotaaccent")]
    XK_Greek_iotaaccent,
    #[strum(serialize = "Greek_iotadieresis")]
    XK_Greek_iotadieresis,
    #[strum(serialize = "Greek_iotaaccentdieresis")]
    XK_Greek_iotaaccentdieresis,
    #[strum(serialize = "Greek_omicronaccent")]
    XK_Greek_omicronaccent,
    #[strum(serialize = "Greek_upsilonaccent")]
    XK_Greek_upsilonaccent,
    #[strum(serialize = "Greek_upsilondieresis")]
    XK_Greek_upsilondieresis,
    #[strum(serialize = "Greek_upsilonaccentdieresis")]
    XK_Greek_upsilonaccentdieresis,
    #[strum(serialize = "Greek_omegaaccent")]
    XK_Greek_omegaaccent,
    #[strum(serialize = "Greek_ALPHA")]
    XK_Greek_ALPHA,
    #[strum(serialize = "Greek_BETA")]
    XK_Greek_BETA,
    #[strum(serialize = "Greek_GAMMA")]
    XK_Greek_GAMMA,
    #[strum(serialize = "Greek_DELTA")]
    XK_Greek_DELTA,
    #[strum(serialize = "Greek_EPSILON")]
    XK_Greek_EPSILON,
    #[strum(serialize = "Greek_ZETA")]
    XK_Greek_ZETA,
    #[strum(serialize = "Greek_ETA")]
    XK_Greek_ETA,
    #[strum(serialize = "Greek_THETA")]
    XK_Greek_THETA,
    #[strum(serialize = "Greek_IOTA")]
    XK_Greek_IOTA,
    #[strum(serialize = "Greek_KAPPA")]
    XK_Greek_KAPPA,
    #[strum(serialize = "Greek_LAMDA")]
    XK_Greek_LAMDA,
    #[strum(serialize = "Greek_LAMBDA")]
    XK_Greek_LAMBDA,
    #[strum(serialize = "Greek_MU")]
    XK_Greek_MU,
    #[strum(serialize = "Greek_NU")]
    XK_Greek_NU,
    #[strum(serialize = "Greek_XI")]
    XK_Greek_XI,
    #[strum(serialize = "Greek_OMICRON")]
    XK_Greek_OMICRON,
    #[strum(serialize = "Greek_PI")]
    XK_Greek_PI,
    #[strum(serialize = "Greek_RHO")]
    XK_Greek_RHO,
    #[strum(serialize = "Greek_SIGMA")]
    XK_Greek_SIGMA,
    #[strum(serialize = "Greek_TAU")]
    XK_Greek_TAU,
    #[strum(serialize = "Greek_UPSILON")]
    XK_Greek_UPSILON,
    #[strum(serialize = "Greek_PHI")]
    XK_Greek_PHI,
    #[strum(serialize = "Greek_CHI")]
    XK_Greek_CHI,
    #[strum(serialize = "Greek_PSI")]
    XK_Greek_PSI,
    #[strum(serialize = "Greek_OMEGA")]
    XK_Greek_OMEGA,
    #[strum(serialize = "Greek_alpha")]
    XK_Greek_alpha,
    #[strum(serialize = "Greek_beta")]
    XK_Greek_beta,
    #[strum(serialize = "Greek_gamma")]
    XK_Greek_gamma,
    #[strum(serialize = "Greek_delta")]
    XK_Greek_delta,
    #[strum(serialize = "Greek_epsilon")]
    XK_Greek_epsilon,
    #[strum(serialize = "Greek_zeta")]
    XK_Greek_zeta,
    #[strum(serialize = "Greek_eta")]
    XK_Greek_eta,
    #[strum(serialize = "Greek_theta")]
    XK_Greek_theta,
    #[strum(serialize = "Greek_iota")]
    XK_Greek_iota,
    #[strum(serialize = "Greek_kappa")]
    XK_Greek_kappa,
    #[strum(serialize = "Greek_lamda")]
    XK_Greek_lamda,
    #[strum(serialize = "Greek_lambda")]
    XK_Greek_lambda,
    #[strum(serialize = "Greek_mu")]
    XK_Greek_mu,
    #[strum(serialize = "Greek_nu")]
    XK_Greek_nu,
    #[strum(serialize = "Greek_xi")]
    XK_Greek_xi,
    #[strum(serialize = "Greek_omicron")]
    XK_Greek_omicron,
    #[strum(serialize = "Greek_pi")]
    XK_Greek_pi,
    #[strum(serialize = "Greek_rho")]
    XK_Greek_rho,
    #[strum(serialize = "Greek_sigma")]
    XK_Greek_sigma,
    #[strum(serialize = "Greek_finalsmallsigma")]
    XK_Greek_finalsmallsigma,
    #[strum(serialize = "Greek_tau")]
    XK_Greek_tau,
    #[strum(serialize = "Greek_upsilon")]
    XK_Greek_upsilon,
    #[strum(serialize = "Greek_phi")]
    XK_Greek_phi,
    #[strum(serialize = "Greek_chi")]
    XK_Greek_chi,
    #[strum(serialize = "Greek_psi")]
    XK_Greek_psi,
    #[strum(serialize = "Greek_omega")]
    XK_Greek_omega,
    #[strum(serialize = "Greek_switch")]
    XK_Greek_switch,
    #[strum(serialize = "leftradical")]
    XK_leftradical,
    #[strum(serialize = "topleftradical")]
    XK_topleftradical,
    #[strum(serialize = "horizconnector")]
    XK_horizconnector,
    #[strum(serialize = "topintegral")]
    XK_topintegral,
    #[strum(serialize = "botintegral")]
    XK_botintegral,
    #[strum(serialize = "vertconnector")]
    XK_vertconnector,
    #[strum(serialize = "topleftsqbracket")]
    XK_topleftsqbracket,
    #[strum(serialize = "botleftsqbracket")]
    XK_botleftsqbracket,
    #[strum(serialize = "toprightsqbracket")]
    XK_toprightsqbracket,
    #[strum(serialize = "botrightsqbracket")]
    XK_botrightsqbracket,
    #[strum(serialize = "topleftparens")]
    XK_topleftparens,
    #[strum(serialize = "botleftparens")]
    XK_botleftparens,
    #[strum(serialize = "toprightparens")]
    XK_toprightparens,
    #[strum(serialize = "botrightparens")]
    XK_botrightparens,
    #[strum(serialize = "leftmiddlecurlybrace")]
    XK_leftmiddlecurlybrace,
    #[strum(serialize = "rightmiddlecurlybrace")]
    XK_rightmiddlecurlybrace,
    #[strum(serialize = "topleftsummation")]
    XK_topleftsummation,
    #[strum(serialize = "botleftsummation")]
    XK_botleftsummation,
    #[strum(serialize = "topvertsummationconnector")]
    XK_topvertsummationconnector,
    #[strum(serialize = "botvertsummationconnector")]
    XK_botvertsummationconnector,
    #[strum(serialize = "toprightsummation")]
    XK_toprightsummation,
    #[strum(serialize = "botrightsummation")]
    XK_botrightsummation,
    #[strum(serialize = "rightmiddlesummation")]
    XK_rightmiddlesummation,
    #[strum(serialize = "lessthanequal")]
    XK_lessthanequal,
    #[strum(serialize = "notequal")]
    XK_notequal,
    #[strum(serialize = "greaterthanequal")]
    XK_greaterthanequal,
    #[strum(serialize = "integral")]
    XK_integral,
    #[strum(serialize = "therefore")]
    XK_therefore,
    #[strum(serialize = "variation")]
    XK_variation,
    #[strum(serialize = "infinity")]
    XK_infinity,
    #[strum(serialize = "nabla")]
    XK_nabla,
    #[strum(serialize = "approximate")]
    XK_approximate,
    #[strum(serialize = "similarequal")]
    XK_similarequal,
    #[strum(serialize = "ifonlyif")]
    XK_ifonlyif,
    #[strum(serialize = "implies")]
    XK_implies,
    #[strum(serialize = "identical")]
    XK_identical,
    #[strum(serialize = "radical")]
    XK_radical,
    #[strum(serialize = "includedin")]
    XK_includedin,
    #[strum(serialize = "includes")]
    XK_includes,
    #[strum(serialize = "intersection")]
    XK_intersection,
    #[strum(serialize = "union")]
    XK_union,
    #[strum(serialize = "logicaland")]
    XK_logicaland,
    #[strum(serialize = "logicalor")]
    XK_logicalor,
    #[strum(serialize = "partialderivative")]
    XK_partialderivative,
    #[strum(serialize = "function")]
    XK_function,
    #[strum(serialize = "leftarrow")]
    XK_leftarrow,
    #[strum(serialize = "uparrow")]
    XK_uparrow,
    #[strum(serialize = "rightarrow")]
    XK_rightarrow,
    #[strum(serialize = "downarrow")]
    XK_downarrow,
    #[strum(serialize = "blank")]
    XK_blank,
    #[strum(serialize = "soliddiamond")]
    XK_soliddiamond,
    #[strum(serialize = "checkerboard")]
    XK_checkerboard,
    #[strum(serialize = "ht")]
    XK_ht,
    #[strum(serialize = "ff")]
    XK_ff,
    #[strum(serialize = "cr")]
    XK_cr,
    #[strum(serialize = "lf")]
    XK_lf,
    #[strum(serialize = "nl")]
    XK_nl,
    #[strum(serialize = "vt")]
    XK_vt,
    #[strum(serialize = "lowrightcorner")]
    XK_lowrightcorner,
    #[strum(serialize = "uprightcorner")]
    XK_uprightcorner,
    #[strum(serialize = "upleftcorner")]
    XK_upleftcorner,
    #[strum(serialize = "lowleftcorner")]
    XK_lowleftcorner,
    #[strum(serialize = "crossinglines")]
    XK_crossinglines,
    #[strum(serialize = "horizlinescan1")]
    XK_horizlinescan1,
    #[strum(serialize = "horizlinescan3")]
    XK_horizlinescan3,
    #[strum(serialize = "horizlinescan5")]
    XK_horizlinescan5,
    #[strum(serialize = "horizlinescan7")]
    XK_horizlinescan7,
    #[strum(serialize = "horizlinescan9")]
    XK_horizlinescan9,
    #[strum(serialize = "leftt")]
    XK_leftt,
    #[strum(serialize = "rightt")]
    XK_rightt,
    #[strum(serialize = "bott")]
    XK_bott,
    #[strum(serialize = "topt")]
    XK_topt,
    #[strum(serialize = "vertbar")]
    XK_vertbar,
    #[strum(serialize = "emspace")]
    XK_emspace,
    #[strum(serialize = "enspace")]
    XK_enspace,
    #[strum(serialize = "em3space")]
    XK_em3space,
    #[strum(serialize = "em4space")]
    XK_em4space,
    #[strum(serialize = "digitspace")]
    XK_digitspace,
    #[strum(serialize = "punctspace")]
    XK_punctspace,
    #[strum(serialize = "thinspace")]
    XK_thinspace,
    #[strum(serialize = "hairspace")]
    XK_hairspace,
    #[strum(serialize = "emdash")]
    XK_emdash,
    #[strum(serialize = "endash")]
    XK_endash,
    #[strum(serialize = "signifblank")]
    XK_signifblank,
    #[strum(serialize = "ellipsis")]
    XK_ellipsis,
    #[strum(serialize = "doubbaselinedot")]
    XK_doubbaselinedot,
    #[strum(serialize = "onethird")]
    XK_onethird,
    #[strum(serialize = "twothirds")]
    XK_twothirds,
    #[strum(serialize = "onefifth")]
    XK_onefifth,
    #[strum(serialize = "twofifths")]
    XK_twofifths,
    #[strum(serialize = "threefifths")]
    XK_threefifths,
    #[strum(serialize = "fourfifths")]
    XK_fourfifths,
    #[strum(serialize = "onesixth")]
    XK_onesixth,
    #[strum(serialize = "fivesixths")]
    XK_fivesixths,
    #[strum(serialize = "careof")]
    XK_careof,
    #[strum(serialize = "figdash")]
    XK_figdash,
    #[strum(serialize = "leftanglebracket")]
    XK_leftanglebracket,
    #[strum(serialize = "decimalpoint")]
    XK_decimalpoint,
    #[strum(serialize = "rightanglebracket")]
    XK_rightanglebracket,
    #[strum(serialize = "marker")]
    XK_marker,
    #[strum(serialize = "oneeighth")]
    XK_oneeighth,
    #[strum(serialize = "threeeighths")]
    XK_threeeighths,
    #[strum(serialize = "fiveeighths")]
    XK_fiveeighths,
    #[strum(serialize = "seveneighths")]
    XK_seveneighths,
    #[strum(serialize = "trademark")]
    XK_trademark,
    #[strum(serialize = "signaturemark")]
    XK_signaturemark,
    #[strum(serialize = "trademarkincircle")]
    XK_trademarkincircle,
    #[strum(serialize = "leftopentriangle")]
    XK_leftopentriangle,
    #[strum(serialize = "rightopentriangle")]
    XK_rightopentriangle,
    #[strum(serialize = "emopencircle")]
    XK_emopencircle,
    #[strum(serialize = "emopenrectangle")]
    XK_emopenrectangle,
    #[strum(serialize = "leftsinglequotemark")]
    XK_leftsinglequotemark,
    #[strum(serialize = "rightsinglequotemark")]
    XK_rightsinglequotemark,
    #[strum(serialize = "leftdoublequotemark")]
    XK_leftdoublequotemark,
    #[strum(serialize = "rightdoublequotemark")]
    XK_rightdoublequotemark,
    #[strum(serialize = "prescription")]
    XK_prescription,
    #[strum(serialize = "minutes")]
    XK_minutes,
    #[strum(serialize = "seconds")]
    XK_seconds,
    #[strum(serialize = "latincross")]
    XK_latincross,
    #[strum(serialize = "hexagram")]
    XK_hexagram,
    #[strum(serialize = "filledrectbullet")]
    XK_filledrectbullet,
    #[strum(serialize = "filledlefttribullet")]
    XK_filledlefttribullet,
    #[strum(serialize = "filledrighttribullet")]
    XK_filledrighttribullet,
    #[strum(serialize = "emfilledcircle")]
    XK_emfilledcircle,
    #[strum(serialize = "emfilledrect")]
    XK_emfilledrect,
    #[strum(serialize = "enopencircbullet")]
    XK_enopencircbullet,
    #[strum(serialize = "enopensquarebullet")]
    XK_enopensquarebullet,
    #[strum(serialize = "openrectbullet")]
    XK_openrectbullet,
    #[strum(serialize = "opentribulletup")]
    XK_opentribulletup,
    #[strum(serialize = "opentribulletdown")]
    XK_opentribulletdown,
    #[strum(serialize = "openstar")]
    XK_openstar,
    #[strum(serialize = "enfilledcircbullet")]
    XK_enfilledcircbullet,
    #[strum(serialize = "enfilledsqbullet")]
    XK_enfilledsqbullet,
    #[strum(serialize = "filledtribulletup")]
    XK_filledtribulletup,
    #[strum(serialize = "filledtribulletdown")]
    XK_filledtribulletdown,
    #[strum(serialize = "leftpointer")]
    XK_leftpointer,
    #[strum(serialize = "rightpointer")]
    XK_rightpointer,
    #[strum(serialize = "club")]
    XK_club,
    #[strum(serialize = "diamond")]
    XK_diamond,
    #[strum(serialize = "heart")]
    XK_heart,
    #[strum(serialize = "maltesecross")]
    XK_maltesecross,
    #[strum(serialize = "dagger")]
    XK_dagger,
    #[strum(serialize = "doubledagger")]
    XK_doubledagger,
    #[strum(serialize = "checkmark")]
    XK_checkmark,
    #[strum(serialize = "ballotcross")]
    XK_ballotcross,
    #[strum(serialize = "musicalsharp")]
    XK_musicalsharp,
    #[strum(serialize = "musicalflat")]
    XK_musicalflat,
    #[strum(serialize = "malesymbol")]
    XK_malesymbol,
    #[strum(serialize = "femalesymbol")]
    XK_femalesymbol,
    #[strum(serialize = "telephone")]
    XK_telephone,
    #[strum(serialize = "telephonerecorder")]
    XK_telephonerecorder,
    #[strum(serialize = "phonographcopyright")]
    XK_phonographcopyright,
    #[strum(serialize = "caret")]
    XK_caret,
    #[strum(serialize = "singlelowquotemark")]
    XK_singlelowquotemark,
    #[strum(serialize = "doublelowquotemark")]
    XK_doublelowquotemark,
    #[strum(serialize = "cursor")]
    XK_cursor,
    #[strum(serialize = "leftcaret")]
    XK_leftcaret,
    #[strum(serialize = "rightcaret")]
    XK_rightcaret,
    #[strum(serialize = "downcaret")]
    XK_downcaret,
    #[strum(serialize = "upcaret")]
    XK_upcaret,
    #[strum(serialize = "overbar")]
    XK_overbar,
    #[strum(serialize = "downtack")]
    XK_downtack,
    #[strum(serialize = "upshoe")]
    XK_upshoe,
    #[strum(serialize = "downstile")]
    XK_downstile,
    #[strum(serialize = "underbar")]
    XK_underbar,
    #[strum(serialize = "jot")]
    XK_jot,
    #[strum(serialize = "quad")]
    XK_quad,
    #[strum(serialize = "uptack")]
    XK_uptack,
    #[strum(serialize = "circle")]
    XK_circle,
    #[strum(serialize = "upstile")]
    XK_upstile,
    #[strum(serialize = "downshoe")]
    XK_downshoe,
    #[strum(serialize = "rightshoe")]
    XK_rightshoe,
    #[strum(serialize = "leftshoe")]
    XK_leftshoe,
    #[strum(serialize = "lefttack")]
    XK_lefttack,
    #[strum(serialize = "righttack")]
    XK_righttack,
    #[strum(serialize = "hebrew_doublelowline")]
    XK_hebrew_doublelowline,
    #[strum(serialize = "hebrew_aleph")]
    XK_hebrew_aleph,
    #[strum(serialize = "hebrew_bet")]
    XK_hebrew_bet,
    #[strum(serialize = "hebrew_beth")]
    XK_hebrew_beth,
    #[strum(serialize = "hebrew_gimel")]
    XK_hebrew_gimel,
    #[strum(serialize = "hebrew_gimmel")]
    XK_hebrew_gimmel,
    #[strum(serialize = "hebrew_dalet")]
    XK_hebrew_dalet,
    #[strum(serialize = "hebrew_daleth")]
    XK_hebrew_daleth,
    #[strum(serialize = "hebrew_he")]
    XK_hebrew_he,
    #[strum(serialize = "hebrew_waw")]
    XK_hebrew_waw,
    #[strum(serialize = "hebrew_zain")]
    XK_hebrew_zain,
    #[strum(serialize = "hebrew_zayin")]
    XK_hebrew_zayin,
    #[strum(serialize = "hebrew_chet")]
    XK_hebrew_chet,
    #[strum(serialize = "hebrew_het")]
    XK_hebrew_het,
    #[strum(serialize = "hebrew_tet")]
    XK_hebrew_tet,
    #[strum(serialize = "hebrew_teth")]
    XK_hebrew_teth,
    #[strum(serialize = "hebrew_yod")]
    XK_hebrew_yod,
    #[strum(serialize = "hebrew_finalkaph")]
    XK_hebrew_finalkaph,
    #[strum(serialize = "hebrew_kaph")]
    XK_hebrew_kaph,
    #[strum(serialize = "hebrew_lamed")]
    XK_hebrew_lamed,
    #[strum(serialize = "hebrew_finalmem")]
    XK_hebrew_finalmem,
    #[strum(serialize = "hebrew_mem")]
    XK_hebrew_mem,
    #[strum(serialize = "hebrew_finalnun")]
    XK_hebrew_finalnun,
    #[strum(serialize = "hebrew_nun")]
    XK_hebrew_nun,
    #[strum(serialize = "hebrew_samech")]
    XK_hebrew_samech,
    #[strum(serialize = "hebrew_samekh")]
    XK_hebrew_samekh,
    #[strum(serialize = "hebrew_ayin")]
    XK_hebrew_ayin,
    #[strum(serialize = "hebrew_finalpe")]
    XK_hebrew_finalpe,
    #[strum(serialize = "hebrew_pe")]
    XK_hebrew_pe,
    #[strum(serialize = "hebrew_finalzade")]
    XK_hebrew_finalzade,
    #[strum(serialize = "hebrew_finalzadi")]
    XK_hebrew_finalzadi,
    #[strum(serialize = "hebrew_zade")]
    XK_hebrew_zade,
    #[strum(serialize = "hebrew_zadi")]
    XK_hebrew_zadi,
    #[strum(serialize = "hebrew_qoph")]
    XK_hebrew_qoph,
    #[strum(serialize = "hebrew_kuf")]
    XK_hebrew_kuf,
    #[strum(serialize = "hebrew_resh")]
    XK_hebrew_resh,
    #[strum(serialize = "hebrew_shin")]
    XK_hebrew_shin,
    #[strum(serialize = "hebrew_taw")]
    XK_hebrew_taw,
    #[strum(serialize = "hebrew_taf")]
    XK_hebrew_taf,
    #[strum(serialize = "Hebrew_switch")]
    XK_Hebrew_switch,

    #[strum(serialize = "XF86ModeLock")]
    XF86XK_ModeLock,
    #[strum(serialize = "XF86MonBrightnessUp")]
    XF86XK_MonBrightnessUp,
    #[strum(serialize = "XF86MonBrightnessDown")]
    XF86XK_MonBrightnessDown,
    #[strum(serialize = "XF86KbdLightOnOff")]
    XF86XK_KbdLightOnOff,
    #[strum(serialize = "XF86KbdBrightnessUp")]
    XF86XK_KbdBrightnessUp,
    #[strum(serialize = "XF86KbdBrightnessDown")]
    XF86XK_KbdBrightnessDown,
    #[strum(serialize = "XF86Standby")]
    XF86XK_Standby,
    #[strum(serialize = "XF86AudioLowerVolume")]
    XF86XK_AudioLowerVolume,
    #[strum(serialize = "XF86AudioMute")]
    XF86XK_AudioMute,
    #[strum(serialize = "XF86AudioRaiseVolume")]
    XF86XK_AudioRaiseVolume,
    #[strum(serialize = "XF86AudioPlay")]
    XF86XK_AudioPlay,
    #[strum(serialize = "XF86AudioStop")]
    XF86XK_AudioStop,
    #[strum(serialize = "XF86AudioPrev")]
    XF86XK_AudioPrev,
    #[strum(serialize = "XF86AudioNext")]
    XF86XK_AudioNext,
    #[strum(serialize = "XF86HomePage")]
    XF86XK_HomePage,
    #[strum(serialize = "XF86Mail")]
    XF86XK_Mail,
    #[strum(serialize = "XF86Start")]
    XF86XK_Start,
    #[strum(serialize = "XF86Search")]
    XF86XK_Search,
    #[strum(serialize = "XF86AudioRecord")]
    XF86XK_AudioRecord,
    #[strum(serialize = "XF86Calculator")]
    XF86XK_Calculator,
    #[strum(serialize = "XF86Memo")]
    XF86XK_Memo,
    #[strum(serialize = "XF86ToDoList")]
    XF86XK_ToDoList,
    #[strum(serialize = "XF86Calendar")]
    XF86XK_Calendar,
    #[strum(serialize = "XF86PowerDown")]
    XF86XK_PowerDown,
    #[strum(serialize = "XF86ContrastAdjust")]
    XF86XK_ContrastAdjust,
    #[strum(serialize = "XF86RockerUp")]
    XF86XK_RockerUp,
    #[strum(serialize = "XF86RockerDown")]
    XF86XK_RockerDown,
    #[strum(serialize = "XF86RockerEnter")]
    XF86XK_RockerEnter,
    #[strum(serialize = "XF86Back")]
    XF86XK_Back,
    #[strum(serialize = "XF86Forward")]
    XF86XK_Forward,
    #[strum(serialize = "XF86Stop")]
    XF86XK_Stop,
    #[strum(serialize = "XF86Refresh")]
    XF86XK_Refresh,
    #[strum(serialize = "XF86PowerOff")]
    XF86XK_PowerOff,
    #[strum(serialize = "XF86WakeUp")]
    XF86XK_WakeUp,
    #[strum(serialize = "XF86Eject")]
    XF86XK_Eject,
    #[strum(serialize = "XF86ScreenSaver")]
    XF86XK_ScreenSaver,
    #[strum(serialize = "XF86WWW")]
    XF86XK_WWW,
    #[strum(serialize = "XF86Sleep")]
    XF86XK_Sleep,
    #[strum(serialize = "XF86Favorites")]
    XF86XK_Favorites,
    #[strum(serialize = "XF86AudioPause")]
    XF86XK_AudioPause,
    #[strum(serialize = "XF86AudioMedia")]
    XF86XK_AudioMedia,
    #[strum(serialize = "XF86MyComputer")]
    XF86XK_MyComputer,
    #[strum(serialize = "XF86VendorHome")]
    XF86XK_VendorHome,
    #[strum(serialize = "XF86LightBulb")]
    XF86XK_LightBulb,
    #[strum(serialize = "XF86Shop")]
    XF86XK_Shop,
    #[strum(serialize = "XF86History")]
    XF86XK_History,
    #[strum(serialize = "XF86OpenURL")]
    XF86XK_OpenURL,
    #[strum(serialize = "XF86AddFavorite")]
    XF86XK_AddFavorite,
    #[strum(serialize = "XF86HotLinks")]
    XF86XK_HotLinks,
    #[strum(serialize = "XF86BrightnessAdjust")]
    XF86XK_BrightnessAdjust,
    #[strum(serialize = "XF86Finance")]
    XF86XK_Finance,
    #[strum(serialize = "XF86Community")]
    XF86XK_Community,
    #[strum(serialize = "XF86AudioRewind")]
    XF86XK_AudioRewind,
    #[strum(serialize = "XF86BackForward")]
    XF86XK_BackForward,
    #[strum(serialize = "XF86Launch0")]
    XF86XK_Launch0,
    #[strum(serialize = "XF86Launch1")]
    XF86XK_Launch1,
    #[strum(serialize = "XF86Launch2")]
    XF86XK_Launch2,
    #[strum(serialize = "XF86Launch3")]
    XF86XK_Launch3,
    #[strum(serialize = "XF86Launch4")]
    XF86XK_Launch4,
    #[strum(serialize = "XF86Launch5")]
    XF86XK_Launch5,
    #[strum(serialize = "XF86Launch6")]
    XF86XK_Launch6,
    #[strum(serialize = "XF86Launch7")]
    XF86XK_Launch7,
    #[strum(serialize = "XF86Launch8")]
    XF86XK_Launch8,
    #[strum(serialize = "XF86Launch9")]
    XF86XK_Launch9,
    #[strum(serialize = "XF86LaunchA")]
    XF86XK_LaunchA,
    #[strum(serialize = "XF86LaunchB")]
    XF86XK_LaunchB,
    #[strum(serialize = "XF86LaunchC")]
    XF86XK_LaunchC,
    #[strum(serialize = "XF86LaunchD")]
    XF86XK_LaunchD,
    #[strum(serialize = "XF86LaunchE")]
    XF86XK_LaunchE,
    #[strum(serialize = "XF86LaunchF")]
    XF86XK_LaunchF,
    #[strum(serialize = "XF86ApplicationLeft")]
    XF86XK_ApplicationLeft,
    #[strum(serialize = "XF86ApplicationRight")]
    XF86XK_ApplicationRight,
    #[strum(serialize = "XF86Book")]
    XF86XK_Book,
    #[strum(serialize = "XF86CD")]
    XF86XK_CD,
    #[strum(serialize = "XF86Calculater")]
    XF86XK_Calculater,
    #[strum(serialize = "XF86Clear")]
    XF86XK_Clear,
    #[strum(serialize = "XF86Close")]
    XF86XK_Close,
    #[strum(serialize = "XF86Copy")]
    XF86XK_Copy,
    #[strum(serialize = "XF86Cut")]
    XF86XK_Cut,
    #[strum(serialize = "XF86Display")]
    XF86XK_Display,
    #[strum(serialize = "XF86DOS")]
    XF86XK_DOS,
    #[strum(serialize = "XF86Documents")]
    XF86XK_Documents,
    #[strum(serialize = "XF86Excel")]
    XF86XK_Excel,
    #[strum(serialize = "XF86Explorer")]
    XF86XK_Explorer,
    #[strum(serialize = "XF86Game")]
    XF86XK_Game,
    #[strum(serialize = "XF86Go")]
    XF86XK_Go,
    #[strum(serialize = "XF86iTouch")]
    XF86XK_iTouch,
    #[strum(serialize = "XF86LogOff")]
    XF86XK_LogOff,
    #[strum(serialize = "XF86Market")]
    XF86XK_Market,
    #[strum(serialize = "XF86Meeting")]
    XF86XK_Meeting,
    #[strum(serialize = "XF86MenuKB")]
    XF86XK_MenuKB,
    #[strum(serialize = "XF86MenuPB")]
    XF86XK_MenuPB,
    #[strum(serialize = "XF86MySites")]
    XF86XK_MySites,
    #[strum(serialize = "XF86New")]
    XF86XK_New,
    #[strum(serialize = "XF86News")]
    XF86XK_News,
    #[strum(serialize = "XF86OfficeHome")]
    XF86XK_OfficeHome,
    #[strum(serialize = "XF86Open")]
    XF86XK_Open,
    #[strum(serialize = "XF86Option")]
    XF86XK_Option,
    #[strum(serialize = "XF86Paste")]
    XF86XK_Paste,
    #[strum(serialize = "XF86Phone")]
    XF86XK_Phone,
    #[strum(serialize = "XF86Q")]
    XF86XK_Q,
    #[strum(serialize = "XF86Reply")]
    XF86XK_Reply,
    #[strum(serialize = "XF86Reload")]
    XF86XK_Reload,
    #[strum(serialize = "XF86RotateWindows")]
    XF86XK_RotateWindows,
    #[strum(serialize = "XF86RotationPB")]
    XF86XK_RotationPB,
    #[strum(serialize = "XF86RotationKB")]
    XF86XK_RotationKB,
    #[strum(serialize = "XF86Save")]
    XF86XK_Save,
    #[strum(serialize = "XF86ScrollUp")]
    XF86XK_ScrollUp,
    #[strum(serialize = "XF86ScrollDown")]
    XF86XK_ScrollDown,
    #[strum(serialize = "XF86ScrollClick")]
    XF86XK_ScrollClick,
    #[strum(serialize = "XF86Send")]
    XF86XK_Send,
    #[strum(serialize = "XF86Spell")]
    XF86XK_Spell,
    #[strum(serialize = "XF86SplitScreen")]
    XF86XK_SplitScreen,
    #[strum(serialize = "XF86Support")]
    XF86XK_Support,
    #[strum(serialize = "XF86TaskPane")]
    XF86XK_TaskPane,
    #[strum(serialize = "XF86Terminal")]
    XF86XK_Terminal,
    #[strum(serialize = "XF86Tools")]
    XF86XK_Tools,
    #[strum(serialize = "XF86Travel")]
    XF86XK_Travel,
    #[strum(serialize = "XF86UserPB")]
    XF86XK_UserPB,
    #[strum(serialize = "XF86User1KB")]
    XF86XK_User1KB,
    #[strum(serialize = "XF86User2KB")]
    XF86XK_User2KB,
    #[strum(serialize = "XF86Video")]
    XF86XK_Video,
    #[strum(serialize = "XF86WheelButton")]
    XF86XK_WheelButton,
    #[strum(serialize = "XF86Word")]
    XF86XK_Word,
    #[strum(serialize = "XF86Xfer")]
    XF86XK_Xfer,
    #[strum(serialize = "XF86ZoomIn")]
    XF86XK_ZoomIn,
    #[strum(serialize = "XF86ZoomOut")]
    XF86XK_ZoomOut,
    #[strum(serialize = "XF86Away")]
    XF86XK_Away,
    #[strum(serialize = "XF86Messenger")]
    XF86XK_Messenger,
    #[strum(serialize = "XF86WebCam")]
    XF86XK_WebCam,
    #[strum(serialize = "XF86MailForward")]
    XF86XK_MailForward,
    #[strum(serialize = "XF86Pictures")]
    XF86XK_Pictures,
    #[strum(serialize = "XF86Music")]
    XF86XK_Music,
    #[strum(serialize = "XF86Battery")]
    XF86XK_Battery,
    #[strum(serialize = "XF86Bluetooth")]
    XF86XK_Bluetooth,
    #[strum(serialize = "XF86WLAN")]
    XF86XK_WLAN,
    #[strum(serialize = "XF86UWB")]
    XF86XK_UWB,
    #[strum(serialize = "XF86AudioForward")]
    XF86XK_AudioForward,
    #[strum(serialize = "XF86AudioRepeat")]
    XF86XK_AudioRepeat,
    #[strum(serialize = "XF86AudioRandomPlay")]
    XF86XK_AudioRandomPlay,
    #[strum(serialize = "XF86Subtitle")]
    XF86XK_Subtitle,
    #[strum(serialize = "XF86AudioCycleTrack")]
    XF86XK_AudioCycleTrack,
    #[strum(serialize = "XF86CycleAngle")]
    XF86XK_CycleAngle,
    #[strum(serialize = "XF86FrameBack")]
    XF86XK_FrameBack,
    #[strum(serialize = "XF86FrameForward")]
    XF86XK_FrameForward,
    #[strum(serialize = "XF86Time")]
    XF86XK_Time,
    #[strum(serialize = "XF86Select")]
    XF86XK_Select,
    #[strum(serialize = "XF86View")]
    XF86XK_View,
    #[strum(serialize = "XF86TopMenu")]
    XF86XK_TopMenu,
    #[strum(serialize = "XF86Red")]
    XF86XK_Red,
    #[strum(serialize = "XF86Green")]
    XF86XK_Green,
    #[strum(serialize = "XF86Yellow")]
    XF86XK_Yellow,
    #[strum(serialize = "XF86Blue")]
    XF86XK_Blue,
    #[strum(serialize = "XF86Suspend")]
    XF86XK_Suspend,
    #[strum(serialize = "XF86Hibernate")]
    XF86XK_Hibernate,
    #[strum(serialize = "XF86TouchpadToggle")]
    XF86XK_TouchpadToggle,
    #[strum(serialize = "XF86TouchpadOn")]
    XF86XK_TouchpadOn,
    #[strum(serialize = "XF86TouchpadOff")]
    XF86XK_TouchpadOff,
    #[strum(serialize = "XF86AudioMicMute")]
    XF86XK_AudioMicMute,
    #[strum(serialize = "XF86Switch_VT_1")]
    XF86XK_Switch_VT_1,
    #[strum(serialize = "XF86Switch_VT_2")]
    XF86XK_Switch_VT_2,
    #[strum(serialize = "XF86Switch_VT_3")]
    XF86XK_Switch_VT_3,
    #[strum(serialize = "XF86Switch_VT_4")]
    XF86XK_Switch_VT_4,
    #[strum(serialize = "XF86Switch_VT_5")]
    XF86XK_Switch_VT_5,
    #[strum(serialize = "XF86Switch_VT_6")]
    XF86XK_Switch_VT_6,
    #[strum(serialize = "XF86Switch_VT_7")]
    XF86XK_Switch_VT_7,
    #[strum(serialize = "XF86Switch_VT_8")]
    XF86XK_Switch_VT_8,
    #[strum(serialize = "XF86Switch_VT_9")]
    XF86XK_Switch_VT_9,
    #[strum(serialize = "XF86Switch_VT_10")]
    XF86XK_Switch_VT_10,
    #[strum(serialize = "XF86Switch_VT_11")]
    XF86XK_Switch_VT_11,
    #[strum(serialize = "XF86Switch_VT_12")]
    XF86XK_Switch_VT_12,
    #[strum(serialize = "XF86Ungrab")]
    XF86XK_Ungrab,
    #[strum(serialize = "XF86ClearGrab")]
    XF86XK_ClearGrab,
    #[strum(serialize = "XF86Next_VMode")]
    XF86XK_Next_VMode,
    #[strum(serialize = "XF86Prev_VMode")]
    XF86XK_Prev_VMode,
    #[strum(serialize = "XF86LogWindowTree")]
    XF86XK_LogWindowTree,
    #[strum(serialize = "XF86LogGrabInfo")]
    XF86XK_LogGrabInfo,

    #[strum(serialize = "ISO_Lock")]
    XK_ISO_Lock,
    #[strum(serialize = "ISO_Level2_Latch")]
    XK_ISO_Level2_Latch,
    #[strum(serialize = "ISO_Level3_Shift")]
    XK_ISO_Level3_Shift,
    #[strum(serialize = "ISO_Level3_Latch")]
    XK_ISO_Level3_Latch,
    #[strum(serialize = "ISO_Level3_Lock")]
    XK_ISO_Level3_Lock,
    #[strum(serialize = "ISO_Level5_Shift")]
    XK_ISO_Level5_Shift,
    #[strum(serialize = "ISO_Level5_Latch")]
    XK_ISO_Level5_Latch,
    #[strum(serialize = "ISO_Level5_Lock")]
    XK_ISO_Level5_Lock,
    #[strum(serialize = "ISO_Group_Shift")]
    XK_ISO_Group_Shift,
    #[strum(serialize = "ISO_Group_Latch")]
    XK_ISO_Group_Latch,
    #[strum(serialize = "ISO_Group_Lock")]
    XK_ISO_Group_Lock,
    #[strum(serialize = "ISO_Next_Group")]
    XK_ISO_Next_Group,
    #[strum(serialize = "ISO_Next_Group_Lock")]
    XK_ISO_Next_Group_Lock,
    #[strum(serialize = "ISO_Prev_Group")]
    XK_ISO_Prev_Group,
    #[strum(serialize = "ISO_Prev_Group_Lock")]
    XK_ISO_Prev_Group_Lock,
    #[strum(serialize = "ISO_First_Group")]
    XK_ISO_First_Group,
    #[strum(serialize = "ISO_First_Group_Lock")]
    XK_ISO_First_Group_Lock,
    #[strum(serialize = "ISO_Last_Group")]
    XK_ISO_Last_Group,
    #[strum(serialize = "ISO_Last_Group_Lock")]
    XK_ISO_Last_Group_Lock,

    #[strum(serialize = "ISO_Left_Tab")]
    XK_ISO_Left_Tab,
    #[strum(serialize = "ISO_Move_Line_Up")]
    XK_ISO_Move_Line_Up,
    #[strum(serialize = "ISO_Move_Line_Down")]
    XK_ISO_Move_Line_Down,
    #[strum(serialize = "ISO_Partial_Line_Up")]
    XK_ISO_Partial_Line_Up,
    #[strum(serialize = "ISO_Partial_Line_Down")]
    XK_ISO_Partial_Line_Down,
    #[strum(serialize = "ISO_Partial_Space_Left")]
    XK_ISO_Partial_Space_Left,
    #[strum(serialize = "ISO_Partial_Space_Right")]
    XK_ISO_Partial_Space_Right,
    #[strum(serialize = "ISO_Set_Margin_Left")]
    XK_ISO_Set_Margin_Left,
    #[strum(serialize = "ISO_Set_Margin_Right")]
    XK_ISO_Set_Margin_Right,
    #[strum(serialize = "ISO_Release_Margin_Left")]
    XK_ISO_Release_Margin_Left,
    #[strum(serialize = "ISO_Release_Margin_Right")]
    XK_ISO_Release_Margin_Right,
    #[strum(serialize = "ISO_Release_Both_Margins")]
    XK_ISO_Release_Both_Margins,
    #[strum(serialize = "ISO_Fast_Cursor_Left")]
    XK_ISO_Fast_Cursor_Left,
    #[strum(serialize = "ISO_Fast_Cursor_Right")]
    XK_ISO_Fast_Cursor_Right,
    #[strum(serialize = "ISO_Fast_Cursor_Up")]
    XK_ISO_Fast_Cursor_Up,
    #[strum(serialize = "ISO_Fast_Cursor_Down")]
    XK_ISO_Fast_Cursor_Down,
    #[strum(serialize = "ISO_Continuous_Underline")]
    XK_ISO_Continuous_Underline,
    #[strum(serialize = "ISO_Discontinuous_Underline")]
    XK_ISO_Discontinuous_Underline,
    #[strum(serialize = "ISO_Emphasize")]
    XK_ISO_Emphasize,
    #[strum(serialize = "ISO_Center_Object")]
    XK_ISO_Center_Object,
    #[strum(serialize = "ISO_Enter")]
    XK_ISO_Enter,

    #[strum(serialize = "dead_grave")]
    XK_dead_grave,
    #[strum(serialize = "dead_acute")]
    XK_dead_acute,
    #[strum(serialize = "dead_circumflex")]
    XK_dead_circumflex,
    #[strum(serialize = "dead_tilde")]
    XK_dead_tilde,
    #[strum(serialize = "dead_perispomeni")]
    XK_dead_perispomeni,
    #[strum(serialize = "dead_macron")]
    XK_dead_macron,
    #[strum(serialize = "dead_breve")]
    XK_dead_breve,
    #[strum(serialize = "dead_abovedot")]
    XK_dead_abovedot,
    #[strum(serialize = "dead_diaeresis")]
    XK_dead_diaeresis,
    #[strum(serialize = "dead_abovering")]
    XK_dead_abovering,
    #[strum(serialize = "dead_doubleacute")]
    XK_dead_doubleacute,
    #[strum(serialize = "dead_caron")]
    XK_dead_caron,
    #[strum(serialize = "dead_cedilla")]
    XK_dead_cedilla,
    #[strum(serialize = "dead_ogonek")]
    XK_dead_ogonek,
    #[strum(serialize = "dead_iota")]
    XK_dead_iota,
    #[strum(serialize = "dead_voiced_sound")]
    XK_dead_voiced_sound,
    #[strum(serialize = "dead_semivoiced_sound")]
    XK_dead_semivoiced_sound,
    #[strum(serialize = "dead_belowdot")]
    XK_dead_belowdot,
    #[strum(serialize = "dead_hook")]
    XK_dead_hook,
    #[strum(serialize = "dead_horn")]
    XK_dead_horn,
    #[strum(serialize = "dead_stroke")]
    XK_dead_stroke,
    #[strum(serialize = "dead_abovecomma")]
    XK_dead_abovecomma,
    #[strum(serialize = "dead_psili")]
    XK_dead_psili,
    #[strum(serialize = "dead_abovereversedcomma")]
    XK_dead_abovereversedcomma,
    #[strum(serialize = "dead_dasia")]
    XK_dead_dasia,
    #[strum(serialize = "dead_doublegrave")]
    XK_dead_doublegrave,
    #[strum(serialize = "dead_belowring")]
    XK_dead_belowring,
    #[strum(serialize = "dead_belowmacron")]
    XK_dead_belowmacron,
    #[strum(serialize = "dead_belowcircumflex")]
    XK_dead_belowcircumflex,
    #[strum(serialize = "dead_belowtilde")]
    XK_dead_belowtilde,
    #[strum(serialize = "dead_belowbreve")]
    XK_dead_belowbreve,
    #[strum(serialize = "dead_belowdiaeresis")]
    XK_dead_belowdiaeresis,
    #[strum(serialize = "dead_invertedbreve")]
    XK_dead_invertedbreve,
    #[strum(serialize = "dead_belowcomma")]
    XK_dead_belowcomma,
    #[strum(serialize = "dead_currency")]
    XK_dead_currency,

    #[strum(serialize = "dead_lowline")]
    XK_dead_lowline,
    #[strum(serialize = "dead_aboveverticalline")]
    XK_dead_aboveverticalline,
    #[strum(serialize = "dead_belowverticalline")]
    XK_dead_belowverticalline,
    #[strum(serialize = "dead_longsolidusoverlay")]
    XK_dead_longsolidusoverlay,

    #[strum(serialize = "dead_a")]
    XK_dead_a,
    #[strum(serialize = "dead_A")]
    XK_dead_A,
    #[strum(serialize = "dead_e")]
    XK_dead_e,
    #[strum(serialize = "dead_E")]
    XK_dead_E,
    #[strum(serialize = "dead_i")]
    XK_dead_i,
    #[strum(serialize = "dead_I")]
    XK_dead_I,
    #[strum(serialize = "dead_o")]
    XK_dead_o,
    #[strum(serialize = "dead_O")]
    XK_dead_O,
    #[strum(serialize = "dead_u")]
    XK_dead_u,
    #[strum(serialize = "dead_U")]
    XK_dead_U,
    #[strum(serialize = "dead_small_schwa")]
    XK_dead_small_schwa,
    #[strum(serialize = "dead_capital_schwa")]
    XK_dead_capital_schwa,

    #[strum(serialize = "dead_greek")]
    XK_dead_greek,

    #[strum(serialize = "First_Virtual_Screen")]
    XK_First_Virtual_Screen,
    #[strum(serialize = "Prev_Virtual_Screen")]
    XK_Prev_Virtual_Screen,
    #[strum(serialize = "Next_Virtual_Screen")]
    XK_Next_Virtual_Screen,
    #[strum(serialize = "Last_Virtual_Screen")]
    XK_Last_Virtual_Screen,
    #[strum(serialize = "Terminate_Server")]
    XK_Terminate_Server,

    #[strum(serialize = "AccessX_Enable")]
    XK_AccessX_Enable,
    #[strum(serialize = "AccessX_Feedback_Enable")]
    XK_AccessX_Feedback_Enable,
    #[strum(serialize = "RepeatKeys_Enable")]
    XK_RepeatKeys_Enable,
    #[strum(serialize = "SlowKeys_Enable")]
    XK_SlowKeys_Enable,
    #[strum(serialize = "BounceKeys_Enable")]
    XK_BounceKeys_Enable,
    #[strum(serialize = "StickyKeys_Enable")]
    XK_StickyKeys_Enable,
    #[strum(serialize = "MouseKeys_Enable")]
    XK_MouseKeys_Enable,
    #[strum(serialize = "MouseKeys_Accel_Enable")]
    XK_MouseKeys_Accel_Enable,
    #[strum(serialize = "Overlay1_Enable")]
    XK_Overlay1_Enable,
    #[strum(serialize = "Overlay2_Enable")]
    XK_Overlay2_Enable,
    #[strum(serialize = "AudibleBell_Enable")]
    XK_AudibleBell_Enable,

    #[strum(serialize = "Pointer_Left")]
    XK_Pointer_Left,
    #[strum(serialize = "Pointer_Right")]
    XK_Pointer_Right,
    #[strum(serialize = "Pointer_Up")]
    XK_Pointer_Up,
    #[strum(serialize = "Pointer_Down")]
    XK_Pointer_Down,
    #[strum(serialize = "Pointer_UpLeft")]
    XK_Pointer_UpLeft,
    #[strum(serialize = "Pointer_UpRight")]
    XK_Pointer_UpRight,
    #[strum(serialize = "Pointer_DownLeft")]
    XK_Pointer_DownLeft,
    #[strum(serialize = "Pointer_DownRight")]
    XK_Pointer_DownRight,
    #[strum(serialize = "Pointer_Button_Dflt")]
    XK_Pointer_Button_Dflt,
    #[strum(serialize = "Pointer_Button1")]
    XK_Pointer_Button1,
    #[strum(serialize = "Pointer_Button2")]
    XK_Pointer_Button2,
    #[strum(serialize = "Pointer_Button3")]
    XK_Pointer_Button3,
    #[strum(serialize = "Pointer_Button4")]
    XK_Pointer_Button4,
    #[strum(serialize = "Pointer_Button5")]
    XK_Pointer_Button5,
    #[strum(serialize = "Pointer_DblClick_Dflt")]
    XK_Pointer_DblClick_Dflt,
    #[strum(serialize = "Pointer_DblClick1")]
    XK_Pointer_DblClick1,
    #[strum(serialize = "Pointer_DblClick2")]
    XK_Pointer_DblClick2,
    #[strum(serialize = "Pointer_DblClick3")]
    XK_Pointer_DblClick3,
    #[strum(serialize = "Pointer_DblClick4")]
    XK_Pointer_DblClick4,
    #[strum(serialize = "Pointer_DblClick5")]
    XK_Pointer_DblClick5,
    #[strum(serialize = "Pointer_Drag_Dflt")]
    XK_Pointer_Drag_Dflt,
    #[strum(serialize = "Pointer_Drag1")]
    XK_Pointer_Drag1,
    #[strum(serialize = "Pointer_Drag2")]
    XK_Pointer_Drag2,
    #[strum(serialize = "Pointer_Drag3")]
    XK_Pointer_Drag3,
    #[strum(serialize = "Pointer_Drag4")]
    XK_Pointer_Drag4,
    #[strum(serialize = "Pointer_Drag5")]
    XK_Pointer_Drag5,

    #[strum(serialize = "Pointer_EnableKeys")]
    XK_Pointer_EnableKeys,
    #[strum(serialize = "Pointer_Accelerate")]
    XK_Pointer_Accelerate,
    #[strum(serialize = "Pointer_DfltBtnNext")]
    XK_Pointer_DfltBtnNext,
    #[strum(serialize = "Pointer_DfltBtnPrev")]
    XK_Pointer_DfltBtnPrev,

    #[strum(serialize = "ch")]
    XK_ch,
    #[strum(serialize = "Ch")]
    XK_Ch,
    #[strum(serialize = "CH")]
    XK_CH,
    #[strum(serialize = "c_h")]
    XK_c_h,
    #[strum(serialize = "C_h")]
    XK_C_h,
    #[strum(serialize = "C_H")]
    XK_C_H,
}

impl XKeySym {
    /// Convert this keysym to its utf8 representation if possible
    pub fn as_utf8_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(
            (match self {
                XK_BackSpace => 0xFF08,
                XK_Tab => 0xFF09,
                XK_Linefeed => 0xFF0A,
                XK_Clear => 0xFF0B,
                XK_Return => 0xFF0D,
                XK_Pause => 0xFF13,
                XK_Scroll_Lock => 0xFF14,
                XK_Sys_Req => 0xFF15,
                XK_Escape => 0xFF1B,
                XK_Delete => 0xFFFF,
                XK_Multi_key => 0xFF20,
                XK_Kanji => 0xFF21,
                XK_Muhenkan => 0xFF22,
                XK_Henkan_Mode => 0xFF23,
                XK_Henkan => 0xFF23,
                XK_Romaji => 0xFF24,
                XK_Hiragana => 0xFF25,
                XK_Katakana => 0xFF26,
                XK_Hiragana_Katakana => 0xFF27,
                XK_Zenkaku => 0xFF28,
                XK_Hankaku => 0xFF29,
                XK_Zenkaku_Hankaku => 0xFF2A,
                XK_Touroku => 0xFF2B,
                XK_Massyo => 0xFF2C,
                XK_Kana_Lock => 0xFF2D,
                XK_Kana_Shift => 0xFF2E,
                XK_Eisu_Shift => 0xFF2F,
                XK_Eisu_toggle => 0xFF30,
                XK_Home => 0xFF50,
                XK_Left => 0xFF51,
                XK_Up => 0xFF52,
                XK_Right => 0xFF53,
                XK_Down => 0xFF54,
                XK_Prior => 0xFF55,
                XK_Page_Up => 0xFF55,
                XK_Next => 0xFF56,
                XK_Page_Down => 0xFF56,
                XK_End => 0xFF57,
                XK_Begin => 0xFF58,
                XK_Win_L => 0xFF5B,
                XK_Win_R => 0xFF5C,
                XK_App => 0xFF5D,
                XK_Select => 0xFF60,
                XK_Print => 0xFF61,
                XK_Execute => 0xFF62,
                XK_Insert => 0xFF63,
                XK_Undo => 0xFF65,
                XK_Redo => 0xFF66,
                XK_Menu => 0xFF67,
                XK_Find => 0xFF68,
                XK_Cancel => 0xFF69,
                XK_Help => 0xFF6A,
                XK_Break => 0xFF6B,
                XK_Mode_switch => 0xFF7E,
                XK_script_switch => 0xFF7E,
                XK_Num_Lock => 0xFF7F,
                XK_KP_Space => 0xFF80,
                XK_KP_Tab => 0xFF89,
                XK_KP_Enter => 0xFF8D,
                XK_KP_F1 => 0xFF91,
                XK_KP_F2 => 0xFF92,
                XK_KP_F3 => 0xFF93,
                XK_KP_F4 => 0xFF94,
                XK_KP_Home => 0xFF95,
                XK_KP_Left => 0xFF96,
                XK_KP_Up => 0xFF97,
                XK_KP_Right => 0xFF98,
                XK_KP_Down => 0xFF99,
                XK_KP_Prior => 0xFF9A,
                XK_KP_Page_Up => 0xFF9A,
                XK_KP_Next => 0xFF9B,
                XK_KP_Page_Down => 0xFF9B,
                XK_KP_End => 0xFF9C,
                XK_KP_Begin => 0xFF9D,
                XK_KP_Insert => 0xFF9E,
                XK_KP_Delete => 0xFF9F,
                XK_KP_Equal => 0xFFBD,
                XK_KP_Multiply => 0xFFAA,
                XK_KP_Add => 0xFFAB,
                XK_KP_Separator => 0xFFAC,
                XK_KP_Subtract => 0xFFAD,
                XK_KP_Decimal => 0xFFAE,
                XK_KP_Divide => 0xFFAF,
                XK_KP_0 => 0xFFB0,
                XK_KP_1 => 0xFFB1,
                XK_KP_2 => 0xFFB2,
                XK_KP_3 => 0xFFB3,
                XK_KP_4 => 0xFFB4,
                XK_KP_5 => 0xFFB5,
                XK_KP_6 => 0xFFB6,
                XK_KP_7 => 0xFFB7,
                XK_KP_8 => 0xFFB8,
                XK_KP_9 => 0xFFB9,
                XK_F1 => 0xFFBE,
                XK_F2 => 0xFFBF,
                XK_F3 => 0xFFC0,
                XK_F4 => 0xFFC1,
                XK_F5 => 0xFFC2,
                XK_F6 => 0xFFC3,
                XK_F7 => 0xFFC4,
                XK_F8 => 0xFFC5,
                XK_F9 => 0xFFC6,
                XK_F10 => 0xFFC7,
                XK_F11 => 0xFFC8,
                XK_L1 => 0xFFC8,
                XK_F12 => 0xFFC9,
                XK_L2 => 0xFFC9,
                XK_F13 => 0xFFCA,
                XK_L3 => 0xFFCA,
                XK_F14 => 0xFFCB,
                XK_L4 => 0xFFCB,
                XK_F15 => 0xFFCC,
                XK_L5 => 0xFFCC,
                XK_F16 => 0xFFCD,
                XK_L6 => 0xFFCD,
                XK_F17 => 0xFFCE,
                XK_L7 => 0xFFCE,
                XK_F18 => 0xFFCF,
                XK_L8 => 0xFFCF,
                XK_F19 => 0xFFD0,
                XK_L9 => 0xFFD0,
                XK_F20 => 0xFFD1,
                XK_L10 => 0xFFD1,
                XK_F21 => 0xFFD2,
                XK_R1 => 0xFFD2,
                XK_F22 => 0xFFD3,
                XK_R2 => 0xFFD3,
                XK_F23 => 0xFFD4,
                XK_R3 => 0xFFD4,
                XK_F24 => 0xFFD5,
                XK_R4 => 0xFFD5,
                XK_F25 => 0xFFD6,
                XK_R5 => 0xFFD6,
                XK_F26 => 0xFFD7,
                XK_R6 => 0xFFD7,
                XK_F27 => 0xFFD8,
                XK_R7 => 0xFFD8,
                XK_F28 => 0xFFD9,
                XK_R8 => 0xFFD9,
                XK_F29 => 0xFFDA,
                XK_R9 => 0xFFDA,
                XK_F30 => 0xFFDB,
                XK_R10 => 0xFFDB,
                XK_F31 => 0xFFDC,
                XK_R11 => 0xFFDC,
                XK_F32 => 0xFFDD,
                XK_R12 => 0xFFDD,
                XK_F33 => 0xFFDE,
                XK_R13 => 0xFFDE,
                XK_F34 => 0xFFDF,
                XK_R14 => 0xFFDF,
                XK_F35 => 0xFFE0,
                XK_R15 => 0xFFE0,
                XK_Shift_L => 0xFFE1,
                XK_Shift_R => 0xFFE2,
                XK_Control_L => 0xFFE3,
                XK_Control_R => 0xFFE4,
                XK_Caps_Lock => 0xFFE5,
                XK_Shift_Lock => 0xFFE6,
                XK_Meta_L => 0xFFE7,
                XK_Meta_R => 0xFFE8,
                XK_Alt_L => 0xFFE9,
                XK_Alt_R => 0xFFEA,
                XK_Super_L => 0xFFEB,
                XK_Super_R => 0xFFEC,
                XK_Hyper_L => 0xFFED,
                XK_Hyper_R => 0xFFEE,
                XK_space => 0x020,
                XK_exclam => 0x021,
                XK_quotedbl => 0x022,
                XK_numbersign => 0x023,
                XK_dollar => 0x024,
                XK_percent => 0x025,
                XK_ampersand => 0x026,
                XK_apostrophe => 0x027,
                XK_quoteright => 0x027,
                XK_parenleft => 0x028,
                XK_parenright => 0x029,
                XK_asterisk => 0x02A,
                XK_plus => 0x02B,
                XK_comma => 0x02C,
                XK_minus => 0x02D,
                XK_period => 0x02E,
                XK_slash => 0x02F,
                XK_0 => 0x030,
                XK_1 => 0x031,
                XK_2 => 0x032,
                XK_3 => 0x033,
                XK_4 => 0x034,
                XK_5 => 0x035,
                XK_6 => 0x036,
                XK_7 => 0x037,
                XK_8 => 0x038,
                XK_9 => 0x039,
                XK_colon => 0x03A,
                XK_semicolon => 0x03B,
                XK_less => 0x03C,
                XK_equal => 0x03D,
                XK_greater => 0x03E,
                XK_question => 0x03F,
                XK_at => 0x040,
                XK_A => 0x041,
                XK_B => 0x042,
                XK_C => 0x043,
                XK_D => 0x044,
                XK_E => 0x045,
                XK_F => 0x046,
                XK_G => 0x047,
                XK_H => 0x048,
                XK_I => 0x049,
                XK_J => 0x04A,
                XK_K => 0x04B,
                XK_L => 0x04C,
                XK_M => 0x04D,
                XK_N => 0x04E,
                XK_O => 0x04F,
                XK_P => 0x050,
                XK_Q => 0x051,
                XK_R => 0x052,
                XK_S => 0x053,
                XK_T => 0x054,
                XK_U => 0x055,
                XK_V => 0x056,
                XK_W => 0x057,
                XK_X => 0x058,
                XK_Y => 0x059,
                XK_Z => 0x05A,
                XK_bracketleft => 0x05B,
                XK_backslash => 0x05C,
                XK_bracketright => 0x05D,
                XK_asciicircum => 0x05E,
                XK_underscore => 0x05F,
                XK_grave => 0x060,
                XK_quoteleft => 0x060,
                XK_a => 0x061,
                XK_b => 0x062,
                XK_c => 0x063,
                XK_d => 0x064,
                XK_e => 0x065,
                XK_f => 0x066,
                XK_g => 0x067,
                XK_h => 0x068,
                XK_i => 0x069,
                XK_j => 0x06A,
                XK_k => 0x06B,
                XK_l => 0x06C,
                XK_m => 0x06D,
                XK_n => 0x06E,
                XK_o => 0x06F,
                XK_p => 0x070,
                XK_q => 0x071,
                XK_r => 0x072,
                XK_s => 0x073,
                XK_t => 0x074,
                XK_u => 0x075,
                XK_v => 0x076,
                XK_w => 0x077,
                XK_x => 0x078,
                XK_y => 0x079,
                XK_z => 0x07A,
                XK_braceleft => 0x07B,
                XK_bar => 0x07C,
                XK_braceright => 0x07D,
                XK_asciitilde => 0x07E,
                XK_nobreakspace => 0x0A0,
                XK_exclamdown => 0x0A1,
                XK_cent => 0x0A2,
                XK_sterling => 0x0A3,
                XK_currency => 0x0A4,
                XK_yen => 0x0A5,
                XK_brokenbar => 0x0A6,
                XK_section => 0x0A7,
                XK_diaeresis => 0x0A8,
                XK_copyright => 0x0A9,
                XK_ordfeminine => 0x0AA,
                XK_guillemotleft => 0x0AB,
                XK_notsign => 0x0AC,
                XK_hyphen => 0x0AD,
                XK_registered => 0x0AE,
                XK_macron => 0x0AF,
                XK_degree => 0x0B0,
                XK_plusminus => 0x0B1,
                XK_twosuperior => 0x0B2,
                XK_threesuperior => 0x0B3,
                XK_acute => 0x0B4,
                XK_mu => 0x0B5,
                XK_paragraph => 0x0B6,
                XK_periodcentered => 0x0B7,
                XK_cedilla => 0x0B8,
                XK_onesuperior => 0x0B9,
                XK_masculine => 0x0BA,
                XK_guillemotright => 0x0BB,
                XK_onequarter => 0x0BC,
                XK_onehalf => 0x0BD,
                XK_threequarters => 0x0BE,
                XK_questiondown => 0x0BF,
                XK_Agrave => 0x0C0,
                XK_Aacute => 0x0C1,
                XK_Acircumflex => 0x0C2,
                XK_Atilde => 0x0C3,
                XK_Adiaeresis => 0x0C4,
                XK_Aring => 0x0C5,
                XK_AE => 0x0C6,
                XK_Ccedilla => 0x0C7,
                XK_Egrave => 0x0C8,
                XK_Eacute => 0x0C9,
                XK_Ecircumflex => 0x0CA,
                XK_Ediaeresis => 0x0CB,
                XK_Igrave => 0x0CC,
                XK_Iacute => 0x0CD,
                XK_Icircumflex => 0x0CE,
                XK_Idiaeresis => 0x0CF,
                XK_ETH => 0x0D0,
                XK_Eth => 0x0D0,
                XK_Ntilde => 0x0D1,
                XK_Ograve => 0x0D2,
                XK_Oacute => 0x0D3,
                XK_Ocircumflex => 0x0D4,
                XK_Otilde => 0x0D5,
                XK_Odiaeresis => 0x0D6,
                XK_multiply => 0x0D7,
                XK_Ooblique => 0x0D8,
                XK_Ugrave => 0x0D9,
                XK_Uacute => 0x0DA,
                XK_Ucircumflex => 0x0DB,
                XK_Udiaeresis => 0x0DC,
                XK_Yacute => 0x0DD,
                XK_THORN => 0x0DE,
                XK_Thorn => 0x0DE,
                XK_ssharp => 0x0DF,
                XK_agrave => 0x0E0,
                XK_aacute => 0x0E1,
                XK_acircumflex => 0x0E2,
                XK_atilde => 0x0E3,
                XK_adiaeresis => 0x0E4,
                XK_aring => 0x0E5,
                XK_ae => 0x0E6,
                XK_ccedilla => 0x0E7,
                XK_egrave => 0x0E8,
                XK_eacute => 0x0E9,
                XK_ecircumflex => 0x0EA,
                XK_ediaeresis => 0x0EB,
                XK_igrave => 0x0EC,
                XK_iacute => 0x0ED,
                XK_icircumflex => 0x0EE,
                XK_idiaeresis => 0x0EF,
                XK_eth => 0x0F0,
                XK_ntilde => 0x0F1,
                XK_ograve => 0x0F2,
                XK_oacute => 0x0F3,
                XK_ocircumflex => 0x0F4,
                XK_otilde => 0x0F5,
                XK_odiaeresis => 0x0F6,
                XK_division => 0x0F7,
                XK_oslash => 0x0F8,
                XK_ugrave => 0x0F9,
                XK_uacute => 0x0FA,
                XK_ucircumflex => 0x0FB,
                XK_udiaeresis => 0x0FC,
                XK_yacute => 0x0FD,
                XK_thorn => 0x0FE,
                XK_ydiaeresis => 0x0FF,
                XK_Aogonek => 0x1A1,
                XK_breve => 0x1A2,
                XK_Lstroke => 0x1A3,
                XK_Lcaron => 0x1A5,
                XK_Sacute => 0x1A6,
                XK_Scaron => 0x1A9,
                XK_Scedilla => 0x1AA,
                XK_Tcaron => 0x1AB,
                XK_Zacute => 0x1AC,
                XK_Zcaron => 0x1AE,
                XK_Zabovedot => 0x1AF,
                XK_aogonek => 0x1B1,
                XK_ogonek => 0x1B2,
                XK_lstroke => 0x1B3,
                XK_lcaron => 0x1B5,
                XK_sacute => 0x1B6,
                XK_caron => 0x1B7,
                XK_scaron => 0x1B9,
                XK_scedilla => 0x1BA,
                XK_tcaron => 0x1BB,
                XK_zacute => 0x1BC,
                XK_doubleacute => 0x1BD,
                XK_zcaron => 0x1BE,
                XK_zabovedot => 0x1BF,
                XK_Racute => 0x1C0,
                XK_Abreve => 0x1C3,
                XK_Lacute => 0x1C5,
                XK_Cacute => 0x1C6,
                XK_Ccaron => 0x1C8,
                XK_Eogonek => 0x1CA,
                XK_Ecaron => 0x1CC,
                XK_Dcaron => 0x1CF,
                XK_Dstroke => 0x1D0,
                XK_Nacute => 0x1D1,
                XK_Ncaron => 0x1D2,
                XK_Odoubleacute => 0x1D5,
                XK_Rcaron => 0x1D8,
                XK_Uring => 0x1D9,
                XK_Udoubleacute => 0x1DB,
                XK_Tcedilla => 0x1DE,
                XK_racute => 0x1E0,
                XK_abreve => 0x1E3,
                XK_lacute => 0x1E5,
                XK_cacute => 0x1E6,
                XK_ccaron => 0x1E8,
                XK_eogonek => 0x1EA,
                XK_ecaron => 0x1EC,
                XK_dcaron => 0x1EF,
                XK_dstroke => 0x1F0,
                XK_nacute => 0x1F1,
                XK_ncaron => 0x1F2,
                XK_odoubleacute => 0x1F5,
                XK_udoubleacute => 0x1FB,
                XK_rcaron => 0x1F8,
                XK_uring => 0x1F9,
                XK_tcedilla => 0x1FE,
                XK_abovedot => 0x1FF,
                XK_Hstroke => 0x2A1,
                XK_Hcircumflex => 0x2A6,
                XK_Iabovedot => 0x2A9,
                XK_Gbreve => 0x2AB,
                XK_Jcircumflex => 0x2AC,
                XK_hstroke => 0x2B1,
                XK_hcircumflex => 0x2B6,
                XK_idotless => 0x2B9,
                XK_gbreve => 0x2BB,
                XK_jcircumflex => 0x2BC,
                XK_Cabovedot => 0x2C5,
                XK_Ccircumflex => 0x2C6,
                XK_Gabovedot => 0x2D5,
                XK_Gcircumflex => 0x2D8,
                XK_Ubreve => 0x2DD,
                XK_Scircumflex => 0x2DE,
                XK_cabovedot => 0x2E5,
                XK_ccircumflex => 0x2E6,
                XK_gabovedot => 0x2F5,
                XK_gcircumflex => 0x2F8,
                XK_ubreve => 0x2FD,
                XK_scircumflex => 0x2FE,
                XK_kra => 0x3A2,
                XK_kappa => 0x3A2,
                XK_Rcedilla => 0x3A3,
                XK_Itilde => 0x3A5,
                XK_Lcedilla => 0x3A6,
                XK_Emacron => 0x3AA,
                XK_Gcedilla => 0x3AB,
                XK_Tslash => 0x3AC,
                XK_rcedilla => 0x3B3,
                XK_itilde => 0x3B5,
                XK_lcedilla => 0x3B6,
                XK_emacron => 0x3BA,
                XK_gcedilla => 0x3BB,
                XK_tslash => 0x3BC,
                XK_ENG => 0x3BD,
                XK_eng => 0x3BF,
                XK_Amacron => 0x3C0,
                XK_Iogonek => 0x3C7,
                XK_Eabovedot => 0x3CC,
                XK_Imacron => 0x3CF,
                XK_Ncedilla => 0x3D1,
                XK_Omacron => 0x3D2,
                XK_Kcedilla => 0x3D3,
                XK_Uogonek => 0x3D9,
                XK_Utilde => 0x3DD,
                XK_Umacron => 0x3DE,
                XK_amacron => 0x3E0,
                XK_iogonek => 0x3E7,
                XK_eabovedot => 0x3EC,
                XK_imacron => 0x3EF,
                XK_ncedilla => 0x3F1,
                XK_omacron => 0x3F2,
                XK_kcedilla => 0x3F3,
                XK_uogonek => 0x3F9,
                XK_utilde => 0x3FD,
                XK_umacron => 0x3FE,
                XK_overline => 0x47E,
                XK_kana_fullstop => 0x4A1,
                XK_kana_openingbracket => 0x4A2,
                XK_kana_closingbracket => 0x4A3,
                XK_kana_comma => 0x4A4,
                XK_kana_conjunctive => 0x4A5,
                XK_kana_middledot => 0x4A5,
                XK_kana_WO => 0x4A6,
                XK_kana_a => 0x4A7,
                XK_kana_i => 0x4A8,
                XK_kana_u => 0x4A9,
                XK_kana_e => 0x4AA,
                XK_kana_o => 0x4AB,
                XK_kana_ya => 0x4AC,
                XK_kana_yu => 0x4AD,
                XK_kana_yo => 0x4AE,
                XK_kana_tsu => 0x4AF,
                XK_kana_tu => 0x4AF,
                XK_prolongedsound => 0x4B0,
                XK_kana_A => 0x4B1,
                XK_kana_I => 0x4B2,
                XK_kana_U => 0x4B3,
                XK_kana_E => 0x4B4,
                XK_kana_O => 0x4B5,
                XK_kana_KA => 0x4B6,
                XK_kana_KI => 0x4B7,
                XK_kana_KU => 0x4B8,
                XK_kana_KE => 0x4B9,
                XK_kana_KO => 0x4BA,
                XK_kana_SA => 0x4BB,
                XK_kana_SHI => 0x4BC,
                XK_kana_SU => 0x4BD,
                XK_kana_SE => 0x4BE,
                XK_kana_SO => 0x4BF,
                XK_kana_TA => 0x4C0,
                XK_kana_CHI => 0x4C1,
                XK_kana_TI => 0x4C1,
                XK_kana_TSU => 0x4C2,
                XK_kana_TU => 0x4C2,
                XK_kana_TE => 0x4C3,
                XK_kana_TO => 0x4C4,
                XK_kana_NA => 0x4C5,
                XK_kana_NI => 0x4C6,
                XK_kana_NU => 0x4C7,
                XK_kana_NE => 0x4C8,
                XK_kana_NO => 0x4C9,
                XK_kana_HA => 0x4CA,
                XK_kana_HI => 0x4CB,
                XK_kana_FU => 0x4CC,
                XK_kana_HU => 0x4CC,
                XK_kana_HE => 0x4CD,
                XK_kana_HO => 0x4CE,
                XK_kana_MA => 0x4CF,
                XK_kana_MI => 0x4D0,
                XK_kana_MU => 0x4D1,
                XK_kana_ME => 0x4D2,
                XK_kana_MO => 0x4D3,
                XK_kana_YA => 0x4D4,
                XK_kana_YU => 0x4D5,
                XK_kana_YO => 0x4D6,
                XK_kana_RA => 0x4D7,
                XK_kana_RI => 0x4D8,
                XK_kana_RU => 0x4D9,
                XK_kana_RE => 0x4DA,
                XK_kana_RO => 0x4DB,
                XK_kana_WA => 0x4DC,
                XK_kana_N => 0x4DD,
                XK_voicedsound => 0x4DE,
                XK_semivoicedsound => 0x4DF,
                XK_kana_switch => 0xFF7E,
                XK_Arabic_comma => 0x5AC,
                XK_Arabic_semicolon => 0x5BB,
                XK_Arabic_question_mark => 0x5BF,
                XK_Arabic_hamza => 0x5C1,
                XK_Arabic_maddaonalef => 0x5C2,
                XK_Arabic_hamzaonalef => 0x5C3,
                XK_Arabic_hamzaonwaw => 0x5C4,
                XK_Arabic_hamzaunderalef => 0x5C5,
                XK_Arabic_hamzaonyeh => 0x5C6,
                XK_Arabic_alef => 0x5C7,
                XK_Arabic_beh => 0x5C8,
                XK_Arabic_tehmarbuta => 0x5C9,
                XK_Arabic_teh => 0x5CA,
                XK_Arabic_theh => 0x5CB,
                XK_Arabic_jeem => 0x5CC,
                XK_Arabic_hah => 0x5CD,
                XK_Arabic_khah => 0x5CE,
                XK_Arabic_dal => 0x5CF,
                XK_Arabic_thal => 0x5D0,
                XK_Arabic_ra => 0x5D1,
                XK_Arabic_zain => 0x5D2,
                XK_Arabic_seen => 0x5D3,
                XK_Arabic_sheen => 0x5D4,
                XK_Arabic_sad => 0x5D5,
                XK_Arabic_dad => 0x5D6,
                XK_Arabic_tah => 0x5D7,
                XK_Arabic_zah => 0x5D8,
                XK_Arabic_ain => 0x5D9,
                XK_Arabic_ghain => 0x5DA,
                XK_Arabic_tatweel => 0x5E0,
                XK_Arabic_feh => 0x5E1,
                XK_Arabic_qaf => 0x5E2,
                XK_Arabic_kaf => 0x5E3,
                XK_Arabic_lam => 0x5E4,
                XK_Arabic_meem => 0x5E5,
                XK_Arabic_noon => 0x5E6,
                XK_Arabic_ha => 0x5E7,
                XK_Arabic_heh => 0x5E7,
                XK_Arabic_waw => 0x5E8,
                XK_Arabic_alefmaksura => 0x5E9,
                XK_Arabic_yeh => 0x5EA,
                XK_Arabic_fathatan => 0x5EB,
                XK_Arabic_dammatan => 0x5EC,
                XK_Arabic_kasratan => 0x5ED,
                XK_Arabic_fatha => 0x5EE,
                XK_Arabic_damma => 0x5EF,
                XK_Arabic_kasra => 0x5F0,
                XK_Arabic_shadda => 0x5F1,
                XK_Arabic_sukun => 0x5F2,
                XK_Arabic_switch => 0xFF7E,
                XK_Serbian_dje => 0x6A1,
                XK_Macedonia_gje => 0x6A2,
                XK_Cyrillic_io => 0x6A3,
                XK_Ukrainian_ie => 0x6A4,
                XK_Ukranian_je => 0x6A4,
                XK_Macedonia_dse => 0x6A5,
                XK_Ukrainian_i => 0x6A6,
                XK_Ukranian_i => 0x6A6,
                XK_Ukrainian_yi => 0x6A7,
                XK_Ukranian_yi => 0x6A7,
                XK_Cyrillic_je => 0x6A8,
                XK_Serbian_je => 0x6A8,
                XK_Cyrillic_lje => 0x6A9,
                XK_Serbian_lje => 0x6A9,
                XK_Cyrillic_nje => 0x6AA,
                XK_Serbian_nje => 0x6AA,
                XK_Serbian_tshe => 0x6AB,
                XK_Macedonia_kje => 0x6AC,
                XK_Byelorussian_shortu => 0x6AE,
                XK_Cyrillic_dzhe => 0x6AF,
                XK_Serbian_dze => 0x6AF,
                XK_numerosign => 0x6B0,
                XK_Serbian_DJE => 0x6B1,
                XK_Macedonia_GJE => 0x6B2,
                XK_Cyrillic_IO => 0x6B3,
                XK_Ukrainian_IE => 0x6B4,
                XK_Ukranian_JE => 0x6B4,
                XK_Macedonia_DSE => 0x6B5,
                XK_Ukrainian_I => 0x6B6,
                XK_Ukranian_I => 0x6B6,
                XK_Ukrainian_YI => 0x6B7,
                XK_Ukranian_YI => 0x6B7,
                XK_Cyrillic_JE => 0x6B8,
                XK_Serbian_JE => 0x6B8,
                XK_Cyrillic_LJE => 0x6B9,
                XK_Serbian_LJE => 0x6B9,
                XK_Cyrillic_NJE => 0x6BA,
                XK_Serbian_NJE => 0x6BA,
                XK_Serbian_TSHE => 0x6BB,
                XK_Macedonia_KJE => 0x6BC,
                XK_Byelorussian_SHORTU => 0x6BE,
                XK_Cyrillic_DZHE => 0x6BF,
                XK_Serbian_DZE => 0x6BF,
                XK_Cyrillic_yu => 0x6C0,
                XK_Cyrillic_a => 0x6C1,
                XK_Cyrillic_be => 0x6C2,
                XK_Cyrillic_tse => 0x6C3,
                XK_Cyrillic_de => 0x6C4,
                XK_Cyrillic_ie => 0x6C5,
                XK_Cyrillic_ef => 0x6C6,
                XK_Cyrillic_ghe => 0x6C7,
                XK_Cyrillic_ha => 0x6C8,
                XK_Cyrillic_i => 0x6C9,
                XK_Cyrillic_shorti => 0x6CA,
                XK_Cyrillic_ka => 0x6CB,
                XK_Cyrillic_el => 0x6CC,
                XK_Cyrillic_em => 0x6CD,
                XK_Cyrillic_en => 0x6CE,
                XK_Cyrillic_o => 0x6CF,
                XK_Cyrillic_pe => 0x6D0,
                XK_Cyrillic_ya => 0x6D1,
                XK_Cyrillic_er => 0x6D2,
                XK_Cyrillic_es => 0x6D3,
                XK_Cyrillic_te => 0x6D4,
                XK_Cyrillic_u => 0x6D5,
                XK_Cyrillic_zhe => 0x6D6,
                XK_Cyrillic_ve => 0x6D7,
                XK_Cyrillic_softsign => 0x6D8,
                XK_Cyrillic_yeru => 0x6D9,
                XK_Cyrillic_ze => 0x6DA,
                XK_Cyrillic_sha => 0x6DB,
                XK_Cyrillic_e => 0x6DC,
                XK_Cyrillic_shcha => 0x6DD,
                XK_Cyrillic_che => 0x6DE,
                XK_Cyrillic_hardsign => 0x6DF,
                XK_Cyrillic_YU => 0x6E0,
                XK_Cyrillic_A => 0x6E1,
                XK_Cyrillic_BE => 0x6E2,
                XK_Cyrillic_TSE => 0x6E3,
                XK_Cyrillic_DE => 0x6E4,
                XK_Cyrillic_IE => 0x6E5,
                XK_Cyrillic_EF => 0x6E6,
                XK_Cyrillic_GHE => 0x6E7,
                XK_Cyrillic_HA => 0x6E8,
                XK_Cyrillic_I => 0x6E9,
                XK_Cyrillic_SHORTI => 0x6EA,
                XK_Cyrillic_KA => 0x6EB,
                XK_Cyrillic_EL => 0x6EC,
                XK_Cyrillic_EM => 0x6ED,
                XK_Cyrillic_EN => 0x6EE,
                XK_Cyrillic_O => 0x6EF,
                XK_Cyrillic_PE => 0x6F0,
                XK_Cyrillic_YA => 0x6F1,
                XK_Cyrillic_ER => 0x6F2,
                XK_Cyrillic_ES => 0x6F3,
                XK_Cyrillic_TE => 0x6F4,
                XK_Cyrillic_U => 0x6F5,
                XK_Cyrillic_ZHE => 0x6F6,
                XK_Cyrillic_VE => 0x6F7,
                XK_Cyrillic_SOFTSIGN => 0x6F8,
                XK_Cyrillic_YERU => 0x6F9,
                XK_Cyrillic_ZE => 0x6FA,
                XK_Cyrillic_SHA => 0x6FB,
                XK_Cyrillic_E => 0x6FC,
                XK_Cyrillic_SHCHA => 0x6FD,
                XK_Cyrillic_CHE => 0x6FE,
                XK_Cyrillic_HARDSIGN => 0x6FF,
                XK_Greek_ALPHAaccent => 0x7A1,
                XK_Greek_EPSILONaccent => 0x7A2,
                XK_Greek_ETAaccent => 0x7A3,
                XK_Greek_IOTAaccent => 0x7A4,
                XK_Greek_IOTAdiaeresis => 0x7A5,
                XK_Greek_OMICRONaccent => 0x7A7,
                XK_Greek_UPSILONaccent => 0x7A8,
                XK_Greek_UPSILONdieresis => 0x7A9,
                XK_Greek_OMEGAaccent => 0x7AB,
                XK_Greek_accentdieresis => 0x7AE,
                XK_Greek_horizbar => 0x7AF,
                XK_Greek_alphaaccent => 0x7B1,
                XK_Greek_epsilonaccent => 0x7B2,
                XK_Greek_etaaccent => 0x7B3,
                XK_Greek_iotaaccent => 0x7B4,
                XK_Greek_iotadieresis => 0x7B5,
                XK_Greek_iotaaccentdieresis => 0x7B6,
                XK_Greek_omicronaccent => 0x7B7,
                XK_Greek_upsilonaccent => 0x7B8,
                XK_Greek_upsilondieresis => 0x7B9,
                XK_Greek_upsilonaccentdieresis => 0x7BA,
                XK_Greek_omegaaccent => 0x7BB,
                XK_Greek_ALPHA => 0x7C1,
                XK_Greek_BETA => 0x7C2,
                XK_Greek_GAMMA => 0x7C3,
                XK_Greek_DELTA => 0x7C4,
                XK_Greek_EPSILON => 0x7C5,
                XK_Greek_ZETA => 0x7C6,
                XK_Greek_ETA => 0x7C7,
                XK_Greek_THETA => 0x7C8,
                XK_Greek_IOTA => 0x7C9,
                XK_Greek_KAPPA => 0x7CA,
                XK_Greek_LAMDA => 0x7CB,
                XK_Greek_LAMBDA => 0x7CB,
                XK_Greek_MU => 0x7CC,
                XK_Greek_NU => 0x7CD,
                XK_Greek_XI => 0x7CE,
                XK_Greek_OMICRON => 0x7CF,
                XK_Greek_PI => 0x7D0,
                XK_Greek_RHO => 0x7D1,
                XK_Greek_SIGMA => 0x7D2,
                XK_Greek_TAU => 0x7D4,
                XK_Greek_UPSILON => 0x7D5,
                XK_Greek_PHI => 0x7D6,
                XK_Greek_CHI => 0x7D7,
                XK_Greek_PSI => 0x7D8,
                XK_Greek_OMEGA => 0x7D9,
                XK_Greek_alpha => 0x7E1,
                XK_Greek_beta => 0x7E2,
                XK_Greek_gamma => 0x7E3,
                XK_Greek_delta => 0x7E4,
                XK_Greek_epsilon => 0x7E5,
                XK_Greek_zeta => 0x7E6,
                XK_Greek_eta => 0x7E7,
                XK_Greek_theta => 0x7E8,
                XK_Greek_iota => 0x7E9,
                XK_Greek_kappa => 0x7EA,
                XK_Greek_lamda => 0x7EB,
                XK_Greek_lambda => 0x7EB,
                XK_Greek_mu => 0x7EC,
                XK_Greek_nu => 0x7ED,
                XK_Greek_xi => 0x7EE,
                XK_Greek_omicron => 0x7EF,
                XK_Greek_pi => 0x7F0,
                XK_Greek_rho => 0x7F1,
                XK_Greek_sigma => 0x7F2,
                XK_Greek_finalsmallsigma => 0x7F3,
                XK_Greek_tau => 0x7F4,
                XK_Greek_upsilon => 0x7F5,
                XK_Greek_phi => 0x7F6,
                XK_Greek_chi => 0x7F7,
                XK_Greek_psi => 0x7F8,
                XK_Greek_omega => 0x7F9,
                XK_Greek_switch => 0xFF7E,
                XK_leftradical => 0x8A1,
                XK_topleftradical => 0x8A2,
                XK_horizconnector => 0x8A3,
                XK_topintegral => 0x8A4,
                XK_botintegral => 0x8A5,
                XK_vertconnector => 0x8A6,
                XK_topleftsqbracket => 0x8A7,
                XK_botleftsqbracket => 0x8A8,
                XK_toprightsqbracket => 0x8A9,
                XK_botrightsqbracket => 0x8AA,
                XK_topleftparens => 0x8AB,
                XK_botleftparens => 0x8AC,
                XK_toprightparens => 0x8AD,
                XK_botrightparens => 0x8AE,
                XK_leftmiddlecurlybrace => 0x8AF,
                XK_rightmiddlecurlybrace => 0x8B0,
                XK_topleftsummation => 0x8B1,
                XK_botleftsummation => 0x8B2,
                XK_topvertsummationconnector => 0x8B3,
                XK_botvertsummationconnector => 0x8B4,
                XK_toprightsummation => 0x8B5,
                XK_botrightsummation => 0x8B6,
                XK_rightmiddlesummation => 0x8B7,
                XK_lessthanequal => 0x8BC,
                XK_notequal => 0x8BD,
                XK_greaterthanequal => 0x8BE,
                XK_integral => 0x8BF,
                XK_therefore => 0x8C0,
                XK_variation => 0x8C1,
                XK_infinity => 0x8C2,
                XK_nabla => 0x8C5,
                XK_approximate => 0x8C8,
                XK_similarequal => 0x8C9,
                XK_ifonlyif => 0x8CD,
                XK_implies => 0x8CE,
                XK_identical => 0x8CF,
                XK_radical => 0x8D6,
                XK_includedin => 0x8DA,
                XK_includes => 0x8DB,
                XK_intersection => 0x8DC,
                XK_union => 0x8DD,
                XK_logicaland => 0x8DE,
                XK_logicalor => 0x8DF,
                XK_partialderivative => 0x8EF,
                XK_function => 0x8F6,
                XK_leftarrow => 0x8FB,
                XK_uparrow => 0x8FC,
                XK_rightarrow => 0x8FD,
                XK_downarrow => 0x8FE,
                XK_blank => 0x9DF,
                XK_soliddiamond => 0x9E0,
                XK_checkerboard => 0x9E1,
                XK_ht => 0x9E2,
                XK_ff => 0x9E3,
                XK_cr => 0x9E4,
                XK_lf => 0x9E5,
                XK_nl => 0x9E8,
                XK_vt => 0x9E9,
                XK_lowrightcorner => 0x9EA,
                XK_uprightcorner => 0x9EB,
                XK_upleftcorner => 0x9EC,
                XK_lowleftcorner => 0x9ED,
                XK_crossinglines => 0x9EE,
                XK_horizlinescan1 => 0x9EF,
                XK_horizlinescan3 => 0x9F0,
                XK_horizlinescan5 => 0x9F1,
                XK_horizlinescan7 => 0x9F2,
                XK_horizlinescan9 => 0x9F3,
                XK_leftt => 0x9F4,
                XK_rightt => 0x9F5,
                XK_bott => 0x9F6,
                XK_topt => 0x9F7,
                XK_vertbar => 0x9F8,
                XK_emspace => 0xAA1,
                XK_enspace => 0xAA2,
                XK_em3space => 0xAA3,
                XK_em4space => 0xAA4,
                XK_digitspace => 0xAA5,
                XK_punctspace => 0xAA6,
                XK_thinspace => 0xAA7,
                XK_hairspace => 0xAA8,
                XK_emdash => 0xAA9,
                XK_endash => 0xAAA,
                XK_signifblank => 0xAAC,
                XK_ellipsis => 0xAAE,
                XK_doubbaselinedot => 0xAAF,
                XK_onethird => 0xAB0,
                XK_twothirds => 0xAB1,
                XK_onefifth => 0xAB2,
                XK_twofifths => 0xAB3,
                XK_threefifths => 0xAB4,
                XK_fourfifths => 0xAB5,
                XK_onesixth => 0xAB6,
                XK_fivesixths => 0xAB7,
                XK_careof => 0xAB8,
                XK_figdash => 0xABB,
                XK_leftanglebracket => 0xABC,
                XK_decimalpoint => 0xABD,
                XK_rightanglebracket => 0xABE,
                XK_marker => 0xABF,
                XK_oneeighth => 0xAC3,
                XK_threeeighths => 0xAC4,
                XK_fiveeighths => 0xAC5,
                XK_seveneighths => 0xAC6,
                XK_trademark => 0xAC9,
                XK_signaturemark => 0xACA,
                XK_trademarkincircle => 0xACB,
                XK_leftopentriangle => 0xACC,
                XK_rightopentriangle => 0xACD,
                XK_emopencircle => 0xACE,
                XK_emopenrectangle => 0xACF,
                XK_leftsinglequotemark => 0xAD0,
                XK_rightsinglequotemark => 0xAD1,
                XK_leftdoublequotemark => 0xAD2,
                XK_rightdoublequotemark => 0xAD3,
                XK_prescription => 0xAD4,
                XK_minutes => 0xAD6,
                XK_seconds => 0xAD7,
                XK_latincross => 0xAD9,
                XK_hexagram => 0xADA,
                XK_filledrectbullet => 0xADB,
                XK_filledlefttribullet => 0xADC,
                XK_filledrighttribullet => 0xADD,
                XK_emfilledcircle => 0xADE,
                XK_emfilledrect => 0xADF,
                XK_enopencircbullet => 0xAE0,
                XK_enopensquarebullet => 0xAE1,
                XK_openrectbullet => 0xAE2,
                XK_opentribulletup => 0xAE3,
                XK_opentribulletdown => 0xAE4,
                XK_openstar => 0xAE5,
                XK_enfilledcircbullet => 0xAE6,
                XK_enfilledsqbullet => 0xAE7,
                XK_filledtribulletup => 0xAE8,
                XK_filledtribulletdown => 0xAE9,
                XK_leftpointer => 0xAEA,
                XK_rightpointer => 0xAEB,
                XK_club => 0xAEC,
                XK_diamond => 0xAED,
                XK_heart => 0xAEE,
                XK_maltesecross => 0xAF0,
                XK_dagger => 0xAF1,
                XK_doubledagger => 0xAF2,
                XK_checkmark => 0xAF3,
                XK_ballotcross => 0xAF4,
                XK_musicalsharp => 0xAF5,
                XK_musicalflat => 0xAF6,
                XK_malesymbol => 0xAF7,
                XK_femalesymbol => 0xAF8,
                XK_telephone => 0xAF9,
                XK_telephonerecorder => 0xAFA,
                XK_phonographcopyright => 0xAFB,
                XK_caret => 0xAFC,
                XK_singlelowquotemark => 0xAFD,
                XK_doublelowquotemark => 0xAFE,
                XK_cursor => 0xAFF,
                XK_leftcaret => 0xBA3,
                XK_rightcaret => 0xBA6,
                XK_downcaret => 0xBA8,
                XK_upcaret => 0xBA9,
                XK_overbar => 0xBC0,
                XK_downtack => 0xBC2,
                XK_upshoe => 0xBC3,
                XK_downstile => 0xBC4,
                XK_underbar => 0xBC6,
                XK_jot => 0xBCA,
                XK_quad => 0xBCC,
                XK_uptack => 0xBCE,
                XK_circle => 0xBCF,
                XK_upstile => 0xBD3,
                XK_downshoe => 0xBD6,
                XK_rightshoe => 0xBD8,
                XK_leftshoe => 0xBDA,
                XK_lefttack => 0xBDC,
                XK_righttack => 0xBFC,
                XK_hebrew_doublelowline => 0xCDF,
                XK_hebrew_aleph => 0xCE0,
                XK_hebrew_bet => 0xCE1,
                XK_hebrew_beth => 0xCE1,
                XK_hebrew_gimel => 0xCE2,
                XK_hebrew_gimmel => 0xCE2,
                XK_hebrew_dalet => 0xCE3,
                XK_hebrew_daleth => 0xCE3,
                XK_hebrew_he => 0xCE4,
                XK_hebrew_waw => 0xCE5,
                XK_hebrew_zain => 0xCE6,
                XK_hebrew_zayin => 0xCE6,
                XK_hebrew_chet => 0xCE7,
                XK_hebrew_het => 0xCE7,
                XK_hebrew_tet => 0xCE8,
                XK_hebrew_teth => 0xCE8,
                XK_hebrew_yod => 0xCE9,
                XK_hebrew_finalkaph => 0xCEA,
                XK_hebrew_kaph => 0xCEB,
                XK_hebrew_lamed => 0xCEC,
                XK_hebrew_finalmem => 0xCED,
                XK_hebrew_mem => 0xCEE,
                XK_hebrew_finalnun => 0xCEF,
                XK_hebrew_nun => 0xCF0,
                XK_hebrew_samech => 0xCF1,
                XK_hebrew_samekh => 0xCF1,
                XK_hebrew_ayin => 0xCF2,
                XK_hebrew_finalpe => 0xCF3,
                XK_hebrew_pe => 0xCF4,
                XK_hebrew_finalzade => 0xCF5,
                XK_hebrew_finalzadi => 0xCF5,
                XK_hebrew_zade => 0xCF6,
                XK_hebrew_zadi => 0xCF6,
                XK_hebrew_qoph => 0xCF7,
                XK_hebrew_kuf => 0xCF7,
                XK_hebrew_resh => 0xCF8,
                XK_hebrew_shin => 0xCF9,
                XK_hebrew_taw => 0xCFA,
                XK_hebrew_taf => 0xCFA,
                XK_Hebrew_switch => 0xFF7E,

                XF86XK_ModeLock => 0x1008FF01,
                XF86XK_MonBrightnessUp => 0x1008FF02,
                XF86XK_MonBrightnessDown => 0x1008FF03,
                XF86XK_KbdLightOnOff => 0x1008FF04,
                XF86XK_KbdBrightnessUp => 0x1008FF05,
                XF86XK_KbdBrightnessDown => 0x1008FF06,
                XF86XK_Standby => 0x1008FF10,
                XF86XK_AudioLowerVolume => 0x1008FF11,
                XF86XK_AudioMute => 0x1008FF12,
                XF86XK_AudioRaiseVolume => 0x1008FF13,
                XF86XK_AudioPlay => 0x1008FF14,
                XF86XK_AudioStop => 0x1008FF15,
                XF86XK_AudioPrev => 0x1008FF16,
                XF86XK_AudioNext => 0x1008FF17,
                XF86XK_HomePage => 0x1008FF18,
                XF86XK_Mail => 0x1008FF19,
                XF86XK_Start => 0x1008FF1A,
                XF86XK_Search => 0x1008FF1B,
                XF86XK_AudioRecord => 0x1008FF1C,
                XF86XK_Calculator => 0x1008FF1D,
                XF86XK_Memo => 0x1008FF1E,
                XF86XK_ToDoList => 0x1008FF1F,
                XF86XK_Calendar => 0x1008FF20,
                XF86XK_PowerDown => 0x1008FF21,
                XF86XK_ContrastAdjust => 0x1008FF22,
                XF86XK_RockerUp => 0x1008FF23,
                XF86XK_RockerDown => 0x1008FF24,
                XF86XK_RockerEnter => 0x1008FF25,
                XF86XK_Back => 0x1008FF26,
                XF86XK_Forward => 0x1008FF27,
                XF86XK_Stop => 0x1008FF28,
                XF86XK_Refresh => 0x1008FF29,
                XF86XK_PowerOff => 0x1008FF2A,
                XF86XK_WakeUp => 0x1008FF2B,
                XF86XK_Eject => 0x1008FF2C,
                XF86XK_ScreenSaver => 0x1008FF2D,
                XF86XK_WWW => 0x1008FF2E,
                XF86XK_Sleep => 0x1008FF2F,
                XF86XK_Favorites => 0x1008FF30,
                XF86XK_AudioPause => 0x1008FF31,
                XF86XK_AudioMedia => 0x1008FF32,
                XF86XK_MyComputer => 0x1008FF33,
                XF86XK_VendorHome => 0x1008FF34,
                XF86XK_LightBulb => 0x1008FF35,
                XF86XK_Shop => 0x1008FF36,
                XF86XK_History => 0x1008FF37,
                XF86XK_OpenURL => 0x1008FF38,
                XF86XK_AddFavorite => 0x1008FF39,
                XF86XK_HotLinks => 0x1008FF3A,
                XF86XK_BrightnessAdjust => 0x1008FF3B,
                XF86XK_Finance => 0x1008FF3C,
                XF86XK_Community => 0x1008FF3D,
                XF86XK_AudioRewind => 0x1008FF3E,
                XF86XK_BackForward => 0x1008FF3F,
                XF86XK_Launch0 => 0x1008FF40,
                XF86XK_Launch1 => 0x1008FF41,
                XF86XK_Launch2 => 0x1008FF42,
                XF86XK_Launch3 => 0x1008FF43,
                XF86XK_Launch4 => 0x1008FF44,
                XF86XK_Launch5 => 0x1008FF45,
                XF86XK_Launch6 => 0x1008FF46,
                XF86XK_Launch7 => 0x1008FF47,
                XF86XK_Launch8 => 0x1008FF48,
                XF86XK_Launch9 => 0x1008FF49,
                XF86XK_LaunchA => 0x1008FF4A,
                XF86XK_LaunchB => 0x1008FF4B,
                XF86XK_LaunchC => 0x1008FF4C,
                XF86XK_LaunchD => 0x1008FF4D,
                XF86XK_LaunchE => 0x1008FF4E,
                XF86XK_LaunchF => 0x1008FF4F,
                XF86XK_ApplicationLeft => 0x1008FF50,
                XF86XK_ApplicationRight => 0x1008FF51,
                XF86XK_Book => 0x1008FF52,
                XF86XK_CD => 0x1008FF53,
                XF86XK_Calculater => 0x1008FF54,
                XF86XK_Clear => 0x1008FF55,
                XF86XK_Close => 0x1008FF56,
                XF86XK_Copy => 0x1008FF57,
                XF86XK_Cut => 0x1008FF58,
                XF86XK_Display => 0x1008FF59,
                XF86XK_DOS => 0x1008FF5A,
                XF86XK_Documents => 0x1008FF5B,
                XF86XK_Excel => 0x1008FF5C,
                XF86XK_Explorer => 0x1008FF5D,
                XF86XK_Game => 0x1008FF5E,
                XF86XK_Go => 0x1008FF5F,
                XF86XK_iTouch => 0x1008FF60,
                XF86XK_LogOff => 0x1008FF61,
                XF86XK_Market => 0x1008FF62,
                XF86XK_Meeting => 0x1008FF63,
                XF86XK_MenuKB => 0x1008FF65,
                XF86XK_MenuPB => 0x1008FF66,
                XF86XK_MySites => 0x1008FF67,
                XF86XK_New => 0x1008FF68,
                XF86XK_News => 0x1008FF69,
                XF86XK_OfficeHome => 0x1008FF6A,
                XF86XK_Open => 0x1008FF6B,
                XF86XK_Option => 0x1008FF6C,
                XF86XK_Paste => 0x1008FF6D,
                XF86XK_Phone => 0x1008FF6E,
                XF86XK_Q => 0x1008FF70,
                XF86XK_Reply => 0x1008FF72,
                XF86XK_Reload => 0x1008FF73,
                XF86XK_RotateWindows => 0x1008FF74,
                XF86XK_RotationPB => 0x1008FF75,
                XF86XK_RotationKB => 0x1008FF76,
                XF86XK_Save => 0x1008FF77,
                XF86XK_ScrollUp => 0x1008FF78,
                XF86XK_ScrollDown => 0x1008FF79,
                XF86XK_ScrollClick => 0x1008FF7A,
                XF86XK_Send => 0x1008FF7B,
                XF86XK_Spell => 0x1008FF7C,
                XF86XK_SplitScreen => 0x1008FF7D,
                XF86XK_Support => 0x1008FF7E,
                XF86XK_TaskPane => 0x1008FF7F,
                XF86XK_Terminal => 0x1008FF80,
                XF86XK_Tools => 0x1008FF81,
                XF86XK_Travel => 0x1008FF82,
                XF86XK_UserPB => 0x1008FF84,
                XF86XK_User1KB => 0x1008FF85,
                XF86XK_User2KB => 0x1008FF86,
                XF86XK_Video => 0x1008FF87,
                XF86XK_WheelButton => 0x1008FF88,
                XF86XK_Word => 0x1008FF89,
                XF86XK_Xfer => 0x1008FF8A,
                XF86XK_ZoomIn => 0x1008FF8B,
                XF86XK_ZoomOut => 0x1008FF8C,
                XF86XK_Away => 0x1008FF8D,
                XF86XK_Messenger => 0x1008FF8E,
                XF86XK_WebCam => 0x1008FF8F,
                XF86XK_MailForward => 0x1008FF90,
                XF86XK_Pictures => 0x1008FF91,
                XF86XK_Music => 0x1008FF92,
                XF86XK_Battery => 0x1008FF93,
                XF86XK_Bluetooth => 0x1008FF94,
                XF86XK_WLAN => 0x1008FF95,
                XF86XK_UWB => 0x1008FF96,
                XF86XK_AudioForward => 0x1008FF97,
                XF86XK_AudioRepeat => 0x1008FF98,
                XF86XK_AudioRandomPlay => 0x1008FF99,
                XF86XK_Subtitle => 0x1008FF9A,
                XF86XK_AudioCycleTrack => 0x1008FF9B,
                XF86XK_CycleAngle => 0x1008FF9C,
                XF86XK_FrameBack => 0x1008FF9D,
                XF86XK_FrameForward => 0x1008FF9E,
                XF86XK_Time => 0x1008FF9F,
                XF86XK_Select => 0x1008FFA0,
                XF86XK_View => 0x1008FFA1,
                XF86XK_TopMenu => 0x1008FFA2,
                XF86XK_Red => 0x1008FFA3,
                XF86XK_Green => 0x1008FFA4,
                XF86XK_Yellow => 0x1008FFA5,
                XF86XK_Blue => 0x1008FFA6,
                XF86XK_Suspend => 0x1008FFA7,
                XF86XK_Hibernate => 0x1008FFA8,
                XF86XK_TouchpadToggle => 0x1008FFA9,
                XF86XK_TouchpadOn => 0x1008FFB0,
                XF86XK_TouchpadOff => 0x1008FFB1,
                XF86XK_AudioMicMute => 0x1008FFB2,
                XF86XK_Switch_VT_1 => 0x1008FE01,
                XF86XK_Switch_VT_2 => 0x1008FE02,
                XF86XK_Switch_VT_3 => 0x1008FE03,
                XF86XK_Switch_VT_4 => 0x1008FE04,
                XF86XK_Switch_VT_5 => 0x1008FE05,
                XF86XK_Switch_VT_6 => 0x1008FE06,
                XF86XK_Switch_VT_7 => 0x1008FE07,
                XF86XK_Switch_VT_8 => 0x1008FE08,
                XF86XK_Switch_VT_9 => 0x1008FE09,
                XF86XK_Switch_VT_10 => 0x1008FE0A,
                XF86XK_Switch_VT_11 => 0x1008FE0B,
                XF86XK_Switch_VT_12 => 0x1008FE0C,
                XF86XK_Ungrab => 0x1008FE20,
                XF86XK_ClearGrab => 0x1008FE21,
                XF86XK_Next_VMode => 0x1008FE22,
                XF86XK_Prev_VMode => 0x1008FE23,
                XF86XK_LogWindowTree => 0x1008FE24,
                XF86XK_LogGrabInfo => 0x1008FE25,

                XK_ISO_Lock => 0xFE01,
                XK_ISO_Level2_Latch => 0xFE02,
                XK_ISO_Level3_Shift => 0xFE03,
                XK_ISO_Level3_Latch => 0xFE04,
                XK_ISO_Level3_Lock => 0xFE05,
                XK_ISO_Level5_Shift => 0xFE11,
                XK_ISO_Level5_Latch => 0xFE12,
                XK_ISO_Level5_Lock => 0xFE13,
                XK_ISO_Group_Shift => 0xFF7E,
                XK_ISO_Group_Latch => 0xFE06,
                XK_ISO_Group_Lock => 0xFE07,
                XK_ISO_Next_Group => 0xFE08,
                XK_ISO_Next_Group_Lock => 0xFE09,
                XK_ISO_Prev_Group => 0xFE0A,
                XK_ISO_Prev_Group_Lock => 0xFE0B,
                XK_ISO_First_Group => 0xFE0C,
                XK_ISO_First_Group_Lock => 0xFE0D,
                XK_ISO_Last_Group => 0xFE0E,
                XK_ISO_Last_Group_Lock => 0xFE0F,

                XK_ISO_Left_Tab => 0xFE20,
                XK_ISO_Move_Line_Up => 0xFE21,
                XK_ISO_Move_Line_Down => 0xFE22,
                XK_ISO_Partial_Line_Up => 0xFE23,
                XK_ISO_Partial_Line_Down => 0xFE24,
                XK_ISO_Partial_Space_Left => 0xFE25,
                XK_ISO_Partial_Space_Right => 0xFE26,
                XK_ISO_Set_Margin_Left => 0xFE27,
                XK_ISO_Set_Margin_Right => 0xFE28,
                XK_ISO_Release_Margin_Left => 0xFE29,
                XK_ISO_Release_Margin_Right => 0xFE2A,
                XK_ISO_Release_Both_Margins => 0xFE2B,
                XK_ISO_Fast_Cursor_Left => 0xFE2C,
                XK_ISO_Fast_Cursor_Right => 0xFE2D,
                XK_ISO_Fast_Cursor_Up => 0xFE2E,
                XK_ISO_Fast_Cursor_Down => 0xFE2F,
                XK_ISO_Continuous_Underline => 0xFE30,
                XK_ISO_Discontinuous_Underline => 0xFE31,
                XK_ISO_Emphasize => 0xFE32,
                XK_ISO_Center_Object => 0xFE33,
                XK_ISO_Enter => 0xFE34,

                XK_dead_grave => 0xFE50,
                XK_dead_acute => 0xFE51,
                XK_dead_circumflex => 0xFE52,
                XK_dead_tilde => 0xFE53,
                XK_dead_perispomeni => 0xFE53,
                XK_dead_macron => 0xFE54,
                XK_dead_breve => 0xFE55,
                XK_dead_abovedot => 0xFE56,
                XK_dead_diaeresis => 0xFE57,
                XK_dead_abovering => 0xFE58,
                XK_dead_doubleacute => 0xFE59,
                XK_dead_caron => 0xFE5A,
                XK_dead_cedilla => 0xFE5B,
                XK_dead_ogonek => 0xFE5C,
                XK_dead_iota => 0xFE5D,
                XK_dead_voiced_sound => 0xFE5E,
                XK_dead_semivoiced_sound => 0xFE5F,
                XK_dead_belowdot => 0xFE60,
                XK_dead_hook => 0xFE61,
                XK_dead_horn => 0xFE62,
                XK_dead_stroke => 0xFE63,
                XK_dead_abovecomma => 0xFE64,
                XK_dead_psili => 0xFE64,
                XK_dead_abovereversedcomma => 0xFE65,
                XK_dead_dasia => 0xFE65,
                XK_dead_doublegrave => 0xFE66,
                XK_dead_belowring => 0xFE67,
                XK_dead_belowmacron => 0xFE68,
                XK_dead_belowcircumflex => 0xFE69,
                XK_dead_belowtilde => 0xFE6A,
                XK_dead_belowbreve => 0xFE6B,
                XK_dead_belowdiaeresis => 0xFE6C,
                XK_dead_invertedbreve => 0xFE6D,
                XK_dead_belowcomma => 0xFE6E,
                XK_dead_currency => 0xFE6F,

                XK_dead_lowline => 0xFE90,
                XK_dead_aboveverticalline => 0xFE91,
                XK_dead_belowverticalline => 0xFE92,
                XK_dead_longsolidusoverlay => 0xFE93,

                XK_dead_a => 0xFE80,
                XK_dead_A => 0xFE81,
                XK_dead_e => 0xFE82,
                XK_dead_E => 0xFE83,
                XK_dead_i => 0xFE84,
                XK_dead_I => 0xFE85,
                XK_dead_o => 0xFE86,
                XK_dead_O => 0xFE87,
                XK_dead_u => 0xFE88,
                XK_dead_U => 0xFE89,
                XK_dead_small_schwa => 0xFE8A,
                XK_dead_capital_schwa => 0xFE8B,

                XK_dead_greek => 0xFE8C,

                XK_First_Virtual_Screen => 0xFED0,
                XK_Prev_Virtual_Screen => 0xFED1,
                XK_Next_Virtual_Screen => 0xFED2,
                XK_Last_Virtual_Screen => 0xFED4,
                XK_Terminate_Server => 0xFED5,

                XK_AccessX_Enable => 0xFE70,
                XK_AccessX_Feedback_Enable => 0xFE71,
                XK_RepeatKeys_Enable => 0xFE72,
                XK_SlowKeys_Enable => 0xFE73,
                XK_BounceKeys_Enable => 0xFE74,
                XK_StickyKeys_Enable => 0xFE75,
                XK_MouseKeys_Enable => 0xFE76,
                XK_MouseKeys_Accel_Enable => 0xFE77,
                XK_Overlay1_Enable => 0xFE78,
                XK_Overlay2_Enable => 0xFE79,
                XK_AudibleBell_Enable => 0xFE7A,

                XK_Pointer_Left => 0xFEE0,
                XK_Pointer_Right => 0xFEE1,
                XK_Pointer_Up => 0xFEE2,
                XK_Pointer_Down => 0xFEE3,
                XK_Pointer_UpLeft => 0xFEE4,
                XK_Pointer_UpRight => 0xFEE5,
                XK_Pointer_DownLeft => 0xFEE6,
                XK_Pointer_DownRight => 0xFEE7,
                XK_Pointer_Button_Dflt => 0xFEE8,
                XK_Pointer_Button1 => 0xFEE9,
                XK_Pointer_Button2 => 0xFEEA,
                XK_Pointer_Button3 => 0xFEEB,
                XK_Pointer_Button4 => 0xFEEC,
                XK_Pointer_Button5 => 0xFEED,
                XK_Pointer_DblClick_Dflt => 0xFEEE,
                XK_Pointer_DblClick1 => 0xFEEF,
                XK_Pointer_DblClick2 => 0xFEF0,
                XK_Pointer_DblClick3 => 0xFEF1,
                XK_Pointer_DblClick4 => 0xFEF2,
                XK_Pointer_DblClick5 => 0xFEF3,
                XK_Pointer_Drag_Dflt => 0xFEF4,
                XK_Pointer_Drag1 => 0xFEF5,
                XK_Pointer_Drag2 => 0xFEF6,
                XK_Pointer_Drag3 => 0xFEF7,
                XK_Pointer_Drag4 => 0xFEF8,
                XK_Pointer_Drag5 => 0xFEFD,

                XK_Pointer_EnableKeys => 0xFEF9,
                XK_Pointer_Accelerate => 0xFEFA,
                XK_Pointer_DfltBtnNext => 0xFEFB,
                XK_Pointer_DfltBtnPrev => 0xFEFC,

                XK_ch => 0xFEA0,
                XK_Ch => 0xFEA1,
                XK_CH => 0xFEA2,
                XK_c_h => 0xFEA3,
                XK_C_h => 0xFEA4,
                XK_C_H => 0xFEA5,
            } as u32)
                .to_le_bytes()
                .to_vec()
                .into_iter()
                .filter(|&b| b > 0)
                .collect(),
        )
    }
}
