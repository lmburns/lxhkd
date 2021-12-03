#![allow(unused)]
#![deny(
    clippy::all,
    clippy::correctness,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    bad_style,
    const_err,
    // dead_code,
    keyword_idents,
    improper_ctypes,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    no_mangle_generic_items,
    non_shorthand_field_patterns,
    noop_method_call,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    pointer_structural_match,
    private_in_public,
    semicolon_in_expressions_from_macros,
    // single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unaligned_references,
    unconditional_recursion,
    unreachable_pub,
    unsafe_code,
    // unused,
    unused_allocation,
    unused_comparisons,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_parens,
    unused_qualifications,
    // variant_size_differences,
    while_true
)]
#![allow(
    clippy::similar_names,
    clippy::struct_excessive_bools,
    clippy::shadow_reuse,
    clippy::too_many_lines,
    clippy::doc_markdown,
    clippy::single_match_else,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::upper_case_acronyms,
    clippy::enum_variant_names
)]

mod cli;
mod config;
mod keys;
mod macros;
mod parse;
mod types;
mod utils;
mod xcb_utils;

use crate::keys::keys::CharacterMap;
use anyhow::{Context, Result};
use clap::Parser;
use cli::Opts;
use colored::Colorize;
use config::Config;
use keys::{chord::Chord2, keyboard::Keyboard};
use parse::parser::Line;
use xcb_utils::XUtility;

use x11_keysymdef as ksdef;

fn main() -> Result<()> {
    if users::get_effective_uid() == 0 || users::get_current_uid() == 0 {
        lxhkd_fatal!("this program is not meant to be ran as a root user. Try again");
    }

    let config = Config::load_default().context("failed to load default configuration file")?;
    let args = Opts::parse();

    if let Ok(dir) = utils::initialize_logging(config.global.log_dir, &args) {
        log::info!("log files can be found: {}", dir.display());
    } else {
        log::info!("logging failed to initialize");
    }

    let (conn, screen_num) = XUtility::setup_connection()?;
    let keyboard = Keyboard::new(&conn, screen_num)?;

    if args.keysyms {
        keyboard.list_keysyms()?;
    }

    if let Some(bindings) = config.bindings {
        let lines = bindings.keys();
        for (mut idx, l) in lines.enumerate() {
            idx += 1;

            let line = Line::new_plus(l, idx);
            let mut tokenized = line.tokenize();
            tokenized.further_tokenize()?;

            let chord = tokenized.convert_to_chord(&keyboard.charmap)?;
            println!("CHORD: {:#?}", chord);
            // let charmaps = Chord2::from_flatoke(&keyboard.charmap, flat);

            // log::debug!("{}: {}", "Line".red().bold(), l);
            // log::debug!("{}: {:#?}", "Tokenized".blue().bold(), charmaps);
        }
    }

    // let a = ksdef::lookup_by_name("ISO_Level3_Shift");
    // println!("hyper: lookup {:#?}", a);

    Ok(())
}
