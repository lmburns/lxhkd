//! Configuration options

use crate::keys::chord::{Chain, Chord};
use anyhow::{Context, Result};
use colored::Colorize;
use format_serde_error::SerdeError;
use indexmap::IndexMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::HashMap,
    env,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    str::FromStr,
};

const CONFIG_FILE: &str = "lxhkd.yml";
const LOG_TO_FILE_DEFAULT: bool = true;

/// Default shell to run commands within
pub(crate) static SHELL: Lazy<String> =
    Lazy::new(|| env::var("SHELL").unwrap_or_else(|_| String::from("/bin/bash")));

/// I guess that functions are required to set a `serde` default value
fn log_to_file_default() -> bool {
    LOG_TO_FILE_DEFAULT
}

// TODO: Test configuration and make sure no crash if empty
// TODO: Allow for specifying of config file

// https://unix.stackexchange.com/questions/320373/
// how-to-remap-keyboard-keys-based-on-how-long-you-hold-the-key/320474

// =============== GlobalSettings =================

/// Global configuration settings
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub(crate) struct GlobalSettings {
    /// The shell to use for running commands
    pub(crate) shell: Option<String>,

    // TODO: Implement this
    /// The timeout between keypresses
    pub(crate) timeout: Option<u32>,

    /// The delay in which keys begin to repeat
    #[serde(alias = "autorepeat-delay")]
    pub(crate) autorepeat_delay: Option<u16>,

    /// The speed in which keys repeat after the delay
    #[serde(alias = "autorepeat-interval")]
    pub(crate) autorepeat_interval: Option<u16>,

    /// The file to write the PID to
    #[serde(alias = "pid-file")]
    pub(crate) pid_file: Option<PathBuf>,

    /// Whether logs should be written to a file
    #[serde(alias = "log-to-file")]
    #[serde(default = "log_to_file_default")]
    pub(crate) log_to_file: bool,

    /// The directory to write the log to
    #[serde(alias = "log-dir")]
    pub(crate) log_dir: Option<PathBuf>,
}

// =================== Config =====================

/// Configuration file to parse.
///
/// `IndexMap` is used to guarantee that if duplicate bindings are created by
/// accident, the first one will be the one that is used
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub(crate) struct Config {
    /// Global settings
    #[serde(flatten)]
    pub(crate) global:   GlobalSettings,
    /// The mappings of keys to shell commands
    pub(crate) bindings: Option<IndexMap<String, String>>,
    /// The mappings of keys to other keybindings
    pub(crate) remaps:   Option<IndexMap<String, String>>,

    /// Mappings of modifiers to one key when pressed & another when held down
    pub(crate) xcape: Option<IndexMap<String, String>>,
}

impl Config {
    /// Create the default configuration file
    pub(crate) fn create_default<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            fs::create_dir_all(path).context("unable to create configuration directory")?;
        }

        let path = path.join(CONFIG_FILE);

        if !path.is_file() {
            let initialization = include_str!("../example/lxhkd.yml");

            let mut config_file: fs::File = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&path)
                .with_context(|| format!("could not create lxhkd config: '{}'", path.display()))?;

            config_file
                .write_all(initialization.as_bytes())
                .with_context(|| format!("could not create lxhkd config: '{}'", path.display()))?;
            config_file.flush()?;
        }

        Self::load(path)
    }

    /// Load the configuration file from a given path
    pub(crate) fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        // let file = fs::read(&path).context("failed to read config file")?;
        // serde_yaml::from_slice(&file).context("failed to deserialize config file")

        let file = fs::read_to_string(&path).context("failed to read config file")?;
        Ok(serde_yaml::from_str(&file).map_err(|e| SerdeError::new(file, e))?)
    }

    /// Load the default configuration file
    pub(crate) fn load_default() -> Result<Self> {
        let path = get_config_path()?;
        log::debug!("loading default config: {}", path.display());
        Self::create_default(path)
    }
}

// =================== Action =====================

/// The action that a mapping will do
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Action {
    /// A shell command    (i.e, opening a terminal)
    Shell(String),

    /// Another keymapping (i.e., Caps_Lock => Escape)
    Remap(Chord),

    /// `xscape` type bindings, where a key is different if pressed vs held
    /// For example:
    ///     - `Caps_Lock` => `Escape` when tapped
    ///     - `Caps_Lock` => `Hyper_L` when held
    Xcape(String),
}

impl Action {
    /// Spawn a shell from the given keybind mapping
    pub(crate) fn spawn_shell(cmd: &str, shell: &str) {
        // If the user has something like 'zsh -euy' as their command
        let shell_cmd = shell
            .split_whitespace()
            .into_iter()
            .map(str::trim)
            .collect::<Vec<_>>();

        let mut builder = Command::new(shell_cmd[0]);
        if shell_cmd.len() > 1 {
            builder.arg(shell_cmd[1]);
        }

        builder.arg("-c");
        builder.arg(cmd);

        log::debug!("running command: {}", cmd.green().bold());
        match builder.stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
            Ok(mut child) => {
                std::thread::spawn(move || match child.wait() {
                    Ok(status) => {
                        log::debug!("exited with a code {:?}", status.code());
                    },
                    Err(e) => {
                        log::debug!("exited with an error {}", e);
                    },
                });
            },
            Err(e) => {
                log::error!("there was an error spawning {}: {}", cmd.green().bold(), e);
            },
        }
    }

    /// Run the given `Action`
    pub(crate) fn run(&self, shell: &Option<String>) {
        match self {
            Self::Shell(cmd) => {
                println!("=== Running ===");
                Self::spawn_shell(cmd, shell.as_ref().unwrap_or(&SHELL.to_string()));
            },
            Self::Remap(remap) => {
                println!("found remap: {}", remap);
            },
            Self::Xcape(xcape) => {
                println!("found xcape: {}", xcape);
            },
        }
    }
}

// ================ Helper Funcs ==================

/// Get the default location of the configuration file
pub(crate) fn get_config_path() -> Result<PathBuf> {
    dirs::config_dir()
        .map(|p| p.join("lxhkd"))
        .context("unable to join config path")
}
