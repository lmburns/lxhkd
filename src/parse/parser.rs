use crate::{
    config::{Action, Config},
    keys::chord::{Chain, Chord},
};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::HashMap,
    ops::Range,
    process::{Command, Stdio},
    str::FromStr,
};

#[rustfmt::skip]
const MODIFIER_STR: &[&str] = &[
    "alt",   "lalt",   "ralt",
    "shift", "lshift", "rshift",
    "super", "lsuper", "rsuper",
    "meta",  "lmeta",  "rmeta",
    "ctrl",  "lctrl",  "rctrl",
    "mod1",  "mod2",   "mod3",
    "mod4",  "mod5",   "lock",
    "fn",    "hyper",  "meh",
];
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    Identifier,
    Command,
    Modifier,
    Key,
    Keysym,
    Keycode,
    Release,
    Mouse,
    MouseRelease,
    Comma,
    Plus,
    Dash,
    OptionStart,
    OptionEnd,
    Text(String),
    Unknown,
}

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

/// A line in the configuration file after being tokenized
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TokenizedLine<'a> {
    /// The split line
    pub(crate) line:      Line<'a>,
    /// The tokenized line
    pub(crate) tokenized: Vec<Vec<Token>>,
}

/// A single line in a configuration file
#[derive(Debug, Clone)]
pub(crate) struct SingleLine {
    /// Set of keys that will be mapped to a shell command or remapped to
    /// another set of keys
    pub(crate) chain:  Chain,
    /// Action to be executed (shell command or a remap)
    pub(crate) action: Action,
}

impl<'a> Line<'a> {
    /// Create a new `Line`
    pub(crate) fn new(line: &'a str, idx: usize) -> Self {
        let split = line.split('+').map(str::trim).collect::<Vec<_>>();
        Self {
            line_num: idx,
            char_len: line.len(),
            n_keys:   split.len(),
            vector:   split,
        }
    }

    /// Create a `TokenizedLine`
    pub(crate) fn tokenize(&self) -> TokenizedLine {
        static MOUSE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"mouse([0-9]+)").unwrap());
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
            let mut chars = tok.chars().peekable();
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
                    '~' => {
                        let rest = tok[start..].to_string();
                        token_text(&mut text, &mut res);
                        if MOUSE_PATTERN.is_match(&rest) {
                            res.push(Token::MouseRelease);
                        } else {
                            res.push(Token::Release);
                        }
                    },
                    ch => {
                        text.push(ch);
                    },
                }
                chars.next();
            }
            token_text(&mut text, &mut res);
            all_res.push(res);
        }

        TokenizedLine {
            line:      self.clone(),
            tokenized: all_res,
        }
    }
}
