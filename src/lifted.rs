use data::kleisli::Kleisli;
use data::option_t::OptionT;
use futures::Future;
use std::marker::PhantomData;

pub trait HKT {}

pub struct Nothing {}
impl Iterator for Nothing {
    type Item = Nothing;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unimplemented!()
    }
}

pub enum Lifted<
    'a,
    F,           // The HKT of this Lifted
    A,           // The type of the first parameter of F
    B = Nothing, // The type of a second optional parameter to F
    G = Nothing, // The type of an optional nested HKT, G
> where
    F: 'a,
    A: 'a,
    B: 'a,
    G: 'a,
{
    Option(Option<A>),
    OptionRef(Option<&'a A>),
    OptionMut(Option<&'a mut A>),

    Result(Result<A, B>),
    OptionT(Box<OptionT<'a, G, A, B>>),
    Kleisli(Kleisli<'a, F, A, B, G>),
    Iterator(Box<Iterator<Item = A> + 'a>),

    Future(Box<Future<Item = A, Error = B> + 'a>),
    __Marker(F),
}

pub trait Lift<'a, F, A, Z = Nothing, G = Nothing> {
    fn lift(self) -> Lifted<'a, F, A, Z, G>;
}

pub trait LiftAsRef<'a, F, A, Z = Nothing, G = Nothing> {
    fn lift_as_ref(&self) -> Lifted<'a, F, A, Z, G>;
}

pub trait LiftAsMut<'a, F, A, Z = Nothing, G = Nothing> {
    fn lift_as_mut(&mut self) -> Lifted<'a, F, A, Z, G>;
}

pub trait Unlift<F> {
    type Out;
    fn unlift(self) -> Self::Out;
}

pub trait UnliftAsRef<F> {
    type Out;
    fn unlift(&self) -> Self::Out;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    #[test]
    fn test_lifted_size() {
        let s = Some("foo".to_owned()).lift();
        let s2 = Some("foo".to_owned());
        assert_eq!(mem::size_of_val(&s), mem::size_of_val(&s2) + 8)
    }
}
