use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

const CONFIG_FILE: &str = "lxhkd.yml";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GlobalSettings {
    /// The shell to use for running commands
    pub(crate) shell:           String,
    /// The timeout between keypresses
    pub(crate) timeout:         usize,
    /// TODO: The rate at which keys are repeated
    #[serde(alias = "key-repeat-rate")]
    pub(crate) key_repeat_rate: usize,
}

/// Configuration file to parse
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    /// Global settings
    #[serde(flatten)]
    pub(crate) global:   GlobalSettings,
    /// The mappings of keys to actions/other keys
    /// TODO: Change bindings to types
    pub(crate) bindings: HashMap<String, String>,
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
        Ok(serde_yaml::from_slice(&file).context("failed to deserialize config file")?)
    }

    /// Load the default configuration file
    pub(crate) fn load_default() -> Result<Self> {
        Self::create_default(get_config_path()?)
    }
}

/// Get the default location of the configuration file
pub(crate) fn get_config_path() -> Result<PathBuf> {
    dirs::config_dir()
        .map(|p| p.join("lxhkd"))
        .context("unable to join config path")
}
