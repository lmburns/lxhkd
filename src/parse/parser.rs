//! Parse a configuration file of bindings. Turn them into tokens

use crate::{
    config::{Action, Config},
    keys::{
        chord::{Chain, Chord},
        keys::{CharacterMap, ModifierMask},
        keysym::KeysymHash,
    },
    lxhkd_fatal,
};
use anyhow::{Context, Result};
use colored::Colorize;
use indexmap::IndexMap;
use itertools::{Itertools, PeekingNext, PeekingTakeWhile};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt,
    num::ParseIntError,
    ops::Range,
    process::{Command, Stdio},
    str::FromStr,
};
use thiserror::Error;

// =================== Errors =====================

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("failed to lookup codepoint from string: {0}")]
    CodepointLookup(String),
    #[error("failed to lookup `CharacterMap` from string: {0}")]
    UTFLookup(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Self::Keycode(code) => format!("Keycode({})", code),
            Self::Release => String::from("Release"),
            Self::KeysymStart => String::from("["),
            Self::KeysymEnd => String::from("]"),
            Self::KeysymCode(code) => format!("Keysym({})", code),
            Self::KeysymString(ref string) => format!("Keysym({})", string),
            Self::Mouse(code) => format!("Mouse({})", code),
            Self::Modifier(ref modifier) => format!("Modifier({})", modifier),
            Self::Text(ref text) => format!("Text({})", text),
            Self::Char(ch) => format!("Char({})", ch),
            Self::UnknownChar(ref ch) => format!("UnknownChar({})", ch),
            Self::OptionGroup(ref group) => format!("OptionGroup({:#?})", group),
            Self::OptionStart => String::from("{"),
            Self::OptionEnd => String::from("}"),
            Self::RangeGroup(ref group) => format!("RangeGroup({:#?})", group),
            Self::Comma => String::from(","),
            Self::Plus => String::from("+"),
            Self::Dash => String::from("-"),
            Self::Unknown(ref string) => format!("UnknownString({})", string),
            Self::Invalid => String::from("Invalid"),
        })
    }
}

// =================== Tokens =====================

/// Various tokens used to parse the keybindings
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    // MouseRelease(usize),
    // Command,
    Keycode(u8),

    /// Release event (mouse and key)
    Release,
    /// Start of specifying a keysym code '['
    KeysymStart,
    /// End of specifying a keysym code ']'
    KeysymEnd,
    /// A keysym code keysym that is between the above two
    KeysymCode(u32),
    /// A keysym string
    KeysymString(String),
    /// Final: A mouse button
    Mouse(usize),
    /// Final: A modifier key
    Modifier(String),
    /// A catch all used before further parsing
    Text(String),
    /// Final: A single character key. E.g., 'a' in this: super + 'a'
    Char(char),
    /// A single character that is not found in `KeysymHash`
    UnknownChar(char),
    /// The start of an option '{'
    OptionStart,
    /// The end of an option '}'
    OptionEnd,
    /// The expanded items within an Option
    OptionGroup(Vec<char>),
    /// The expanded items within an Option
    RangeGroup(Vec<char>),
    /// A comma
    Comma,
    /// A plus sign
    Plus,
    /// A dash, usually located within an option
    Dash,
    Unknown(String),
    /// Used as a finalized placeholder to filter results that do not contain
    /// this
    Invalid,
}

// ==================== Line ======================

// TODO: Keycode within option range

// /// A single line in a configuration file
// #[derive(Debug, Clone)]
// pub(crate) struct SingleLine {
//     /// Set of keys that will be mapped to a shell command or remapped to
//     /// another set of keys
//     pub(crate) chain:  Chain,
//     /// Action to be executed (shell command or a remap)
//     pub(crate) action: Action,
// }

/// A split line in the configuration file
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Line<'a> {
    /// Index the line is in the `IndexMap` of keys and bindings
    pub(crate) line_num: usize,
    /// The length of the characters in the line
    pub(crate) char_len: usize,
    /// The number of possible `Tokens`
    pub(crate) n_keys:   usize,
    /// The line split on '+'
    pub(crate) vector:   Vec<&'a str>,
}

impl<'a> Line<'a> {
    /// Create a new `Line` by splitting on '+'
    pub(crate) fn new(line: &'a str, idx: usize) -> Self {
        if line.is_empty() {
            Self {
                line_num: idx,
                char_len: line.len(),
                n_keys:   0,
                vector:   vec![],
            }
        } else {
            let split = line.split('+').map(str::trim).collect::<Vec<_>>();
            Self {
                line_num: idx,
                char_len: line.len(),
                n_keys:   split.len(),
                vector:   split,
            }
        }
    }

    /// Create a new `Line` by splitting on '+' but keeping the '+' characters
    /// in the split
    pub(crate) fn new_plus(line: &'a str, idx: usize) -> Self {
        let mut split = vec![];
        let mut last = 0;

        for (idx, matched) in line.match_indices(|c| c == '+') {
            // Pushes 'super'
            if last != idx {
                split.push(line[last..idx].trim());
            }
            // Pushes '+'
            split.push(matched.trim());

            last = idx + matched.len();
        }
        // Pushes rest
        if last < line.len() {
            split.push(line[last..].trim());
        }

        // This is for whenever there are no '+' within the binding.
        // For example, "~mouse2"
        if split.is_empty() {
            Self::new(line, idx)
        } else {
            Self {
                line_num: idx,
                char_len: line.len(),
                n_keys:   split.len(),
                vector:   split,
            }
        }
    }

    /// Create a `TokenizedLine`
    pub(crate) fn tokenize(&self) -> TokenizedLine {
        let mut all_res = vec![];
        let token_text = |text: &mut String, res: &mut Vec<Token>| {
            if !text.is_empty() {
                res.push(Token::Text(String::from(&*text)));
                text.clear();
            }
        };

        for tok in &self.vector {
            let mut res = vec![];
            let mut text = String::new();
            let mut chars = tok.chars().multipeek();
            let mut start = 0;

            while let Some(&c) = chars.peek() {
                start += 0;
                match c {
                    '{' => {
                        token_text(&mut text, &mut res);
                        res.push(Token::OptionStart);
                    },
                    '}' => {
                        token_text(&mut text, &mut res);
                        res.push(Token::OptionEnd);
                    },
                    '[' => {
                        token_text(&mut text, &mut res);
                        res.push(Token::KeysymStart);
                    },
                    ']' => {
                        token_text(&mut text, &mut res);
                        res.push(Token::KeysymEnd);
                    },
                    '-' => {
                        token_text(&mut text, &mut res);
                        res.push(Token::Dash);
                    },
                    '+' => {
                        token_text(&mut text, &mut res);
                        res.push(Token::Plus);
                    },
                    ',' => {
                        token_text(&mut text, &mut res);
                        res.push(Token::Comma);
                    },
                    '~' => {
                        token_text(&mut text, &mut res);
                        res.push(Token::Release);
                    },
                    ' ' => {
                        token_text(&mut text, &mut res);
                    },
                    ch => {
                        // println!("letter: {:#?} peek1: {:#?}", ch, chars.peek());
                        text.push(ch);
                    },
                }
                chars.next();
            }
            // println!("RES: {:#?}", res);
            // println!("ALL: {:#?}", all_res);
            if !text.is_empty() {
                res.push(Token::Text(text));
            }
            if !res.is_empty() {
                all_res.push(res);
            }
        }

        TokenizedLine { line: self.clone(), tokenized: all_res }
    }
}

impl fmt::Display for Line<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.vector.join(" "))
    }
}

// ============= Regex + Modifiers ================

// TODO: Do something with `fn`

#[rustfmt::skip]
const MODIFIER_STR: &[&str] = &[
    "alt",   "lalt",      "ralt",       "Alt_L",     "Alt_R",
    "shift", "lshift",    "rshift",     "Shift_L",   "Shift_R",
    "super", "lsuper",    "rsuper",     "Super_L",   "Super_R",
    "meta",  "lmeta",     "rmeta",      "Meta_L",    "Meta_R",
    "ctrl",  "lctrl",     "rctrl",      "Control_L", "Control_R",
    "hyper", "lhyper",    "rhyper",     "Hyper_L",   "Hyper_R",
    "mod1",  "mod2",      "mod3",       "mod4",      "mod5",
    "lock",  "Caps_Lock", "Shift_Lock", "Num_Lock",  "Scroll_Lock", // Unsure if these are needed
    "fn",    "meh",
];

// static BIND_RANGE_PATTERN: Lazy<Regex> =
//     Lazy::new(|| Regex::new(r"([0-9a-z])-([0-9a-z])").unwrap());
// static NUM_RANGE_PATTERN: Lazy<Regex> = Lazy::new(||
// Regex::new(r"([0-9]+)-([0-9]+)").unwrap()); static ALPHA_RANGE_PATTERN:
// Lazy<Regex> = Lazy::new(|| Regex::new(r"([a-z])-([a-z])").unwrap());
// static CMD_RANGE_PATTERN: Lazy<Regex> =
//     Lazy::new(||
// Regex::new(r"(?m)^(([0-9]+)-([0-9]+))|(([a-z]+)-([a-z]+))$").unwrap());

static MOD_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(&MODIFIER_STR.join("|")).unwrap());
static MOUSE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"mouse([0-9]+)").unwrap());

// =============== TokenizedLine ==================

/// A line in the configuration file after being tokenized
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TokenizedLine<'a> {
    /// The split line
    pub(crate) line:      Line<'a>,
    /// The tokenized line
    pub(crate) tokenized: Vec<Vec<Token>>,
}

impl<'a> TokenizedLine<'a> {
    /// Test whether the keybinding contains a `Token::Release`
    pub(crate) fn contains_release(&self, idx: usize) -> bool {
        self.tokenized[idx].contains(&Token::Release)
    }

    /// Test whether the keybinding is a `Release`
    pub(crate) fn is_release(&self, idx: usize) -> bool {
        self.tokenized[idx][0] == Token::Release
    }

    /// Flatten the vector of vectors into one vector
    pub(crate) fn flatten_it(&self) -> Vec<&Token> {
        self.tokenized.iter().fold(Vec::new(), |mut acc, c| {
            c.iter().for_each(|i| acc.push(i));
            acc
        })
    }

    /// Finalize the tokenization by returning a flattened vector that is split
    /// on `Token::Plus`
    pub(crate) fn finalize_split(&mut self) -> Vec<&Token> {
        let tokes = self.flatten_it();

        // Another fold is required because after the split, a vector of slices is
        // returned
        let split = tokes
            .split(|c| **c == Token::Plus)
            .fold(Vec::new(), |mut acc, c| {
                c.iter().for_each(|i| acc.push(*i));
                acc
            })
            .iter()
            .copied()
            .collect::<Vec<_>>();

        split
    }

    /// Map modifiers to their correct UTF-8 representations
    pub(crate) fn map_modifiers(charmaps: &'a [CharacterMap], tomatch: &'a str) -> &'a str {
        // Will match the `modmask` level of `modN` keys with the `CharacterMap`
        // database, returning the actual modifier for that `mod` key. For example,
        // `match_modmask(3, "Alt_L")` will match `mod1` and if it fails, `Alt_L` will
        // be used instead. `Alt_L` is used because that is what it is by default
        let match_modmask = |mask: u16, or: &'a str| -> &'a str {
            charmaps
                .iter()
                .find(|m| m.modmask == (1 << mask))
                .map_or(or, |a| &a.utf)
        };

        match tomatch.trim() {
            "super" | "lsuper" => "Super_L",
            "rsuper" => "Super_R",
            "hyper" | "lhyper" => "Hyper_L",
            "rhyper" => "Hyper_R",
            "alt" | "lalt" => "Alt_L",
            "ralt" => "Alt_R",
            "shift" | "lshift" => "Shift_L",
            "rshift" => "Shift_R",
            "ctrl" | "lctrl" => "Control_L",
            "rctrl" => "Control_R",
            "mod1" => match_modmask(3, "Alt_L"),
            "mod2" => match_modmask(4, "Num_Lock"),
            // This one is probably not set on most people's keyboards
            "mod3" => match_modmask(5, "Hyper_L"),
            "mod4" => match_modmask(6, "Super_L"),
            "mod5" => match_modmask(7, "ISO_Level3_Lock"),

            // This will catch the actual `Keysym` named modifiers
            // i.e., the names on the right side of the match above
            other => other,
        }
    }

    /// Convert a `TokenizedLine` to a `Chain`
    pub(crate) fn convert_to_chain(&'a mut self, charmaps: &'a [CharacterMap]) -> Option<Chain> {
        let line = self.finalize_split();
        let mut chords = vec![];
        let mut is_release = false;
        let mut modmask = ModifierMask::new(0);

        if line.contains(&&Token::Invalid) {
            return None;
        }

        for bind in line {
            if let Token::Modifier(modifier) = bind {
                let mapped = Self::map_modifiers(charmaps, modifier);
                if let Some(charmap) = CharacterMap::charmap_from_keysym_utf(charmaps, mapped) {
                    log::debug!(
                        "found `modifier`: {}\n{:#?}",
                        mapped.yellow().bold(),
                        charmap
                    );
                    modmask.combine_u16(charmap.modmask);
                    chords.push(Chord::new(&charmap, charmap.modmask));
                } else {
                    log::info!("{} was not found in the `CharacterMap` database", mapped);
                }
            } else if let Token::Char(ch) = bind {
                if let Some(charmap) =
                    CharacterMap::charmap_from_keysym_utf(charmaps, &ch.to_string())
                {
                    log::debug!(
                        "found `char`: {}\n{:#?}",
                        ch.to_string().yellow().bold(),
                        charmap
                    );
                    chords.push(Chord::new(&charmap, charmap.modmask));
                } else {
                    log::info!("{} was not found in the `CharacterMap` database", ch);
                }
            } else if *bind == Token::Release {
                is_release = true;
            }
        }

        Some(Chain::new(chords, is_release, modmask))
    }

    /// A function that runs all of the tokenizing/parsing functions in the
    /// order they're supposed to be ran
    pub(crate) fn parse_tokens(&mut self) -> Result<()> {
        // Check for `Keysym` indicators before matching with regex
        // which may convert it to a `char`
        self.check_keysym();

        /// Tokenize with regular expressions. Perhaps only using these, with
        /// less iteration and less tokenizing would be easier
        self.regex_token()?;

        /// Expand ranges ({a-c}) after converting items to `char` from
        /// `regex_token`
        self.expand_range();

        /// Split `Options` ({a,c}) after converting items to `char` from
        /// `regex_token`
        self.split_option();

        log::debug!("{}: {:#?}", "Binding".green().bold(), self.tokenized);
        // println!("RES: {:#?}", self.tokenized);

        Ok(())
    }

    /// Further tokenize the `Token::Text` items in the line by using regular
    /// expressions. Modifies the `TokenizedLine` in place, resulting in a
    /// `Mouse`, `Modifier`, and `Char` `Token`s being added
    pub(crate) fn regex_token(&mut self) -> Result<()> {
        let hash = KeysymHash::HASH;

        for vec_idx in 0..self.tokenized.len() {
            for tok_idx in 0..self.tokenized[vec_idx].len() {
                match &self.tokenized[vec_idx][tok_idx] {
                    Token::Text(text) | Token::KeysymString(text) =>
                        if MOD_PATTERN.is_match(text) {
                            log::trace!("Found {}({})", "Token::Modifier".red().bold(), text);
                            self.tokenized[vec_idx][tok_idx] = Token::Modifier(text.to_string());
                        } else if MOUSE_PATTERN.is_match(text) {
                            log::trace!("Found {}({})", "Token::Mouse".red().bold(), text);
                            self.tokenized[vec_idx][tok_idx] =
                                Token::Mouse(text.replace("mouse", "").parse::<usize>().context(
                                    "mouse buttons are defined by 'mouseN' where 'N' is a number \
                                     1-5",
                                )?);
                        } else if text.len() == 1 {
                            let char = text
                                .parse::<char>()
                                .context("failure in determining length of character")?;

                            if hash.get_keysym_code_from_char(char).is_some() {
                                log::debug!("Found {}({})", "Token::Char".red().bold(), text);
                                self.tokenized[vec_idx][tok_idx] = Token::Char(char);
                            } else {
                                log::info!(
                                    "Unkown {}({})",
                                    "Token::UnknownChar".red().bold(),
                                    text
                                );
                                self.tokenized[vec_idx][tok_idx] = Token::UnknownChar(char);
                            }
                        },
                    _ => {},
                }
            }
        }

        Ok(())
    }

    /// Check if the `TokenizedLine` contains a `Keysym` start indicator '[' to
    /// map the string to the correct code
    pub(crate) fn check_keysym(&mut self) -> Result<()> {
        let hash = KeysymHash::HASH;

        for vec_idx in 0..self.tokenized.len() {
            for tok_idx in 0..self.tokenized[vec_idx].len() {
                if self.tokenized[vec_idx][tok_idx] == Token::KeysymStart {
                    if let Token::Text(keysym_code) = &self.tokenized[vec_idx][tok_idx + 1] {
                        let code = {
                            if keysym_code.starts_with("0x") {
                                u32::from_str_radix(keysym_code.trim_start_matches("0x"), 16)
                            } else {
                                keysym_code.parse::<u32>()
                            }
                        }
                        .with_context(|| {
                            format!(
                                "a Keysym code was indicated in the configuration file, but {} is \
                                 not a number. Either remove the brackets '[]' or use a valid \
                                 Keysym code",
                                keysym_code.red().bold()
                            )
                        })?;

                        if let Some(keysym_str) = hash.get_str_from_keysym_code(code) {
                            log::info!(
                                "<{}> - Found {}({}) => {}",
                                self.line.to_string().purple().bold(),
                                "Token::Keysym".red().bold(),
                                keysym_code.yellow().bold(),
                                keysym_str.green().bold()
                            );
                            let mut res = vec![];
                            if self.contains_release(vec_idx) {
                                res.push(Token::Release);
                            }
                            res.push(Token::KeysymString(keysym_str.clone()));
                            self.tokenized[vec_idx] = res;
                            break;
                        }
                        log::info!(
                            "<{}> - Unknown keysym code: {}",
                            self.line.to_string().purple().bold(),
                            keysym_code.yellow().bold()
                        );
                        self.tokenized[vec_idx][tok_idx + 1] =
                            Token::Unknown(keysym_code.to_string());
                    }
                }
            }
        }

        Ok(())
    }

    /// Split the comma within an option to convert: {a,c,b} to an
    /// `OptionGroup('a', 'c', 'b')`
    pub(crate) fn split_option(&mut self) {
        for vec_idx in 0..self.tokenized.len() {
            if self.tokenized[vec_idx].contains(&Token::OptionStart)
                && self.tokenized[vec_idx].contains(&Token::Comma)
                && self.tokenized[vec_idx].contains(&Token::OptionEnd)
            {
                for tok_idx in 0..self.tokenized[vec_idx].len() {
                    let mut is_release = false;
                    if self.is_release(vec_idx) {
                        is_release = true;
                        self.tokenized[vec_idx].remove(0);
                    }
                    // Don't match the end option because there can be more than two `char`
                    if let [Token::OptionStart, Token::Char(_), Token::Comma, Token::Char(_)] =
                        &self.tokenized[vec_idx][..4]
                    {
                        // Split and take every other item
                        let splits = &self.tokenized[vec_idx]
                            .par_iter()
                            .skip(1)
                            .step_by(2)
                            .cloned()
                            .collect::<Vec<_>>();

                        if splits.par_iter().all(|c| matches!(c, Token::Char(_))) {
                            self.tokenized[vec_idx] = vec![Token::OptionGroup(
                                splits
                                    .par_iter()
                                    .filter_map(|c| match *c {
                                        Token::Char(ch) => Some(ch),
                                        _ => None,
                                    })
                                    .collect::<Vec<_>>(),
                            )];
                            if is_release {
                                self.tokenized[vec_idx].insert(0, Token::Release);
                            }
                            break;
                        }
                    }
                    log::warn!(
                        "invalid option found in configuration: {}\n{} may only be single 'char's \
                         or digits. For example: {{a,b,c}}",
                        "Options".green().bold(),
                        self.line.vector[vec_idx].to_string().red().bold()
                    );
                    // Was an invalid option so replace item with an `Invalid` token
                    self.tokenized[vec_idx] = vec![Token::Invalid];
                }
            }
        }
    }

    // TODO: Possibly add support for keysym codes in ranges
    // TODO: Check char len, if ranges are single chars, it will be 5

    /// Expand the range operator within an option to convert {a-c} to
    /// `RangeGroup('a', 'b', 'c')`
    #[rustfmt::skip] // Can't use on statements without nightly
    pub(crate) fn expand_range(&mut self) {
        for vec_idx in 0..self.tokenized.len() {
            // Shitty way, but check if it contains {..}
            if self.tokenized[vec_idx].contains(&Token::OptionStart)
                && self.tokenized[vec_idx].contains(&Token::Dash)
                && self.tokenized[vec_idx].contains(&Token::OptionEnd)
            {
                let tok_idx = 0;
                let mut is_release = false;
                // Remove release to add back later after parsing
                if self.is_release(vec_idx) {
                    is_release = true;
                    self.tokenized[vec_idx].remove(0);
                }

                if self.tokenized[vec_idx][tok_idx] == Token::OptionStart {
                    if let [
                        Token::OptionStart,
                        Token::Char(_),
                        Token::Dash,
                        Token::Char(_),
                        Token::OptionEnd
                    ] =
                        &self.tokenized[vec_idx][..]
                    {
                        let start = &self.tokenized[vec_idx][tok_idx + 1];
                        let end = &self.tokenized[vec_idx][tok_idx + 3];
                        log::info!(
                            "<{}> - Found {}({})",
                            self.line.to_string().purple().bold(),
                            "Token::RangeGroup".red().bold(),
                            format!("{}-{}", start, end)
                        );

                        let mut chars = vec![];
                        if let Token::Char(s) = start {
                            if let Token::Char(e) = end {
                                if e < s {
                                    log::warn!(
                                        "<{}>: range end ({}) must be less than range start ({})",
                                        self.line.vector[vec_idx].red().bold(),
                                        e,
                                        s
                                    );
                                    self.tokenized[vec_idx] = vec![Token::Invalid];
                                    break;
                                }

                                for n in *s..=*e {
                                    chars.push(n);
                                }
                            }
                        }

                        if !chars.is_empty() {
                            self.tokenized[vec_idx] = vec![Token::RangeGroup(chars)];
                            if is_release {
                                self.tokenized[vec_idx].insert(0, Token::Release);
                            }
                            break;
                        }
                    }
                }
                log::warn!(
                    "invalid range found in configuration: {}",
                    "Ranges".green().bold(),
                );
                log::warn!(
                    "{} may only be single 'char's or digits. For example: {{a-e}}",
                    self.line.vector[vec_idx].to_string().red().bold()
                );
                // Was an invalid range so replace item with an `Invalid` token
                self.tokenized[vec_idx] = vec![Token::Invalid];
            }
        }
    }
}

// ==================== Tests =====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_empty() -> Result<()> {
        let line = Line::new_plus("", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert!(tokenized.tokenized.is_empty());
        Ok(())
    }

    #[test]
    fn token_char() -> Result<()> {
        let line = Line::new_plus("a", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::Char('a')]]);
        Ok(())
    }

    #[test]
    fn token_text() -> Result<()> {
        let line = Line::new_plus("abc", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::Text(String::from(
            "abc"
        ))]]);
        Ok(())
    }

    #[test]
    fn token_modifier() -> Result<()> {
        let line = Line::new_plus("super", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::Modifier(
            String::from("super")
        )]]);
        Ok(())
    }

    #[test]
    fn token_modifier_left() -> Result<()> {
        let line = Line::new_plus("lsuper", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::Modifier(
            String::from("lsuper")
        )]]);
        Ok(())
    }

    #[test]
    fn token_modifier_left_keysym_str() -> Result<()> {
        let line = Line::new_plus("Hyper_L", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::Modifier(
            String::from("Hyper_L")
        )]]);
        Ok(())
    }

    #[test]
    fn token_range_char() -> Result<()> {
        let line = Line::new_plus("a-c", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![
            Token::Char('a'),
            Token::Dash,
            Token::Char('c'),
        ]]);
        Ok(())
    }

    #[test]
    fn token_range_char_space() -> Result<()> {
        let line = Line::new_plus("a - c", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![
            Token::Char('a'),
            Token::Dash,
            Token::Char('c'),
        ]]);
        Ok(())
    }

    // NOTE: This should never be allowed in the configuration
    #[test]
    fn token_range_str() -> Result<()> {
        let line = Line::new_plus("abc-cba", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![
            Token::Text(String::from("abc")),
            Token::Dash,
            Token::Text(String::from("cba")),
        ]]);
        Ok(())
    }

    // NOTE: This should never be allowed in the configuration
    #[test]
    fn token_range_str_space() -> Result<()> {
        let line = Line::new_plus("abc - cba", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![
            Token::Text(String::from("abc")),
            Token::Dash,
            Token::Text(String::from("cba")),
        ]]);
        Ok(())
    }

    #[test]
    fn token_plus() -> Result<()> {
        let line = Line::new_plus("super+t", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Char('t')],
        ]);
        Ok(())
    }

    #[test]
    fn token_plus_space() -> Result<()> {
        let line = Line::new_plus("super + t", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Char('t')],
        ]);
        Ok(())
    }

    #[test]
    fn token_option() -> Result<()> {
        let line = Line::new_plus("{a,b}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::OptionGroup(vec![
            'a', 'b'
        ])]]);
        Ok(())
    }

    #[test]
    fn token_option_space() -> Result<()> {
        let line = Line::new_plus("{a, b}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::OptionGroup(vec![
            'a', 'b'
        ])]]);
        Ok(())
    }

    #[test]
    fn token_option_three() -> Result<()> {
        let line = Line::new_plus("{a,b,z}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::OptionGroup(vec![
            'a', 'b', 'z'
        ])]]);
        Ok(())
    }

    #[test]
    fn token_option_three_space() -> Result<()> {
        let line = Line::new_plus("{a, b, z}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::OptionGroup(vec![
            'a', 'b', 'z'
        ])]]);
        Ok(())
    }

    #[test]
    fn token_option_range() -> Result<()> {
        let line = Line::new_plus("{a-b}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::RangeGroup(vec![
            'a', 'b'
        ])]]);
        Ok(())
    }

    #[test]
    fn token_option_range_space() -> Result<()> {
        let line = Line::new_plus("{a - b}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::RangeGroup(vec![
            'a', 'b'
        ])]]);
        Ok(())
    }

    #[test]
    fn token_option_range_three() -> Result<()> {
        let line = Line::new_plus("{a-c}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::RangeGroup(vec![
            'a', 'b', 'c'
        ])]]);
        Ok(())
    }

    #[test]
    fn token_option_range_three_space() -> Result<()> {
        let line = Line::new_plus("{a - c}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::RangeGroup(vec![
            'a', 'b', 'c'
        ])]]);
        Ok(())
    }

    #[test]
    fn token_option_range_less_than_char() -> Result<()> {
        let line = Line::new_plus("{c-a}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::Invalid]]);
        Ok(())
    }

    #[test]
    fn token_option_range_less_than_digit() -> Result<()> {
        let line = Line::new_plus("{3-1}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::Invalid]]);
        Ok(())
    }

    #[test]
    fn token_option_range_less_than_char_space() -> Result<()> {
        let line = Line::new_plus("{c - a}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::Invalid]]);
        Ok(())
    }

    #[test]
    fn token_option_range_less_than_digit_space() -> Result<()> {
        let line = Line::new_plus("{3 - 1}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::Invalid]]);
        Ok(())
    }

    #[test]
    fn token_option_range_less_double_char() -> Result<()> {
        let line = Line::new_plus("{a-dd}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        println!("=== TOKENS == {:#?}", tokenized);
        assert_eq!(tokenized.tokenized, vec![vec![Token::Invalid]]);
        Ok(())
    }

    #[test]
    fn token_mouse() -> Result<()> {
        let line = Line::new_plus("mouse3", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::Mouse(3)]]);
        Ok(())
    }

    #[test]
    fn token_release_mouse() -> Result<()> {
        let line = Line::new_plus("~mouse3", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![
            Token::Release,
            Token::Mouse(3)
        ],]);
        Ok(())
    }

    #[test]
    fn token_keysym_code_hex() -> Result<()> {
        let line = Line::new_plus("[0xffed]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::Modifier(
            String::from("Hyper_L")
        ),],]);
        Ok(())
    }

    #[test]
    fn token_keysym_code() -> Result<()> {
        let line = Line::new_plus("[65517]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![Token::Modifier(
            String::from("Hyper_L")
        ),],]);
        Ok(())
    }

    #[test]
    fn token_modifier_plus_keysym_code_hex() -> Result<()> {
        let line = Line::new_plus("super+[0x61]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_modifier_plus_keysym_code_hex_space() -> Result<()> {
        let line = Line::new_plus("super + [0x61]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_release_char() -> Result<()> {
        let line = Line::new_plus("~a", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![
            Token::Release,
            Token::Char('a')
        ],]);
        Ok(())
    }

    #[test]
    fn token_release_modifier() -> Result<()> {
        let line = Line::new_plus("super+~a", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Release, Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_release_modifier_space() -> Result<()> {
        let line = Line::new_plus("super + ~a", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Release, Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_release_keysym_code() -> Result<()> {
        let line = Line::new_plus("~[65517]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![vec![
            Token::Release,
            Token::Modifier(String::from("Hyper_L")),
        ],]);
        Ok(())
    }

    #[test]
    fn token_release_modifier_plus_keysym_code() -> Result<()> {
        let line = Line::new_plus("super+~[97]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Release, Token::Char('a'),],
        ]);
        Ok(())
    }

    #[test]
    fn token_release_modifier_plus_keysym_code_space() -> Result<()> {
        let line = Line::new_plus("super + ~[97]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Release, Token::Char('a'),],
        ]);
        Ok(())
    }

    #[test]
    fn token_release_modifier_plus_keysym_code_hex() -> Result<()> {
        let line = Line::new_plus("super+~[0x61]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Release, Token::Char('a'),],
        ]);
        Ok(())
    }

    #[test]
    fn token_release_modifier_plus_keysym_code_hex_space() -> Result<()> {
        let line = Line::new_plus("super + ~[0x61]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Release, Token::Char('a'),],
        ]);
        Ok(())
    }

    #[test]
    fn token_two_modifiers() -> Result<()> {
        let line = Line::new_plus("super+ctrl", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Modifier(String::from("ctrl"))],
        ]);
        Ok(())
    }

    #[test]
    fn token_two_modifiers_space() -> Result<()> {
        let line = Line::new_plus("super + ctrl", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Modifier(String::from("ctrl"))],
        ]);
        Ok(())
    }

    #[test]
    fn token_two_modifiers_plus_char() -> Result<()> {
        let line = Line::new_plus("super+ctrl+a", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Modifier(String::from("ctrl")),],
            vec![Token::Plus],
            vec![Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_two_modifiers_plus_char_space() -> Result<()> {
        let line = Line::new_plus("super + ctrl + a", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Modifier(String::from("ctrl")),],
            vec![Token::Plus],
            vec![Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_two_modifiers_plus_keysym_code_hex() -> Result<()> {
        let line = Line::new_plus("super+ctrl+[0x61]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Modifier(String::from("ctrl")),],
            vec![Token::Plus],
            vec![Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_two_modifiers_plus_keysym_code_hex_space() -> Result<()> {
        let line = Line::new_plus("super + ctrl + [0x61]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Modifier(String::from("ctrl")),],
            vec![Token::Plus],
            vec![Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_two_modifiers_plus_keysym_code() -> Result<()> {
        let line = Line::new_plus("super+ctrl+[97]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Modifier(String::from("ctrl")),],
            vec![Token::Plus],
            vec![Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_two_modifiers_plus_keysym_code_space() -> Result<()> {
        let line = Line::new_plus("super + ctrl + [97]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Modifier(String::from("ctrl")),],
            vec![Token::Plus],
            vec![Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_two_modifiers_one_modifier_keysym_code_hex() -> Result<()> {
        let line = Line::new_plus("super+[0xffe3]+a", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Modifier(String::from("Control_L")),],
            vec![Token::Plus],
            vec![Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_two_modifiers_one_modifier_keysym_code_hex_space() -> Result<()> {
        let line = Line::new_plus("super + [0xffe3] + a", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Modifier(String::from("Control_L")),],
            vec![Token::Plus],
            vec![Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_two_modifiers_modifier_keysym_code() -> Result<()> {
        let line = Line::new_plus("super+[65507]+a", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Modifier(String::from("Control_L")),],
            vec![Token::Plus],
            vec![Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_two_modifiers_modifier_keysym_code_hex() -> Result<()> {
        let line = Line::new_plus("super + [65507] + a", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Modifier(String::from("Control_L")),],
            vec![Token::Plus],
            vec![Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_two_modifiers_two_keysym_code_hex() -> Result<()> {
        let line = Line::new_plus("super+[0xffe3]+[0x61]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Modifier(String::from("Control_L")),],
            vec![Token::Plus],
            vec![Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_two_modifiers_two_keysym_code_hex_space() -> Result<()> {
        let line = Line::new_plus("super + [0xffe3] + [0x61]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Modifier(String::from("Control_L")),],
            vec![Token::Plus],
            vec![Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_two_modifiers_two_keysym_code() -> Result<()> {
        let line = Line::new_plus("super+[65507]+[97]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Modifier(String::from("Control_L")),],
            vec![Token::Plus],
            vec![Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_two_modifiers_two_keysym_code_space() -> Result<()> {
        let line = Line::new_plus("super + [65507] + [97]", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Modifier(String::from("Control_L")),],
            vec![Token::Plus],
            vec![Token::Char('a')],
        ]);
        Ok(())
    }

    #[test]
    fn token_release_range() -> Result<()> {
        let line = Line::new_plus("super+~{a-b}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Release, Token::RangeGroup(vec!['a', 'b'])]
        ]);
        Ok(())
    }

    #[test]
    fn token_release_range_space() -> Result<()> {
        let line = Line::new_plus("super + ~{a-b}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Release, Token::RangeGroup(vec!['a', 'b'])]
        ]);
        Ok(())
    }

    #[test]
    fn token_release_option() -> Result<()> {
        let line = Line::new_plus("super+~{a,b}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Release, Token::OptionGroup(vec!['a', 'b'])]
        ]);
        Ok(())
    }

    #[test]
    fn token_release_option_space() -> Result<()> {
        let line = Line::new_plus("super + ~{a,b}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Release, Token::OptionGroup(vec!['a', 'b'])]
        ]);
        Ok(())
    }

    #[test]
    fn token_release_option_three() -> Result<()> {
        let line = Line::new_plus("super+~{a,b,c}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Release, Token::OptionGroup(vec!['a', 'b', 'c'])]
        ]);
        Ok(())
    }

    #[test]
    fn token_release_option_three_space() -> Result<()> {
        let line = Line::new_plus("super + ~{a, b, c}", 1);
        let mut tokenized = line.tokenize();
        tokenized.parse_tokens()?;
        assert_eq!(tokenized.tokenized, vec![
            vec![Token::Modifier(String::from("super"))],
            vec![Token::Plus],
            vec![Token::Release, Token::OptionGroup(vec!['a', 'b', 'c'])]
        ]);
        Ok(())
    }
}
