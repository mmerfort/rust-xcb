#![crate_name = "xcb"]
#![crate_type = "lib"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature(globs)]

extern crate libc;

pub use xcb::*;
mod xcb;
