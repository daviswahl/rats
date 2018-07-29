#![feature(test)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(tool_attributes)]
#![feature(futures_api)]
extern crate futures;
extern crate test;
pub mod applicative;
pub mod data;
pub mod foldable;
pub mod functor;
pub mod instances;
pub mod lifted;
pub mod monad;
pub mod monoid;
pub mod semigroup;
pub mod traverse;
// pub mod trampoline;
// TODO: Put this somewhere.
pub fn identity<A>(a: A) -> A {
    a
}
