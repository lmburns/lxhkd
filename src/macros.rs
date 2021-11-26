/// Expand to an error message
#[macro_export]
macro_rules! lxhkd_error {
    ($($err:tt)*) => ({
        eprintln!("{}: {}", "[lxhkd error]".red().bold(), format!($($err)*));
    })
}

/// Expand to a fatal message
#[macro_export]
macro_rules! lxhkd_fatal {
    ($($err:tt)*) => ({
        eprintln!("{}: {}", "[lxhkd fatal]".yellow().bold(), format!($($err)*));
        std::process::exit(1);
    })
}
