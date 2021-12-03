//! Various helper-utilities

use crate::cli::Opts;
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
    sync::Once,
};

// /// Used to initialize logging
// static ONCE: Once = Once::new();

/// Initializes logging for this crate
pub(crate) fn initialize_logging(log_dir: Option<PathBuf>, args: &Opts) -> Result<PathBuf> {
    /// Customize the format of the log (colored)
    fn colored_format(
        w: &mut dyn Write,
        now: &mut DeferredNow,
        record: &Record,
    ) -> Result<(), io::Error> {
        let level = record.level();
        write!(
            w,
            "[{:>}] {:<5} [{}:{}]: {}",
            style(level, now.now().format("%Y-%m-%d %H:%M:%S")),
            style(level, level),
            style(Level::Trace, record.file().unwrap_or("<unnamed>")),
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
        // crate when writing to a file. I'm not sure that this gets ran when writing to
        // a file
        write!(
            w,
            "[{:>}] {:>} [{}:{}]: {}",
            now.now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.file().unwrap_or("<unnamed>"),
            record.line().unwrap_or(0),
            String::from_utf8(strip_ansi_escapes::strip(
                &record.args().to_string().as_bytes()
            )?)
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

    let log_dir = if let Some(dir) = log_dir {
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
    let logger = Logger::try_with_str(env::var("LXHKD_LOG").unwrap_or_else(
        |_| match args.verbose {
            1 => String::from("debug"),
            2 => String::from("trace"),
            _ => String::from("info"),
        },
    ))?
    .log_to_file(
        FileSpec::default()
            .basename(crate_name!())
            .directory(&log_dir),
    )
    .write_mode(WriteMode::BufferAndFlush)
    .adaptive_format_for_stderr(AdaptiveFormat::Custom(uncolored_format, colored_format))
    .format_for_files(uncolored_format)
    .duplicate_to_stderr(Duplicate::All)
    .rotate(
        Criterion::AgeOrSize(Age::Day, 50_000_000),
        Naming::Numbers,
        Cleanup::KeepLogFiles(4),
    )
    .set_palette(String::from("9;11;14;5;13"))
    .start();

    Ok(log_dir)
}
