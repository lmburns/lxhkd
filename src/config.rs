use std::collections::HashMap;

/// Configuration file to parse
pub(crate) struct Config {
    pub(crate) bindings: HashMap<String, String>,
}
