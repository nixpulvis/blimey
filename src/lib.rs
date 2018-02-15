#![feature(proc_macro)]

extern crate blimey_macros;

use std::any::Any;

pub type Contract = &'static Fn(&Any) -> bool;

pub use blimey_macros::{contract, contractual};
