#![feature(test)]
#![feature(associated_type_defaults)]
//#![feature(specialization)]
#![feature(tool_attributes)]
//#![feature(generators, generator_trait)]
//#![feature(async_await, futures)]
//#![feature(gen_future)]
//#![feature(proc_macro)]
//#![feature(never_type)]
#![feature(
    async_await,
    futures_api,
    await_macro,
    pin,
    arbitrary_self_types
)]
#![feature(fn_traits, unboxed_closures)]
#![feature(generator_trait, generators)]
//extern crate futures;
extern crate core;
extern crate futures;
extern crate test;
#[allow(non_camel_case_types)]
//pub mod applicative;
//pub mod v1.data;
//pub mod foldable;
//pub mod function_k;
//pub mod functor;
//pub mod v1.instances;
//pub mod kind;
//pub mod v1.kinds;
//pub mod traverse;
//
//mod tests;
////mod scratch;
//mod monad;
mod v2;
//use sandbox::lifted4;

// TODO: Put this somewhere.
pub fn identity<A>(a: A) -> A {
    a
}
