#![no_std]

pub extern crate embedded_hal as hal;

pub use paste;

#[cfg(feature = "same54p20a")]
pub use atsame54p20a as target_device;

#[cfg(feature = "use_rtt")]
pub use jlink_rtt;

#[cfg(feature = "use_rtt")]
#[macro_export]
macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut out = $crate::jlink_rtt::NonBlockingOutput::new();
            writeln!(out, $($arg)*).ok();
        }
    };
}

#[cfg(all(not(feature = "use_rtt"), not(feature = "use_uart_debug")))]
#[macro_export]
macro_rules! dbgprint {
    ($($arg:tt)*) => {{}};
}

#[macro_use]
pub mod common;
pub use self::common::*;

#[cfg(feature = "same54")]
pub mod same54;
#[cfg(feature = "same54")]
pub use self::same54::*;

