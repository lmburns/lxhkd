//! Various utilities specifically dealing with X

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

// ================== XUtility ====================

pub(crate) struct XUtility;

impl XUtility {
    /// Setup X11 connection
    pub(crate) fn setup_connection() -> Result<(RustConnection, usize), Error> {
        RustConnection::connect(None).map_err(Error::Connection)
    }

    // NOTE: Xtrace

    // /// Parse some data and print the resulting object.
    // ///
    // /// The result of parsing is returned, but output is already generated on
    // both success and error. fn print_parse_return<T: TryParse +
    // std::fmt::Debug>(data: &[u8]) -> Result<T, ParseError> {     match T::
    // try_parse(data) {         Err(e) => {
    //             println!("Error while parsing: {:?}", e);
    //             Err(e)
    //         }
    //         Ok((obj, _remaining)) => {
    //             println!("{:?}", obj);
    //             Ok(obj)
    //         }
    //     }
    // }
    //
    // /// Parse some data and print the resulting object.
    // fn print_parse<T: TryParse + std::fmt::Debug>(data: &[u8]) {
    //     let _ = print_parse_return::<T>(data);
    // }
}
