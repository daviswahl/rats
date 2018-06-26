#![feature(associated_type_defaults)]
#![feature(allocator_api)]
#![feature(ptr_internals)]
#![feature(alloc)]
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

#[cfg(test)]
mod tests {}
