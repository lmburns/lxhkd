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
    #[clap(long, short, global = true, parse(from_occurrences))]
    /// Display debugging messages on 4 levels
    pub(crate) verbose: u8,
    /// Location of configuration file
    #[clap(
        long, short,
        number_of_values = 1,
        value_name = "config",
        value_hint = ValueHint::FilePath,
        validator = |t| fs::metadata(t)
                            .map_err(|_| "must be a valid path")
                            .map(|_| ())
                            .map_err(|e| e.to_string()),
    )]
    pub(crate) dir:     Option<PathBuf>,
}
