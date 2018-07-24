use data::kleisli::Kleisli;
use data::option_t::OptionT;
use std::marker::PhantomData;
pub trait HKT {}

pub struct Nothing {}

pub enum Lifted<F, A, B = Nothing, G = Nothing> {
    Option(Option<A>),
    Result(Result<A, B>),
    OptionT(OptionT<G, A, B>),
    Kleisli(Kleisli<F, A, B, G>),
    Iterator(G),
    __marker(F),
}

pub trait Lift<F, A, B = Nothing, G = Nothing> {
    fn lift(self) -> Lifted<F, A, B, G>;
}

pub trait Unlift<F> {
    type Out;
    fn unlift(self) -> Self::Out;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    #[test]
    fn test_lifted_size() {
        let s = Some("foo").lift();
        let s2 = Some("foo");
        assert_eq!(mem::size_of_val(&s), mem::size_of_val(&s2) + 8)
    }
}
