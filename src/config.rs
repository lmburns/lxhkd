use crate::keys::chord::{Chain, Chord};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::HashMap,
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    str::FromStr,
};

const CONFIG_FILE: &str = "lxhkd.yml";

// TODO: Test configuration and make sure no crash if empty

// TODO: The rate at which keys are repeated
// TODO: Allow for if keys are pressed they're one thing and if they're held
// then they're another
// - https://unix.stackexchange.com/questions/320373/how-to-remap-keyboard-keys-based-on-how-long-you-hold-the-key/320474

/// Global configuration settings
#[derive(Debug, Serialize, Deserialize, Default)]
pub(crate) struct GlobalSettings {
    /// The shell to use for running commands
    pub(crate) shell:               Option<String>,
    /// The timeout between keypresses
    pub(crate) timeout:             Option<usize>,
    /// The delay in which keys begin to repeat
    #[serde(alias = "autorepeat-delay")]
    pub(crate) autorepeat_delay:    Option<usize>,
    /// The speed in which keys repeat after the delay
    #[serde(alias = "autorepeat-interval")]
    pub(crate) autorepeat_interval: Option<usize>,
}

/// Configuration file to parse
#[derive(Debug, Serialize, Deserialize, Default)]
pub(crate) struct Config {
    /// Global settings
    #[serde(flatten)]
    pub(crate) global:   GlobalSettings,
    /// The mappings of keys to shell commands
    pub(crate) bindings: Option<HashMap<String, String>>,
    /// The mappings of keys to other keybindings
    pub(crate) remaps:   Option<HashMap<String, String>>,

    /// Mappings of modifiers to one key when pressed & another when held down
    pub(crate) xcape: Option<HashMap<String, String>>,
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
        let file = fs::read(&path).context("failed to read config file")?;
        serde_yaml::from_slice(&file).context("failed to deserialize config file")
    }

    /// Load the default configuration file
    pub(crate) fn load_default() -> Result<Self> {
        let path = get_config_path()?;
        log::debug!("loading default config: {}", path.display());
        Self::create_default(path)
    }
}

/// The action that a mapping will do
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Action {
    /// A shell command    (i.e, opening a terminal)
    ShellCmd(String),
    /// Another keymapping (i.e., Caps_Lock => Escape)
    Remap(Chord),
}

impl Action {
    /// Spawn a shell from the given keybind mapping
    pub(crate) fn spawn_shell<I, S>(cmd: I, shell: S)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let shell_cmd = shell.as_ref().split_whitespace().collect::<Vec<_>>();
        let mut builder = {
            if shell_cmd.len() > 1 {
                let mut builder = Command::new(shell_cmd[0]);
                builder.arg(shell_cmd[1]);
                builder
            } else {
                let mut builder = Command::new(shell_cmd[0]);
                builder.arg("-c");
                builder
            }
        };

        for arg in cmd {
            let arg = arg.as_ref();
            builder.arg(arg);
        }

        builder
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .ok();
    }

    /// Run the given `Action`, given it is a command or a remap
    pub(crate) fn run(&self, config: &Config) {
        match self {
            Self::ShellCmd(cmd) => {
                let possible_shell =
                    env::var("SHELL").unwrap_or_else(|_| String::from("/bin/bash"));
                Self::spawn_shell(
                    cmd.split_whitespace().map(str::trim).collect::<Vec<_>>(),
                    config.global.shell.as_ref().unwrap_or(&possible_shell),
                );
            },
            Self::Remap(remap) => {
                println!("found remap: {}", remap);
            },
        }
    }
}

/// Get the default location of the configuration file
pub(crate) fn get_config_path() -> Result<PathBuf> {
    dirs::config_dir()
        .map(|p| p.join("lxhkd"))
        .context("unable to join config path")
}
