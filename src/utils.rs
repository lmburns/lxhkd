//! Various helper-utilities

use crate::{cli::Opts, config::Config};
use anyhow::{Context, Result};
use clap::crate_name;
use flexi_logger::{
    opt_format,
    style,
    AdaptiveFormat,
    Age,
    Cleanup,
    Criterion,
    DeferredNow,
    Duplicate,
    FileSpec,
    FlexiLoggerError,
    Level,
    Logger,
    LoggerHandle,
    Naming,
    Record,
    WriteMode,
};
use log::LevelFilter;
use shellexpand::LookupError;
use std::{
    borrow::Cow,
    env,
    io::{self, Write},
    panic,
    path::PathBuf,
    thread,
};
use time::{format_description::FormatItem, macros::format_description};

// How can I use the updated flexi_logger with a static string?
const TIMESTAMP_FMT: &[FormatItem<'static>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

/// Shorter way of testing if the user wants color for the output of `--help`
pub(crate) fn wants_color() -> bool {
    env::var_os("NO_COLOR").is_none()
}

// TODO: Perhaps use a `SyslogWriter`

/// Initializes logging for this crate
pub(crate) fn initialize_logging(config: &Config, args: &Opts) -> Result<PathBuf> {
    /// Customize the format of the log (colored)
    fn colored_format(
        w: &mut dyn Write,
        now: &mut DeferredNow,
        record: &Record,
    ) -> Result<(), io::Error> {
        let level = record.level();
        // style(level, now.now().format("%d %H:%M:%S")),

        write!(
            w,
            "{:<5} [{}:{}]: {}",
            style(level).paint(level.to_string()),
            style(Level::Trace).paint(record.file().unwrap_or("<unnamed>")),
            record.line().unwrap_or(0),
            &record.args() // style(level, &record.args())
        )
    }

    /// Customize the format of the log (uncolored)
    fn uncolored_format(
        w: &mut dyn Write,
        now: &mut DeferredNow,
        record: &Record,
    ) -> Result<(), io::Error> {
        // Strip the ansi sequences that I have put in log messages using the `colored`
        // crate when writing to a file. Also use a date
        write!(
            w,
            "[{:>}] {:<5} [{}:{}]: {}",
            now.format(TIMESTAMP_FMT),
            record.level(),
            record.file().unwrap_or("<unnamed>"),
            record.line().unwrap_or(0),
            String::from_utf8(strip_ansi_escapes::strip(&record.args().to_string().as_bytes())?)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        )
    }

    // This provides much better backtraces, in a Python manner. This makes it
    // easier to see exactly where errors have occured and is useful with this crate
    // because of the communication with the X-Server
    if cfg!(debug_assertions) {
        better_panic::install();
        panic::set_hook(Box::new(|panic_info| {
            better_panic::Settings::auto().create_panic_handler()(panic_info);
        }));
    }

    let log_dir = if let Some(dir) = &config.global.log_dir {
        PathBuf::from(
            shellexpand::full(&dir.display().to_string())
                .unwrap_or_else(|_| {
                    Cow::from(
                        LookupError {
                            var_name: "Unkown Environment Variable".into(),
                            cause:    env::VarError::NotPresent,
                        }
                        .to_string(),
                    )
                })
                .to_string(),
        )
    } else {
        env::temp_dir().join(crate_name!())
    };

    // .create_symlink()
    // .format(colored_format)
    let mut logger =
        Logger::try_with_str(env::var("LXHKD_LOG").unwrap_or_else(|_| match args.verbose {
            1 => String::from("debug"),
            2 => String::from("trace"),
            _ => String::from("info"),
        }))?
        .write_mode(WriteMode::BufferAndFlush)
        .adaptive_format_for_stderr(AdaptiveFormat::Custom(uncolored_format, colored_format))
        .set_palette(String::from("9;11;14;5;13"));

    if config.global.log_to_file {
        logger = logger
            .duplicate_to_stderr(Duplicate::All)
            .rotate(
                Criterion::AgeOrSize(Age::Day, 50_000_000),
                Naming::Numbers,
                Cleanup::KeepLogFiles(2),
            )
            .log_to_file(FileSpec::default().basename(crate_name!()).directory(&log_dir))
            .format_for_files(uncolored_format);
    }

    logger.start();

    Ok(log_dir)
}
