use id::Id;
use std::marker::PhantomData;

pub trait HKT: Sized + 'static {}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Kind<K: HKT, A> {
    Vec(Vec<A>),
    Option(Option<A>),
    Id(Id<A>),
    // Is this valid? also need to understand which pointer type to use here
    __MARKER(PhantomData<*const K>),
}

pub trait Reify<K: HKT, A> {
    type Out;
    fn reify(self) -> Self::Out;
}

pub trait IntoKind<K: HKT, T> {
    type Kind: HKT;
    fn into_kind(self) -> Kind<K, T>;
}
