// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2023 Andre Richter <andre.o.richter@gmail.com>

//! System console.

mod null_console;
use crate::synchronization::{self, NullLock};

//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------

/// Console interfaces.
pub mod interface {
    use core::fmt;

    pub trait Write {
        // write single character
        fn write_char(&self, c: char);

        // write a rust format string
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;

        // block until last buffered character is physically put on TX wire
        fn flush(&self);
    }

    pub trait Read {
        // read a single character
        fn read_char(&self) -> char {
            ' '
        }

        // clear RX buffers, if any
        fn clear_rx(&self);
    }

    pub trait Statistics {
        fn chars_written(&self) -> usize {
            0
        }

        fn chars_read(&self) -> usize {
            0
        }
    }

    // trait alias for a full-fledged console
    pub trait All: Write + Read + Statistics {}
}

//--------------------------------------------------------------------------------------------------
// Global instances
//--------------------------------------------------------------------------------------------------

static CUR_CONSOLE: NullLock<&'static (dyn interface::All + Sync)> =
    NullLock::new(&null_console::NULL_CONSOLE);

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
use synchronization::interface::Mutex;

/// Register a new console.
pub fn register_console(new_console: &'static (dyn interface::All + Sync)) {
    CUR_CONSOLE.lock(|con| *con = new_console);
}

/// Return a reference to the currently registered console.
///
/// This is the global console used by all printing macros.
pub fn console() -> &'static dyn interface::All {
    CUR_CONSOLE.lock(|con| *con)
}