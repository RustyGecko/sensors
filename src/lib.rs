#![no_std]
#![crate_type="lib"]
#![crate_name="sensors"]
#![feature(no_std)]
#![feature(core)]

#[macro_use]
extern crate core;
extern crate emlib;

pub mod si7013;
