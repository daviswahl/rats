use data::kleisli::Kleisli;
use data::option_t::OptionT;
use std::marker::PhantomData;
pub trait HKT {}

pub struct Nothing {}

pub enum Lifted<F, A, B = Nothing, G = Nothing, Func = Nothing> {
    Option(Option<A>),
    Result(Result<A, B>),
    OptionT(OptionT<G, A, B>),
    Kleisli(Kleisli<F, A, B, Func>),
    __marker(F),
}

pub trait Lift<F, A> {
    fn lift(&self) -> Lifted<F, A>;
}

pub trait Unlift<F> {
    type Out;
    fn unlift(&self) -> Self::Out;
}
