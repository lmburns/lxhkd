//! The command line arguments

use clap::{crate_version, AppSettings, ArgSettings, Parser, Subcommand, ValueHint};
use std::{env, fs, path::PathBuf};

#[derive(Parser, Default, Clone, Debug, PartialEq)]
#[clap(
    version = crate_version!(),
    max_term_width = 100,
    color = clap::ColorChoice::Auto,
    global_setting = AppSettings::DisableHelpSubcommand,
    global_setting = AppSettings::DeriveDisplayOrder,
    global_setting = AppSettings::HidePossibleValuesInHelp,
    global_setting = AppSettings::InferSubcommands,
)]
pub(crate) struct Opts {
    /// Display debugging messages on 4 levels
    #[clap(long, short, global = true, parse(from_occurrences))]
    pub(crate) verbose: u8,

    /// Location of configuration file
    #[clap(
        long,
        short,
        takes_value = true,
        number_of_values = 1,
        value_name = "file",
        value_hint = ValueHint::FilePath,
        validator = |t| {
            fs::metadata(
                PathBuf::from(t).components()
                    .collect::<Vec<_>>()
                    .len()
                    .gt(&1_usize)
                    .then(|| PathBuf::from(t))
                    .unwrap_or_else(|| env::current_dir().unwrap_or(PathBuf::from(".")).join(PathBuf::from(t))),
            )
            .map_err(|_| "must be a valid path")
            .map(|_| ())
            .map_err(|e| e.to_string())
        }
    )]
    pub(crate) config: Option<PathBuf>,

    /// List the available Keysyms
    #[clap(long = "list-keysyms", short = 'L', takes_value = false)]
    pub(crate) keysyms: bool,

    // /// Reload configuration file
    // #[clap(long = "reload", short = 'r', takes_value = false)]
    // pub(crate) reload: bool,
    /// Kill the daemon if it is running
    #[cfg(feature = "daemonize")]
    #[clap(name = "kill", long = "kill", short = 'k', takes_value = false)]
    pub(crate) kill: bool,

    /// Daemonize the process by sending it to the background
    #[cfg(feature = "daemonize")]
    #[clap(
        name = "daemonize",
        long = "daemonize",
        short = 'd',
        takes_value = false,
        conflicts_with = "kill"
    )]
    pub(crate) daemonize: bool,

    /// Create a temporary file to test keybindings
    #[clap(long = "temporary", short = 't', takes_value = false)]
    pub(crate) temporary: bool,

    /// Specify a PID file
    #[clap(
        long = "pidfile",
        short = 'p',
        takes_value = true,
        number_of_values = 1,
        value_name = "file",
        value_hint = ValueHint::FilePath,
    )]
    pub(crate) pidfile: Option<PathBuf>,
}
