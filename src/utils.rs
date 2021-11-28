use crate::cli::Opts;
use env_logger::fmt::Color as LogColor;
use log::LevelFilter;
use std::{panic, sync::Once, io::Write};

/// Used to initialize logging
static ONCE: Once = Once::new();

/// Initialize the logger in a pretty fashion
pub(crate) fn initialize_logging(args: &Opts) {
    ONCE.call_once(|| {
        // This provides much better backtraces, in a Python manner. This makes it
        // easier to see exactly where errors have occured and is useful with this crate
        // because a lot of the commands are C-bindings to the X-Server
        better_panic::install();
        panic::set_hook(Box::new(|panic_info| {
            better_panic::Settings::auto().create_panic_handler()(panic_info);
        }));

        env_logger::Builder::new()
            .format_timestamp(None)
            .format(|buf, record| {
                let mut style = buf.style();
                let level_style = match record.level() {
                    log::Level::Warn => style.set_color(LogColor::Yellow),
                    log::Level::Info => style.set_color(LogColor::Green),
                    log::Level::Debug => style.set_color(LogColor::Magenta),
                    log::Level::Trace => style.set_color(LogColor::Cyan),
                    log::Level::Error => style.set_color(LogColor::Red),
                };

                let mut style = buf.style();
                let target_style = style.set_color(LogColor::Ansi256(14));

                writeln!(
                    buf,
                    " {}: {} {}",
                    level_style.value(record.level()),
                    target_style.value(record.target()),
                    record.args()
                )
            })
            .filter(None, match &args.verbose {
                1 => LevelFilter::Warn,
                2 => LevelFilter::Info,
                3 => LevelFilter::Debug,
                4 => LevelFilter::Trace,
                _ => LevelFilter::Off,
            })
            .init();
    });
}
