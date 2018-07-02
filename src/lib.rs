#![feature(test)]
#![feature(associated_type_defaults)]
#![feature(specialization)]
#![feature(tool_attributes)]

extern crate futures;
extern crate test;

pub mod applicative;
pub mod data;
pub mod foldable;
pub mod function_k;
pub mod functor;
pub mod instances;
pub mod kind;
pub mod kinds;
pub mod traverse;

mod tests;
mod scratch;
mod monad;


// TODO: Put this somewhere.
pub fn identity<A>(a: A) -> A {
    a
}
