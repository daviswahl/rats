#![feature(test)]
#![feature(tool_attributes)]
extern crate core;
extern crate futures;
extern crate test;
pub mod data;
pub mod functor;
pub mod instances;
pub mod lifted;
pub mod monoid;
pub mod semigroup;
use instances::*;
// TODO: Put this somewhere.
pub fn identity<A>(a: A) -> A {
    a
}
