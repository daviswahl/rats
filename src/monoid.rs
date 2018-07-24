use semigroup::Semigroup;

pub trait Monoid<A>: Semigroup<A> {
    fn empty() -> A;
}
