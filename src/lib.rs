#![feature(alloc)]
#![feature(associated_type_defaults)]
#![feature(ptr_internals)]
#![feature(allocator_api)]
extern crate alloc;
extern crate core;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate bincode;

pub mod functor;
pub mod kind;
pub mod kinds;
pub mod context;
#[cfg(test)]
mod test {
    use super::*;
}
