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

// mod app;
mod cli;
mod config;
mod keys;
mod macros;
mod parse;
mod types;
mod utils;
mod xcb_utils;

use crate::{config::Action, keys::keys::CharacterMap};
use anyhow::{Context, Result};
use clap::Parser;
use cli::Opts;
use colored::Colorize;
use config::Config;
use keys::{daemon::Daemon, keyboard::Keyboard};
use nix::{
    sys::signal::{self, Signal},
    unistd::{getpid, getppid, Pid, Uid},
};
use parse::parser::Line;
use std::{env, fs, path::PathBuf};
use x11rb::{connection::Connection, protocol::Event};
use xcb_utils::XUtility;

#[cfg(feature = "daemonize")]
use daemonize::{Daemonize, Stdio, User};

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

        log::info!("pid-path: {}", pidpath.display().to_string().blue().bold());

        if args.daemonize {
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
                    "the daemon is not currently running or the PID file is incorrect: {}",
                    colored_pid
                );
            }

            std::process::exit(1);
        }
    }

    let (conn, screen_num) = XUtility::setup_connection()?;
    let mut keyboard = Keyboard::new(&conn, screen_num, &config)?;

    if args.keysyms {
        keyboard.list_keysyms()?;
        std::process::exit(1);
    }

    let mut daemon = Daemon::new(&keyboard, &config);
    daemon.process_bindings();
    daemon.daemonize()?;

    Ok(())
}
