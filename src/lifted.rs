use data::future_a::FutureA;
use data::kleisli::Kleisli;
use data::option_t::OptionT;
use std::future::Future;
use std::marker::PhantomData;

pub trait HKT {}

pub struct Nothing {}
impl Iterator for Nothing {
    type Item = Nothing;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unimplemented!()
    }
}

pub enum Lifted<'a, F, A, B = Nothing, G = Nothing>
where
    F: 'a,
    A: 'a,
    B: 'a,
    G: 'a,
{
    Option(Option<A>),
    Result(Result<A, B>),
    OptionT(Box<OptionT<'a, G, A, B>>),
    Kleisli(Kleisli<'a, F, A, B, G>),
    Iterator(Box<Iterator<Item = A> + 'a>),
    Future(FutureA<A>),
    __marker(F),
}

pub trait Lift<'a, F, A, Z = Nothing, G = Nothing> {
    fn lift(self) -> Lifted<'a, F, A, Z, G>;
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
