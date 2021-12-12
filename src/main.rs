//! Linux Hotkey Daemon
//!     - Bind keys to shell commands
//!     - Map keys to other keys
//!     - Set key repeat rate and repeat interval

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
    ellipsis_inclusive_range_patterns,
    exported_private_dependencies,
    ill_formed_attribute_input,
    keyword_idents,
    improper_ctypes,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_debug_implementations, // can affect compile time/code size
    missing_docs,
    // missing_doc_code_examples,
    no_mangle_generic_items,
    non_shorthand_field_patterns,
    noop_method_call,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    pointer_structural_match,
    private_in_public,
    pub_use_of_private_extern_crate,
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
#![cfg_attr(
    any(test),
    allow(
        clippy::expect_fun_call,
        clippy::expect_used,
        clippy::panic,
        clippy::panic_in_result_fn,
        clippy::unwrap_in_result,
        clippy::unwrap_used,
        clippy::wildcard_enum_match_arm,
    )
)]

// mod app;
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
use colored::Colorize;
use config::Config;
use keys::{daemon::Daemon, keyboard::Keyboard};
use nix::{
    sys::signal::{self, Signal},
    unistd::{Pid, Uid},
};
use std::{env, fs};
use x11rb::{connection::Connection, protocol::Event};
use xcb_utils::XUtility;

#[cfg(feature = "daemonize")]
use daemonize::{Daemonize, User};

fn main() -> Result<()> {
    if Uid::effective().is_root() || Uid::current().is_root() {
        lxhkd_fatal!("this program is not meant to be ran as a root user. Try again");
    }

    let config = Config::load_default().context("failed to load default configuration file")?;
    let args = Opts::parse();

    // #[cfg(not(test))]
    if let Ok(dir) = utils::initialize_logging(&config, &args) {
        if config.global.log_to_file {
            log::info!(
                "log files can be found: {}",
                dir.display().to_string().blue().bold()
            );
        } else {
            log::info!("logging to file is disabled");
        }
    } else {
        log::info!("logging failed to initialize");
    }

    #[cfg(feature = "daemonize")]
    {
        let runtime_dir = dirs::runtime_dir().unwrap_or_else(env::temp_dir);
        let pidpath = &args.pidfile.unwrap_or_else(|| {
            config
                .clone()
                .global
                .pid_file
                .unwrap_or_else(|| runtime_dir.join("lxhkd.pid"))
        });

        if args.daemonize {
            log::info!("pid-path: {}", pidpath.display().to_string().blue().bold());

            Daemonize::new()
                .pid_file(pidpath)
                .user(User::Id(Uid::current().into()))
                .umask(0o600)
                .exit_action(|| log::info!("> Daemon started <"))
                .start()
                .unwrap_or_else(|_| {
                    lxhkd_fatal!(
                        "daemon is already running: {}",
                        fs::read_to_string(pidpath)
                            .unwrap_or_else(|_| String::from("N/A"))
                            .green()
                            .bold()
                    )
                });
        } else if args.kill {
            // TODO: Check for daemon in background if trying to run in foreground
            let pid_contents =
                fs::read_to_string(pidpath).context("failed to read pidfile to string")?;
            let pid = pid_contents.parse::<i32>().unwrap_or_else(|_| {
                lxhkd_fatal!(
                    "unable to kill the daemon. The process has either been terminated manually, \
                     or the pidfile's contents have been modified. Contents ({})",
                    pid_contents
                )
            });
            let colored_pid = pid.to_string().green().bold();

            // Checking whether or not the process is running before trying to kill it
            if let Some(_process) = psutil::process::processes()
                .context("failed to get list of processes")?
                .iter()
                .filter_map(|p| p.as_ref().ok())
                .find(|p| p.pid() as usize == pid as usize)
            {
                if let Err(e) = signal::kill(Pid::from_raw(pid), Signal::SIGINT) {
                    log::error!("failed to terminate process {}: {}", colored_pid, e);
                } else {
                    log::info!("successfully terminated daemon: {}", colored_pid);
                }
            } else {
                log::error!(
                    "the daemon is not currently running or the PID file has been modified: {}",
                    colored_pid
                );
            }

            std::process::exit(1);
        }
    }

    // FIXME: Do I need 2 or 3 connections?
    // Bind/Map connection
    let (conn, screen_num) = XUtility::setup_connection()?;
    // Xcape control connection
    let (ctrl_conn, _) = XUtility::setup_connection()?;
    // Xcape data read connection
    let (data_conn, _) = XUtility::setup_connection()?;

    let keyboard = Keyboard::new(conn, ctrl_conn, data_conn, screen_num, &config)?;

    if args.keysyms {
        keyboard.list_keysyms()?;
        std::process::exit(1);
    }

    let mut daemon = Daemon::new(keyboard, config);
    daemon.process_configuration()?;
    daemon.daemonize()?;

    Ok(())
}
