//! The command line arguments

use crate::utils::wants_color;
use clap::{crate_description, crate_version, AppSettings, ArgSettings, Parser, Subcommand, ValueHint};
use once_cell::sync::Lazy;
use std::{env, fs, path::PathBuf};

#[derive(Parser, Default, Clone, Debug, PartialEq)]
#[clap(
    version = crate_version!(),
    author = <String as AsRef<str>>::as_ref(&APP_AUTHORS),
    about = <String as AsRef<str>>::as_ref(&APP_ABOUT),
    after_help =  <String as AsRef<str>>::as_ref(&AFTER_HELP),
    override_usage =  <String as AsRef<str>>::as_ref(&OVERRIDE_HELP),
    max_term_width = 100,
    color = clap::ColorChoice::Auto,
    global_setting = AppSettings::DeriveDisplayOrder,
    disable_help_subcommand = true,
    hide_possible_values = true,
    infer_subcommands = true,
)]
pub(crate) struct Opts {
    /// Display debugging messages on various levels
    #[clap(
        long,
        short,
        global = true,
        parse(from_occurrences),
        long_help = "
        Set the verbosity level of the program. There are 2 extra levels after the default (INFO). If \
                     `-v` is used, DEBUG messages are displayed, and if `-vv` is used TRACE messages \
                     are displayed. The verbosity can also be set with the `LXHKD_LOG` environment \
                     variable"
    )]
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
        },
        long_help = "\
        Specify the location of the confiugration file. The default location is \
                `$XDG_CONFIG_HOME/lxhkd/lxhkd.yml`"
    )]
    pub(crate) config: Option<PathBuf>,

    /// List the available Keysyms. Similar output to `xmodmap`
    #[clap(
        name = "keysyms",
        long = "list-keysyms",
        short = 'L',
        takes_value = false,
        conflicts_with = "kill",
        long_help = "\
        List the available keysyms that one can use for mapping. If this option is used, it is the only \
                     option that should be used and will exit the program after display the keysyms
        "
    )]
    pub(crate) keysyms: bool,

    /// Kill the daemon if it is running
    #[cfg(feature = "daemonize")]
    #[clap(
        name = "kill",
        long = "kill",
        short = 'k',
        takes_value = false,
        long_help = "\
        Kill the daemonized process if it is currently running. If a PID file was specified on the \
                     command line when the process was first daemonized, that same PID file must be \
                     specified again for the process to be killed. Otherwise, the PID file will be \
                     written to and read from a default location"
    )]
    pub(crate) kill: bool,

    /// Daemonize the process by sending it to the background
    #[cfg(feature = "daemonize")]
    #[clap(
        name = "daemonize",
        long = "daemonize",
        short = 'd',
        takes_value = false,
        conflicts_with_all = &["kill", "keysyms", "temporary"],
        long_help = "Send the process to the background and write the PID of the process to a \
                      PID file"
    )]
    pub(crate) daemonize: bool,

    /// Create a temporary file to test keybindings
    #[clap(name = "temporary", long = "temporary", short = 't', takes_value = false)]
    pub(crate) temporary: bool,

    /// Specify a PID file. A default PID file should be used most of the time
    #[cfg(feature = "daemonize")]
    #[clap(
        long = "pidfile",
        short = 'p',
        takes_value = true,
        number_of_values = 1,
        value_name = "file",
        value_hint = ValueHint::FilePath,
        long_help = "\
        Allows specifying the location of the PID file. This option is usually discouraged if \
                one wishes to have the ability to use the `--kill` flag later. The `--kill` flag \
                still can be used as long as the same PID file that was used to start the process is \
                passed to lxhkd again"
    )]
    pub(crate) pidfile: Option<PathBuf>,

    /// When to colorize output
    #[clap(
        name = "color",
        long = "color",
        short = 'C',
        value_name = "when",
        possible_values = &["never", "auto", "always"],
        long_help = "\
        When to colorize output (usually meant for piping). Valid values are: always, \
                auto, never. The always selection only applies to the path as of now."
    )]
    pub(crate) color_when: Option<String>,
}

// /// Reload configuration file
// #[clap(long = "reload", short = 'r', takes_value = false)]
// pub(crate) reload: bool,

// =============== Prettify Help ==================

const YELLOW: &str = "\x1b[0;33m";
const GREEN: &str = "\x1b[0;32m";
const BRCYAN: &str = "\x1b[38;5;14m";
const BRGREEN: &str = "\x1b[38;5;10m";
const BRRED: &str = "\x1b[38;5;9m";
const BRED: &str = "\x1b[01;38;5;1m";
const RES: &str = "\x1b[0m";

/// Colored options used in the output of `--help`
pub(crate) static APP_ABOUT: Lazy<String> = Lazy::new(|| {
    wants_color()
        .then(|| format!("{}DESCRIPTION: {}{}{}", YELLOW, GREEN, crate_description!(), RES))
        .unwrap_or_else(|| crate_description!().to_string())
});

/// Colorized message to override the generated help message
pub(crate) static OVERRIDE_HELP: Lazy<String> = Lazy::new(|| {
    wants_color()
        .then(|| {
            format!(
                "{}lxhkd{} [{}FLAGS{}/{}OPTIONS{}]",
                BRED, RES, GREEN, RES, GREEN, RES
            )
        })
        .unwrap_or_else(|| String::from("lxhkd [FLAGS/OPTIONS]"))
});

/// Colorized message displayed after the help message
pub(crate) static AFTER_HELP: Lazy<String> = Lazy::new(|| {
    wants_color()
        .then(|| {
            format!(
                "See {}lxhkd{} {}--help{} for longer explanations of some options.",
                BRED, RES, GREEN, RES
            )
        })
        .unwrap_or_else(|| String::from("See lxhkd --help for longer explanations of some options."))
});

/// Colorized message about the app's authors
pub(crate) static APP_AUTHORS: Lazy<String> = Lazy::new(|| {
    wants_color()
        .then(|| {
            format!(
                "{}Lucas Burns{}   <{}lmb@lmburns.com{}>",
                BRRED, RES, BRGREEN, RES,
            )
        })
        .unwrap_or_else(|| String::from("Lucas Burns   <lmb@lmburns.com>"))
});
