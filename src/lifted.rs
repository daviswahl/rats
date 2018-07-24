use std::marker::PhantomData;
pub trait HKT {}

pub struct Nothing {}

pub enum Lifted<F,A,B=Nothing,C=Nothing, G=Nothing> {
    Option(Option<A>),
    Result(Result<A,B>),
    OptionT(),
    __marker(F)
}