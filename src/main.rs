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
    single_use_lifetimes,
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

use anyhow::{Context, Result};
use clap::Parser;
use cli::Opts;
use config::Config;
use keys::keyboard::Keyboard;
use xcb_utils::XUtility;
use colored::Colorize;

fn main() -> Result<()> {
    if users::get_effective_uid() == 0 || users::get_current_uid() == 0 {
        lxhkd_fatal!("this program is not meant to be ran as a root user. Try again");
    }

    let config = Config::load_default().context("failed to load default configuration file")?;
    let args = Opts::parse();
    utils::initialize_logging(&args);

    println!("config: {:#?}", config);

    let (conn, screen_num) = XUtility::setup_connection()?;
    let keyboard = Keyboard::new(&conn, screen_num)?;

    if args.keysyms {
        keyboard.list_keysyms()?;
    }

    // println!("char: {:#?}", keyboard.charmap());

    Ok(())
}
