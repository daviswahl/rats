use data::kleisli::Kleisli;
use data::option_t::OptionT;
use either::Either;
use futures::future::LocalFutureObj;
use std::collections::VecDeque;
use std::ops::Deref;
use std::ops::DerefMut;

pub trait HKT: 'static {}

pub struct Nothing {}
impl Iterator for Nothing {
    type Item = Nothing;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unreachable!()
    }
}

/// Lifted represents a
pub enum Lifted<
    'a,
    F,           // The HKT of this Lifted
    A,           // The type of the first parameter of F
    B = Nothing, // The type of a second optional parameter to F
    G = Nothing, // The type of an optional nested HKT, G
> where
    F: 'static,
    G: 'static,
    A: 'a,
    B: 'a,
{
    Option(Option<A>),
    VecDeque(VecDeque<A>),

    Result(Result<A, B>),
    Either(Either<A, B>),

    // Implementing OptionT as an HKT causes drop-check recursion
    //    OptionT(Box<OptionT<'a, G, A, B>>),
    Kleisli(Box<dyn Kleisli<'a, F, A, B, G>>),
    Iterator(Box<dyn Iterator<Item = A> + 'a>),
    Future(LocalFutureObj<'a, A>),

    __MarkerF(*const F),
    __MarkerG(*const G),
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

pub trait UnliftRef<F>: Unlift<F> {
    fn unlift_as_ref(&self) -> &Self::Out;
}

pub trait UnliftMut<F>: Unlift<F> {
    fn unlift_mut(&mut self) -> &mut Self::Out;
}

impl<'a, A, B, G, F> Deref for Lifted<'a, F, A, B, G>
where
    Lifted<'a, F, A, B, G>: UnliftRef<F>,
{
    type Target = <Self as Unlift<F>>::Out;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.unlift_as_ref()
    }
}

impl<'a, A, B, G, F> DerefMut for Lifted<'a, F, A, B, G>
where
    Lifted<'a, F, A, B, G>: UnliftMut<F> + Deref<Target = <Self as Unlift<F>>::Out>,
{
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        self.unlift_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    #[test]
    fn test_lifted_size() {
        let s = Some("foo").lift();
        let s2 = Some("foo");
        assert_eq!(mem::size_of_val(&s), mem::size_of_val(&s2) + 24) // yikes!
    }

    #[test]
    fn deref_test() {
        let s = Some("foo".to_owned()).lift();
        let _r = &*s;
    }
}
