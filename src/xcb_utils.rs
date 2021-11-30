use crate::lxhkd_fatal;
use anyhow::Result;
use colored::Colorize;
use thiserror::Error;

use x11rb::{
    connection::{Connection, RequestConnection},
    errors::ConnectError,
    rust_connection::RustConnection,
    wrapper::ConnectionExt as _,
};

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("failed to connect to the X11 server: {0}")]
    Connection(#[from] ConnectError),
}

pub(crate) struct XUtility;

impl XUtility {
    /// Setup X11 connection
    pub(crate) fn setup_connection() -> Result<(RustConnection, usize), Error> {
        RustConnection::connect(None).map_err(Error::Connection)
    }
}
