//! A structure that contains all parsaed arguments for the command line

use crate::{cli::Opts, config::Config};
use anyhow::{Context, Result};
use std::{env, path::PathBuf};

/// The structure containing parsed arguments
#[derive(Debug, Clone)]
pub(crate) struct App {
    color_when:   String,
    config:       Config,
    list_keysyms: bool,
    temporary:    bool,

    #[cfg(feature = "daemonize")]
    daemonize:    bool,
    #[cfg(feature = "daemonize")]
    pidfile:      PathBuf,
    #[cfg(feature = "daemonize")]
    kill:         bool,
}

impl App {
    /// Run the application
    pub(crate) fn run(opts: Opts, config: Config) -> Result<()> {
        let mut app = Self::new(&opts, config);
        Ok(())
    }

    /// Parse the configuration and command line arguments
    pub(crate) fn new(opts: &Opts, config: Config) -> Self {
        let color_when = match opts.color_when {
            Some(ref s) if s == "always" => "always",
            Some(ref s) if s == "never" => "never",
            _ =>
                if env::var_os("NO_COLOR").is_none() {
                    "auto"
                } else {
                    "never"
                },
        };

        Self {
            color_when:   color_when.to_string(),
            config:       config.clone(),
            list_keysyms: opts.keysyms,
            temporary:    opts.temporary,

            #[cfg(feature = "daemonize")]
            daemonize:    opts.daemonize,
            #[cfg(feature = "daemonize")]
            pidfile:      opts.pidfile.clone().unwrap_or_else(|| {
                config.global.pid_file.unwrap_or_else(|| {
                    dirs::runtime_dir()
                        .unwrap_or_else(env::temp_dir)
                        .join("lxhkd.pid")
                })
            }),
            #[cfg(feature = "daemonize")]
            kill:         opts.kill,
        }
    }

    pub(crate) fn run_commands(&self) -> Result<()> {
        if self.color_when == "never" {
            colored::control::SHOULD_COLORIZE.set_override(false);
        } else if self.color_when == "always" {
            colored::control::SHOULD_COLORIZE.set_override(true);
        }

        Ok(())
    }
}
