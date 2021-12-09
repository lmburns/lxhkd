//! Various utilities specifically dealing with X

use crate::lxhkd_fatal;
use anyhow::Result;
use colored::Colorize;
use thiserror::Error;

use x11rb::{
    connection::{Connection, RequestConnection},
    errors::ConnectError,
    protocol::{record, xproto},
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

    /// Generate the [`record`] configuration
    /// ([`Range`](x11rb::protocol::record::Range))
    pub(crate) fn gen_record_range() -> record::Range {
        let empty = record::Range8 { first: 0, last: 0 };
        let empty_ext =
            record::ExtRange { major: empty, minor: record::Range16 { first: 0, last: 0 } };

        record::Range {
            core_requests:    empty,
            core_replies:     empty,
            ext_requests:     empty_ext,
            ext_replies:      empty_ext,
            delivered_events: empty,
            device_events:    record::Range8 {
                // Want notification of core X11 events from key press (2) to motion notify (6)
                // KeyPress = 2, KeyRelease = 3, ButtonPress = 4, ButtonRelease = 5
                first: xproto::KEY_PRESS_EVENT,
                last:  xproto::MOTION_NOTIFY_EVENT,
            },
            errors:           empty, // core and ext errors
            client_started:   false, // connection setup reply from server
            client_died:      false, // notification of client disconnect
        }
    }
}
