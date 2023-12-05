#![no_std]
#![allow(dead_code)]

extern crate alloc;
#[macro_use]
extern crate log;

mod defs;
mod rings;
mod driver;

pub use driver::mdio_write;
pub use driver::Starfive2Hal;
pub use driver::Starfive2NetDevice;
