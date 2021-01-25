
#![no_std]

extern crate prusti_contracts;
use prusti_contracts::*;

pub use uuid::Uuid;

#[ensures(result != 0)]
pub fn f() -> u32 { 0 }
