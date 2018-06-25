#![feature(alloc)]
#![feature(associated_type_defaults)]
#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(test)]

extern crate alloc;
extern crate core;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate bincode;
extern crate test;

pub mod v1;
pub mod v2;
pub use v1::*;
pub use v2::*;

pub fn foo() -> i32 {
    let i = 5;
}

#[cfg(test)]
mod tests {}
