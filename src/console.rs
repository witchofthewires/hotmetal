// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2023 Andre Richter <andre.o.richter@gmail.com>

//! System console.

use crate::bsp;

//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------

/// Console interfaces.
pub mod interface {
    use core::fmt;

    pub trait Write {
        // write a rust format string
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
    }

    pub trait Statistics {
        fn chars_written(&self) -> usize {
            0
        }
    }

    // trait alias for a full-fledged console
    pub trait All: Write + Statistics {}
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// Return a reference to the console.
///
/// This is the global console used by all printing macros.
pub fn console() -> &'static dyn interface::All {
    bsp::console::console()
}